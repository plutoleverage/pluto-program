use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;

#[derive(InitSpace, Derivative, Default, Copy, PartialEq)]
#[derivative(Debug)]
#[account]
pub struct Investor {
    pub bump: u8,
    pub owner: Pubkey,
    pub program_account: Pubkey,
    pub token_mint: Pubkey,
    pub principal: u64,
    pub borrowed: u64,
    pub unit: u64,
    pub index: u128,
}