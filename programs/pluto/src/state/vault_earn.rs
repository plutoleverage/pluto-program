use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{Errors, ErrorEarn, ErrorMath};
use crate::error::ErrorMath::MathOverflow;
use crate::state::{EarnConfig, Rate};
use crate::util::{constant, decimals};
use crate::util::constant::{INDEX_DECIMALS, INDEX_ONE, PERCENT_DECIMALS, PROTOCOL_CAP_RATIO, UNIT_DECIMALS};

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct VaultEarn {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub protocol: Pubkey,
    pub earn_stats: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub earn_config: Pubkey,
    pub vault_liquidity: Pubkey,
    pub price_oracle: Pubkey,
    pub price_feed: [u8; 64],
    pub token_program: Pubkey,
    pub token_mint: Pubkey,
    #[derivative(Default(value="9"))] // 9 decimal places
    pub token_decimal: u8,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 7],
    pub last_updated: i64,
    pub unit_supply: u128, // total supply of unit 1 = 10^9
    pub unit_borrowed: u128, // total borrowed unit 1 = 10^9
    pub unit_lent: u128, // total lent unit 1 = 10^9
    pub unit_leverage: u128, // total leverage unit 1 = 10^9
    #[derivative(Default(value="10u128.pow(12)"))]
    pub index: u128,
    pub last_index_updated: i64,
    pub apy: Rate,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 64],
}

impl Default for VaultEarn {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0; 5],
            protocol: Pubkey::default(),
            earn_stats: Pubkey::default(),
            creator: Pubkey::default(),
            authority: Pubkey::default(),
            earn_config: Default::default(),
            vault_liquidity: Pubkey::default(),
            price_oracle: Pubkey::default(),
            price_feed: [0; 64],
            token_program: Pubkey::default(),
            token_mint: Pubkey::default(),
            token_decimal: 0,
            align1: [0; 7],
            last_updated: 0,
            index: INDEX_ONE,
            last_index_updated: 0,
            unit_supply: 0,
            unit_borrowed: 0,
            unit_lent: 0,
            unit_leverage: 0,
            padding1: [0; 64],
            apy: Rate::default(),
        }
    }
}

