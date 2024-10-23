use anchor_lang::prelude::*;

#[event]
pub struct EventVaultLeverageCreated {
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub leverage_stats: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub leverage_config: Pubkey,
    pub borrow_vault: Pubkey,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8; 64],
    pub token_collateral_token_mint: Pubkey,
    pub token_collateral_token_decimals: u8,
    pub token_collateral_vault_ata: Pubkey,
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_token_decimals: u8,
    pub native_collateral_vault_ata: Pubkey,
    pub borrowing_index: u128,
    pub index: u128,
}