use anchor_lang::{InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{Errors};
use crate::util::action::LeverageAction;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[zero_copy(unsafe)]
#[repr(C)]
pub struct PositionState {
    pub action: LeverageAction,
    #[derivative(Debug = "ignore")]
    pub align01: [u8; 7],
    // ORACLE
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8; 64],
    pub token_collateral_price: u64,
    pub token_collateral_price_exponent: u32,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 4],
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_price: u64,
    pub native_collateral_price_exponent: u32,
    // CONFIG
    pub protocol_fee: u32,
    pub leverage_fee: u32,
    pub deleverage_fee: u32,
    pub closing_fee: u32,
    pub spread_rate: u32,
    pub liquidation_fee: u32,
    pub liquidation_threshold: u32,
    pub liquidation_protocol_ratio: u32,
    pub slippage_rate: u32,
    pub emergency_eject_period: i64,
    pub saver_threshold: u32,
    pub saver_target_reduction: u32,
    // LEVERAGE
    pub fund_amount: u64,
    pub leverage_fee_amount: u64,
    pub borrow_amount: u64,
    pub borrowing_fee_amount: u64,
    pub borrowing_unit: u64,
    pub borrowing_index: u128,
    pub leveraged_amount: u64,
    pub min_native_collateral_output: u64,
    // DELEVERAGE
    pub release_amount: u64,
    pub release_unit: u64,
    pub release_index: u128,
    pub release_rate: u32,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 4],
    pub repay_amount: u64,
    pub repay_unit: u64,
    pub repay_index: u128,
    pub release_min_output: u64,
    pub release_current_leverage: u32,
    pub release_target_leverage: u32,
    pub utilization_rate: u32,
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 4],
    pub protocol_fee_factor: u128,
    pub protocol_fee_amount: u64,
    pub repay_borrow_amount: u64,
    // Liquidation
    pub liquidation_fee_amount: u64,
    pub health_factor: u32,
    #[derivative(Debug = "ignore")]
    pub align3: [u8; 4],
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 63],
}

impl Default for PositionState {
    fn default() -> Self {
        Self {
            action: LeverageAction::Idle,
            align01: [0; 7],
            token_collateral_price_oracle: Default::default(),
            token_collateral_price_feed: [0; 64],
            token_collateral_price: 0,
            token_collateral_price_exponent: 0,
            align0: [0; 4],
            native_collateral_price_oracle: Default::default(),
            native_collateral_price_feed: [0; 64],
            native_collateral_price: 0,
            native_collateral_price_exponent: 0,
            protocol_fee: 0,
            leverage_fee: 0,
            deleverage_fee: 0,
            closing_fee: 0,
            spread_rate: 0,
            liquidation_fee: 0,
            liquidation_threshold: 0,
            liquidation_protocol_ratio: 0,
            slippage_rate: 0,
            emergency_eject_period: 0,
            saver_threshold: 0,
            saver_target_reduction: 0,
            fund_amount: 0,
            leverage_fee_amount: 0,
            borrow_amount: 0,
            borrowing_unit: 0,
            borrowing_index: 0,
            borrowing_fee_amount: 0,
            leveraged_amount: 0,
            min_native_collateral_output: 0,
            release_amount: 0,
            release_unit: 0,
            release_index: 0,
            release_rate: 0,
            align1: [0; 4],
            repay_amount: 0,
            repay_unit: 0,
            repay_index: 0,
            release_min_output: 0,
            release_current_leverage: 0,
            release_target_leverage: 0,
            utilization_rate: 0,
            align2: [0; 4],
            protocol_fee_factor: 0,
            protocol_fee_amount: 0,
            repay_borrow_amount: 0,
            liquidation_fee_amount: 0,
            health_factor: 0,
            align3: [0; 4],
            padding1: [0; 63],
        }
    }
}

impl PositionState {
    pub fn new() -> Self {
        Self {
            action: LeverageAction::Idle,
            align01: [0; 7],
            token_collateral_price_oracle: Default::default(),
            token_collateral_price_feed: [0; 64],
            token_collateral_price: 0,
            token_collateral_price_exponent: 0,
            align0: [0; 4],
            native_collateral_price_oracle: Default::default(),
            native_collateral_price_feed: [0; 64],
            native_collateral_price: 0,
            native_collateral_price_exponent: 0,
            protocol_fee: 0,
            leverage_fee: 0,
            deleverage_fee: 0,
            closing_fee: 0,
            spread_rate: 0,
            liquidation_fee: 0,
            liquidation_threshold: 0,
            liquidation_protocol_ratio: 0,
            slippage_rate: 0,
            emergency_eject_period: 0,
            saver_threshold: 0,
            saver_target_reduction: 0,
            fund_amount: 0,
            leverage_fee_amount: 0,
            borrow_amount: 0,
            borrowing_unit: 0,
            borrowing_index: 0,
            borrowing_fee_amount: 0,
            leveraged_amount: 0,
            min_native_collateral_output: 0,
            release_amount: 0,
            release_unit: 0,
            release_index: 0,
            release_rate: 0,
            align1: [0; 4],
            repay_amount: 0,
            repay_unit: 0,
            repay_index: 0,
            release_min_output: 0,
            release_current_leverage: 0,
            release_target_leverage: 0,
            utilization_rate: 0,
            align2: [0; 4],
            protocol_fee_factor: 0,
            protocol_fee_amount: 0,
            repay_borrow_amount: 0,
            liquidation_fee_amount: 0,
            health_factor: 0,
            align3: [0; 4],
            padding1: [0; 63],
        }
    }

    pub fn halt_on_leveraging(&mut self) -> Result<()> {
        match self.action {
            LeverageAction::Idle => {
                Ok(())
            }
            LeverageAction::Open => {
                Err(Errors::IncompleteLeveragingProcess.into())
            }
            LeverageAction::AddCollateral => {
                Err(Errors::IncompleteLeveragingProcess.into())
            }
            LeverageAction::AddPosition => {
                Err(Errors::IncompleteLeveragingProcess.into())
            }
            LeverageAction::Close => {
                Ok(())
            }
            LeverageAction::Safe => {
                Ok(())
            }
            LeverageAction::Eject => {
                Ok(())
            }
            LeverageAction::Liquidate => {
                Ok(())
            }
            LeverageAction::Deleverage => {
                Ok(())
            }
            LeverageAction::TakeProfit => {
                Ok(())
            }
        }
    }

    pub fn halt_on_deleveraging(&mut self) -> Result<()> {
        match self.action {
            LeverageAction::Idle => {
                Ok(())
            }
            LeverageAction::Open => {
                Ok(())
            }
            LeverageAction::AddCollateral => {
                Ok(())
            }
            LeverageAction::AddPosition => {
                Ok(())
            }
            LeverageAction::Close => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
            LeverageAction::Safe => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
            LeverageAction::Eject => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
            LeverageAction::Liquidate => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
            LeverageAction::Deleverage => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
            LeverageAction::TakeProfit => {
                Err(Errors::IncompleteDeleveragingProcess.into())
            }
        }
    }
}