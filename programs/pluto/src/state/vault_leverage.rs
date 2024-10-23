use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;
use crate::error::{Errors, ErrorLeverage, ErrorMath};
use crate::state::Rate;
use crate::util::{constant, decimals};
use crate::util::constant::{FLOOR_CAP_RATIO, INDEX_DECIMALS, PERCENT_DECIMALS, PROTOCOL_CAP_RATIO};

#[derive(InitSpace, Derivative, PartialEq)]
#[derivative(Debug)]
#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct VaultLeverage {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    #[derivative(Debug = "ignore")]
    pub align0: [u8; 5],
    pub protocol: Pubkey,
    pub leverage_stats: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub leverage_config: Pubkey,
    pub borrow_vault: Pubkey,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8; 64],
    pub token_collateral_token_program: Pubkey,
    pub token_collateral_token_mint: Pubkey,
    pub token_collateral_vault_liquidity: Pubkey,
    #[derivative(Default(value="9"))] // 9 decimal places
    pub token_collateral_token_decimal: u8,
    #[derivative(Debug = "ignore")]
    pub align1: [u8; 7],
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_token_program: Pubkey,
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_vault_liquidity: Pubkey,
    #[derivative(Default(value="9"))] // 9 decimal places
    pub native_collateral_token_decimal: u8,
    #[derivative(Debug = "ignore")]
    pub align2: [u8; 7],
    pub last_updated: i64,
    pub borrowing_unit_supply: u128, // total supply of unit 1 = 10^9
    #[derivative(Default(value="10u128.pow(12)"))]
    pub borrowing_index: u128,
    pub unit_supply: u128, // total supply of unit 1 = 10^9
    #[derivative(Default(value="10u128.pow(12)"))]
    pub index: u128,
    pub last_index_updated: i64,
    pub borrowing_apy: Rate,
    pub apy: Rate,
    #[derivative(Debug = "ignore")]
    pub padding1: [u64; 64],
}

impl Default for VaultLeverage {
    fn default() -> Self {
        Self {
            is_initialized: false,
            version: 0,
            bump: 0,
            align0: [0; 5],
            protocol: Pubkey::default(),
            leverage_stats: Pubkey::default(),
            creator: Pubkey::default(),
            authority: Pubkey::default(),
            leverage_config: Default::default(),
            borrow_vault: Pubkey::default(),
            token_collateral_price_oracle: Pubkey::default(),
            token_collateral_price_feed: [0; 64],
            token_collateral_token_program: Pubkey::default(),
            token_collateral_token_mint: Pubkey::default(),
            token_collateral_vault_liquidity: Pubkey::default(),
            token_collateral_token_decimal: 0,
            align1: [0; 7],
            native_collateral_price_oracle: Pubkey::default(),
            native_collateral_price_feed: [0; 64],
            native_collateral_token_program: Pubkey::default(),
            native_collateral_token_mint: Pubkey::default(),
            native_collateral_vault_liquidity: Pubkey::default(),
            native_collateral_token_decimal: 0,
            align2: [0; 7],
            last_updated: 0,
            borrowing_unit_supply: 0,
            borrowing_index: 0,
            unit_supply: 0,
            index: 0,
            last_index_updated: 0,
            borrowing_apy: Rate::default(),
            apy: Rate::default(),
            padding1: [0; 64],
        }
    }
}

