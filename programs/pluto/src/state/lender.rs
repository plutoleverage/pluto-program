use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{ErrorEarn, ErrorMath};
use crate::util::decimals;

#[derive(InitSpace, Derivative, Default, Copy, PartialEq)]
#[derivative(Debug)]
#[account]
pub struct Lender {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    pub owner: Pubkey,
    pub program_account: Pubkey,
    pub token_mint: Pubkey,
    #[derivative(Default(value="9"))] // 9 decimal places
    pub token_decimal: u8,
    pub last_updated: i64,
    pub principal: u64,
    pub unit: u128,
    pub index: u128,
}

impl Lender {
    pub fn init(&mut self, params: InitLenderParams) -> Result<()> {
        let clock = Clock::get()?;
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.owner = params.owner;
        self.program_account = params.program_account;
        self.token_mint = params.token_mint;
        self.token_decimal = params.token_decimal;
        self.last_updated = clock.unix_timestamp;

        Ok(())
    }

    pub fn valuation_amount(&mut self, index: u128) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        let mut val = index.checked_mul(self.unit).ok_or(ErrorMath::MathOverflow)?;
        val = val.checked_div(decimals::INDEX_MULTIPLICATION as u128).ok_or(ErrorMath::MathOverflow)?;
        Ok(val as u64)
    }

    pub fn deposit(&mut self, index: u128, amount: u64) -> Result<u128> {
        self.principal = self.principal.checked_add(amount).ok_or(ErrorMath::MathOverflow)?;
        self.index = index;
        let unit = decimals::div(decimals::UNIT_DECIMALS, amount as u128, self.token_decimal, self.index, decimals::INDEX_DECIMALS)?;
        self.unit = self.unit.checked_add(unit).ok_or(ErrorMath::MathOverflow)?;

        Ok(unit)
    }

    pub fn withdraw(&mut self, index: u128, amount: u64) -> Result<u128> {
        self.principal = self.principal.checked_sub(amount).ok_or(ErrorMath::MathOverflow)?;
        self.index = index;
        let unit = decimals::div(decimals::UNIT_DECIMALS, amount as u128, self.token_decimal, self.index, decimals::INDEX_DECIMALS)?;
        self.unit = self.unit.checked_sub(unit).ok_or(ErrorEarn::InsufficientFund)?;

        Ok(unit)
    }
}

pub struct InitLenderParams {
    pub bump: u8,
    pub owner: Pubkey,
    pub program_account: Pubkey,
    pub token_mint: Pubkey,
    pub token_decimal: u8,
}