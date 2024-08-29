use anchor_lang::prelude::*;

#[event]
pub struct EventVaultEarnCreated {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub ltv: u16,
    pub deposit_limit: u64,
    pub withdraw_limit: u64,
    pub index: u128,
}