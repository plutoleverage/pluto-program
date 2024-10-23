use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::Errors;
use crate::event::{EventVaultEarnChangedPriceOracle};
use crate::state::{Protocol, VaultEarn};

pub fn handle(ctx: Context<VaultEarnChangePriceOracle>, price_feed: [u8; 64]) -> Result<()> {
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let owner = &mut ctx.accounts.owner;
    let old_price_oracle = vault.price_oracle;
    let old_price_feed = vault.price_feed;
    vault.change_price_oracle(ctx.accounts.price_oracle.key(), price_feed)?;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("old price oracle address: {:?}", old_price_oracle);
    msg!("new price oracle address: {:?}", ctx.accounts.price_oracle.key());
    msg!("old price feed: {:?}", old_price_feed);
    msg!("new price feed: {:?}", price_feed);

    emit!(EventVaultEarnChangedPriceOracle {
        vault: ctx.accounts.vault.key(),
        old_price_oracle,
        new_price_oracle: ctx.accounts.price_oracle.key(),
        old_price_feed,
        new_price_feed: price_feed,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct VaultEarnChangePriceOracle<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = price_oracle,
    )]
    pub vault: AccountLoader<'info, VaultEarn>,

    pub price_oracle: Box<Account<'info, PriceUpdateV2>>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}