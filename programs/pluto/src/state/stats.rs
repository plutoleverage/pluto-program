use anchor_lang::{InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct Stats {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub creator: Pubkey,
    pub last_updated: i64,
    pub active_user: u128,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 128],
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0; 5],
            protocol: Default::default(),
            vault: Default::default(),
            creator: Default::default(),
            last_updated: 0,
            active_user: 0,
            padding1: [0; 128],
        }
    }
}

impl Stats {
    pub fn init(&mut self, params: InitStatsParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol = params.protocol;
        self.vault = params.vault;
        self.creator = params.creator;
        self.last_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn add_user(&mut self) -> Result<()> {
        self.active_user += 1;
        Ok(())
    }

    pub fn remove_user(&mut self) -> Result<()> {
        self.active_user -= 1;
        Ok(())
    }

    pub fn update_time(&mut self) -> Result<()> {
        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

pub struct InitStatsParams {
    pub bump: u8,
    pub protocol: Pubkey,
    pub vault: Pubkey,
    pub creator: Pubkey,
}