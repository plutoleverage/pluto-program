use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::error::Errors;
use crate::event::EventEarnConfigCreated;
use crate::state::{EarnConfig, InitEarnConfigParams, Protocol};
use crate::util::{
    seeds,
    constant::{INDEX_ONE},
};

pub fn handle(ctx: Context<EarnConfigCreate>, freeze: bool, protocol_fee: u32, ltv: u32, deposit_fee: u32, min_deposit_limit: u64, max_deposit_limit: u64, withdraw_fee: u32, min_withdraw_limit: u64, max_withdraw_limit: u64,borrow_fee: u32, min_borrow_limit: u64, max_borrow_limit: u64, floor_cap_rate: u32) -> Result<()> {
    let indexer = &ctx.accounts.indexer;
    let fee_vault = &ctx.accounts.fee_vault;
    let config = &mut ctx.accounts.config.load_init()?;
    let owner = &mut ctx.accounts.payer;
    config.init(InitEarnConfigParams{
        protocol: ctx.accounts.protocol.key(),
        bump: ctx.bumps.config,
        creator: *owner.key,
        authority: *ctx.accounts.config_authority.key,
        indexer: *indexer.key,
        earn_fee_vault: *fee_vault.key,
        freeze,
        protocol_fee,
        ltv,
        deposit_fee,
        min_deposit_limit,
        max_deposit_limit,
        withdraw_fee,
        min_withdraw_limit,
        max_withdraw_limit,
        borrow_fee,
        min_borrow_limit,
        max_borrow_limit,
        floor_cap_rate,
    })?;

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("earn config address: {:?}", ctx.accounts.config.key());
    msg!("earn config authority address: {:?}", ctx.accounts.config_authority.key());
    msg!("earn config owner address: {:?}", owner.key);
    msg!("earn config indexer address: {:?}", indexer.key);
    msg!("earn config fee vault address: {:?}", fee_vault.key);
    msg!("earn config freeze: {:?}", freeze);
    msg!("earn config protocol fee: {:?}", protocol_fee);
    msg!("earn config ltv: {:?}", ltv);
    msg!("earn config deposit fee: {:?}", deposit_fee);
    msg!("earn config min deposit limit: {:?}", min_deposit_limit);
    msg!("earn config max deposit limit: {:?}", max_deposit_limit);
    msg!("earn config withdraw fee: {:?}", withdraw_fee);
    msg!("earn config min withdraw limit: {:?}", min_withdraw_limit);
    msg!("earn config max withdraw limit: {:?}", max_withdraw_limit);
    msg!("earn config borrow fee: {:?}", borrow_fee);
    msg!("earn config min borrow limit: {:?}", min_borrow_limit);
    msg!("earn config max borrow limit: {:?}", max_borrow_limit);
    msg!("earn config floor cap rate: {:?}", floor_cap_rate);

    emit!(EventEarnConfigCreated{
        protocol: ctx.accounts.protocol.key(),
        creator: *owner.key,
        authority: *ctx.accounts.config_authority.key,
        indexer: *indexer.key,
        fee_vault: *fee_vault.key,
        freeze,
        protocol_fee,
        ltv,
        deposit_fee,
        min_deposit_limit,
        max_deposit_limit,
        withdraw_fee,
        min_withdraw_limit,
        max_withdraw_limit,
        borrow_fee,
        min_borrow_limit,
        max_borrow_limit,
        floor_cap_rate,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct EarnConfigCreate<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    /// CHECK Safe
    #[account()]
    pub indexer: AccountInfo<'info>,
    /// CHECK Safe
    #[account()]
    pub fee_vault: AccountInfo<'info>,
    /// CHECK CONFIG EARN AUTHORITY
    #[account(
        seeds = [seeds::CONFIG_EARN_AUTH, config.key().as_ref()],
        bump,
    )]
    pub config_authority: AccountInfo<'info>,
    #[account(
        init,
        seeds = [seeds::CONFIG_EARN, protocol.key().as_ref(), token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = EarnConfig::INIT_SPACE+(3*8),
    )]
    pub config: AccountLoader<'info, EarnConfig>,
    /// CHECK Safe
    pub token_mint: AccountInfo<'info>,

    #[account(mut, address = protocol.load()?.owner @ Errors::NotOwner)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}