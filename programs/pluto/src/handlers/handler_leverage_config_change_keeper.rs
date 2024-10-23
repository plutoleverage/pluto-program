use anchor_lang::prelude::*;
use crate::error::Errors;
use crate::event::EventLeverageConfigChangeKeeper;
use crate::state::{LeverageConfig, Protocol};
use crate::util::{
    seeds,
};

pub fn handle(ctx: Context<LeverageConfigChangeKeeper>, new_keeper: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config.load_mut()?;
    let owner = &mut ctx.accounts.payer;
    let old_keeper = config.keeper;
    config.keeper = new_keeper;

    msg!("Config keeper changed from {:?} to {:?}", old_keeper, new_keeper);

    emit!(EventLeverageConfigChangeKeeper{
        old_keeper,
        keeper: new_keeper,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct LeverageConfigChangeKeeper<'info> {
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