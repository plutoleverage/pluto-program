use anchor_lang::{Discriminator};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::EventLeverageBorrow;
use crate::state::{EarnConfig, InitObligationParams, InitPositionParams, LeverageConfig, Obligation, Position, PositionSettings, Protocol, Stats, VaultEarn, VaultLeverage};
use crate::util::{constant, decimals, seeds, transfer_token::transfer_token};
use crate::util::transfer_token::transfer_token_with_signer;
use crate::util::constant::{UNIT_DECIMALS, PERCENT_DECIMALS, LEVERAGE_ONE, INDEX_DECIMALS, MAX_ORACLE_AGE, LEVERAGE_MAX_SAFETY, PERCENT_MAX};

#[inline(never)]
pub fn handle(ctx: Context<VaultLeverageFund>, settings: PositionSettings, amount: u64, leverage: u32) -> Result<()> {
    require!(amount > 0, ErrorLeverage::InvalidAmount);
    require_gt!(leverage, 0, ErrorLeverage::InvalidLeverage);
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let config = &ctx.accounts.leverage_config.load()?;
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let obligation = &mut if ctx.accounts.obligation.load().is_ok() {
        ctx.accounts.obligation.load_mut()?
    } else {
        ctx.accounts.obligation.load_init()?
    };

    require!(!config.freeze, ErrorLeverage::VaultFrozen);

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("vault config address: {:?}", ctx.accounts.leverage_config.key());
    msg!("leverage fee vault address: {:?}", ctx.accounts.leverage_fee_vault.key());
    msg!("obligation address: {:?}", ctx.accounts.obligation.key());

    msg!("min_leverage: {:?}", config.min_leverage);
    msg!("max_leverage: {:?}", config.max_leverage);
    msg!("min_leverage_limit: {:?}", config.min_leverage_limit);
    msg!("max_leverage_limit: {:?}", config.max_leverage_limit);
    msg!("leverage_fee: {:?}", config.leverage_fee);

    require_gte!(leverage, config.min_leverage, ErrorLeverage::InvalidLeverage);
    require_gte!(config.max_leverage, leverage, ErrorLeverage::InvalidLeverage);
    if settings.profit_target_rate > 0 {
        require_gt!(settings.profit_taking_rate, 0, ErrorLeverage::InvalidProfitTakingRate);
    }
    require_gte!(settings.profit_target_rate, settings.profit_taking_rate, ErrorLeverage::ProfitTakingRateMustBeLessThanProfitTargetRate);

    if !obligation.is_initialized {
        msg!("obligation is created");
        obligation.init(InitObligationParams {
            bump: ctx.bumps.obligation,
            owner: *ctx.accounts.user.key,
            protocol: ctx.accounts.protocol.key(),
            vault: *ctx.accounts.vault.to_account_info().key,
        })?;

        let stats = &mut ctx.accounts.leverage_stats.load_mut()?;
        stats.add_user()?;
    }else{
        obligation.update_time()?;
    }

    let mut leverage_fee_amount = 0u64;
    if config.leverage_fee > 0 {
        let fee = decimals::mul_ceil(vault.token_collateral_token_decimal, amount as u128, vault.token_collateral_token_decimal, config.leverage_fee as u128, PERCENT_DECIMALS)?;
        leverage_fee_amount = decimals::div_ceil(vault.token_collateral_token_decimal, fee, vault.token_collateral_token_decimal, 100, 0)? as u64;
        if leverage_fee_amount > 0 {
            transfer_token(
                ctx.accounts.user_ata.to_account_info(),
                ctx.accounts.leverage_fee_vault.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.token_collateral_token_program.to_account_info(),
                ctx.accounts.token_collateral_token_mint.to_account_info(),
                leverage_fee_amount,
                ctx.accounts.token_collateral_token_mint.decimals
            )?;
        }
    }

    let amount_after_fee = amount.checked_sub(leverage_fee_amount).ok_or(MathOverflow)?;

    msg!("leverage_fee_amount: {:?}", leverage_fee_amount);
    msg!("amount_after_fee: {:?}", amount_after_fee);

    let position_id = obligation.generate_id()?;
    let position = obligation.find_or_add_position(position_id, |position| {
        position.init(InitPositionParams {
            owner: *ctx.accounts.user.key,
            id: position_id,
            tag_id: [0; 64],
        })
    })?;

    position.safety_mode = settings.safety_mode;
    position.emergency_eject = settings.emergency_eject;
    position.profit_taker = settings.profit_taker;
    if position.profit_taker {
        position.profit_target_rate = settings.profit_target_rate;
        position.profit_taking_rate = settings.profit_taking_rate;
    }

    if leverage > LEVERAGE_MAX_SAFETY {
        position.safety_mode = false;
    }

    position.fund(amount, leverage_fee_amount)?;

    // BORROW
    borrow(&ctx, vault, config, position, leverage)?;

    let leveraged_amount = position.state.fund_amount.checked_add(position.state.borrow_amount).ok_or(MathOverflow)?;
    let min_native_collateral_output = calculate_min_native_collateral_output(&ctx, vault, config, position)?;
    msg!("leveraged_amount: {:?}", leveraged_amount);
    msg!("min_native_collateral_output: {:?}", min_native_collateral_output);

    position.take_fund(vault.token_collateral_token_decimal)?;
    position.leverage(leveraged_amount, min_native_collateral_output)?;

    Ok(())
}

