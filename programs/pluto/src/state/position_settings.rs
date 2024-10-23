use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PositionSettings{
    pub safety_mode: bool,
    pub emergency_eject: bool,
    pub profit_taker: bool,
    pub profit_target_rate: u32,
    pub profit_taking_rate: u32,
}