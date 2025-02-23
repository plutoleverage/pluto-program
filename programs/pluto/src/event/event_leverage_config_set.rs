use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageConfigSet {
    pub fee_vault: Pubkey,
    pub freeze: bool,
    pub protocol_fee: u32,
    pub min_leverage: u32,
    pub max_leverage: u32,
    pub leverage_step: u32,
    pub leverage_fee: u32,
    pub min_leverage_limit: u64,
    pub max_leverage_limit: u64,
    pub deleverage_fee: u32,
    pub min_deleverage_limit: u64,
    pub max_deleverage_limit: u64,
    pub closing_fee: u32,
    pub spread_rate: u32,
    pub liquidation_fee: u32,
    pub liquidation_threshold: u32,
    pub liquidation_protocol_ratio: u32,
    pub slippage_rate: u32,
    pub emergency_eject_period: i64,
    pub saver_threshold: u32,
    pub saver_target_reduction: u32,
}