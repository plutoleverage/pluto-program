use anchor_lang::prelude::*;

#[event]
pub struct EventVaultLeverageCreated {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub min_leverage: u16,
    pub max_leverage: u16,
    pub leverage_step: u16,
    pub index: u128,
}