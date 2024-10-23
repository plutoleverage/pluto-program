use anchor_lang::prelude::*;
use crate::error::Errors;
use crate::event::EventLeverageConfigChangeIndexer;
use crate::state::{LeverageConfig, Protocol};
use crate::util::{
    seeds,
};

pub fn handle(ctx: Context<LeverageConfigChangeIndexer>, new_indexer: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config.load_mut()?;
    let owner = &mut ctx.accounts.payer;
    let old_indexer = config.indexer;
    config.indexer = new_indexer;

    msg!("Config indexer changed successfully");

    emit!(EventLeverageConfigChangeIndexer{
        old_indexer,
        indexer: new_indexer,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct LeverageConfigChangeIndexer<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    /// CHECK CONFIG LEVERAGE AUTHORITY
    #[account(
        seeds = [seeds::CONFIG_LEVERAGE_AUTH, config.key().as_ref()],
        bump,
    )]
    pub config_authority: AccountInfo<'info>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
    )]
    pub config: AccountLoader<'info, LeverageConfig>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}