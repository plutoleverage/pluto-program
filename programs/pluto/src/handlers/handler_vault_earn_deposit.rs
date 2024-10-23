use anchor_lang::Discriminator;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::{ErrorEarn, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::event::EventEarnDeposit;
use crate::state::{EarnConfig, InitLenderParams, Lender, Protocol, Stats};
use crate::state::vault_earn::VaultEarn;
use crate::util::{decimals, seeds, transfer_token::transfer_token};
use crate::util::constant::{INDEX_DECIMALS, PERCENT_DECIMALS, UNIT_DECIMALS};

#[inline(never)]
pub fn handle(ctx: Context<VaultEarnDeposit>, amount: u64) -> Result<()> {
    verify_ixs(&ctx)?;
    check_freeze(&ctx)?;

    let fee_vault = &ctx.accounts.earn_fee_vault;
    let earn_config = &ctx.accounts.earn_config.load()?;
    let vault = &mut ctx.accounts.vault.load_mut()?;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("vault config address: {:?}", ctx.accounts.earn_config.key());
    msg!("lender address: {:?}", ctx.accounts.lender.key());

    msg!("vault index: {:?}", vault.index);
    msg!("min_deposit_limit: {:?}", earn_config.min_deposit_limit);
    msg!("max_deposit_limit: {:?}", earn_config.max_deposit_limit);
    msg!("deposit_fee: {:?}", earn_config.deposit_fee);

    require_gte!(amount as u128, earn_config.min_deposit_limit as u128, ErrorEarn::DepositMinLimitNotMet);
    require_gte!(earn_config.max_deposit_limit as u128, amount as u128, ErrorEarn::DepositMaxLimitExceeded);

    let lender = &mut if ctx.accounts.lender.load().is_ok() {
        msg!("lender is loaded");
        ctx.accounts.lender.load_mut()?
    } else {
        msg!("lender is initialized");
        ctx.accounts.lender.load_init()?
    };

    let user = &mut ctx.accounts.user;
    let vault_liquidity = &ctx.accounts.vault_liquidity;
    let user_ata = &ctx.accounts.user_ata;

    if user_ata.amount < amount {
        return Err(ErrorEarn::InsufficientFund.into());
    }

    let mut fee_amount = 0u64;

    if earn_config.deposit_fee > 0 {
        let fee = decimals::mul_ceil(vault.token_decimal, amount as u128, vault.token_decimal, earn_config.deposit_fee as u128, PERCENT_DECIMALS)?;
        fee_amount = decimals::div_ceil(vault.token_decimal, fee, vault.token_decimal, 100, 0)? as u64;
        if fee_amount > 0 {
            transfer_token(
                user_ata.to_account_info(),
                fee_vault.to_account_info(),
                user.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                fee_amount,
                ctx.accounts.token_mint.decimals,
            )?;
        }
    }

    let amount_after_fee = amount.checked_sub(fee_amount).ok_or(MathOverflow)?;

    msg!("fee_amount: {:?}", fee_amount);
    msg!("amount_after_fee: {:?}", amount_after_fee);

    // Transfer token from user to vault
    transfer_token(
        user_ata.to_account_info(),
        vault_liquidity.to_account_info(),
        user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_mint.to_account_info(),
        amount_after_fee,
        ctx.accounts.token_mint.decimals,
    )?;

    if !lender.is_initialized {
        msg!("lender is created");
        lender.init(InitLenderParams{
            bump: ctx.bumps.lender,
            owner: *user.key,
            protocol: ctx.accounts.vault.key(),
        })?;

        let stats = &mut ctx.accounts.earn_stats.load_mut()?;
        stats.add_user()?;
    }

    let unit = decimals::div_floor(UNIT_DECIMALS, amount_after_fee as u128, vault.token_decimal, vault.index, INDEX_DECIMALS)? as u64;
    lender.deposit(amount_after_fee, unit, vault.index)?;
    vault.mint(earn_config, unit)?;

    msg!("unit: {:?}", unit);
    msg!("avg_index: {:?}", lender.index);

    lender.confirm_deposit(vault.token_decimal)?;
    msg!("confirm deposit");

    emit!(EventEarnDeposit{
        vault: ctx.accounts.vault.key(),
        user: *user.to_account_info().key,
        lender: ctx.accounts.lender.key(),
        token_mint: vault.token_mint.key(),
        amount,
        unit: lender.unit,
        index: vault.index,
        pending_amount: lender.pending_deposit_amount,
        pending_unit: lender.pending_deposit_unit,
        pending_index: lender.pending_deposit_index,
        unit_supply: vault.unit_supply,
        vault_index: vault.index,
        fee_amount,
    });

    Ok(())
}

fn verify_ixs(ctx: &Context<VaultEarnDeposit>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultEarnDeposit>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_earn), ErrorEarn::VaultFrozen);
    require!(!ctx.accounts.earn_config.load()?.freeze, ErrorEarn::VaultFrozen);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnDeposit<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        has_one = earn_fee_vault,
    )]
    pub earn_config: AccountLoader<'info, EarnConfig>,
    /// CHECK Safe
    #[account(mut)]
    pub earn_fee_vault: AccountInfo<'info>,
    /// CHECK VAULT EARN AUTHORITY
    #[account(
        seeds = [seeds::VAULT_EARN_AUTH, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = earn_config @ Errors::InvalidConfig,
        has_one = earn_stats,
        has_one = token_program,
        has_one = token_mint,
    )]
    pub vault: AccountLoader<'info, VaultEarn>,
    #[account(mut)]
    pub earn_stats: AccountLoader<'info, Stats>,

    #[account(
        init_if_needed,
        seeds = [seeds::LENDER, vault.key().as_ref(), token_mint.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = Lender::INIT_SPACE+8+8+8,
    )]
    pub lender: AccountLoader<'info, Lender>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: check instructions account
    #[account(address = sysvar::instructions::ID @Errors::InvalidAddress)]
    pub instructions: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}