use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::Errors;
use crate::event::EventVaultEarnCreated;
use crate::state::{EarnConfig, InitStatsParams, InitVaultEarnParams, Protocol, Stats};
use crate::state::vault_earn::VaultEarn;
use crate::util::{
    seeds,
    constant::{UNIT_DECIMALS, INDEX_ONE},
};

pub fn handle(ctx: Context<VaultEarnCreate>, price_feed: [u8; 64]) -> Result<()> {
    let earn_config = &ctx.accounts.earn_config.load()?;
    let vault = &mut ctx.accounts.vault.load_init()?;
    let stats = &mut ctx.accounts.stats.load_init()?;
    let owner = &ctx.accounts.payer;

    vault.init(InitVaultEarnParams{
        bump: ctx.bumps.vault,
        protocol: ctx.accounts.protocol.key(),
        earn_stats: ctx.accounts.stats.key(),
        creator: *owner.key,
        authority: *ctx.accounts.vault_authority.key,
        earn_config: ctx.accounts.earn_config.key(),
        vault_liquidity: ctx.accounts.vault_liquidity.key(),
        price_oracle: ctx.accounts.price_oracle.key(),
        price_feed,
        token_program: ctx.accounts.token_program.key(),
        token_mint: ctx.accounts.token_mint.key(),
        token_decimal: ctx.accounts.token_mint.decimals,
        index: INDEX_ONE as u128,
    })?;

    stats.init(InitStatsParams{
        bump: ctx.bumps.stats,
        protocol: ctx.accounts.protocol.key(),
        vault: ctx.accounts.vault.key(),
        creator: *owner.key,
    })?;

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("earn stats address: {:?}", ctx.accounts.stats.key());
    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("authority address: {:?}", ctx.accounts.vault_authority.key());
    msg!("earn_config address: {:?}", ctx.accounts.earn_config.key());
    msg!("vault_liquidity address: {:?}", ctx.accounts.vault_liquidity.key());
    msg!("price_oracle address: {:?}", ctx.accounts.price_oracle.key());
    msg!("price_feed: {:?}", price_feed);
    msg!("token_program address: {:?}", ctx.accounts.token_program.key());
    msg!("token_mint address: {:?}", ctx.accounts.token_mint.key());
    msg!("token_decimal: {:?}", ctx.accounts.token_mint.decimals);
    msg!("index: {:?}", vault.index);

    emit!(EventVaultEarnCreated {
        protocol: ctx.accounts.protocol.key(),
        earn_stats: ctx.accounts.stats.key(),
        vault: ctx.accounts.vault.key(),
        creator: vault.creator,
        authority: vault.authority,
        earn_config: vault.earn_config,
        vault_ata: vault.vault_liquidity,
        price_oracle: vault.price_oracle,
        price_feed,
        token_program: vault.token_program,
        token_mint: vault.token_mint,
        token_decimal: vault.token_decimal,
        index: vault.index,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnCreate<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        has_one = protocol @ Errors::InvalidProtocol,
    )]
    pub earn_config: AccountLoader<'info, EarnConfig>,
    /// CHECK VAULT EARN AUTHORITY
    #[account(
        seeds = [seeds::VAULT_EARN_AUTH, vault.key().as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(
        init,
        seeds = [seeds::VAULT_EARN, token_mint.key().as_ref(), protocol.key().as_ref()],
        bump,
        payer = payer,
        space = VaultEarn::INIT_SPACE+8+8+8,
    )]
    pub vault: AccountLoader<'info, VaultEarn>,
    #[account(
        init,
        payer = payer,
        seeds = [seeds::STATS, vault.key().as_ref()],
        bump,
        space = Stats::INIT_SPACE+8+8,
    )]
    pub stats: AccountLoader<'info, Stats>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_liquidity: Box<InterfaceAccount<'info, TokenAccount>>,

    pub price_oracle: Box<Account<'info, PriceUpdateV2>>,
    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}