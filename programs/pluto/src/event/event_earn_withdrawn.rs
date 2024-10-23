use anchor_lang::prelude::*;

#[event]
pub struct EventEarnWithdrawn {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub lender: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub unit: u64,
    pub index: u128,
    pub pending_amount: u64,
    pub pending_unit: u64,
    pub pending_index: u128,
    pub unit_supply: u128,
    pub vault_index: u128,
    pub protocol_fee: u32,
    pub align0: [u8; 4],
    pub protocol_fee_amount: u64,
    pub fee_amount: u64,
    pub padding: [u64; 32]
}