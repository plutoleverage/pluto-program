mod event;
mod state;
mod error;
mod handlers;
mod util;

use anchor_lang::prelude::*;
use crate::handlers::*;

declare_id!("8JABYdaQA9jspWE4oFmBNF1LbKS1nyrNGhQHm94iwURi");

#[program]
pub mod solplay {
    use super::*;

    pub fn create(ctx: Context<VaultEarnCreate>, ltv: u16, deposit_limit: u64, withdraw_limit: u64) -> Result<()> {
        handler_vault_earn_create::handle(ctx, ltv, deposit_limit, withdraw_limit)
    }

    pub fn deposit(ctx: Context<VaultEarnDeposit>, amount: u64) -> Result<()> {
        handler_vault_earn_deposit::handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<VaultEarnWithdraw>, amount: u64) -> Result<()> {
        handler_vault_earn_withdraw::handle(ctx, amount)
    }
}

