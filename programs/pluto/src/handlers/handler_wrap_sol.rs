use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{sync_native, TokenInterface, Mint, TokenAccount, SyncNative};
use crate::error::{Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::util::{decimals, seeds, transfer_token::transfer_token, constant::WSOL_TOKEN_MINT};

#[inline(never)]
pub fn handle(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let user_ata = &ctx.accounts.user_ata;

    require_gte!(user.lamports(), amount, Errors::InsufficientFunds);

    // TRANSFER SOL to WSOL ATA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: user.to_account_info(),
            to: user_ata.to_account_info(),
        },
    );

    let ix = transfer(cpi_ctx, amount)?;

    // SYNC SOL AS WSOL
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative{
            account: user_ata.to_account_info(),
        },
    );

    let ix = sync_native(cpi_ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WrapSol<'info> {
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

    #[account(address = WSOL_TOKEN_MINT)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}