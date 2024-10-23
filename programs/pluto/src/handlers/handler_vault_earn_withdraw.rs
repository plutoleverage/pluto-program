use anchor_lang::{Accounts, Discriminator, system_program};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_lang::solana_program::sysvar::instructions::{load_current_index_checked, load_instruction_at_checked};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{close_account, CloseAccount, TokenInterface, Mint, TokenAccount};
use crate::error::{ErrorEarn, Errors};
use crate::event::{EventEarnWithdraw, EventEarnWithdrawn};
use crate::handlers::VaultEarnDeposit;
use crate::state::{EarnConfig, Lender, Protocol, Stats};
use crate::state::vault_earn::VaultEarn;
use crate::util::{decimals, seeds, transfer_token::{transfer_token, transfer_token_with_signer}};
use crate::util::constant::{PERCENT_DECIMALS, UNIT_DECIMALS, INDEX_DECIMALS};

pub fn handle(ctx: Context<VaultEarnWithdraw>, unit: u64, min_output_amount: u64) -> Result<()> {
    verify_ixs(&ctx)?;
    check_freeze(&ctx)?;
    {
        let fee_vault = &ctx.accounts.earn_fee_vault;
        let earn_config = &ctx.accounts.earn_config.load()?;
        let vault = &mut ctx.accounts.vault.load_mut()?;
        let lender = &mut ctx.accounts.lender.load_mut()?;

        msg!("vault address: {:?}", ctx.accounts.vault.key());
        msg!("vault config address: {:?}", ctx.accounts.earn_config.key());
        msg!("lender address: {:?}", ctx.accounts.lender.key());

        msg!("vault index: {:?}", vault.index);
        msg!("min_withdraw_limit: {:?}", earn_config.min_withdraw_limit);
        msg!("max_withdraw_limit: {:?}", earn_config.max_withdraw_limit);
        msg!("withdraw_fee: {:?}", earn_config.withdraw_fee);

        msg!("lender unit: {:?}", lender.unit);
        msg!("lender index: {:?}", lender.index);

        let amount = vault.unit_to_amount(unit as u128)? as u64;
        msg!("unit: {:?}, min_output_amount: {:?}, amount: {:?}", unit, min_output_amount, amount);

        require_gte!(amount as u128, earn_config.min_withdraw_limit as u128, ErrorEarn::WithdrawMinLimitNotMet);
        require_gte!(earn_config.max_withdraw_limit as u128, amount as u128, ErrorEarn::WithdrawMaxLimitExceeded);

        let vault_liquidity = &ctx.accounts.vault_liquidity;

        let user = &mut ctx.accounts.user;
        let user_ata = &ctx.accounts.user_ata;

        if !lender.is_initialized {
            return Err(ErrorEarn::InvalidFund.into());
        }

        if lender.owner != *ctx.accounts.user.key {
            return Err(ErrorEarn::InvalidOwner.into());
        }

        if lender.unit < unit {
            return Err(ErrorEarn::InsufficientFund.into());
        }

        if vault_liquidity.amount < min_output_amount {
            return Err(ErrorEarn::InsufficientLiquidityInPool.into());
        }

        if lender.valuation_amount(vault.index, vault.token_decimal)? < min_output_amount {
            return Err(ErrorEarn::InsufficientFund.into());
        }

        if amount < min_output_amount {
            return Err(ErrorEarn::OutputTooSmall.into());
        }

        lender.withdraw(amount, unit, vault.index)?;

        let vault_key = ctx.accounts.vault.key();
        let seeds = &[
            seeds::VAULT_EARN_AUTH,
            vault_key.as_ref(),
            &[ctx.bumps.vault_authority],
        ];

        let signer_seeds = &[&seeds[..]];

        let mut fee_amount = 0u64;

        if earn_config.withdraw_fee > 0 {
            let fee = decimals::mul_ceil(vault.token_decimal, lender.pending_withdraw_amount as u128, vault.token_decimal, earn_config.withdraw_fee as u128, PERCENT_DECIMALS)?;
            fee_amount = decimals::div_ceil(vault.token_decimal, fee, vault.token_decimal, 100, 0)? as u64;
        }

        msg!("fee_amount: {:?}", fee_amount);

        let utilization_rate = vault.utilization_rate()?;
        msg!("utilization_rate: {:?}", utilization_rate);
        let protocol_fee_factor = vault.protocol_fee_factor(earn_config.protocol_fee, utilization_rate, lender.index, vault.index)?;
        msg!("protocol_fee_factor: {:?}", protocol_fee_factor);

        let protocol_fee_amount = (decimals::mul_ceil(
            vault.token_decimal, unit as u128, UNIT_DECIMALS,
            protocol_fee_factor, INDEX_DECIMALS
        )? as u64).saturating_div(100);

        msg!("protocol_fee_amount: {:?}", protocol_fee_amount);

        if fee_amount > 0 || protocol_fee_amount > 0 {
            transfer_token_with_signer(
                vault_liquidity.to_account_info(),
                fee_vault.to_account_info(),
                ctx.accounts.vault_authority.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_mint.to_account_info(),
                fee_amount.checked_add(protocol_fee_amount).ok_or(ErrorEarn::InsufficientFund)?,
                ctx.accounts.token_mint.decimals,
                signer_seeds,
            )?;
        }

        let amount_after_fee = amount.checked_sub(fee_amount).ok_or(ErrorEarn::InsufficientFund)?.checked_sub(protocol_fee_amount).ok_or(ErrorEarn::InsufficientFund)?;

        msg!("amount_after_fee: {:?}", amount_after_fee);

        transfer_token_with_signer(
            vault_liquidity.to_account_info(),
            user_ata.to_account_info(),
            ctx.accounts.vault_authority.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
            amount_after_fee,
            ctx.accounts.token_mint.decimals,
            signer_seeds,
        )?;

        msg!("confirm_withdraw");
        lender.confirm_withdraw()?;
        msg!("burn unit");
        vault.burn(earn_config, unit)?;

        emit!(EventEarnWithdraw{
            vault: ctx.accounts.vault.key(),
            user: ctx.accounts.user.key(),
            lender: ctx.accounts.lender.key(),
            token_mint: ctx.accounts.token_mint.key(),
            amount,
            unit: lender.unit,
            index: lender.index,
            pending_amount: lender.pending_withdraw_amount,
            pending_unit: lender.pending_withdraw_unit,
            pending_index: lender.pending_withdraw_index,
            unit_supply: vault.unit_supply,
            vault_index: vault.index,
            fee_amount,
        });

        emit!(EventEarnWithdrawn{
            vault: ctx.accounts.vault.key(),
            user: ctx.accounts.user.key(),
            lender: ctx.accounts.lender.key(),
            token_mint: ctx.accounts.token_mint.key(),
            amount,
            unit: lender.unit,
            index: lender.index,
            pending_amount: lender.pending_withdraw_amount,
            pending_unit: lender.pending_withdraw_unit,
            pending_index: lender.pending_withdraw_index,
            unit_supply: vault.unit_supply,
            vault_index: vault.index,
            protocol_fee: earn_config.protocol_fee,
            align0: [0;4],
            protocol_fee_amount,
            fee_amount,
            padding: [0;32],
        });
    }

    let mut need_close = false;

    if (UNIT_DECIMALS as i8) > (ctx.accounts.vault.load_mut()?.token_decimal as i8) {
        if ctx.accounts.lender.load_mut()?.unit <= 10u64.pow((UNIT_DECIMALS - ctx.accounts.vault.load()?.token_decimal) as u32) {
            need_close = true;
        }
    } else {
        if ctx.accounts.lender.load_mut()?.unit == 0 {
            need_close = true;
        }
    }

    if need_close {
        msg!("lender closed");
        close_lender(&ctx)?;

        let stats = &mut ctx.accounts.earn_stats.load_mut()?;
        stats.remove_user()?;
    }

    Ok(())
}

