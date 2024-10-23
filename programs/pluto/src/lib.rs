mod event;
mod state;
mod error;
mod handlers;
mod util;

use anchor_lang::prelude::*;
use crate::handlers::*;
use crate::state::PositionSettings;

declare_id!("BeaiD9HF7V2Byz6Md6bWn6B3Zq7Djry2gt4KK9oUwjgZ");
//declare_id!("G7x8ig9axyVrLZZY8WgrNhZqWwWoWoJTrUdj3dsefpkf");
//declare_id!("5UFYdXHgXLMsDzHyv6pQW9zv3fNkRSNqHwhR7UPnkhzy");

#[program]
pub mod pluto {
    use super::*;

    #[inline(never)]
    pub fn wrap_sol(ctx: Context<WrapSol>, amount: u64) -> Result<()> {
        handler_wrap_sol::handle(ctx, amount)
    }

    #[inline(never)]
    pub fn unwrap_sol(ctx: Context<UnwrapSol>, amount: u64) -> Result<()> {
        handler_unwrap_sol::handle(ctx, amount)
    }

    #[inline(never)]
    pub fn protocol_create(ctx: Context<ProtocolCreate>, freeze: bool, freeze_earn: bool, freeze_lend: bool, freeze_leverage: bool) -> Result<()> {
        handler_protocol_create::handle(ctx, freeze, freeze_earn, freeze_lend, freeze_leverage)
    }

    #[inline(never)]
    pub fn protocol_set(ctx: Context<ProtocolSet>, freeze: bool, freeze_earn: bool, freeze_lend: bool, freeze_leverage: bool) -> Result<()> {
        handler_protocol_set::handle(ctx, freeze, freeze_earn, freeze_lend, freeze_leverage)
    }

    #[inline(never)]
    pub fn protocol_change_owner(ctx: Context<ProtocolChangeOwner>, new_owner: Pubkey) -> Result<()> {
        handler_protocol_change_owner::handle(ctx, new_owner)
    }

    #[inline(never)]
    pub fn earn_config_create(ctx: Context<EarnConfigCreate>, freeze: bool, protocol_fee: u32, ltv: u32, deposit_fee: u32, min_deposit_limit: u64, max_deposit_limit: u64, withdraw_fee: u32, min_withdraw_limit: u64, max_withdraw_limit: u64, borrow_fee: u32, min_borrow_limit: u64, max_borrow_limit: u64, floor_cap_rate: u32) -> Result<()> {
        handler_earn_config_create::handle(ctx, freeze, protocol_fee, ltv, deposit_fee, min_deposit_limit, max_deposit_limit, withdraw_fee, min_withdraw_limit, max_withdraw_limit, borrow_fee, min_borrow_limit, max_borrow_limit, floor_cap_rate)
    }

    #[inline(never)]
    pub fn earn_config_set(ctx: Context<EarnConfigSet>, freeze: bool, protocol_fee: u32, ltv: u32, deposit_fee: u32, min_deposit_limit: u64, max_deposit_limit: u64, withdraw_fee: u32, min_withdraw_limit: u64, max_withdraw_limit: u64, borrow_fee: u32, min_borrow_limit: u64, max_borrow_limit: u64, floor_cap_rate: u32) -> Result<()> {
        handler_earn_config_set::handle(ctx, freeze, protocol_fee, ltv, deposit_fee, min_deposit_limit, max_deposit_limit, withdraw_fee, min_withdraw_limit, max_withdraw_limit, borrow_fee, min_borrow_limit, max_borrow_limit, floor_cap_rate)
    }

    #[inline(never)]
    pub fn earn_config_change_indexer(ctx: Context<EarnConfigChangeIndexer>, new_indexer: Pubkey) -> Result<()> {
        handler_earn_config_change_indexer::handle(ctx, new_indexer)
    }

    #[inline(never)]
    pub fn earn_vault_create(ctx: Context<VaultEarnCreate>, token_decimal: [u8; 64]) -> Result<()> {
        handler_vault_earn_create::handle(ctx, token_decimal)
    }

    #[inline(never)]
    pub fn earn_vault_change_price_oracle(ctx: Context<VaultEarnChangePriceOracle>, token_decimal: [u8; 64]) -> Result<()> {
        handler_vault_earn_change_price_oracle::handle(ctx, token_decimal)
    }

    #[inline(never)]
    pub fn earn_vault_deposit(ctx: Context<VaultEarnDeposit>, amount: u64) -> Result<()> {
        handler_vault_earn_deposit::handle(ctx, amount)
    }

    #[inline(never)]
    pub fn earn_vault_withdraw(ctx: Context<VaultEarnWithdraw>, unit: u64, min_output_amount: u64) -> Result<()> {
        handler_vault_earn_withdraw::handle(ctx, unit, min_output_amount)
    }

    #[inline(never)]
    pub fn leverage_config_create(ctx: Context<LeverageConfigCreate>, freeze: bool, protocol_fee: u32, min_leverage: u32, max_leverage: u32, leverage_step: u32, leverage_fee: u32, min_leverage_limit: u64, max_leverage_limit: u64, deleverage_fee: u32, min_deleverage_limit: u64, max_deleverage_limit: u64, closing_fee: u32, spread_rate: u32, liquidation_fee: u32, liquidation_threshold: u32, liquidation_protocol_ratio: u32, slippage_rate: u32, emergency_eject_period: i64, saver_threshold: u32, saver_target_reduction: u32) -> Result<()> {
        handler_leverage_config_create::handle(ctx, freeze, protocol_fee, min_leverage, max_leverage, leverage_step, leverage_fee, min_leverage_limit, max_leverage_limit, deleverage_fee, min_deleverage_limit, max_deleverage_limit, closing_fee, spread_rate, liquidation_fee, liquidation_threshold, liquidation_protocol_ratio, slippage_rate, emergency_eject_period, saver_threshold, saver_target_reduction)
    }

