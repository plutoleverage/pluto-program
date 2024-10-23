use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageBorrow {
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
    pub old_borrowing_amount: u64,
    pub old_borrowing_unit: u64,
    pub old_borrowing_index: u128,
    pub borrowing_amount: u64,
    pub borrowing_unit: u64,
    pub borrowing_index: u128,
    pub borrow_fee_vault: Pubkey,
    pub borrow_fee: u32,
    pub borrow_fee_amount: u64,
}