fn verify_ixs(ctx: &Context<VaultEarnWithdraw>) -> Result<()> {
    let ixs = ctx.accounts.instructions.to_account_info();

    // make sure this isnt a cpi call
    let current_index = load_current_index_checked(&ixs)? as usize;
    let current_ix = load_instruction_at_checked(current_index, &ixs)?;
    if current_ix.program_id != *ctx.program_id {
        return Err(Errors::InvalidProgram.into());
    }

    Ok(())
}

fn check_freeze(ctx: &Context<VaultEarnWithdraw>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_earn), ErrorEarn::VaultFrozen);
    require!(!ctx.accounts.earn_config.load()?.freeze, ErrorEarn::VaultFrozen);
    Ok(())
}

fn close_lender(ctx: &Context<VaultEarnWithdraw>) -> Result<()> {
    let dest_lamports = ctx.accounts.user.to_account_info().lamports();
    let close_lamports = ctx.accounts.lender.to_account_info().lamports();

    **ctx.accounts.lender.to_account_info().try_borrow_mut_lamports()? = 0;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? = dest_lamports.checked_add(close_lamports).unwrap();

    ctx.accounts.lender.to_account_info().assign(&system_program::ID);
    ctx.accounts.lender.to_account_info().realloc(0, false)?;

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnWithdraw<'info> {
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
        mut,
        seeds = [seeds::LENDER, vault.key().as_ref(), token_mint.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub lender: AccountLoader<'info, Lender>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Box<InterfaceAccount<'info, TokenAccount>>,

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