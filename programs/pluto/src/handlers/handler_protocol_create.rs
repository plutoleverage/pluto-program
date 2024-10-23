use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::event::EventProtocolCreated;
use crate::state::{Protocol, InitProtocolParams};
use crate::util::{
    seeds,
    constant::{INDEX_ONE},
};

pub fn handle(ctx: Context<ProtocolCreate>, freeze: bool, freeze_earn: bool, freeze_lend: bool, freeze_leverage: bool) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol.load_init()?;
    let owner = &mut ctx.accounts.payer;
    protocol.init(InitProtocolParams{
        bump: ctx.bumps.protocol,
        creator: *owner.key,
        owner: *owner.key,
        freeze,
        freeze_earn,
        freeze_lend,
        freeze_leverage,
    })?;

    msg!("earn protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("earn protocol owner address: {:?}", owner.key);
    msg!("earn protocol freeze: {:?}", freeze);
    msg!("earn protocol freeze earn: {:?}", freeze_earn);
    msg!("earn protocol freeze lend: {:?}", freeze_lend);
    msg!("earn protocol freeze leverage: {:?}", freeze_leverage);

    emit!(EventProtocolCreated{
        creator: *owner.key,
        owner: *owner.key,
        freeze,
        freeze_earn,
        freeze_lend,
        freeze_leverage,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ProtocolCreate<'info> {
    #[account(
        init,
        seeds = [seeds::PROTOCOL, payer.key().as_ref()],
        bump,
        payer = payer,
        space = Protocol::INIT_SPACE+(2*8),
    )]
    pub protocol: AccountLoader<'info, Protocol>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}