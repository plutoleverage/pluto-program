use anchor_lang::prelude::*;

#[event]
pub struct EventVaultEarnChangedPriceOracle {
    pub vault: Pubkey,
    pub old_price_oracle: Pubkey,
    pub new_price_oracle: Pubkey,
    pub old_price_feed: [u8; 64],
    pub new_price_feed: [u8; 64],
}