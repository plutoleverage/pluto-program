use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{Errors, ErrorEarn};
use crate::error::ErrorMath::MathOverflow;
use crate::util::{constant, decimals};
use crate::util::constant::{INDEX_DECIMALS, INDEX_ONE, PERCENT_DECIMALS, UNIT_DECIMALS};

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct EarnConfig {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub protocol: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub indexer: Pubkey,
    pub earn_fee_vault: Pubkey,
    pub freeze: bool,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 7],
    #[derivative(Default(value="0u32"))] // 0%
    pub protocol_fee: u32, // protocol fee in percentage 100% = 10^5
    #[derivative(Default(value="5 * 10u32.pow(4)"))] // 50%
    pub ltv: u32, // loan to value ratio in percentage 100% = 10^5
    #[derivative(Default(value="0u32"))] // 0%
    pub deposit_fee: u32, // deposit fee in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 4],
    #[derivative(Default(value="10u64.pow(6)"))]
    pub min_deposit_limit: u64,
    #[derivative(Default(value="10u64.pow(12)"))]
    pub max_deposit_limit: u64,
    #[derivative(Default(value="0u32"))] // 0%
    pub withdraw_fee: u32, // withdraw fee in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align3: [u8; 4],
    #[derivative(Default(value="0u64"))]
    pub min_withdraw_limit: u64,
    #[derivative(Default(value="10u64.pow(12)"))]
    pub max_withdraw_limit: u64,
    #[derivative(Default(value="0u32"))] // 0%
    pub borrow_fee: u32, // borrow fee in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align4: [u8; 4],
    #[derivative(Default(value="0u64"))]
    pub min_borrow_limit: u64,
    #[derivative(Default(value="10u64.pow(11)"))]
    pub max_borrow_limit: u64,
    #[derivative(Default(value="0u32"))] // 0%
    pub floor_cap_rate: u32, // floor cap rate in percentage 100% = 10^5
    #[derivative(Debug = "ignore")]
    pub align5: [u8; 4],
    pub last_updated: i64,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 32],
}

impl Default for EarnConfig {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0;5],
            protocol: Pubkey::default(),
            creator: Pubkey::default(),
            authority: Pubkey::default(),
            indexer: Pubkey::default(),
            earn_fee_vault: Pubkey::default(),
            freeze: false,
            align1: [0;7],
            protocol_fee: 0,
            ltv: 0,
            deposit_fee: 0,
            align2: [0;4],
            min_deposit_limit: 0,
            max_deposit_limit: 0,
            withdraw_fee: 0,
            align3: [0;4],
            min_withdraw_limit: 0,
            max_withdraw_limit: 0,
            borrow_fee: 0,
            align4: [0;4],
            min_borrow_limit: 0,
            max_borrow_limit: 0,
            floor_cap_rate: 0,
            align5: [0;4],
            last_updated: 0,
            padding1: [0; 32],
        }
    }
}

