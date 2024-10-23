use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageOpen {
    pub borrow_vault: Pubkey,
    pub vault: Pubkey,
    pub user: Pubkey,
    pub obligation: Pubkey,
    pub position_number: u8,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8;64],
    pub token_collateral_token_mint: Pubkey,
    pub token_collateral_token_decimals: u8,
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8;64],
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_token_decimals: u8,
    pub leveraged_amount: u64,
    pub min_native_collateral_output: u64,
    pub real_native_collateral_output: u64,
    pub unit: u64,
    pub index: u128,
}