use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::error::ErrorEarn;
use crate::event::EventDepositEarn;
use crate::state::InitLenderParams;
use crate::state::lender::Lender;
use crate::state::vault_earn::VaultEarn;
use crate::util::{seeds, transfer_token::transfer_token};

pub fn handle(ctx: Context<VaultEarnDeposit>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let lender = &mut ctx.accounts.lender;
    let user = &mut ctx.accounts.user;
    let vault_ata = &ctx.accounts.vault_ata;
    let user_ata = &ctx.accounts.user_ata;

    if user_ata.amount < amount {
        return Err(ErrorEarn::InsufficientFund.into());
    }

    msg!("Transferring from user to vault with amount: {}", amount);

    transfer_token(
        user_ata.to_account_info(),
        vault_ata.to_account_info(),
        user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
    )?;

    msg!("Trannsferred to vault with amount: {}", amount);
    if !lender.is_initialized {
        lender.init(InitLenderParams{
            bump: ctx.bumps.lender,
            owner: *user.key,
            program_account: *vault.to_account_info().key,
            token_mint: vault.token_mint,
            token_decimal: vault.token_decimal,
        })?;
    }

    msg!("Deposit to lender with amount: {}", amount);
    let mint_unit = lender.deposit(vault.index, amount)?;
    vault.deposit(amount)?;

    emit!(EventDepositEarn{
        vault: *vault.to_account_info().key,
        token_mint: *ctx.accounts.token_mint.to_account_info().key,
        user: *user.to_account_info().key,
        amount,
        index: vault.index,
        unit: lender.unit,
        mint_unit,
        unit_supply: vault.unit_supply,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnDeposit<'info> {
    #[account(mut)]
    pub vault: Box<Account<'info, VaultEarn>>,

    #[account(
        init_if_needed,
        seeds = [seeds::LENDER, vault.owner.key().as_ref(), token_mint.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = Lender::INIT_SPACE+8
    )]
    pub lender: Account<'info, Lender>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
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
    pub rent: Sysvar<'info, Rent>,
}