use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{close_account, CloseAccount, TokenInterface, Mint, TokenAccount};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::{EventLeverageOpen};
use crate::state::{LeverageConfig, Obligation, Protocol, VaultLeverage};
use crate::util::{decimals, seeds, transfer_token::transfer_token};
use crate::util::constant::{PERCENT_DECIMALS, INDEX_DECIMALS, UNIT_DECIMALS, PERCENT_MAX};

pub fn handle(ctx: Context<VaultLeverageConfiscate>) -> Result<()> {
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let config = &ctx.accounts.leverage_config.load()?;
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let obligation = &mut ctx.accounts.obligation.load_mut()?;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("vault config address: {:?}", ctx.accounts.leverage_config.key());
    msg!("obligation address: {:?}", ctx.accounts.obligation.key());

    msg!("vault index: {:?}", vault.index);

    let position = obligation.find_pending_leveraged_position()?;

    let old_unit = position.unit;
    let old_index = position.avg_index;

    let leveraged_amount = position.state.leveraged_amount;
    let min_native_collateral_output = position.state.min_native_collateral_output;

    let clock = Clock::get()?;

    msg!("pending_leveraged_amount: {:?}", leveraged_amount);
    msg!("pending_min_native_collateral_output: {:?}", min_native_collateral_output);

    let mut fair_native_collateral_output = decimals::mul_ceil(vault.native_collateral_token_decimal, min_native_collateral_output as u128, vault.native_collateral_token_decimal, 100, 0)? as u64;
    fair_native_collateral_output = decimals::div_ceil(vault.native_collateral_token_decimal, fair_native_collateral_output as u128, vault.native_collateral_token_decimal, PERCENT_MAX.checked_sub(config.slippage_rate).ok_or(MathOverflow)? as u128, PERCENT_DECIMALS)? as u64;

    msg!("slippage_rate: {:?}", config.slippage_rate);
    msg!("user ata amount: {:?}", ctx.accounts.user_ata.amount);
    msg!("fair_native_collateral_output: {:?}", fair_native_collateral_output);

    let mut taking_amount = 0u64;

    if ctx.accounts.user_ata.amount < min_native_collateral_output {
        return Err(ErrorLeverage::SlippageReached.into());
    } else if ctx.accounts.user_ata.amount >= fair_native_collateral_output {
        taking_amount = fair_native_collateral_output;
    } else {
        taking_amount = ctx.accounts.user_ata.amount;
    }

    // SEND BACK TO VAULT AFTER SWAP
    transfer_token(
        ctx.accounts.user_ata.to_account_info(),
        ctx.accounts.native_collateral_vault_liquidity.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.native_collateral_token_program.to_account_info(),
        ctx.accounts.native_collateral_token_mint.to_account_info(),
        taking_amount,
        ctx.accounts.native_collateral_token_mint.decimals,
    )?;

    let token_to_collateral_ratio = decimals::div_ceil(INDEX_DECIMALS, taking_amount as u128, vault.native_collateral_token_decimal, position.state.leveraged_amount as u128, vault.token_collateral_token_decimal)?;
    let unit = decimals::div_ceil(UNIT_DECIMALS, taking_amount as u128, vault.native_collateral_token_decimal, vault.index, INDEX_DECIMALS)? as u64;

    position.confiscate(vault.native_collateral_token_decimal, token_to_collateral_ratio, unit, vault.index)?;
    vault.mint(unit)?;

    msg!("unit: {:?} index: {:?} borrowing_unit: {:?} borrowing_index: {:?}", position.unit, position.avg_index, position.borrowing_unit, position.avg_borrowing_index);

    emit!(EventLeverageOpen{
        borrow_vault: vault.borrow_vault,
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
        min_native_collateral_output,
        real_native_collateral_output: taking_amount,
        unit,
        index: vault.index,
    });

    if ctx.accounts.user_ata.amount == 0 {
        close_user_ata(&ctx)?;
    }

    Ok(())
}

fn verify_next_ixs(ctx: &Context<VaultLeverageConfiscate>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultLeverageConfiscate>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

fn close_user_ata(ctx: &Context<VaultLeverageConfiscate>) -> Result<()> {
    let cpi_program = ctx.accounts.native_collateral_token_program.to_account_info();

    let cpi_accounts = CloseAccount {
        account: ctx.accounts.user_ata.to_account_info().clone(),
        destination: ctx.accounts.user.to_account_info().clone(),
        authority: ctx.accounts.user.to_account_info().clone(),
    };

    let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);

    close_account(cpi_ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageConfiscate<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account()]
    pub leverage_config: AccountLoader<'info, LeverageConfig>,

    /// CHECK VAULT LEVERAGE AUTHORITY
    #[account(
        seeds = [seeds::VAULT_LEVERAGE_AUTH, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = leverage_config @ Errors::InvalidConfig,
        has_one = token_collateral_token_program,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_program,
        has_one = native_collateral_token_mint,
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

    #[account(
        mut,
        associated_token::token_program = native_collateral_token_program,
        associated_token::mint = native_collateral_token_mint,
        associated_token::authority = vault_authority,
    )]
    pub native_collateral_vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = native_collateral_token_program,
        associated_token::mint = native_collateral_token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_collateral_token_program,
    )]
    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
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