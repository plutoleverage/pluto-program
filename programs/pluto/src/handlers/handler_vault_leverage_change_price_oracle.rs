use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::Errors;
use crate::event::{EventVaultLeverageChangedPriceOracle};
use crate::state::{Protocol, VaultLeverage};

pub fn handle(ctx: Context<VaultLeverageChangePriceOracle>, token_collateral_price_feed: [u8; 64], native_collateral_price_feed: [u8; 64]) -> Result<()> {
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let owner = &mut ctx.accounts.owner;
    let old_token_collateral_price_oracle = vault.token_collateral_price_oracle;
    let old_token_collateral_price_feed = vault.token_collateral_price_feed;
    let old_native_collateral_price_oracle = vault.native_collateral_price_oracle;
    let old_native_collateral_price_feed = vault.native_collateral_price_feed;

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("old principal price oracle address: {:?}", old_token_collateral_price_oracle);
    msg!("new principal price oracle address: {:?}", ctx.accounts.token_collateral_price_oracle.key());
    msg!("old principal price feed: {:?}", old_token_collateral_price_feed);
    msg!("new principal price feed: {:?}", token_collateral_price_feed);
    msg!("old collateral price oracle address: {:?}", old_native_collateral_price_oracle);
    msg!("new collateral price oracle address: {:?}", ctx.accounts.native_collateral_price_oracle.key());
    msg!("old collateral price feed: {:?}", old_native_collateral_price_feed);
    msg!("new collateral price feed: {:?}", native_collateral_price_feed);

    vault.change_price_oracle(ctx.accounts.token_collateral_price_oracle.key(), token_collateral_price_feed, ctx.accounts.native_collateral_price_oracle.key(), native_collateral_price_feed)?;

    emit!(EventVaultLeverageChangedPriceOracle {
        vault: ctx.accounts.vault.key(),
        old_token_collateral_price_oracle,
        new_token_collateral_price_oracle: ctx.accounts.token_collateral_price_oracle.key(),
        old_token_collateral_price_feed,
        new_token_collateral_price_feed: token_collateral_price_feed,
        old_native_collateral_price_oracle,
        new_native_collateral_price_oracle: ctx.accounts.native_collateral_price_oracle.key(),
        old_native_collateral_price_feed,
        new_native_collateral_price_feed: native_collateral_price_feed,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageChangePriceOracle<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,

    pub token_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,
    pub native_collateral_price_oracle: Box<Account<'info, PriceUpdateV2>>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}