impl VaultEarn {
    pub fn init(&mut self, params: InitVaultEarnParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol = params.protocol;
        self.earn_stats = params.earn_stats;
        self.creator = params.creator;
        self.authority = params.authority;
        self.earn_config = params.earn_config;
        self.vault_liquidity = params.vault_liquidity;
        self.price_oracle = params.price_oracle;
        self.price_feed = params.price_feed;
        self.token_program = params.token_program;
        self.token_mint = params.token_mint;
        self.token_decimal = params.token_decimal;
        self.index = params.index;
        self.last_index_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn change_price_oracle(&mut self, price_oracle: Pubkey, price_feed: [u8; 64]) -> Result<()> {
        self.price_oracle = price_oracle;
        self.price_feed = price_feed;
        Ok(())
    }

    pub fn utilization_rate(&mut self) -> Result<u32> {
        if self.unit_supply == 0 {
            return Ok(0);
        }
        let mut utilization_rate = decimals::mul_ceil(UNIT_DECIMALS, self.unit_borrowed, UNIT_DECIMALS,100, 0)?;
        utilization_rate = decimals::div_ceil(PERCENT_DECIMALS, utilization_rate, UNIT_DECIMALS,self.unit_supply, UNIT_DECIMALS)?;
        Ok(utilization_rate as u32)
    }

    pub fn lending_ratio(&mut self) -> Result<u32> {
        if self.unit_borrowed == 0 {
            return Ok(0);
        }
        let mut ratio = decimals::mul_ceil(UNIT_DECIMALS, self.unit_lent, UNIT_DECIMALS, 100, 0)?;
        ratio = decimals::div_ceil(PERCENT_DECIMALS, ratio, UNIT_DECIMALS, self.unit_borrowed, UNIT_DECIMALS)?;
        Ok(ratio as u32)
    }

    pub fn leverage_ratio(&mut self) -> Result<u32> {
        if self.unit_leverage == 0 {
            return Ok(0);
        }
        let mut ratio = decimals::mul_ceil(UNIT_DECIMALS, self.unit_leverage, UNIT_DECIMALS, 100, 0)?;
        ratio = decimals::div_ceil(PERCENT_DECIMALS, ratio, UNIT_DECIMALS, self.unit_borrowed, UNIT_DECIMALS)?;
        Ok(ratio as u32)
    }

    // Multiply with unit to get the token value
    pub fn protocol_fee_factor(&self, protocol_fee: u32, utilization_rate: u32, avg_index: u128, index: u128) -> Result<u128> {
        // Supply APY (before Fee) x (Protocol Fee + (2 x Protocol Fee x (UR - 50%))
        let delta_ur = (utilization_rate as i64).checked_sub(PROTOCOL_CAP_RATIO as i64).ok_or(MathOverflow)?;
        let mut factor = protocol_fee.checked_mul(2).ok_or(MathOverflow)? as i64;
        factor = factor.checked_mul(delta_ur).unwrap().saturating_div(10i64.pow(PERCENT_DECIMALS as u32));
        factor = factor.checked_add(protocol_fee as i64).ok_or(MathOverflow)?;
        let protocol_portion = if factor > 0 {
            factor as u64
        } else {
            0
        };
        let delta_index = index.saturating_sub(avg_index);
        let floor_cap = decimals::mul_floor(INDEX_DECIMALS, delta_index, INDEX_DECIMALS, protocol_portion as u128, PERCENT_DECIMALS)?;
        Ok(floor_cap)
    }

    pub fn borrowable_unit(&mut self, config: &EarnConfig) -> Result<u128> {
        let mut borrowable_unit = decimals::mul_floor(UNIT_DECIMALS, self.unit_supply, UNIT_DECIMALS, config.ltv as u128, PERCENT_DECIMALS)?;
        borrowable_unit = decimals::div_floor(UNIT_DECIMALS, borrowable_unit, UNIT_DECIMALS, 100, 0)?;
        Ok(borrowable_unit)
    }

    pub fn borrowable_amount(&mut self, config: &EarnConfig) -> Result<u128> {
        let mut borrowable = decimals::mul_floor(UNIT_DECIMALS, self.unit_supply, UNIT_DECIMALS, config.ltv as u128, PERCENT_DECIMALS)?;
        borrowable = decimals::div_floor(UNIT_DECIMALS, borrowable, UNIT_DECIMALS, 100, 0)?;
        self.unit_to_amount(borrowable)
    }

    pub fn borrow_available_amount(&mut self, config: &EarnConfig) -> Result<u128> {
        let mut borrowable = decimals::mul_floor(UNIT_DECIMALS, self.unit_supply, UNIT_DECIMALS, config.ltv as u128, PERCENT_DECIMALS)?;
        borrowable = decimals::div_floor(UNIT_DECIMALS, borrowable, UNIT_DECIMALS, 100, 0)?;
        if borrowable == 0 || borrowable <= self.unit_borrowed {
            return Ok(0);
        }
        let borrow_available_unit = borrowable.checked_sub(self.unit_borrowed).ok_or(MathOverflow)?;
        self.unit_to_amount(borrow_available_unit)
    }

    pub fn mint(&mut self, config: &EarnConfig, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_supply = self.unit_supply.checked_add(unit as u128).ok_or(MathOverflow)?;
        Ok(())
    }

    pub fn burn(&mut self, config: &EarnConfig, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_supply = self.unit_supply.checked_sub(unit as u128).ok_or(ErrorEarn::InsufficientFund)?;

        Ok(())
    }

    pub fn lend(&mut self, config: &EarnConfig, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_lent = self.unit_lent.checked_add(unit as u128).ok_or(MathOverflow)?;
        self.unit_borrowed = self.unit_borrowed.checked_add(unit as u128).ok_or(MathOverflow)?;

        Ok(())
    }

    pub fn repay(&mut self, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_lent = self.unit_lent.checked_sub(unit as u128).ok_or(MathOverflow)?;
        self.unit_borrowed = self.unit_borrowed.checked_sub(unit as u128).ok_or(MathOverflow)?;

        Ok(())
    }

    pub fn leverage(&mut self, borrowing_amount: u64) -> Result<()> {
        require!(borrowing_amount > 0, Errors::InvalidAmountZero);
        let unit = decimals::div_ceil(UNIT_DECIMALS, borrowing_amount as u128, self.token_decimal, self.index, INDEX_DECIMALS)?;
        self.unit_leverage = self.unit_leverage.checked_add(unit).ok_or(MathOverflow)?;
        self.unit_borrowed = self.unit_borrowed.checked_add(unit).ok_or(MathOverflow)?;

        Ok(())
    }

    pub fn deleverage(&mut self, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_leverage = self.unit_leverage.checked_sub(unit as u128).ok_or(MathOverflow)?;
        self.unit_borrowed = self.unit_borrowed.checked_sub(unit as u128).ok_or(MathOverflow)?;

        Ok(())
    }

    pub fn set_index(&mut self, index: u128, apy: u32) -> Result<()> {
        require!(index > 0, Errors::InvalidAmountZero);
        self.index = index;
        self.last_index_updated = Clock::get()?.unix_timestamp;
        self.apy.update_rate(apy, self.last_index_updated)?;

        Ok(())
    }

    pub fn unit_to_amount(&mut self, unit: u128) -> Result<u128> {
        // Floor to prevent extra token withdraw
        let amount = decimals::mul_floor(self.token_decimal, unit, UNIT_DECIMALS, self.index, INDEX_DECIMALS)?;
        Ok(amount)
    }
}

pub struct InitVaultEarnParams {
    pub bump: u8,
    pub protocol: Pubkey,
    pub earn_stats: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub earn_config: Pubkey,
    pub vault_liquidity: Pubkey,
    pub price_oracle: Pubkey,
    pub price_feed: [u8; 64],
    pub token_program: Pubkey,
    pub token_mint: Pubkey,
    pub token_decimal: u8,
    pub index: u128,
}