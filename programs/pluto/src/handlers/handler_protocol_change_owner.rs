use anchor_lang::prelude::*;
use crate::error::Errors;
use crate::event::EventProtocolChangeOwner;
use crate::state::{Protocol};

pub fn handle(ctx: Context<ProtocolChangeOwner>, new_owner: Pubkey) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol.load_mut()?;
    let owner = &mut ctx.accounts.payer;
    let old_owner = protocol.owner;
    protocol.owner = new_owner;

    msg!("Config owner changed successfully");

    emit!(EventProtocolChangeOwner{
        old_owner,
        owner: new_owner,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ProtocolChangeOwner<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}