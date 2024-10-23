use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{Errors, ErrorLeverage};
use crate::error::ErrorMath::MathOverflow;
use crate::util::{constant, decimals};
use crate::util::constant::{INDEX_DECIMALS, INDEX_ONE, PERCENT_DECIMALS, UNIT_DECIMALS};

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct LeverageConfig {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align: [u8; 5],
    pub protocol: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub indexer: Pubkey,
    pub keeper: Pubkey,
    pub leverage_fee_vault: Pubkey,
    pub freeze: bool,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 7],
    #[derivative(Default(value="0u32"))] // 0%
    pub protocol_fee: u32, // protocol fee in percentage 100% = 10^5
    #[derivative(Default(value="15 * 10"))] // 1.5x
    pub min_leverage: u32, // minimum leverage value 100x = 10^5
    #[derivative(Default(value="10u32.pow(4)"))] // 10x
    pub max_leverage: u32, // maximum leverage value 100x = 10^5
    #[derivative(Default(value="10u32.pow(2)"))] // 0.5
    pub leverage_step: u32, // step leverage value 1 = 10^2
    #[derivative(Default(value="0u32"))] // 0%
    pub leverage_fee: u32, // leverage fee in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 4],
    #[derivative(Default(value="10u64.pow(6)"))]
    pub min_leverage_limit: u64,
    #[derivative(Default(value="10u64.pow(12)"))]
    pub max_leverage_limit: u64,
    #[derivative(Default(value="0u32"))] // 0%
    pub deleverage_fee: u32, // deleverage fee in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align3: [u8; 4],
    #[derivative(Default(value="0u64"))]
    pub min_deleverage_limit: u64,
    #[derivative(Default(value="10u64.pow(12)"))]
    pub max_deleverage_limit: u64,
    pub closing_fee: u32, // closing fee in percentage 100% = 10^5
    #[derivative(Default(value="5 * 10u32.pow(3)"))] // 5%
    pub spread_rate: u32, // spread rate in percentage 100% = 10^5
    #[derivative(Default(value="5 * 10u32.pow(3)"))] // 5%
    pub liquidation_fee: u32, // liquidation fee in percentage 100% = 10^5
    #[derivative(Default(value="8 * 10u32.pow(4)"))] // 80%
    pub liquidation_threshold: u32, // liquidation threshold in percentage 100% = 10^5
    #[derivative(Default(value="0u32"))] // 80%
    pub liquidation_protocol_ratio: u32, // liquidation protocol rate in percentage 100% = 10^5
    #[derivative(Default(value="3 * 10u32.pow(2)"))] // 0.3%
    pub slippage_rate: u32, // slippage rate in percentage 100% = 10^5
    #[derivative(Default(value="86400"))]
    pub emergency_eject_period: i64, // emergency eject period in seconds
    #[derivative(Default(value="1050"))] // 1.05
    pub saver_threshold: u32, // 1 = 10^3
    #[derivative(Default(value="500"))] // 0.5x by leverage
    pub saver_target_reduction: u32, // 1 = 10^3
    pub last_updated: i64,
    pub profit_target_rate: u32,
    pub profit_taking_rate: u32,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 31],
}

impl Default for LeverageConfig {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align: [0; 5],
            protocol: Pubkey::default(),
            creator: Pubkey::default(),
            authority: Pubkey::default(),
            indexer: Pubkey::default(),
            keeper: Pubkey::default(),
            leverage_fee_vault: Pubkey::default(),
            freeze: false,
            align1: [0; 7],
            protocol_fee: 0,
            min_leverage: 0,
            max_leverage: 0,
            leverage_step: 0,
            leverage_fee: 0,
            align2: [0; 4],
            min_leverage_limit: 0,
            max_leverage_limit: 0,
            deleverage_fee: 0,
            align3: [0; 4],
            min_deleverage_limit: 0,
            max_deleverage_limit: 0,
            closing_fee: 0,
            spread_rate: 0,
            liquidation_fee: 0,
            liquidation_threshold: 0,
            liquidation_protocol_ratio: 0,
            slippage_rate: 0,
            emergency_eject_period: 0,
            saver_threshold: 0,
            saver_target_reduction: 0,
            last_updated: 0,
            profit_target_rate: 0,
            profit_taking_rate: 0,
            padding1: [0; 31],
        }
    }
}

