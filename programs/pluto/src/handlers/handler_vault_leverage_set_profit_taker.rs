use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::error::{ErrorLeverage, Errors};
use crate::event::{EventLeverageSetProfitTaker};
use crate::state::{LeverageConfig, Obligation, Protocol, VaultLeverage};
use crate::util::{
    seeds,
};
use crate::util::constant::MAX_OBLIGATION_POSITIONS;

pub fn handle(ctx: Context<VaultLeverageSetProfitTaker>, number: u8, state: bool, profit: u32, take: u32) -> Result<()> {
    check_freeze(&ctx)?;
    let vault = &mut ctx.accounts.vault.load_mut()?;
    let obligation = &mut ctx.accounts.obligation.load_mut()?;
    let owner = &mut ctx.accounts.owner;

    msg!("vault address: {:?}", ctx.accounts.vault.key());
    msg!("obligation address: {:?}", ctx.accounts.obligation.key());

    require!(number < MAX_OBLIGATION_POSITIONS, ErrorLeverage::InvalidPositionNumber);

    let position = &mut obligation.positions[number as usize];

    require_gt!(position.unit, 0, ErrorLeverage::NoPositionFound);

    let old_state = position.profit_taker;
    let old_profit = position.profit_target_rate;
    let old_take = position.profit_taking_rate;

    position.profit_taker = state;
    position.profit_target_rate = profit;
    position.profit_taking_rate = take;

    emit!(EventLeverageSetProfitTaker {
        vault: ctx.accounts.vault.key(),
        user: ctx.accounts.owner.key(),
        obligation: ctx.accounts.obligation.key(),
        position_id: position.id,
        position_number: number,
        old_state,
        new_state: state,
        old_profit,
        new_profit: profit,
        old_take,
        new_take: take,
    });

    Ok(())
}

#[inline(never)]
fn check_freeze(ctx: &Context<VaultLeverageSetProfitTaker>) -> Result<()> {
    require!(!(ctx.accounts.protocol.load()?.freeze && !ctx.accounts.protocol.load()?.freeze_leverage), ErrorLeverage::VaultFrozen);
    require!(!ctx.accounts.leverage_config.load()?.freeze, ErrorLeverage::VaultFrozen);
    Ok(())
}

#[derive(Accounts)]
pub struct VaultLeverageSetProfitTaker<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    #[account()]
    pub leverage_config: AccountLoader<'info, LeverageConfig>,
    #[account(
        mut,
        has_one = protocol @ Errors::InvalidProtocol,
        has_one = leverage_config @ Errors::InvalidConfig,
        has_one = token_collateral_token_mint,
        has_one = native_collateral_token_mint,
    )]
    pub vault: AccountLoader<'info, VaultLeverage>,
    #[account(
        mut,
        seeds = [seeds::OBLIGATION, vault.key().as_ref(), token_collateral_token_mint.key().as_ref(), native_collateral_token_mint.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub obligation: AccountLoader<'info, Obligation>,

    pub token_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub native_collateral_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}