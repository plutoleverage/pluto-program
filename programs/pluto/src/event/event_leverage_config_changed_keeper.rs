use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageConfigChangeKeeper {
    pub old_keeper: Pubkey,
    pub keeper: Pubkey,
}