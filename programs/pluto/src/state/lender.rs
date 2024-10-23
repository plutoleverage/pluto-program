use std::ops::Add;
use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{ErrorEarn, ErrorMath::MathOverflow, Errors};
use crate::util::{
    decimals,
    constant::{INDEX_DECIMALS, UNIT_DECIMALS},
};

#[derive(InitSpace, Derivative, Default, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Lender {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align: [u8; 5],
    pub owner: Pubkey,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub last_updated: i64,
    pub pending_deposit_amount: u64,
    pub pending_deposit_unit: u64,
    pub pending_deposit_index: u128,
    pub pending_withdraw_amount: u64,
    pub pending_withdraw_unit: u64,
    pub pending_withdraw_index: u128,
    pub unit: u64,
    pub index: u128,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 10],
}

impl Lender {
    pub fn init(&mut self, params: InitLenderParams) -> Result<()> {
        let clock = Clock::get()?;
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.owner = params.owner;
        self.protocol = params.protocol;
        self.last_updated = clock.unix_timestamp;

        Ok(())
    }

    pub fn valuation_amount(&mut self, index: u128, token_decimal: u8) -> Result<u64> {
        if self.unit == 0 {
            return Ok(0);
        }
        let val = decimals::mul_floor(token_decimal, self.unit as u128, UNIT_DECIMALS, index, INDEX_DECIMALS)?;
        Ok(val as u64)
    }

    pub fn deposit(&mut self, amount: u64, unit: u64, index: u128) -> Result<()> {
        require!(amount > 0, Errors::InvalidAmountZero);
        require!(unit > 0, Errors::InvalidAmountZero);
        require_eq!(self.pending_deposit_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.pending_deposit_unit, 0, Errors::IncompleteProcess);
        self.pending_deposit_amount = amount;
        self.pending_deposit_unit = unit;
        self.pending_deposit_index = index;

        Ok(())
    }

    pub fn confirm_deposit(&mut self, token_decimal: u8) -> Result<()> {
        require_gt!(self.pending_deposit_amount, 0, Errors::IncompleteProcess);
        require_gt!(self.pending_deposit_unit, 0, Errors::IncompleteProcess);
        // Floor to prevent extra deposit from rounding
        let cur_amount = decimals::mul_floor(token_decimal, self.unit as u128, UNIT_DECIMALS, self.index, INDEX_DECIMALS)?;
        // Ceil to prevent less index from rounding
        let avg_index = decimals::div_floor(INDEX_DECIMALS, cur_amount.checked_add(self.pending_deposit_amount as u128).ok_or(MathOverflow)?, token_decimal, self.unit.checked_add(self.pending_deposit_unit).ok_or(MathOverflow)? as u128, UNIT_DECIMALS)?;
        self.index = avg_index;
        self.unit = self.unit.checked_add(self.pending_deposit_unit).ok_or(MathOverflow)?;

        self.pending_deposit_amount = 0;
        self.pending_deposit_unit = 0;
        self.pending_deposit_index = 0;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64, unit: u64, index: u128) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        require_eq!(self.pending_withdraw_amount, 0, Errors::IncompleteProcess);
        require_eq!(self.pending_withdraw_unit, 0, Errors::IncompleteProcess);
        self.unit = self.unit.saturating_sub(unit);
        self.pending_withdraw_amount = amount;
        self.pending_withdraw_unit = unit;
        self.pending_withdraw_index = index;

        Ok(())
    }

    pub fn confirm_withdraw(&mut self) -> Result<()> {
        require_gt!(self.pending_withdraw_amount, 0, Errors::InvalidAmountZero);
        require_gt!(self.pending_withdraw_unit, 0, Errors::InvalidAmountZero);
        self.pending_withdraw_amount = 0;
        self.pending_withdraw_unit = 0;
        self.pending_withdraw_index = 0;

        Ok(())
    }
}

pub struct InitLenderParams {
    pub bump: u8,
    pub owner: Pubkey,
    pub protocol: Pubkey,
}