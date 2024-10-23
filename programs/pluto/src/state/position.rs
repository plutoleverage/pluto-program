use anchor_lang::{InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::{ErrorMath, Errors};
use crate::error::ErrorMath::MathOverflow;
use crate::state::{LeverageConfig, PositionState};
use crate::util::{
    constant::{INDEX_DECIMALS, UNIT_DECIMALS},
    decimals,
};
use crate::util::action::LeverageAction;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[zero_copy(unsafe)]
#[repr(C)]
pub struct Position {
    pub owner: Pubkey,
    pub id: Pubkey,
    pub tag_id: [u8; 64],
    pub number: i8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 7],
    pub open_at: i64,
    pub last_updated: i64,
    pub emergency_eject: bool,
    pub safety_mode: bool,
    pub safety_level: u8,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 5],
    pub token_collateral_amount: u64,
    pub token_to_native_ratio: u128,
    pub borrowing_unit: u64,
    pub avg_borrowing_index: u128,
    pub unit: u64,
    pub avg_index: u128,
    pub state: PositionState,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8; 64],
    pub token_collateral_price: u64,
    pub token_collateral_price_exponent: u32,
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 4],
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_price: u64,
    pub native_collateral_price_exponent: u32,
    pub profit_taker: bool,
    #[derivative(Debug = "ignore")]
    pub align3: [u8; 3],
    pub profit_target_rate: u32,
    pub profit_taking_rate: u32,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 63],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            owner: Pubkey::default(),
            id: Pubkey::default(),
            tag_id: [0; 64],
            number: -1,
            align0: [0;7],
            open_at: 0,
            last_updated: 0,
            emergency_eject: false,
            safety_mode: false,
            safety_level: 0,
            align1: [0;5],
            token_collateral_amount: 0,
            token_to_native_ratio: 0,
            borrowing_unit: 0,
            avg_borrowing_index: 0,
            unit: 0,
            avg_index: 0,
            state: PositionState::default(),
            token_collateral_price_oracle: Pubkey::default(),
            token_collateral_price_feed: [0; 64],
            token_collateral_price: 0,
            token_collateral_price_exponent: 0,
            align2: [0; 4],
            native_collateral_price_oracle: Pubkey::default(),
            native_collateral_price_feed: [0; 64],
            native_collateral_price: 0,
            native_collateral_price_exponent: 0,
            align3: [0; 3],
            profit_taker: false,
            padding1: [0; 63],
            profit_target_rate: 0,
            profit_taking_rate: 0,
        }
    }
}

impl Position {
    pub fn new() -> Self {
        Self {
            owner: Pubkey::default(),
            id: Pubkey::default(),
            tag_id: [0; 64],
            number: -1,
            align0: [0;7],
            open_at: 0,
            last_updated: 0,
            emergency_eject: false,
            safety_mode: false,
            safety_level: 0,
            align1: [0;5],
            token_collateral_amount: 0,
            token_to_native_ratio: 0,
            borrowing_unit: 0,
            avg_borrowing_index: 0,
            unit: 0,
            avg_index: 0,
            state: PositionState::new(),
            token_collateral_price_oracle: Pubkey::default(),
            token_collateral_price_feed: [0; 64],
            token_collateral_price: 0,
            token_collateral_price_exponent: 0,
            align2: [0; 4],
            native_collateral_price_oracle: Pubkey::default(),
            native_collateral_price_feed: [0; 64],
            native_collateral_price: 0,
            native_collateral_price_exponent: 0,
            align3: [0; 3],
            profit_taker: false,
            padding1: [0; 63],
            profit_target_rate: 0,
            profit_taking_rate: 0,
        }
    }

    pub fn init(&mut self, params: InitPositionParams) -> Result<()> {
        let clock = Clock::get()?;
        *self = Self::default();
        self.owner = params.owner;
        self.id = params.id;
        self.last_updated = clock.unix_timestamp;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        *self = Self::default();

        Ok(())
    }

