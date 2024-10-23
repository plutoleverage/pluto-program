use anchor_lang::{Discriminator};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{close_account, CloseAccount, TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::{EventLeverageBorrow};
use crate::handlers::{VaultLeverageRelease};
use crate::state::{EarnConfig, InitObligationParams, InitPositionParams, LeverageConfig, Obligation, Position, Protocol, VaultEarn, VaultLeverage};
use crate::util::{constant, decimals, seeds, transfer_token::transfer_token};
use crate::util::transfer_token::transfer_token_with_signer;
use crate::util::constant::{PERCENT_DECIMALS, LEVERAGE_ONE, INDEX_DECIMALS, MAX_ORACLE_AGE, UNIT_DECIMALS, PERCENT_MAX, MAX_OBLIGATION_POSITIONS};

#[inline(never)]
pub fn handle(ctx: Context<VaultLeverageRepayBorrow>, number: u8) -> Result<()> {
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let vault = &mut ctx.accounts.vault.load_mut()?;
    let config = &ctx.accounts.leverage_config.load()?;
    let borrow_vault = &mut ctx.accounts.borrow_vault.load_mut()?;
    let obligation = &mut ctx.accounts.obligation.load_mut()?;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("borrow_vault address: {:?}", ctx.accounts.borrow_vault.key());
    msg!("obligation address: {:?}", ctx.accounts.obligation.key());

    require!(number < MAX_OBLIGATION_POSITIONS, ErrorLeverage::InvalidPositionNumber);

    let position = &mut obligation.positions[number as usize];

    require_gt!(position.unit, 0, ErrorLeverage::NoPositionFound);

    let utilization_rate = borrow_vault.utilization_rate()?;

    // REPAY BORROW
    let borrowing_amount = position.state.repay_amount;

    msg!("borrowing_amount: {:?}", borrowing_amount);

    transfer_token(
        ctx.accounts.user_ata.to_account_info(),
        ctx.accounts.borrow_vault_liquidity.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_collateral_token_program.to_account_info(),
        ctx.accounts.token_collateral_token_mint.to_account_info(),
        borrowing_amount,
        ctx.accounts.token_collateral_token_mint.decimals,
    )?;

    position.repay_borrow(borrowing_amount)?;
    borrow_vault.deleverage(position.state.repay_unit)?;

    // PAY PROTOCOL
    let protocol_fee_factor = vault.protocol_fee_factor(config.protocol_fee, utilization_rate, position.avg_borrowing_index, vault.borrowing_index)?;

    msg!("utilization_rate: {:?}", utilization_rate);
    msg!("protocol_fee_factor: {:?}", protocol_fee_factor);

    let protocol_fee_amount = (decimals::mul_ceil(
        vault.token_collateral_token_decimal, position.state.release_min_output as u128, vault.token_collateral_token_decimal,
        protocol_fee_factor, INDEX_DECIMALS
    )? as u64).saturating_div(100);

    if protocol_fee_amount > 0 {
        transfer_token(
            ctx.accounts.user_ata.to_account_info(),
            ctx.accounts.leverage_fee_vault.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_collateral_token_program.to_account_info(),
            ctx.accounts.token_collateral_token_mint.to_account_info(),
            protocol_fee_amount,
            ctx.accounts.token_collateral_token_mint.decimals,
        )?;
    }

    msg!("protocol_fee_amount: {:?}", protocol_fee_amount);

    position.pay_protocol_fee(utilization_rate, protocol_fee_factor, protocol_fee_amount)?;

    Ok(())
}

#[inline(never)]
fn verify_next_ixs(ctx: &Context<VaultLeverageRepayBorrow>) -> Result<()> {
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
                if ix_discriminator == crate::instruction::LeverageVaultClosing::discriminator() {
                    break;
                } else {
                    return Err(ErrorLeverage::NextInstructionMustBeClosing.into());
                }
            }
        } else {
            // no more instructions, so we're missing a stake
            return Err(ErrorLeverage::MissingClosing.into());
        }

        index += 1
    }

    Ok(())
}

#[inline(never)]
fn check_freeze(ctx: &Context<VaultLeverageRepayBorrow>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageRepayBorrow<'info> {
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
        has_one = borrow_vault,
        has_one = token_collateral_token_program,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_program,
        has_one = native_collateral_token_mint,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,

    /// CHECK VAULT FOR BORROWING AUTHORITY
    #[account(
        seeds = [seeds::VAULT_EARN_AUTH, borrow_vault.key().as_ref()],
        bump,
    )]
    pub borrow_vault_authority: AccountInfo<'info>,
    #[account(
        mut,
    )]
    pub borrow_vault: AccountLoader<'info, VaultEarn>,
    #[account(
        mut,
        associated_token::token_program = token_collateral_token_program,
        associated_token::mint = token_collateral_token_mint,
        associated_token::authority = borrow_vault_authority,
    )]
    pub borrow_vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK VAULT LEVERAGE AUTHORITY
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