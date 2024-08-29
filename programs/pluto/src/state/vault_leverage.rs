use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;
use derivative::Derivative;

#[derive(InitSpace, Derivative, Default, Copy, PartialEq)]
#[derivative(Debug)]
#[account]
pub struct VaultLeverage {
    pub is_initialized: bool,
    pub version: u8,
    pub bump: u8,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    #[derivative(Default(value="15 * 10 ^ 2"))] // 1.5x
    pub min_leverage: u8, // minimum leverage value 100x = 10^5
    #[derivative(Default(value="1 * 10 ^ 4"))] // 10x
    pub max_leverage: u8, // maximum leverage value 100x = 10^5
    #[derivative(Default(value="10 ^ 2"))] // 0.1
    pub leverage_step: u8, // step leverage value 1 = 10^2
    #[derivative(Default(value="10 ^ 27"))]
    pub index: u64,
    pub user_leverage: u32,
    pub fund_principal: u64,
    pub fund_borrowed: u64,
    pub collateral: u64,
}

impl VaultLeverage {
    pub fn init(&mut self, params: InitVaultLeverageParams) -> Result<()> {
        *self = Self::default();
        self.is_initialized = true;
        self.version = 1;
        self.bump = params.bump;
        self.owner = params.owner;
        self.token_mint = params.token_mint;
        self.min_leverage = params.min_leverage;
        self.max_leverage = params.max_leverage;
        self.leverage_step = params.leverage_step;
        self.index = params.index;

        Ok(())
    }
}

pub struct InitVaultLeverageParams {
    pub bump: u8,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub min_leverage: u8,
    pub max_leverage: u8,
    pub leverage_step: u8,
    pub index: u64,
}