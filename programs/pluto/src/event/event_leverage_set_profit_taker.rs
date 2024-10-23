use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageSetProfitTaker {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub obligation: Pubkey,
    pub position_id: Pubkey,
    pub position_number: u8,
    pub old_state: bool,
    pub new_state: bool,
    pub old_profit: u32,
    pub new_profit: u32,
    pub old_take: u32,
    pub new_take: u32,
}