use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::Errors;
use crate::event::EventVaultLeverageCreated;
use crate::handlers::VaultEarnCreate;
use crate::state::{LeverageConfig, InitVaultLeverageParams, VaultLeverage, VaultEarn, EarnConfig, Protocol, Stats, InitStatsParams};
use crate::util::{
    seeds,
    constant::{UNIT_DECIMALS, INDEX_ONE},
};

#[inline(never)]
pub fn handle(ctx: Context<VaultLeverageCreate>, token_collateral_price_feed: [u8; 64], native_collateral_price_feed: [u8; 64]) -> Result<()> {
    let borrow_vault = &ctx.accounts.borrow_vault.load()?;
    let leverage_config = &ctx.accounts.leverage_config.load()?;
    let vault = &mut ctx.accounts.vault.load_init()?;
    let stats = &mut ctx.accounts.stats.load_init()?;

    vault.init(InitVaultLeverageParams{
        bump: ctx.bumps.vault,
        protocol: ctx.accounts.protocol.key(),
        leverage_stats: ctx.accounts.stats.key(),
        creator: *ctx.accounts.owner.key,
        authority: *ctx.accounts.vault_authority.key,
        leverage_config: ctx.accounts.leverage_config.key(),
        borrow_vault: ctx.accounts.borrow_vault.key(),
        token_collateral_price_oracle: ctx.accounts.token_collateral_price_oracle.key(),
        token_collateral_price_feed,
        token_collateral_token_program: ctx.accounts.token_collateral_token_program.key(),
        token_collateral_token_mint: ctx.accounts.token_collateral_token_mint.key(),
        token_collateral_token_decimal: ctx.accounts.token_collateral_token_mint.decimals,
        native_collateral_price_oracle: ctx.accounts.native_collateral_price_oracle.key(),
        native_collateral_price_feed,
        native_collateral_token_program: ctx.accounts.native_collateral_token_program.key(),
        native_collateral_token_mint: ctx.accounts.native_collateral_token_mint.key(),
        native_collateral_token_decimal: ctx.accounts.native_collateral_token_mint.decimals,
        borrowing_index: INDEX_ONE as u128,
        index: INDEX_ONE as u128,
    })?;

    stats.init(InitStatsParams{
        bump: ctx.bumps.stats,
        protocol: ctx.accounts.protocol.key(),
        vault: ctx.accounts.vault.key(),
        creator: *ctx.accounts.owner.key,
    })?;

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("leverage stats address: {:?}", ctx.accounts.stats.key());
    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("vault authority address: {:?}", ctx.accounts.vault_authority.key());
    msg!("vault borrow address: {:?}", ctx.accounts.borrow_vault.key());
    msg!("vault leverage config address: {:?}", ctx.accounts.leverage_config.key());
    msg!("vault token collateral price oracle address: {:?}", ctx.accounts.token_collateral_price_oracle.key());
    msg!("vault token collateral price feed: {:?}", token_collateral_price_feed);
    msg!("vault token collateral token program address: {:?}", ctx.accounts.token_collateral_token_program.key());
    msg!("vault token collateral mint address: {:?}", ctx.accounts.token_collateral_token_mint.key());
    msg!("vault native collateral price oracle address: {:?}", ctx.accounts.native_collateral_price_oracle.key());
    msg!("vault native collateral price feed: {:?}", native_collateral_price_feed);
    msg!("vault native collateral token program address: {:?}", ctx.accounts.native_collateral_token_program.key());
    msg!("vault native collateral mint address: {:?}", ctx.accounts.native_collateral_token_mint.key());

    emit!(EventVaultLeverageCreated{
        protocol: ctx.accounts.protocol.key(),
        leverage_stats: ctx.accounts.stats.key(),
        vault: ctx.accounts.vault.key(),
        creator: vault.creator,
        authority: vault.authority,
        leverage_config: vault.leverage_config,
        borrow_vault: vault.borrow_vault,
        token_collateral_price_oracle: vault.token_collateral_price_oracle,
        token_collateral_price_feed,
        token_collateral_token_mint: vault.token_collateral_token_mint,
        token_collateral_token_decimals: vault.token_collateral_token_decimal,
        token_collateral_vault_ata: vault.token_collateral_vault_liquidity,
        native_collateral_price_oracle: vault.native_collateral_price_oracle,
        native_collateral_price_feed,
        native_collateral_token_mint: vault.native_collateral_token_mint,
        native_collateral_token_decimals: vault.native_collateral_token_decimal,
        native_collateral_vault_ata: vault.native_collateral_vault_liquidity,
        borrowing_index: vault.borrowing_index,
        index: vault.index,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageCreate<'info> {
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
        init,
        seeds = [seeds::VAULT_LEVERAGE, token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), protocol.key().as_ref()],
        bump,
        payer = owner,
        space = VaultLeverage::INIT_SPACE+8+8+8+24,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,
    #[account(
        init,
        payer = owner,
        seeds = [seeds::STATS, vault.key().as_ref()],
        bump,
        space = Stats::INIT_SPACE+8+8,
    )]
    pub stats: AccountLoader<'info, Stats>,

    /// CHECK VAULT FOR BORROWING
    #[account(
        seeds = [seeds::VAULT_EARN, token_collateral_token_mint.key().as_ref(), protocol.key().as_ref()],
        bump,
    )]
    pub borrow_vault: AccountLoader<'info, VaultEarn>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub owner: Signer<'info>,

    pub token_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = token_collateral_token_program,
    )]
    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub native_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = native_collateral_token_program,
    )]
    pub native_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_collateral_token_program: Interface<'info, TokenInterface>,
    pub native_collateral_token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}