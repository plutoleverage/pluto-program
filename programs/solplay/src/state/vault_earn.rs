use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::ErrorMath;
use crate::util::decimals;

#[derive(InitSpace, Derivative, Default, Copy, PartialEq)]
#[derivative(Debug)]
#[account]
pub struct VaultEarn {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    #[derivative(Default(value="9"))] // 9 decimal places
    pub token_decimal: u8,
    #[derivative(Default(value="5 * 10u16.pow(4)"))] // 50%
    pub ltv: u16, // loan to value ratio in percentage 100% = 10^5
    #[derivative(Default(value="10u64.pow(9)"))]
    pub deposit_limit: u64,
    #[derivative(Default(value="10u64.pow(9)"))]
    pub withdraw_limit: u64,
    pub last_updated: i64,
    #[derivative(Default(value="10u64.pow(12)"))]
    pub index: u128,
    pub last_index_updated: i64,
    pub unit_supply: u128, // total supply of unit 1 = 10^9
    pub fund_lent: u128,
    pub fund_reward: u128,
    pub fund_withdrawn: u128,
    pub fund_total: u128,
    pub fund_borrowed: u128,
    pub fund_leverage: u128,
    pub fund_borrow_interest: u128,
    pub fund_leverage_interest: u128,
    pub fund_interest_total: u128,
    pub fund_borrow_repaid: u128,
    pub fund_leverage_repaid: u128,
    pub fund_repaid_total: u128,
    pub fund_borrow_total: u128,
    pub last_interest_updated: i64,
}

impl VaultEarn {
    pub fn init(&mut self, params: InitVaultEarnParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.owner = params.owner;
        self.token_mint = params.token_mint;
        self.token_decimal = params.token_decimal;
        self.ltv = params.ltv;
        self.deposit_limit = params.deposit_limit;
        self.withdraw_limit = params.withdraw_limit;
        self.index = params.index;

        Ok(())
    }

    pub fn utilization_rate(&mut self) -> Result<u16> {
        if self.fund_total == 0 {
            return Ok(0);
        }
        let mut utilization_rate = self.fund_borrow_total.checked_mul(decimals::PERCENT_MULTIPLICATION as u128).ok_or(ErrorMath::MathOverflow)?;
        utilization_rate = utilization_rate.checked_div(self.fund_total).ok_or(ErrorMath::MathOverflow).unwrap();
        Ok(utilization_rate as u16)
    }

    pub fn borrow_ratio(&mut self) -> Result<u16> {
        if self.fund_borrow_total == 0 {
            return Ok(0);
        }
        let mut borrow_ratio = self.fund_borrowed.checked_add(self.fund_borrow_interest).ok_or(ErrorMath::MathOverflow)?;
        borrow_ratio = borrow_ratio.checked_sub(self.fund_borrow_repaid).ok_or(ErrorMath::MathOverflow)?;
        borrow_ratio = borrow_ratio.checked_mul(decimals::PERCENT_MULTIPLICATION as u128).ok_or(ErrorMath::MathOverflow)?;
        borrow_ratio = borrow_ratio.checked_div(self.fund_borrow_total).ok_or(ErrorMath::MathOverflow)?;
        Ok(borrow_ratio as u16)
    }

    pub fn leverage_ratio(&mut self) -> Result<u16> {
        if self.fund_borrow_total == 0 {
            return Ok(0);
        }
        let mut leverage_ratio = self.fund_leverage.checked_add(self.fund_leverage_interest).ok_or(ErrorMath::MathOverflow)?;
        leverage_ratio = leverage_ratio.checked_sub(self.fund_leverage_repaid).ok_or(ErrorMath::MathOverflow)?;
        leverage_ratio = leverage_ratio.checked_mul(decimals::PERCENT_MULTIPLICATION as u128).ok_or(ErrorMath::MathOverflow)?;
        leverage_ratio = leverage_ratio.checked_div(self.fund_borrow_total).ok_or(ErrorMath::MathOverflow)?;
        Ok(leverage_ratio as u16)
    }

    pub fn borrowable_amount(&mut self) -> Result<u128> {
        let mut borrowable_amount = self.fund_total.checked_mul(self.ltv as u128).ok_or(ErrorMath::MathOverflow)?;
        borrowable_amount = borrowable_amount.checked_div(decimals::PERCENT_MULTIPLICATION as u128).ok_or(ErrorMath::MathOverflow)?;
        Ok(borrowable_amount)
    }

    pub fn borrow_available_amount(&mut self) -> Result<u128> {
        let borrowable = self.borrowable_amount().unwrap();
        if borrowable == 0 || borrowable <= self.fund_borrow_total {
            return Ok(0);
        }
        let borrow_available = borrowable.checked_sub(self.fund_borrow_total).ok_or(ErrorMath::MathOverflow)?;
        Ok(borrow_available)
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.fund_lent = self.fund_lent.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_total = self.fund_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        let unit = decimals::div(decimals::UNIT_DECIMALS, amount as u128, self.token_decimal, self.index, decimals::INDEX_DECIMALS)?;

        self.unit_supply = self.unit_supply.checked_add(unit).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.fund_withdrawn = self.fund_withdrawn.checked_sub(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_total = self.fund_total.checked_sub(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        let unit = decimals::div(decimals::UNIT_DECIMALS, amount as u128, self.token_decimal, self.index, decimals::INDEX_DECIMALS)?;

        self.unit_supply = self.unit_supply.checked_sub(unit).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn borrow(&mut self, amount: u64) -> Result<()> {
        self.fund_borrowed = self.fund_borrowed.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn repay(&mut self, amount: u64) -> Result<()> {
        self.fund_borrow_repaid = self.fund_borrowed.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_repaid_total = self.fund_repaid_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_sub(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn leverage(&mut self, amount: u64) -> Result<()> {
        self.fund_leverage = self.fund_leverage.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn deleverage(&mut self, amount: u64) -> Result<()> {
        self.fund_leverage_repaid = self.fund_leverage_repaid.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_repaid_total = self.fund_repaid_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_sub(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        Ok(())
    }

    pub fn borrow_interest(&mut self, amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        self.fund_reward = self.fund_reward.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_total = self.fund_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        self.fund_borrow_interest = self.fund_borrow_interest.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_interest_total = self.fund_interest_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.last_interest_updated = clock.unix_timestamp;

        self.calculate_index()?;

        Ok(())
    }

    pub fn leverage_interest(&mut self, amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        self.fund_reward = self.fund_reward.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_total = self.fund_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;

        self.fund_leverage_interest = self.fund_leverage_interest.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_interest_total = self.fund_interest_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.fund_borrow_total = self.fund_borrow_total.checked_add(amount as u128).ok_or(ErrorMath::MathOverflow)?;
        self.last_interest_updated = clock.unix_timestamp;

        self.calculate_index()?;

        Ok(())
    }

    fn calculate_index(&mut self) -> Result<()> {
        let index = decimals::div(decimals::INDEX_DECIMALS, self.fund_total, self.token_decimal, self.unit_supply, decimals::UNIT_DECIMALS)?;

        self.index = index;
        self.last_index_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

pub struct InitVaultEarnParams {
    pub bump: u8,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub token_decimal: u8,
    pub ltv: u16,
    pub deposit_limit: u64,
    pub withdraw_limit: u64,
    pub index: u128,
}