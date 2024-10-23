use anchor_lang::prelude::*;

#[event]
pub struct EventVaultEarnChangeOwner {
    pub old_owner: Pubkey,
    pub owner: Pubkey,
}