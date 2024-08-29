use anchor_lang::prelude::*;

#[event]
pub struct EventWithdrawEarn {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub index: u128,
    pub unit: u128,
    pub burn_unit: u128,
    pub unit_supply: u128,
}