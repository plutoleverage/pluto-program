use anchor_lang::prelude::*;

#[event]
pub struct EventEarnConfigSet{
    pub fee_vault: Pubkey,
    pub freeze: bool,
    pub protocol_fee: u32,
    pub ltv: u32,
    pub deposit_fee: u32,
    pub min_deposit_limit: u64,
    pub max_deposit_limit: u64,
    pub withdraw_fee: u32,
    pub min_withdraw_limit: u64,
    pub max_withdraw_limit: u64,
    pub borrow_fee: u32,
    pub min_borrow_limit: u64,
    pub max_borrow_limit: u64,
    pub floor_cap_rate: u32,
}