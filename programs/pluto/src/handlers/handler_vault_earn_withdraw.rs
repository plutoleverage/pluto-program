use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::error::ErrorEarn;
use crate::event::EventWithdrawEarn;
use crate::state::lender::Lender;
use crate::state::vault_earn::VaultEarn;
use crate::util::{seeds,transfer_token::transfer_token_with_signer};

pub fn handle(ctx: Context<VaultEarnWithdraw>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let lender = &mut ctx.accounts.lender;
    let user = &mut ctx.accounts.user;
    let token_mint = &ctx.accounts.token_mint;
    let vault_ata = &ctx.accounts.vault_ata;
    let user_ata = &ctx.accounts.user_ata;

    if lender.to_account_info().data_len() == 0 {
        return Err(ErrorEarn::InvalidFund.into());
    }

    if lender.owner != *user.key {
        return Err(ErrorEarn::InvalidOwner.into());
    }

    if lender.valuation_amount(vault.index).unwrap() < amount {
        return Err(ErrorEarn::InsufficientFund.into());
    }

    let bump = vault.bump;
    let vault_owner_key = vault.owner.key();
    let token_mint_key = token_mint.key();
    let seeds = &[
        seeds::VAULT_EARN,
        token_mint_key.as_ref(),
        vault_owner_key.as_ref(),
        &[bump],
    ];

    let signer_seeds = &[&seeds[..]];

    transfer_token_with_signer(
        vault_ata.to_account_info(),
        user_ata.to_account_info(),
        vault.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
        signer_seeds,
    )?;

    let burn_unit = lender.withdraw(vault.index, amount)?;
    vault.withdraw(amount)?;

    emit!(EventWithdrawEarn {
        vault: *vault.to_account_info().key,
        token_mint: *ctx.accounts.token_mint.to_account_info().key,
        user: *user.to_account_info().key,
        amount,
        index: vault.index,
        unit: lender.unit,
        burn_unit,
        unit_supply: vault.unit_supply,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultEarn>,

    #[account(
        mut,
        seeds = [seeds::LENDER, vault.owner.key().as_ref(), token_mint.key().as_ref(), user.key().as_ref()],
        bump,
        realloc = Lender::INIT_SPACE+8,
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}