#[inline(never)]
fn verify_next_ixs(ctx: &Context<VaultLeverageFund>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();
    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    // loop through instructions, looking for an equivalent repay to this borrow
    let mut index = current_index + 1; // jupiter swap
    loop {
        // get the next instruction, die if theres no more
        if let Ok(ix) = load_instruction_at_checked(index, &ixs) {
            if ix.program_id == crate::id() {
                let ix_discriminator: [u8; 8] = ix.data[0..8]
                    .try_into()
                    .map_err(|_| Errors::UnknownInstruction)?;

                // check if we have a toplevel stake toward authority
                if ix_discriminator == crate::instruction::LeverageVaultConfiscate::discriminator() {
                    break;
                } else {
                    return Err(Errors::UnknownInstruction.into());
                }
            }
        } else {
            // no more instructions, so we're missing a stake
            return Err(ErrorLeverage::MissingBorrow.into());
        }

        index += 1
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultLeverageFund>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[inline(never)]
fn borrow(
    ctx: &Context<VaultLeverageFund>,
    vault: &mut VaultLeverage,
    config: &LeverageConfig,
    position: &mut Position,
    leverage: u32,
) -> Result<()> {
    let borrow_vault_config = &ctx.accounts.earn_config.load()?;
    let borrow_vault = &mut ctx.accounts.borrow_vault.load_mut()?;

    msg!("borrow_vault address: {:?}", ctx.accounts.borrow_vault.key());
    msg!("borrow_vault config address: {:?}", ctx.accounts.earn_config.key());
    msg!("borrow_fee_vault address: {:?}", ctx.accounts.earn_fee_vault.key());
    msg!("borrow_fee: {:?}", borrow_vault_config.borrow_fee);

    let old_borrowing_amount = decimals::mul_ceil(
        vault.token_collateral_token_decimal, position.borrowing_unit as u128, UNIT_DECIMALS,
        position.avg_borrowing_index, INDEX_DECIMALS
    )?;
    let old_borrowing_unit = position.borrowing_unit;
    let old_borrowing_index = position.avg_borrowing_index;

    let amount = position.state.fund_amount;

    require_gte!(amount as u128, config.min_leverage_limit as u128, ErrorLeverage::InvalidAmount);
    require_gte!(config.max_leverage_limit as u128, amount as u128, ErrorLeverage::InvalidAmount);

    let borrowing_multiplier = leverage.checked_sub(LEVERAGE_ONE).ok_or(MathOverflow)?;

    msg!("borrowing_multiplier: {:?}", borrowing_multiplier);

    // Floor to prevent rounding overflow the borrow reserve calculation
    let borrowing_amount = decimals::mul(ctx.accounts.token_collateral_token_mint.decimals, amount as u128, ctx.accounts.token_collateral_token_mint.decimals, borrowing_multiplier as u128, PERCENT_DECIMALS)? as u64;
    let borrowing_unit = decimals::div_ceil(UNIT_DECIMALS, borrowing_amount as u128, ctx.accounts.token_collateral_token_mint.decimals, vault.borrowing_index, INDEX_DECIMALS)? as u64;

    msg!("borrowing_amount: {:?}", borrowing_amount);

    if (borrow_vault.borrow_available_amount(borrow_vault_config).unwrap() as u64) < borrowing_amount {
        return Err(ErrorLeverage::InsufficientBorrowableAmount.into());
    }

    if ctx.accounts.borrow_vault_liquidity.amount < borrowing_amount {
        msg!("Insufficient liquidity to borrow");
        msg!("Earn vault available amount: {} of {}", amount, borrow_vault.borrow_available_amount(borrow_vault_config).unwrap());
        return Err(ErrorLeverage::InsufficientLiquidity.into());
    }

    borrow_vault.leverage(borrowing_amount)?;

    // Preparing signer seeds for borrow vault
    let vault_key = ctx.accounts.borrow_vault.key();
    let borrow_vault_seeds = &[
        seeds::VAULT_EARN_AUTH,
        vault_key.as_ref(),
        &[ctx.bumps.borrow_vault_authority],
    ];

    let borrow_vault_signer_seeds = &[&borrow_vault_seeds[..]];

    let mut borrowing_fee_amount = 0u64;

    msg!("borrow_fee: {:?}", borrow_vault_config.borrow_fee);
    msg!("borrowing_amount: {:?}", borrowing_amount);
    msg!("borrowing_unit: {:?}", borrowing_unit);

    if borrow_vault_config.borrow_fee > 0 {
        let fee = decimals::mul_ceil(vault.token_collateral_token_decimal, borrowing_amount as u128, vault.token_collateral_token_decimal, borrow_vault_config.borrow_fee as u128, PERCENT_DECIMALS)?;
        borrowing_fee_amount = decimals::div_ceil(vault.token_collateral_token_decimal, fee, vault.token_collateral_token_decimal, 100, 0)? as u64;
        if borrowing_fee_amount > 0 {
            transfer_token_with_signer(
                ctx.accounts.borrow_vault_liquidity.to_account_info(),
                ctx.accounts.earn_fee_vault.to_account_info(),
                ctx.accounts.borrow_vault_authority.to_account_info(),
                ctx.accounts.token_collateral_token_program.to_account_info(),
                ctx.accounts.token_collateral_token_mint.to_account_info(),
                borrowing_fee_amount,
                ctx.accounts.token_collateral_token_mint.decimals,
                borrow_vault_signer_seeds
            )?;
        }
    }

    let borrowing_amount_after_fee = borrowing_amount.checked_sub(borrowing_fee_amount).ok_or(MathOverflow)?;

    msg!("borrowing_fee_amount: {:?}", borrowing_fee_amount);
    msg!("borrowing_amount_after_fee: {:?}", borrowing_amount_after_fee);

    transfer_token_with_signer(
        ctx.accounts.borrow_vault_liquidity.to_account_info(),
        ctx.accounts.user_ata.to_account_info(),
        ctx.accounts.borrow_vault_authority.to_account_info(),
        ctx.accounts.token_collateral_token_program.to_account_info(),
        ctx.accounts.token_collateral_token_mint.to_account_info(),
        borrowing_amount_after_fee,
        ctx.accounts.token_collateral_token_mint.decimals,
        borrow_vault_signer_seeds
    )?;

    let leveraged_amount = amount.checked_add(borrowing_amount_after_fee).ok_or(MathOverflow)?;

    msg!("leveraged_amount: {:?}", leveraged_amount);

    position.borrow_fund(borrowing_amount, borrowing_unit, vault.borrowing_index, borrowing_fee_amount)?;
    vault.mint_borrow(borrowing_unit)?;

    emit!(EventLeverageBorrow{
        borrow_vault: *ctx.accounts.borrow_vault.to_account_info().key,
        vault: *ctx.accounts.vault.to_account_info().key,
        user: *ctx.accounts.user.to_account_info().key,
        obligation: ctx.accounts.obligation.key(),
        position_number: position.number as u8,
        token_collateral_price_oracle: vault.token_collateral_price_oracle,
        token_collateral_price_feed: vault.token_collateral_price_feed,
        token_collateral_token_mint: *ctx.accounts.token_collateral_token_mint.to_account_info().key,
        token_collateral_token_decimals: vault.token_collateral_token_decimal,
        native_collateral_price_oracle: vault.native_collateral_price_oracle,
        native_collateral_price_feed: vault.native_collateral_price_feed,
        native_collateral_token_mint: *ctx.accounts.native_collateral_token_mint.to_account_info().key,
        native_collateral_token_decimals: vault.native_collateral_token_decimal,
        leveraged_amount,
        old_borrowing_amount: old_borrowing_amount as u64,
        old_borrowing_unit,
        old_borrowing_index,
        borrowing_amount,
        borrowing_unit: position.borrowing_unit,
        borrowing_index: position.avg_borrowing_index,
        borrow_fee_vault: ctx.accounts.earn_fee_vault.key(),
        borrow_fee: borrow_vault_config.borrow_fee,
        borrow_fee_amount: borrowing_fee_amount,
    });

    Ok(())
}

#[inline(never)]
fn calculate_min_native_collateral_output(
    ctx: &Context<VaultLeverageFund>,
    vault: &VaultLeverage,
    config: &LeverageConfig,
    position: &mut Position,
) -> Result<u64> {
    let clock = Clock::get()?;

    let token_collateral_price_oracle = ctx.accounts.token_collateral_price_oracle.get_price_no_older_than(&clock, MAX_ORACLE_AGE, &get_feed_id_from_hex(&String::from_utf8(vault.token_collateral_price_feed.to_vec()).unwrap())?)?;
    let native_collateral_price_oracle = ctx.accounts.native_collateral_price_oracle.get_price_no_older_than(&clock, MAX_ORACLE_AGE, &get_feed_id_from_hex(&String::from_utf8(vault.native_collateral_price_feed.to_vec()).unwrap())?)?;

    let leveraged_usd = decimals::mul_floor(token_collateral_price_oracle.exponent.abs() as u8, position.state.fund_amount.checked_add(position.state.borrow_amount).ok_or(MathOverflow)? as u128, vault.token_collateral_token_decimal, token_collateral_price_oracle.price as u128, token_collateral_price_oracle.exponent.abs() as u8)? as u64;
    let mut min_native_collateral_output = decimals::div_ceil(vault.native_collateral_token_decimal, leveraged_usd as u128, token_collateral_price_oracle.exponent.abs() as u8, native_collateral_price_oracle.price as u128, native_collateral_price_oracle.exponent.abs() as u8)? as u64;
    min_native_collateral_output = decimals::mul_ceil(vault.native_collateral_token_decimal, min_native_collateral_output as u128, vault.native_collateral_token_decimal, PERCENT_MAX.checked_sub(config.slippage_rate).ok_or(MathOverflow)? as u128, PERCENT_DECIMALS)? as u64;
    min_native_collateral_output = decimals::div_ceil(vault.native_collateral_token_decimal, min_native_collateral_output as u128, vault.native_collateral_token_decimal, 100, 0)? as u64;


    msg!("token_collateral_price: {:?}", token_collateral_price_oracle.price);
    msg!("token_collateral_price_exponent: {:?}", token_collateral_price_oracle.exponent);
    msg!("native_collateral_price: {:?}", native_collateral_price_oracle.price);
    msg!("native_collateral_price_exponent: {:?}", native_collateral_price_oracle.exponent);
    msg!("leveraged_usd: {:?}", leveraged_usd);
    msg!("min_native_collateral_output: {:?}", min_native_collateral_output);

    msg!("pending_min_native_collateral_output: {:?}", min_native_collateral_output);

    Ok(min_native_collateral_output)
}

#[derive(Accounts)]
pub struct VaultLeverageFund<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        has_one = leverage_fee_vault,
    )]
    pub leverage_config: AccountLoader<'info, LeverageConfig>,
    /// CHECK Safe
    #[account(mut)]
    pub leverage_fee_vault: AccountInfo<'info>,
    #[account(
        has_one = earn_fee_vault,
    )]
    pub earn_config: AccountLoader<'info, EarnConfig>,
    /// CHECK Safe
    #[account(mut)]
    pub earn_fee_vault: AccountInfo<'info>,

    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = leverage_config @ Errors::InvalidConfig,
        has_one = borrow_vault,
        has_one = leverage_stats,
        has_one = token_collateral_token_program,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_program,
        has_one = native_collateral_token_mint,
        has_one = token_collateral_price_oracle,
        has_one = native_collateral_price_oracle,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,
    #[account(mut)]
    pub leverage_stats: AccountLoader<'info, Stats>,

    /// CHECK VAULT FOR BORROWING AUTHORITY
    #[account(
        seeds = [seeds::VAULT_EARN_AUTH, borrow_vault.key().as_ref()],
        bump,
    )]
    pub borrow_vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = earn_config @ Errors::InvalidConfig,
    )]
    pub borrow_vault: AccountLoader<'info, VaultEarn>,

    #[account(
        init_if_needed,
        seeds = [seeds::OBLIGATION, vault.key().as_ref(), token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = Obligation::INIT_SPACE+8+8+(15 * 8)
    )]
    pub obligation: AccountLoader<'info, Obligation>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::token_program = token_collateral_token_program,
        associated_token::mint = token_collateral_token_mint,
        associated_token::authority = borrow_vault_authority,
    )]
    pub borrow_vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = token_collateral_token_program,
        associated_token::mint = token_collateral_token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_collateral_token_program,
    )]
    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = native_collateral_token_program,
    )]
    pub native_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub native_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,

    /// CHECK: check instructions account
    #[account(address = sysvar::instructions::ID @Errors::InvalidAddress)]
    pub instructions: UncheckedAccount<'info>,

    pub token_collateral_token_program: Interface<'info, TokenInterface>,
    pub native_collateral_token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}