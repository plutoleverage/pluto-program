use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{close_account, CloseAccount, TokenInterface, Mint, TokenAccount};
use crate::error::{Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::util::{decimals, seeds, transfer_token::transfer_token, constant::WSOL_TOKEN_MINT};

#[inline(never)]
pub fn handle(ctx: Context<UnwrapSol>, amount: u64) -> Result<()> {
    let wsol = &mut ctx.accounts.wsol_buffer;
    let user = &mut ctx.accounts.user;
    let wsol_ata = &ctx.accounts.wsol_ata;
    let user_ata = &ctx.accounts.user_ata;

    require_gte!(user.lamports(), amount, Errors::InsufficientFunds);

    // TRANSFER WSOL to TEMP WSOL ACCOUNT
    transfer_token(
        user_ata.to_account_info(),
        wsol_ata.to_account_info(),
        user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_mint.to_account_info(),
        amount,
        ctx.accounts.token_mint.decimals,
    )?;

    let user_key = user.key();
    let seeds = &[
        seeds::WSOL_AUTH,
        user_key.as_ref(),
        &[ctx.bumps.wsol_buffer],
    ];

    let signer_seeds = &[&seeds[..]];

    // CLOSE TOKEN ACCOUNT TO UNWRAP WSOL TO SOL
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: wsol_ata.to_account_info(),
            destination: user.to_account_info(),
            authority: wsol.to_account_info(),
        },
        signer_seeds,
    );
    close_account(cpi_ctx)?;

    Ok(())
}

#[derive(Accounts)]
pub struct UnwrapSol<'info> {
    /// CHECK Safe only for buffer authority
    #[account(
        seeds = [seeds::WSOL_AUTH, user.key().as_ref()],
        bump,
    )]
    pub wsol_buffer: AccountInfo<'info>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = wsol_buffer
    )]
    pub wsol_ata: Box<InterfaceAccount<'info, TokenAccount>>,
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