impl LeverageConfig {
    pub fn init(&mut self, params: InitLeverageConfigParams) -> Result<()> {
        require_gte!(params.protocol_fee, 0, ErrorLeverage::InvalidProtocolFee);
        require_gte!(params.max_leverage, params.min_leverage, ErrorLeverage::InvalidMaxLeverageLessThanMinLeverage);
        require_gt!(params.max_leverage, 1, ErrorLeverage::InvalidMaxLeverage);
        require_gt!(params.min_leverage, 1, ErrorLeverage::InvalidMinLeverage);
        require_gt!(params.leverage_step, 0, ErrorLeverage::InvalidLeverageStep);
        require_gte!(params.max_leverage_limit, params.min_leverage_limit, ErrorLeverage::InvalidMaxLeverageLessThanMinLeverage);
        require_gt!(params.max_leverage_limit, 0, ErrorLeverage::InvalidMaxLeverageLimit);
        require_gt!(params.min_leverage_limit, 0, ErrorLeverage::InvalidMinLeverageLimit);
        require_gte!(params.max_deleverage_limit, params.min_deleverage_limit, ErrorLeverage::InvalidMaxDeleverageLessThanMinDeleverage);
        require_gt!(params.max_deleverage_limit, 0, ErrorLeverage::InvalidMaxDeleverageLimit);
        require_gt!(params.min_deleverage_limit, 0, ErrorLeverage::InvalidMinDeleverageLimit);
        require_gt!(params.spread_rate, 0, ErrorLeverage::InvalidSpreadRate);
        require_gt!(params.saver_threshold, 0, ErrorLeverage::InvalidSaverThreshold);
        require_gt!(params.saver_target_reduction, 0, ErrorLeverage::InvalidSaverTarget);
        require_gte!(params.saver_target_reduction, params.leverage_step, ErrorLeverage::InvalidSaverTargetLessThanLeverageStep);
        require!(params.saver_target_reduction % params.leverage_step == 0, ErrorLeverage::InvalidSaverTargetNotMultipleOfLeverageStep);
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol = params.protocol;
        self.creator = params.creator;
        self.authority = params.authority;
        self.indexer = params.indexer;
        self.keeper = params.keeper;
        self.leverage_fee_vault = params.leverage_fee_vault;
        self.freeze = params.freeze;
        self.protocol_fee = params.protocol_fee;
        self.min_leverage = params.min_leverage;
        self.max_leverage = params.max_leverage;
        self.leverage_step = params.leverage_step;
        self.leverage_fee = params.leverage_fee;
        self.min_leverage_limit = params.min_leverage_limit;
        self.max_leverage_limit = params.max_leverage_limit;
        self.deleverage_fee = params.deleverage_fee;
        self.min_deleverage_limit = params.min_deleverage_limit;
        self.max_deleverage_limit = params.max_deleverage_limit;
        self.closing_fee = params.closing_fee;
        self.spread_rate = params.spread_rate;
        self.liquidation_fee = params.liquidation_fee;
        self.liquidation_threshold = params.liquidation_threshold;
        self.liquidation_protocol_ratio = params.liquidation_protocol_ratio;
        self.slippage_rate = params.slippage_rate;
        self.emergency_eject_period = params.emergency_eject_period;
        self.saver_threshold = params.saver_threshold;
        self.saver_target_reduction = params.saver_target_reduction;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn set_config(&mut self, params: SetLeverageConfigParams) -> Result<()> {
        require_gte!(params.protocol_fee, 0, ErrorLeverage::InvalidProtocolFee);
        require_gte!(params.max_leverage, params.min_leverage, ErrorLeverage::InvalidMaxLeverageLessThanMinLeverage);
        require_gt!(params.max_leverage, 1, ErrorLeverage::InvalidMaxLeverage);
        require_gt!(params.min_leverage, 1, ErrorLeverage::InvalidMinLeverage);
        require_gt!(params.leverage_step, 0, ErrorLeverage::InvalidLeverageStep);
        require_gte!(params.max_leverage_limit, params.min_leverage_limit, ErrorLeverage::InvalidMaxLeverageLessThanMinLeverage);
        require_gt!(params.max_leverage_limit, 0, ErrorLeverage::InvalidMaxLeverageLimit);
        require_gt!(params.min_leverage_limit, 0, ErrorLeverage::InvalidMinLeverageLimit);
        require_gte!(params.max_deleverage_limit, params.min_deleverage_limit, ErrorLeverage::InvalidMaxDeleverageLessThanMinDeleverage);
        require_gt!(params.max_deleverage_limit, 0, ErrorLeverage::InvalidMaxDeleverageLimit);
        require_gt!(params.min_deleverage_limit, 0, ErrorLeverage::InvalidMinDeleverageLimit);
        require_gt!(params.spread_rate, 0, ErrorLeverage::InvalidSpreadRate);
        require_gt!(params.saver_threshold, 0, ErrorLeverage::InvalidSaverThreshold);
        require_gt!(params.saver_target_reduction, 0, ErrorLeverage::InvalidSaverTarget);
        require_gte!(params.saver_target_reduction, params.leverage_step, ErrorLeverage::InvalidSaverTargetLessThanLeverageStep);
        require!(params.saver_target_reduction % params.leverage_step == 0, ErrorLeverage::InvalidSaverTargetNotMultipleOfLeverageStep);
        self.leverage_fee_vault = params.leverage_fee_vault;
        self.freeze = params.freeze;
        self.protocol_fee = params.protocol_fee;
        self.min_leverage = params.min_leverage;
        self.max_leverage = params.max_leverage;
        self.leverage_step = params.leverage_step;
        self.leverage_fee = params.leverage_fee;
        self.min_leverage_limit = params.min_leverage_limit;
        self.max_leverage_limit = params.max_leverage_limit;
        self.deleverage_fee = params.deleverage_fee;
        self.min_deleverage_limit = params.min_deleverage_limit;
        self.max_deleverage_limit = params.max_deleverage_limit;
        self.closing_fee = params.closing_fee;
        self.spread_rate = params.spread_rate;
        self.liquidation_fee = params.liquidation_fee;
        self.liquidation_threshold = params.liquidation_threshold;
        self.liquidation_protocol_ratio = params.liquidation_protocol_ratio;
        self.slippage_rate = params.slippage_rate;
        self.emergency_eject_period = params.emergency_eject_period;
        self.saver_threshold = params.saver_threshold;
        self.saver_target_reduction = params.saver_target_reduction;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn change_indexer(&mut self, indexer: Pubkey) -> Result<()> {
        self.indexer = indexer;
        Ok(())
    }

    pub fn change_keeper(&mut self, keeper: Pubkey) -> Result<()> {
        self.keeper = keeper;
        Ok(())
    }
}

pub struct InitLeverageConfigParams {
    pub bump: u8,
    pub protocol: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub indexer: Pubkey,
    pub keeper: Pubkey,
    pub leverage_fee_vault: Pubkey,
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

pub struct SetLeverageConfigParams {
    pub leverage_fee_vault: Pubkey,
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