impl VaultLeverage {
    pub fn init(&mut self, params: InitVaultLeverageParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.protocol = params.protocol;
        self.leverage_stats = params.leverage_stats;
        self.creator = params.creator;
        self.authority = params.authority;
        self.leverage_config = params.leverage_config;
        self.borrow_vault = params.borrow_vault;
        self.token_collateral_price_oracle = params.token_collateral_price_oracle;
        self.token_collateral_price_feed = params.token_collateral_price_feed;
        self.token_collateral_token_program = params.token_collateral_token_program;
        self.token_collateral_token_mint = params.token_collateral_token_mint;
        self.token_collateral_token_decimal = params.token_collateral_token_decimal;
        self.native_collateral_price_oracle = params.native_collateral_price_oracle;
        self.native_collateral_price_feed = params.native_collateral_price_feed;
        self.native_collateral_token_program = params.native_collateral_token_program;
        self.native_collateral_token_mint = params.native_collateral_token_mint;
        self.native_collateral_token_decimal = params.native_collateral_token_decimal;
        self.borrowing_index = params.borrowing_index;
        self.index = params.index;
        self.last_updated = Clock::get()?.unix_timestamp;
        self.last_index_updated = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn change_price_oracle(&mut self, token_collateral_price_oracle: Pubkey, token_collateral_price_feed: [u8; 64], native_collateral_price_oracle: Pubkey, native_collateral_price_feed: [u8; 64]) -> Result<()> {
        self.token_collateral_price_oracle = token_collateral_price_oracle;
        self.token_collateral_price_feed = token_collateral_price_feed;
        self.native_collateral_price_oracle = native_collateral_price_oracle;
        self.native_collateral_price_feed = native_collateral_price_feed;
        Ok(())
    }

    // Multiply with unit to get the token value
    pub fn protocol_fee_factor(&self, protocol_fee: u32, utilization_rate: u32, open_borrowing_index: u128, close_borrowing_index: u128) -> Result<u128> {
        // Borrow APY (before Fee) x (Protocol Fee + (2 x Protocol Fee x (UR - 50%))
        let delta_ur = (utilization_rate as i64).checked_sub(PROTOCOL_CAP_RATIO as i64).ok_or(ErrorMath::MathOverflow)?;
        let mut factor = protocol_fee.checked_mul(2).ok_or(ErrorMath::MathOverflow)? as i64;
        factor = factor.checked_mul(delta_ur).unwrap().saturating_div(10i64.pow(PERCENT_DECIMALS as u32));
        factor = factor.checked_add(protocol_fee as i64).ok_or(ErrorMath::MathOverflow)?;
        let protocol_portion = if factor > 0 {
            factor as u64
        } else {
            0
        };
        let delta_borrowing_index = close_borrowing_index.saturating_sub(open_borrowing_index);
        let borrowing_floor_cap = decimals::mul_floor(INDEX_DECIMALS, delta_borrowing_index, INDEX_DECIMALS, protocol_portion as u128, PERCENT_DECIMALS)?;
        Ok(borrowing_floor_cap)
    }

    pub fn update_time(&mut self) -> Result<()> {
        self.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn mint(&mut self, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_supply = self.unit_supply.checked_add(unit as u128).ok_or(ErrorMath::MathOverflow)?;
        self.update_time()?;
        Ok(())
    }

    pub fn burn(&mut self, unit: u64) -> Result<()> {
        require!(unit > 0, Errors::InvalidAmountZero);
        self.unit_supply = self.unit_supply.checked_sub(unit as u128).ok_or(ErrorMath::MathOverflow)?;
        self.update_time()?;
        Ok(())
    }

    pub fn mint_borrow(&mut self, borrowing_unit: u64) -> Result<()> {
        require!(borrowing_unit > 0, Errors::InvalidAmountZero);
        self.borrowing_unit_supply = self.borrowing_unit_supply.checked_add(borrowing_unit as u128).ok_or(ErrorMath::MathOverflow)?;
        self.update_time()?;
        Ok(())
    }

    pub fn burn_borrow(&mut self, borrowing_unit: u64) -> Result<()> {
        require!(borrowing_unit > 0, Errors::InvalidAmountZero);
        self.borrowing_unit_supply = self.borrowing_unit_supply.checked_sub(borrowing_unit as u128).ok_or(ErrorMath::MathOverflow)?;
        self.update_time()?;
        Ok(())
    }

    pub fn set_index(&mut self, index: u128, apy: u32, borrowing_index: u128, borrowing_apy: u32) -> Result<()> {
        require!(index > 0, Errors::InvalidAmountZero);
        require!(borrowing_index > 0, Errors::InvalidAmountZero);
        self.last_index_updated = Clock::get()?.unix_timestamp;
        self.index = index;
        self.borrowing_index = borrowing_index;
        self.apy.update_rate(apy, self.last_index_updated)?;
        self.borrowing_apy.update_rate(borrowing_apy, self.last_index_updated)?;

        self.update_time()?;
        Ok(())
    }
}

pub struct InitVaultLeverageParams {
    pub bump: u8,
    pub protocol: Pubkey,
    pub leverage_stats: Pubkey,
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub leverage_config: Pubkey,
    pub borrow_vault: Pubkey,
    pub token_collateral_price_oracle: Pubkey,
    pub token_collateral_price_feed: [u8; 64],
    pub token_collateral_token_program: Pubkey,
    pub token_collateral_token_mint: Pubkey,
    pub token_collateral_token_decimal: u8,
    pub native_collateral_price_oracle: Pubkey,
    pub native_collateral_price_feed: [u8; 64],
    pub native_collateral_token_program: Pubkey,
    pub native_collateral_token_mint: Pubkey,
    pub native_collateral_token_decimal: u8,
    pub borrowing_index: u128,
    pub index: u128,
}