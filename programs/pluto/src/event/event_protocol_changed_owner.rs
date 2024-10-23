use anchor_lang::prelude::*;

#[event]
pub struct EventProtocolChangeOwner {
    pub old_owner: Pubkey,
    pub owner: Pubkey,
}