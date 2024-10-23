use anchor_lang::prelude::*;

#[event]
pub struct EventVaultEarnCreated {
    pub protocol: Pubkey,
    pub earn_stats: Pubkey,
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub earn_config: Pubkey,
    pub vault_ata: Pubkey,
    pub price_oracle: Pubkey,
    pub price_feed: [u8; 64],
    pub token_program: Pubkey,
    pub token_mint: Pubkey,
    pub token_decimal: u8,
    pub index: u128,
}