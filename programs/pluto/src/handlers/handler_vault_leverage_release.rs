use anchor_lang::Discriminator;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token_interface::{close_account, TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
use crate::error::{ErrorLeverage, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::{EventLeverageRelease};
use crate::handlers::{VaultLeverageClose, VaultLeverageKeeperClosing};
use crate::state::{EarnConfig, InitObligationParams, InitPositionParams, LeverageConfig, Obligation, Position, Protocol, VaultEarn, VaultLeverage};
use crate::util::{constant, decimals, seeds, transfer_token::transfer_token};
use crate::util::transfer_token::transfer_token_with_signer;
use crate::util::constant::{PERCENT_DECIMALS, LEVERAGE_ONE, INDEX_DECIMALS, MAX_ORACLE_AGE, UNIT_DECIMALS, PERCENT_MAX, MAX_OBLIGATION_POSITIONS};

pub fn handle(ctx: Context<VaultLeverageRelease>, number: u8) -> Result<()> {
    verify_next_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let vault = &mut ctx.accounts.vault.load_mut()?;
    let obligation = &mut ctx.accounts.obligation.load_mut()?;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("obligation address: {:?}", ctx.accounts.obligation.key());

    msg!("vault index: {:?}", vault.index);
    msg!("vault borrowing index: {:?}", vault.borrowing_index);

    require!(number < MAX_OBLIGATION_POSITIONS, ErrorLeverage::InvalidPositionNumber);

    let position = &mut obligation.positions[number as usize];

    require_gt!(position.unit, 0, ErrorLeverage::NoPositionFound);

    let vault_key = ctx.accounts.vault.key();
    let vault_seeds = &[
        seeds::VAULT_LEVERAGE_AUTH,
        vault_key.as_ref(),
        &[ctx.bumps.vault_authority],
    ];

    let vault_signer_seeds = &[&vault_seeds[..]];

    // SEND BACK TO VAULT AFTER SWAP
    transfer_token_with_signer(
        ctx.accounts.native_collateral_vault_liquidity.to_account_info(),
        ctx.accounts.user_ata.to_account_info(),
        ctx.accounts.vault_authority.to_account_info(),
        ctx.accounts.native_collateral_token_program.to_account_info(),
        ctx.accounts.native_collateral_token_mint.to_account_info(),
        position.state.release_amount,
        ctx.accounts.native_collateral_token_mint.decimals,
        vault_signer_seeds,
    )?;

    Ok(())
}

#[inline(never)]
fn verify_next_ixs(ctx: &Context<VaultLeverageRelease>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    // loop through instructions, looking for an equivalent repay to this borrow
    let mut index = current_index + 1;
    let mut jup_found = false;
    loop {
        // get the next instruction, die if theres no more
        if let Ok(ix) = load_instruction_at_checked(index, &ixs) {
            if ix.program_id == constant::JUPITER_SWAP_PROGRAM_ID {
                jup_found = true;
                index += 1;
                continue;
            }
            if ix.program_id == crate::id() {
                if !jup_found {
                    return Err(ErrorLeverage::MissingJupiterSwap.into());
                }
                let ix_discriminator: [u8; 8] = ix.data[0..8]
                    .try_into()
                    .map_err(|_| Errors::UnknownInstruction)?;

                // check if we have a toplevel stake toward authority
                if ix_discriminator == crate::instruction::LeverageVaultRepayBorrow::discriminator() {
                    break;
                } else {
                    return Err(ErrorLeverage::NextInstructionMustBeRepayBorrow.into());
                }
            }
        } else {
            // no more instructions, so we're missing a stake
            return Err(ErrorLeverage::MissingRepayBorrow.into());
        }

        index += 1
    }

    Ok(())
}

#[inline(never)]
fn check_freeze(ctx: &Context<VaultLeverageRelease>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageRelease<'info> {
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
        init_if_needed,
        payer = user,
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