use anchor_lang::prelude::*;

#[event]
pub struct EventVaultLeverageChangedPriceOracle {
    pub vault: Pubkey,
    pub old_token_collateral_price_oracle: Pubkey,
    pub new_token_collateral_price_oracle: Pubkey,
    pub old_token_collateral_price_feed: [u8; 64],
    pub new_token_collateral_price_feed: [u8; 64],
    pub old_native_collateral_price_oracle: Pubkey,
    pub new_native_collateral_price_oracle: Pubkey,
    pub old_native_collateral_price_feed: [u8; 64],
    pub new_native_collateral_price_feed: [u8; 64],
}