    pub fn borrowing_open_amount(&mut self, token_decimal: u8) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        // Ceil to prevent less debt from rounding
        let val = decimals::mul_ceil(token_decimal, self.borrowing_unit as u128, UNIT_DECIMALS, self.avg_borrowing_index, INDEX_DECIMALS)?;
        Ok(val as u64)
    }

    pub fn borrowing_amount(&mut self, token_decimal: u8, index: u128) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        // Ceil to prevent less debt from rounding
        let val = decimals::mul_ceil(token_decimal, self.borrowing_unit as u128, UNIT_DECIMALS, index, INDEX_DECIMALS)?;
        Ok(val as u64)
    }

    pub fn collateral_open_amount(&mut self, token_decimal: u8) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        // Floor to prevent extra collateral from rounding
        let val = decimals::mul_floor(token_decimal, self.unit as u128, UNIT_DECIMALS, self.avg_index, INDEX_DECIMALS)?;
        Ok(val as u64)
    }

    pub fn collateral_amount(&mut self, token_decimal: u8, index: u128) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        // Floor to prevent extra collateral from rounding
        let val = decimals::mul_floor(token_decimal, self.unit as u128, UNIT_DECIMALS, index, INDEX_DECIMALS)?;
        Ok(val as u64)
    }

    pub fn set_action(&mut self, action: LeverageAction) -> Result<()> {
        self.state.action = action;
        Ok(())
    }

    pub fn set_config(&mut self, config: &LeverageConfig) -> Result<()> {
        self.state.protocol_fee = config.protocol_fee;
        self.state.leverage_fee = config.leverage_fee;
        self.state.deleverage_fee = config.deleverage_fee;
        self.state.closing_fee = config.closing_fee;
        self.state.spread_rate = config.spread_rate;
        self.state.liquidation_fee = config.liquidation_fee;
        self.state.liquidation_threshold = config.liquidation_threshold;
        self.state.liquidation_protocol_ratio = config.liquidation_protocol_ratio;
        self.state.slippage_rate = config.slippage_rate;
        self.state.emergency_eject_period = config.emergency_eject_period;
        self.state.saver_threshold = config.saver_threshold;
        self.state.saver_target_reduction = config.saver_target_reduction;
        Ok(())
    }

    pub fn set_oracle(
        &mut self,
        token_collateral_price_oracle: Pubkey,
        token_collateral_price_feed: [u8; 64],
        token_collateral_price: u64,
        token_collateral_price_exponent: u32,
        native_collateral_price_oracle: Pubkey,
        native_collateral_price_feed: [u8; 64],
        native_collateral_price: u64,
        native_collateral_price_exponent: u32,
    ) -> Result<()> {
        self.state.token_collateral_price_oracle = token_collateral_price_oracle;
        self.state.token_collateral_price_feed = token_collateral_price_feed;
        self.state.token_collateral_price = token_collateral_price;
        self.state.token_collateral_price_exponent = token_collateral_price_exponent;
        self.state.native_collateral_price_oracle = native_collateral_price_oracle;
        self.state.native_collateral_price_feed = native_collateral_price_feed;
        self.state.native_collateral_price = native_collateral_price;
        self.state.native_collateral_price_exponent = native_collateral_price_exponent;

        Ok(())
    }

    pub fn clear_state(&mut self) -> Result<()>{
        self.state = PositionState::new();
        Ok(())
    }

    pub fn halt_on_leveraging(&mut self) -> Result<()> {
        self.state.halt_on_leveraging()
    }

    pub fn halt_on_deleveraging(&mut self) -> Result<()> {
        self.state.halt_on_deleveraging()
    }

    pub fn fund(
        &mut self,
        fund_amount: u64,
        leverage_fee_amount: u64,
    ) -> Result<()> {
        self.halt_on_deleveraging()?;
        require_eq!(self.state.fund_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.leverage_fee_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrow_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrowing_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrowing_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.leveraged_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.min_native_collateral_output, 0, Errors::IncompleteProcess);
        let clock = Clock::get()?;
        self.open_at = clock.unix_timestamp;
        self.state.fund_amount = fund_amount;
        self.state.leverage_fee_amount = leverage_fee_amount;

        Ok(())
    }

    pub fn borrow_fund(
        &mut self,
        borrowing_amount: u64,
        borrowing_unit: u64,
        borrowing_index: u128,
        borrowing_fee_amount: u64,
    ) -> Result<()> {
        self.halt_on_deleveraging()?;
        require_gt!(self.state.fund_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.leverage_fee_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrow_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrowing_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrowing_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.borrowing_fee_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.leveraged_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.min_native_collateral_output, 0, Errors::IncompleteProcess);

        self.state.borrow_amount = borrowing_amount;
        self.state.borrowing_unit = borrowing_unit;
        self.state.borrowing_index = borrowing_index;
        self.state.borrowing_fee_amount = borrowing_fee_amount;

        Ok(())
    }

    pub fn take_fund(&mut self, token_collateral_decimal: u8) -> Result<()> {
        self.halt_on_deleveraging()?;
        require_gt!(self.state.fund_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.leverage_fee_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrow_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_index, 0, Errors::IncompleteProcess);
        require_gte!(self.state.borrowing_fee_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.leveraged_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.min_native_collateral_output, 0, Errors::IncompleteProcess);
        let cur_borrowing_amount = decimals::mul_ceil(token_collateral_decimal, self.borrowing_unit as u128, UNIT_DECIMALS, self.avg_borrowing_index, INDEX_DECIMALS)?;
        let avg_borrowing_index = decimals::div_ceil(
            INDEX_DECIMALS, cur_borrowing_amount.checked_add(self.state.borrow_amount as u128).ok_or(MathOverflow)?, token_collateral_decimal,
            self.state.borrowing_unit.checked_add(self.borrowing_unit).ok_or(MathOverflow)? as u128, UNIT_DECIMALS)?;

        self.token_collateral_amount = self.token_collateral_amount.checked_add(self.state.fund_amount).ok_or(MathOverflow)?;
        self.borrowing_unit = self.borrowing_unit.checked_add(self.state.borrowing_unit).ok_or(MathOverflow)?;
        self.avg_borrowing_index = avg_borrowing_index;

        Ok(())
    }

    pub fn leverage(
        &mut self,
        leveraged_amount: u64,
        min_native_collateral_output: u64
    ) -> Result<()> {
        self.halt_on_deleveraging()?;
        require_gt!(self.state.fund_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.leverage_fee_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrow_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_index, 0, Errors::IncompleteProcess);
        require_gte!(self.state.borrowing_fee_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.leveraged_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.min_native_collateral_output, 0, Errors::IncompleteProcess);
        self.state.leveraged_amount = leveraged_amount;
        self.state.min_native_collateral_output = min_native_collateral_output;

        Ok(())
    }

    pub fn confiscate(
        &mut self,
        native_collateral_decimal: u8,
        token_to_collateral_ratio: u128,
        unit: u64, index: u128,
    ) -> Result<()> {
        self.halt_on_deleveraging()?;
        require_gt!(self.state.fund_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.leverage_fee_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrow_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.borrowing_index, 0, Errors::IncompleteProcess);
        require_gte!(self.state.borrowing_fee_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.leveraged_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.min_native_collateral_output, 0, Errors::IncompleteProcess);
        let cur_position_amount = decimals::mul_ceil(native_collateral_decimal, unit as u128, UNIT_DECIMALS, index, INDEX_DECIMALS)?;
        let avg_position_amount = decimals::mul_floor(native_collateral_decimal, self.unit as u128, UNIT_DECIMALS, self.avg_index, INDEX_DECIMALS)?;
        let avg_index = decimals::div_floor(
            INDEX_DECIMALS,
            avg_position_amount.checked_add(cur_position_amount).ok_or(MathOverflow)?, native_collateral_decimal,
            unit.checked_add(self.unit).ok_or(MathOverflow)? as u128, UNIT_DECIMALS
        )?;
        self.token_to_native_ratio = token_to_collateral_ratio;
        self.avg_index = avg_index;
        self.unit = self.unit.checked_add(unit).ok_or(MathOverflow)?;

        self.token_collateral_price_oracle = self.state.token_collateral_price_oracle;
        self.token_collateral_price_feed = self.state.token_collateral_price_feed;
        self.token_collateral_price = self.state.token_collateral_price;
        self.token_collateral_price_exponent = self.state.token_collateral_price_exponent;
        self.native_collateral_price_oracle = self.state.native_collateral_price_oracle;
        self.native_collateral_price_feed = self.state.native_collateral_price_feed;
        self.native_collateral_price = self.state.native_collateral_price;
        self.native_collateral_price_exponent = self.state.native_collateral_price_exponent;

        self.state = PositionState::default();

        Ok(())
    }

    pub fn release(
        &mut self,
        release_amount: u64, release_unit: u64, release_index: u128,
        release_rate: u32,
        repay_amount: u64, repay_unit: u64, repay_index: u128,
        release_min_output: u64,
    ) -> Result<()> {
        self.halt_on_leveraging()?;
        require_eq!(self.state.release_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_rate, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_min_output, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_factor, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_amount, 0, Errors::IncompleteProcess);
        self.state.release_amount = release_amount;
        self.state.release_unit = release_unit;
        self.state.release_index = release_index;
        self.state.release_rate = release_rate;
        self.state.repay_amount = repay_amount;
        self.state.repay_unit = repay_unit;
        self.state.repay_index = repay_index;
        self.state.release_min_output = release_min_output;

        Ok(())
    }

    pub fn set_health_factor(&mut self, health_factor: u32) -> Result<()> {
        self.state.health_factor = health_factor;
        Ok(())
    }

    pub fn release_reduce(
        &mut self,
        release_amount: u64, release_unit: u64, release_index: u128,
        release_rate: u32,
        repay_amount: u64, repay_unit: u64, repay_index: u128,
        release_min_output: u64, release_current_leverage: u32, release_target_leverage: u32,
    ) -> Result<()> {
        self.halt_on_leveraging()?;
        require_eq!(self.state.release_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_rate, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_unit, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_index, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_min_output, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_current_leverage, 0, Errors::IncompleteProcess);
        require_eq!(self.state.release_target_leverage, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_factor, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_amount, 0, Errors::IncompleteProcess);
        self.state.release_amount = release_amount;
        self.state.release_unit = release_unit;
        self.state.release_index = release_index;
        self.state.release_rate = release_rate;
        self.state.repay_amount = repay_amount;
        self.state.repay_unit = repay_unit;
        self.state.repay_index = repay_index;
        self.state.release_min_output = release_min_output;
        self.state.release_current_leverage = release_current_leverage;
        self.state.release_target_leverage = release_target_leverage;

        Ok(())
    }

    pub fn pay_protocol_fee(
        &mut self,
        utilization_rate: u32,
        protocol_fee_factor: u128,
        protocol_fee_amount: u64,
    ) -> Result<()> {
        self.halt_on_leveraging()?;
        msg!("release_amount: {:?} release_unit: {:?} release_index: {:?} release_rate: {:?} repay_amount: {:?} repay_unit: {:?} repay_index: {:?} release_min_output: {:?} utilization_rate: {:?} protocol_fee_factor: {:?} protocol_fee_amount: {:?}",
             self.state.release_amount, self.state.release_unit, self.state.release_index, self.state.release_rate, self.state.repay_amount, self.state.repay_unit, self.state.repay_index, self.state.release_min_output, self.state.utilization_rate, self.state.protocol_fee_factor, self.state.protocol_fee_amount);
        require_gt!(self.state.release_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_rate, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_unit, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_min_output, 0, Errors::IncompleteProcess);
        require_eq!(self.state.utilization_rate, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_factor, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_amount, 0, Errors::IncompleteProcess);
        self.state.utilization_rate = utilization_rate;
        self.state.protocol_fee_factor = protocol_fee_factor;
        self.state.protocol_fee_amount = protocol_fee_amount;

        Ok(())
    }

    pub fn pay_liquidation_fee(
        &mut self,
        liquidation_fee_amount: u64
    ) -> Result<()> {
        require_eq!(self.state.liquidation_fee_amount, 0, Errors::IncompleteProcess);
        self.state.liquidation_fee_amount = liquidation_fee_amount;

        Ok(())
    }

    pub fn repay_borrow(
        &mut self,
        repay_borrow_amount: u64,
    ) -> Result<()> {
        self.halt_on_leveraging()?;
        require_gt!(self.state.release_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_rate, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_unit, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_min_output, 0, Errors::IncompleteProcess);
        require_eq!(self.state.repay_borrow_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.state.utilization_rate, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_factor, 0, Errors::IncompleteProcess);
        require_eq!(self.state.protocol_fee_amount, 0, Errors::IncompleteProcess);
        self.state.repay_borrow_amount = repay_borrow_amount;

        Ok(())
    }

    pub fn closing(
        &mut self,
    ) -> Result<()> {
        self.halt_on_leveraging()?;
        require_gt!(self.state.release_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_unit, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_rate, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_amount, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_unit, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_index, 0, Errors::IncompleteProcess);
        require_gt!(self.state.release_min_output, 0, Errors::IncompleteProcess);
        require_gte!(self.state.utilization_rate, 0, Errors::IncompleteProcess);
        require_gte!(self.state.repay_borrow_amount, 0, Errors::IncompleteProcess);
        self.unit = self.unit.saturating_sub(self.state.release_unit);
        self.borrowing_unit = self.borrowing_unit.saturating_sub(self.state.repay_unit);

        // Clear state
        self.state = PositionState::default();

        Ok(())
    }
}

pub struct InitPositionParams {
    pub owner: Pubkey,
    pub id: Pubkey,
    pub tag_id: [u8; 64],
}