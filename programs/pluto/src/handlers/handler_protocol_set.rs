use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use crate::error::Errors;
use crate::event::EventProtocolSet;
use crate::state::{Protocol, SetProtocolParams};
use crate::util::{
    seeds,
    constant::{INDEX_ONE},
};

pub fn handle(ctx: Context<ProtocolSet>, freeze:bool, freeze_earn: bool, freeze_lend: bool, freeze_leverage: bool) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol.load_mut()?;
    let owner = &mut ctx.accounts.payer;
    require!(*owner.key == protocol.owner, Errors::NotOwner);
    protocol.set_protocol(SetProtocolParams{
        freeze,
        freeze_earn,
        freeze_lend,
        freeze_leverage,
    })?;

    msg!("earn protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("earn protocol owner address: {:?}", owner.key);
    msg!("earn protocol freeze: {:?}", freeze);
    msg!("earn protocol freeze_earn: {:?}", freeze_earn);
    msg!("earn protocol freeze_lend: {:?}", freeze_lend);
    msg!("earn protocol freeze_leverage: {:?}", freeze_leverage);

    emit!(EventProtocolSet{
        freeze,
        freeze_earn,
        freeze_lend,
        freeze_leverage,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ProtocolSet<'info> {
    #[account(
        mut,
        seeds = [seeds::PROTOCOL, payer.key().as_ref()],
        bump,
    )]
    pub protocol: AccountLoader<'info, Protocol>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}