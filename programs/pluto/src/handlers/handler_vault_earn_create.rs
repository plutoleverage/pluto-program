use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::InitVaultEarnParams;
use crate::state::vault_earn::VaultEarn;
use crate::util::{decimals, seeds};

pub fn handle(ctx: Context<VaultEarnCreate>, ltv: u16, deposit_limit: u64, withdraw_limit: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.init(InitVaultEarnParams{
        bump: ctx.bumps.vault,
        owner: *ctx.accounts.owner.key,
        token_mint: ctx.accounts.token_mint.key(),
        token_decimal: ctx.accounts.token_mint.decimals,
        ltv,
        deposit_limit,
        withdraw_limit,
        index: decimals::INDEX_MULTIPLICATION as u128,
    })?;

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnCreate<'info> {
    #[account(
        init,
        seeds = [seeds::VAULT_EARN, token_mint.key().as_ref(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = VaultEarn::INIT_SPACE+8
    )]
    pub vault: Account<'info, VaultEarn>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = token_mint,
        associated_token::authority = vault
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}