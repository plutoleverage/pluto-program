use anchor_lang::Discriminator;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::{EventLeverageClose};
use crate::state::{EarnConfig, LeverageConfig, Obligation, Position, Protocol, VaultEarn, VaultLeverage};
use crate::util::{constant, decimals, seeds};
use crate::util::action::LeverageAction;
use crate::util::transfer_token::transfer_token_with_signer;
use crate::util::constant::{INDEX_DECIMALS, MAX_OBLIGATION_POSITIONS, MAX_ORACLE_AGE, PERCENT_DECIMALS, PERCENT_MAX, UNIT_DECIMALS};

pub fn handle(ctx: Context<VaultLeverageClose>, number: u8) -> Result<()> {
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;
    let config = &ctx.accounts.leverage_config.load()?;
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let obligation = &mut ctx.accounts.obligation.load_mut()?;
    let user = &mut ctx.accounts.user;

    require!(!config.freeze, ErrorLeverage::VaultFrozen);

    let clock = &Clock::get()?;

    require!(number < MAX_OBLIGATION_POSITIONS, ErrorLeverage::InvalidPositionNumber);

    let position = &mut obligation.positions[number as usize];

    require_gt!(position.unit, 0, ErrorLeverage::NoPositionFound);

    let tctp = &mut ctx.accounts.token_collateral_price_oracle;
    let nctp = &mut ctx.accounts.native_collateral_price_oracle;
    let tcpf = &String::from_utf8(vault.token_collateral_price_feed.to_vec()).unwrap();
    let ncpf = &String::from_utf8(vault.native_collateral_price_feed.to_vec()).unwrap();

    msg!("token_collaral_price_feed: {:?}", tcpf);
    msg!("native_collaral_price_feed: {:?}", ncpf);

    let token_collateral_token_price = tctp.get_price_no_older_than(clock, MAX_ORACLE_AGE, &get_feed_id_from_hex(tcpf)?)?;
    let native_collateral_token_price = nctp.get_price_no_older_than(clock, MAX_ORACLE_AGE, &get_feed_id_from_hex(ncpf)?)?;

    msg!("token_collateral_token_price: {} {}", token_collateral_token_price.price, token_collateral_token_price.exponent.abs());
    msg!("native_collateral_token_price: {} {}", native_collateral_token_price.price, native_collateral_token_price.exponent.abs());

    position.set_action(LeverageAction::Close)?;
    position.set_config(config)?;
    position.set_oracle(
        vault.token_collateral_price_oracle,
        vault.token_collateral_price_feed,
        token_collateral_token_price.price as u64,
        token_collateral_token_price.exponent.abs() as u32,
        vault.native_collateral_price_oracle,
        vault.native_collateral_price_feed,
        native_collateral_token_price.price as u64,
        native_collateral_token_price.exponent.abs() as u32,
    )?;

    let release_amount = decimals::mul_floor(vault.native_collateral_token_decimal, position.unit as u128, UNIT_DECIMALS, vault.index, INDEX_DECIMALS)? as u64;
    let release_amount_usd = decimals::mul_floor(
        position.state.native_collateral_price_exponent as u8,
        release_amount as u128, vault.native_collateral_token_decimal,
        position.state.native_collateral_price as u128, position.state.native_collateral_price_exponent as u8,
    )? as u64;
    let mut release_min_output = decimals::div_ceil(
        vault.token_collateral_token_decimal,
        release_amount_usd as u128, position.state.native_collateral_price_exponent as u8,
        position.state.token_collateral_price as u128, position.state.token_collateral_price_exponent as u8,
    )? as u64;
    release_min_output = decimals::mul_ceil(vault.token_collateral_token_decimal, release_min_output as u128, vault.token_collateral_token_decimal, PERCENT_MAX.saturating_sub(position.state.slippage_rate) as u128, PERCENT_DECIMALS)? as u64;
    release_min_output = decimals::div_ceil(vault.token_collateral_token_decimal, release_min_output as u128, vault.token_collateral_token_decimal, 100, 0)? as u64;

    msg!("release_amount: {}", release_amount);
    msg!("release_amount_usd: {}", release_amount_usd);
    msg!("release_min_output: {}", release_min_output);

    let repay_amount = decimals::mul_ceil(vault.token_collateral_token_decimal, position.borrowing_unit as u128, UNIT_DECIMALS, vault.borrowing_index, INDEX_DECIMALS)? as u64;

    msg!("borrowing_unit: {}", position.borrowing_unit);
    msg!("borrowing_index: {}", vault.borrowing_index);
    msg!("repay_amount: {}", repay_amount);

    position.release(
        release_amount,
        position.unit,
        vault.index,
        PERCENT_MAX,
        repay_amount,
        position.borrowing_unit,
        vault.borrowing_index,
        release_min_output,
    )?;

    Ok(())
}

#[inline(never)]
fn verify_next_ixs(ctx: &Context<VaultLeverageClose>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    // loop through instructions, looking for an equivalent repay to this borrow
    let mut index = current_index + 1;
    loop {
        // get the next instruction, die if theres no more
        if let Ok(ix) = load_instruction_at_checked(index, &ixs) {
            if ix.program_id == crate::id() {
                let ix_discriminator: [u8; 8] = ix.data[0..8]
                    .try_into()
                    .map_err(|_| Errors::UnknownInstruction)?;

                // check if we have a toplevel stake toward authority
                if ix_discriminator == crate::instruction::LeverageVaultRelease::discriminator() {
                    break;
                } else {
                    return Err(ErrorLeverage::NextInstructionMustBeRelease.into());
                }
            }
        } else {
            // no more instructions, so we're missing a stake
            return Err(ErrorLeverage::MissingRelease.into());
        }

        index += 1
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultLeverageClose>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageClose<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account()]
    pub leverage_config: AccountLoader<'info, LeverageConfig>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = leverage_config @ Errors::InvalidConfig,
        has_one = token_collateral_token_program,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_program,
        has_one = native_collateral_token_mint,
        has_one = token_collateral_price_oracle @ Errors::InvalidPriceOracle,
        has_one = native_collateral_price_oracle @ Errors::InvalidPriceOracle,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,

    #[account(
        mut,
        seeds = [seeds::OBLIGATION, vault.key().as_ref(), token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub obligation: AccountLoader<'info, Obligation>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = token_collateral_token_program,
    )]
    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub native_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = native_collateral_token_program,
    )]
    pub native_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: check instructions account
    #[account(address = sysvar::instructions::ID @Errors::InvalidAddress)]
    pub instructions: UncheckedAccount<'info>,

    pub token_collateral_token_program: Interface<'info, TokenInterface>,
    pub native_collateral_token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}