impl EarnConfig {
    pub fn init(&mut self, params: InitEarnConfigParams) -> Result<()> {
        require_gt!(params.ltv, 0, ErrorEarn::InvalidLTV);
        require_gte!(params.max_deposit_limit, params.min_deposit_limit, ErrorEarn::InvalidMaxDepositLimitLessThanMinDepositLimit);
        require_gt!(params.max_deposit_limit, 0, ErrorEarn::InvalidMaxDepositLimit);
        require_gte!(params.max_withdraw_limit, params.min_withdraw_limit, ErrorEarn::InvalidMaxWithdrawLimitLessThanMinWithdrawLimit);
        require_gt!(params.max_withdraw_limit, 0, ErrorEarn::InvalidMaxWithdrawLimit);
        require_gte!(params.max_borrow_limit, params.min_borrow_limit, ErrorEarn::InvalidMaxBorrowLimitLessThanMinBorrowLimit);
        require_gt!(params.max_borrow_limit, 0, ErrorEarn::InvalidMaxBorrowLimit);
        require_gt!(params.floor_cap_rate, 0, ErrorEarn::InvalidFloorCapRate);
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol = params.protocol;
        self.creator = params.creator;
        self.authority = params.authority;
        self.indexer = params.indexer;
        self.earn_fee_vault = params.earn_fee_vault;
        self.freeze = params.freeze;
        self.protocol_fee = params.protocol_fee;
        self.ltv = params.ltv;
        self.deposit_fee = params.deposit_fee;
        self.min_deposit_limit = params.min_deposit_limit;
        self.max_deposit_limit = params.max_deposit_limit;
        self.withdraw_fee = params.withdraw_fee;
        self.min_withdraw_limit = params.min_withdraw_limit;
        self.max_withdraw_limit = params.max_withdraw_limit;
        self.borrow_fee = params.borrow_fee;
        self.min_borrow_limit = params.min_borrow_limit;
        self.max_borrow_limit = params.max_borrow_limit;
        self.floor_cap_rate = params.floor_cap_rate;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn set_config(&mut self, params: SetEarnConfigParams) -> Result<()> {
        require_gt!(params.ltv, 0, ErrorEarn::InvalidLTV);
        require_gte!(params.max_deposit_limit, params.min_deposit_limit, ErrorEarn::InvalidMaxDepositLimitLessThanMinDepositLimit);
        require_gt!(params.max_deposit_limit, 0, ErrorEarn::InvalidMaxDepositLimit);
        require_gte!(params.max_withdraw_limit, params.min_withdraw_limit, ErrorEarn::InvalidMaxWithdrawLimitLessThanMinWithdrawLimit);
        require_gt!(params.max_withdraw_limit, 0, ErrorEarn::InvalidMaxWithdrawLimit);
        require_gte!(params.max_borrow_limit, params.min_borrow_limit, ErrorEarn::InvalidMaxBorrowLimitLessThanMinBorrowLimit);
        require_gt!(params.max_borrow_limit, 0, ErrorEarn::InvalidMaxBorrowLimit);
        require_gt!(params.floor_cap_rate, 0, ErrorEarn::InvalidFloorCapRate);
        self.earn_fee_vault = params.earn_fee_vault;
        self.freeze = params.freeze;
        self.protocol_fee = params.protocol_fee;
        self.ltv = params.ltv;
        self.deposit_fee = params.deposit_fee;
        self.min_deposit_limit = params.min_deposit_limit;
        self.max_deposit_limit = params.max_deposit_limit;
        self.withdraw_fee = params.withdraw_fee;
        self.min_withdraw_limit = params.min_withdraw_limit;
        self.max_withdraw_limit = params.max_withdraw_limit;
        self.borrow_fee = params.borrow_fee;
        self.min_borrow_limit = params.min_borrow_limit;
        self.max_borrow_limit = params.max_borrow_limit;
        self.floor_cap_rate = params.floor_cap_rate;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn change_indexer(&mut self, indexer: Pubkey) -> Result<()> {
        self.indexer = indexer;
        Ok(())
    }
}

pub struct InitEarnConfigParams {
    pub bump: u8,
    pub protocol: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub indexer: Pubkey,
    pub earn_fee_vault: Pubkey,
    pub freeze: bool,
    pub protocol_fee: u32,
    pub ltv: u32,
    pub deposit_fee: u32,
    pub min_deposit_limit: u64,
    pub max_deposit_limit: u64,
    pub withdraw_fee: u32,
    pub min_withdraw_limit: u64,
    pub max_withdraw_limit: u64,
    pub borrow_fee: u32,
    pub min_borrow_limit: u64,
    pub max_borrow_limit: u64,
    pub floor_cap_rate: u32,
}

pub struct SetEarnConfigParams {
    pub earn_fee_vault: Pubkey,
    pub freeze: bool,
    pub protocol_fee: u32,
    pub ltv: u32,
    pub deposit_fee: u32,
    pub min_deposit_limit: u64,
    pub max_deposit_limit: u64,
    pub withdraw_fee: u32,
    pub min_withdraw_limit: u64,
    pub max_withdraw_limit: u64,
    pub borrow_fee: u32,
    pub min_borrow_limit: u64,
    pub max_borrow_limit: u64,
    pub floor_cap_rate: u32,
}