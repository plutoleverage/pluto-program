use anchor_lang::prelude::*;

#[event]
pub struct EventProtocolCreated {
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub freeze: bool,
    pub freeze_earn: bool,
    pub freeze_lend: bool,
    pub freeze_leverage: bool,
}