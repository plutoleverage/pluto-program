use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageClose {
    pub borrow_vault: Pubkey,
    pub vault: Pubkey,
    pub user: Pubkey,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_token_mint: Pubkey,
    pub token_collateral_token_decimals: u8,
    pub token_collateral_vault_ata: Pubkey,
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_token_decimals: u8,
    pub native_collateral_vault_ata: Pubkey,
    pub unit: u64,
    pub index: u128,
    pub token_collateral_amount: u64,
    pub token_collateral_price: u64,
    pub token_collateral_decimals: u8,
    pub token_collateral_usd: u128,
    pub native_collateral_amount: u64,
    pub native_collateral_price: u64,
    pub native_collateral_decimals: u8,
    pub native_collateral_usd: u128,
    pub borrowing_unit: u64,
    pub borrowing_index: u128,
    pub borrowing_repaid_amount: u64,
    pub borrowing_usd: u128,
    pub fee_amount: u64,
    pub fee_usd: u64,
}