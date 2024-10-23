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
pub struct Protocol {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub freeze: bool,
    pub freeze_earn: bool,
    pub freeze_lend: bool,
    pub freeze_leverage: bool,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 4],
    pub last_updated: i64,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 64],
}

impl Default for Protocol {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0;5],
            creator: Pubkey::default(),
            owner: Pubkey::default(),
            freeze: false,
            freeze_earn: false,
            freeze_lend: false,
            freeze_leverage: false,
            align1: [0;4],
            last_updated: 0,
            padding1: [0; 64],
        }
    }
}

impl Protocol {
    pub fn init(&mut self, params: InitProtocolParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.creator = params.creator;
        self.owner = params.owner;
        self.freeze = params.freeze;
        self.freeze_earn = params.freeze_earn;
        self.freeze_lend = params.freeze_lend;
        self.freeze_leverage = params.freeze_leverage;

        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn set_protocol(&mut self, params: SetProtocolParams) -> Result<()> {
        self.freeze = params.freeze;
        self.freeze_earn = params.freeze_earn;
        self.freeze_lend = params.freeze_lend;
        self.freeze_leverage = params.freeze_leverage;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn change_owner(&mut self, owner: Pubkey) -> Result<()> {
        self.owner = owner;
        Ok(())
    }
}

pub struct InitProtocolParams {
    pub bump: u8,
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub freeze: bool,
    pub freeze_earn: bool,
    pub freeze_lend: bool,
    pub freeze_leverage: bool,
}

pub struct SetProtocolParams {
    pub freeze: bool,
    pub freeze_earn: bool,
    pub freeze_lend: bool,
    pub freeze_leverage: bool,
}