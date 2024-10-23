use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageSetEmergencyEject {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub obligation: Pubkey,
    pub position_id: Pubkey,
    pub position_number: u8,
    pub old_state: bool,
    pub new_state: bool,
}