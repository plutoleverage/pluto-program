use anchor_lang::{system_program, Discriminator};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{close_account, CloseAccount, TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::state::{EarnConfig, InitObligationParams, InitPositionParams, LeverageConfig, Obligation, Position, Protocol, Stats, VaultEarn, VaultLeverage};
use crate::util::{constant, decimals, seeds, transfer_token::transfer_token};
use crate::util::transfer_token::transfer_token_with_signer;
use crate::util::constant::{PERCENT_DECIMALS, LEVERAGE_ONE, INDEX_DECIMALS, MAX_ORACLE_AGE, UNIT_DECIMALS, PERCENT_MAX, MAX_OBLIGATION_POSITIONS};

#[inline(never)]
pub fn handle(ctx: Context<VaultLeverageClosing>, number: u8) -> Result<()> {
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let mut clean = true;
    {
        let vault = &mut ctx.accounts.vault.load_mut()?;
        let obligation = &mut ctx.accounts.obligation.load_mut()?;

        msg!("vault address: {:?}", ctx.accounts.vault.key());
        msg!("obligation address: {:?}", ctx.accounts.obligation.key());

        require!(number < MAX_OBLIGATION_POSITIONS, ErrorLeverage::InvalidPositionNumber);

        let mut id = Pubkey::default();
        {
            let position = &mut obligation.positions[number as usize];

            require_gt!(position.unit, 0, ErrorLeverage::NoPositionFound);

            vault.burn(position.state.release_unit)?;
            vault.burn_borrow(position.state.repay_unit)?;

            let mut closing_fee_amount = 0u64;
            if position.state.closing_fee > 0 {
                closing_fee_amount = decimals::mul_ceil(vault.token_collateral_token_decimal, position.state.release_amount as u128, vault.token_collateral_token_decimal, position.state.closing_fee as u128, PERCENT_DECIMALS)? as u64;
                closing_fee_amount = decimals::div_ceil(vault.token_collateral_token_decimal, closing_fee_amount as u128, vault.token_collateral_token_decimal, 100, 0)? as u64;

                if closing_fee_amount > 0 {
                    transfer_token(
                        ctx.accounts.user_ata.to_account_info(),
                        ctx.accounts.leverage_fee_vault.to_account_info(),
                        ctx.accounts.user.to_account_info(),
                        ctx.accounts.token_collateral_token_program.to_account_info(),
                        ctx.accounts.token_collateral_token_mint.to_account_info(),
                        closing_fee_amount,
                        ctx.accounts.token_collateral_token_mint.decimals,
                    )?;
                }
            }

            if position.unit == position.state.release_unit {
                position.closing()?;
                id = position.id;
            }
        }

        if id != Pubkey::default() {
            obligation.close_position(id)?;
        }

        for position in obligation.positions.iter_mut() {
            if position.id != Pubkey::default() {
                clean = false;
                break;
            }
        }
    }

    if clean {
        close_obligation(&ctx)?;

        let stats = &mut ctx.accounts.leverage_stats.load_mut()?;
        stats.remove_user()?;
    }

    Ok(())
}

#[inline(never)]
fn verify_next_ixs(ctx: &Context<VaultLeverageClosing>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultLeverageClosing>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[inline(never)]
fn close_obligation(ctx: &Context<VaultLeverageClosing>) -> Result<()> {
    let dest_lamports = ctx.accounts.user.to_account_info().lamports();
    let close_lamports = ctx.accounts.obligation.to_account_info().lamports();

    **ctx.accounts.obligation.to_account_info().try_borrow_mut_lamports()? = 0;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? = dest_lamports.checked_add(close_lamports).unwrap();

    ctx.accounts.obligation.to_account_info().assign(&system_program::ID);
    ctx.accounts.obligation.to_account_info().realloc(0, false)?;

    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageClosing<'info> {
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
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = leverage_config @ Errors::InvalidConfig,
        has_one = leverage_stats,
        has_one = token_collateral_token_program,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_program,
        has_one = native_collateral_token_mint,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,
    #[account(mut)]
    pub leverage_stats: AccountLoader<'info, Stats>,

    #[account(
        mut,
        seeds = [seeds::OBLIGATION, vault.key().as_ref(), token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::token_program = token_collateral_token_program,
        associated_token::mint = token_collateral_token_mint,
        associated_token::authority = user,
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