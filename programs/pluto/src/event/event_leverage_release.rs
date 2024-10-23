use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageRelease {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub obligation: Pubkey,
    pub user_ata: Pubkey,
    pub native_collateral_vault_liquidity: Pubkey,
    pub position_number: u8,
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_token_decimals: u8,
    pub unit: u64,
    pub index: u128,
    pub borrowing_unit: u64,
    pub borrowing_index: u128,
    pub release_amount: u64,
    pub release_unit: u64,
    pub release_index: u128,
    pub release_rate: u64,
    pub repay_amount: u64,
    pub repay_unit: u64,
    pub repay_index: u128,
}