    #[inline(never)]
    pub fn leverage_config_set(ctx: Context<LeverageConfigSet>, freeze: bool, protocol_fee: u32, min_leverage: u32, max_leverage: u32, leverage_step: u32, leverage_fee: u32, min_leverage_limit: u64, max_leverage_limit: u64, deleverage_fee: u32, min_deleverage_limit: u64, max_deleverage_limit: u64, closing_fee: u32, spread_rate: u32, liquidation_fee: u32, liquidation_threshold: u32, liquidation_protocol_ratio: u32, slippage_rate: u32, emergency_eject_period: i64, saver_threshold: u32, saver_target_reduction: u32) -> Result<()> {
        handler_leverage_config_set::handle(ctx, freeze, protocol_fee, min_leverage, max_leverage, leverage_step, leverage_fee, min_leverage_limit, max_leverage_limit, deleverage_fee, min_deleverage_limit, max_deleverage_limit, closing_fee, spread_rate, liquidation_fee, liquidation_threshold, liquidation_protocol_ratio, slippage_rate, emergency_eject_period, saver_threshold, saver_target_reduction)
    }

    #[inline(never)]
    pub fn leverage_config_change_indexer(ctx: Context<LeverageConfigChangeIndexer>, new_indexer: Pubkey) -> Result<()> {
        handler_leverage_config_change_indexer::handle(ctx, new_indexer)
    }

    #[inline(never)]
    pub fn leverage_config_change_keeper(ctx: Context<LeverageConfigChangeKeeper>, new_keeper: Pubkey) -> Result<()> {
        handler_leverage_config_change_keeper::handle(ctx, new_keeper)
    }

    #[inline(never)]
    pub fn leverage_vault_create(ctx: Context<VaultLeverageCreate>, token_collateral_decimal: [u8; 64], native_collateral_decimal: [u8; 64]) -> Result<()> {
        handler_vault_leverage_create::handle(ctx, token_collateral_decimal, native_collateral_decimal)
    }

    #[inline(never)]
    pub fn leverage_vault_create_liquidity(ctx: Context<VaultLeverageCreateLiquidity>) -> Result<()> {
        handler_vault_leverage_create_liquidity::handle(ctx)
    }

    #[inline(never)]
    pub fn leverage_vault_change_price_oracle(ctx: Context<VaultLeverageChangePriceOracle>, token_collateral_decimal: [u8; 64], native_collateral_decimal: [u8; 64]) -> Result<()> {
        handler_vault_leverage_change_price_oracle::handle(ctx, token_collateral_decimal, native_collateral_decimal)
    }

    #[inline(never)]
    pub fn leverage_vault_fund(ctx: Context<VaultLeverageFund>, settings: PositionSettings, amount: u64, leverage: u32) -> Result<()> {
        handler_vault_leverage_fund::handle(ctx, settings, amount, leverage)
    }

    #[inline(never)]
    pub fn leverage_vault_confiscate(ctx: Context<VaultLeverageConfiscate>) -> Result<()> {
        handler_vault_leverage_confiscate::handle(ctx)
    }

    #[inline(never)]
    pub fn leverage_vault_set_safety_mode(ctx: Context<VaultLeverageSetSafetyMode>, number:u8, safety_mode: bool) -> Result<()> {
        handler_vault_leverage_set_safety_mode::handle(ctx, number, safety_mode)
    }

    #[inline(never)]
    pub fn leverage_vault_set_emergency_eject(ctx: Context<VaultLeverageSetEmergencyEject>, number:u8, emergency_eject: bool) -> Result<()> {
        handler_vault_leverage_set_emergency_eject::handle(ctx, number, emergency_eject)
    }

    #[inline(never)]
    pub fn leverage_vault_set_profit_taker(ctx: Context<VaultLeverageSetProfitTaker>, number:u8, profit_taker: bool, profit: u32, take: u32) -> Result<()> {
        handler_vault_leverage_set_profit_taker::handle(ctx, number, profit_taker, profit, take)
    }

    #[inline(never)]
    pub fn leverage_vault_close(ctx: Context<VaultLeverageClose>, number: u8) -> Result<()> {
        handler_vault_leverage_close::handle(ctx, number)
    }

    #[inline(never)]
    pub fn leverage_vault_release(ctx: Context<VaultLeverageRelease>, number: u8) -> Result<()> {
        handler_vault_leverage_release::handle(ctx, number)
    }

    #[inline(never)]
    pub fn leverage_vault_repay_borrow(ctx: Context<VaultLeverageRepayBorrow>, number: u8) -> Result<()> {
        handler_vault_leverage_repay_borrow::handle(ctx, number)
    }

    #[inline(never)]
    pub fn leverage_vault_closing(ctx: Context<VaultLeverageClosing>, number: u8) -> Result<()> {
        handler_vault_leverage_closing::handle(ctx, number)
    }
}
