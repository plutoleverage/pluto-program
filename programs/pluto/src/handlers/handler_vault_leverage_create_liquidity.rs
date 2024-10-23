use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use crate::error::Errors;
use crate::state::{LeverageConfig, VaultLeverage, Protocol};
use crate::util::{
    seeds,
};

#[inline(never)]
pub fn handle(ctx: Context<VaultLeverageCreateLiquidity>) -> Result<()> {
    let vault = &mut ctx.accounts.vault.load_mut()?;

    vault.token_collateral_vault_liquidity = ctx.accounts.token_collateral_vault_liquidity.key();
    vault.native_collateral_vault_liquidity = ctx.accounts.native_collateral_vault_liquidity.key();

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("vault token collateral token program address: {:?}", ctx.accounts.token_collateral_token_program.key());
    msg!("vault token collateral mint address: {:?}", ctx.accounts.token_collateral_token_mint.key());
    msg!("vault token collateral vault address: {:?}", ctx.accounts.token_collateral_vault_liquidity.to_account_info().key);
    msg!("vault native collateral token program address: {:?}", ctx.accounts.native_collateral_token_program.key());
    msg!("vault native collateral mint address: {:?}", ctx.accounts.native_collateral_token_mint.key());
    msg!("vault native collateral vault address: {:?}", ctx.accounts.native_collateral_vault_liquidity.to_account_info().key);

    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageCreateLiquidity<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        has_one = protocol @ Errors::InvalidProtocol,
    )]
    pub leverage_config: AccountLoader<'info, LeverageConfig>,
    /// CHECK VAULT LEVERAGE AUTHORITY
    #[account(
        seeds = [seeds::VAULT_LEVERAGE_AUTH, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [seeds::VAULT_LEVERAGE, token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), protocol.key().as_ref()],
        bump,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        associated_token::token_program = token_collateral_token_program,
        associated_token::mint = token_collateral_token_mint,
        associated_token::authority = vault_authority,
    )]
    pub token_collateral_vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = owner,
        associated_token::token_program = native_collateral_token_program,
        associated_token::mint = native_collateral_token_mint,
        associated_token::authority = vault_authority,
    )]
    pub native_collateral_vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mint::token_program = token_collateral_token_program,
    )]
    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mint::token_program = native_collateral_token_program,
    )]
    pub native_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_collateral_token_program: Interface<'info, TokenInterface>,
    pub native_collateral_token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}