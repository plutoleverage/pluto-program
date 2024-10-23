use anchor_lang::prelude::*;

#[event]
pub struct EventVaultLeverageChangeOwner {
    pub old_owner: Pubkey,
    pub owner: Pubkey,
}