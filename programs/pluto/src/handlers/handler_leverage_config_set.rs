use anchor_lang::prelude::*;
use crate::error::Errors;
use crate::event::EventLeverageConfigSet;
use crate::state::{LeverageConfig, Protocol, SetLeverageConfigParams};
use crate::util::{
    seeds,
};

pub fn handle(
    ctx: Context<LeverageConfigSet>, freeze: bool,
    protocol_fee: u32, min_leverage: u32, max_leverage: u32, leverage_step: u32,
    leverage_fee: u32, min_leverage_limit: u64, max_leverage_limit: u64,
    deleverage_fee: u32, min_deleverage_limit: u64, max_deleverage_limit: u64,
    closing_fee: u32, spread_rate: u32, liquidation_fee: u32, liquidation_threshold: u32,
    liquidation_protocol_ratio: u32, slippage_rate: u32,
    emergency_eject_period: i64, saver_threshold: u32, saver_target_reduction: u32) -> Result<()> {
    let fee_vault = &ctx.accounts.fee_vault;
    let config = &mut ctx.accounts.config.load_mut()?;
    let owner = &mut ctx.accounts.payer;
    config.set_config(SetLeverageConfigParams{
        leverage_fee_vault: *fee_vault.key,
        freeze,
        protocol_fee,
        min_leverage,
        max_leverage,
        leverage_step,
        leverage_fee,
        min_leverage_limit,
        max_leverage_limit,
        deleverage_fee,
        min_deleverage_limit,
        max_deleverage_limit,
        closing_fee,
        spread_rate,
        liquidation_fee,
        liquidation_threshold,
        liquidation_protocol_ratio,
        slippage_rate,
        emergency_eject_period,
        saver_threshold,
        saver_target_reduction: saver_target_reduction,
    })?;

    msg!("protocol address: {:?}", ctx.accounts.protocol.key());
    msg!("leverage config address: {:?}", ctx.accounts.config.key());
    msg!("leverage config authority address: {:?}", ctx.accounts.config_authority.key());
    msg!("leverage config owner address: {:?}", owner.key);
    msg!("leverage config fee vault address: {:?}", fee_vault.key);
    msg!("leverage config freeze: {:?}", freeze);
    msg!("leverage config protocol fee: {:?}", protocol_fee);
    msg!("leverage config min leverage: {:?}", min_leverage);
    msg!("leverage config max leverage: {:?}", max_leverage);
    msg!("leverage config leverage step: {:?}", leverage_step);
    msg!("leverage config leverage fee: {:?}", leverage_fee);
    msg!("leverage config min leverage limit: {:?}", min_leverage_limit);
    msg!("leverage config max leverage limit: {:?}", max_leverage_limit);
    msg!("leverage config deleverage fee: {:?}", deleverage_fee);
    msg!("leverage config min deleverage limit: {:?}", min_deleverage_limit);
    msg!("leverage config max deleverage limit: {:?}", max_deleverage_limit);
    msg!("leverage config closing fee: {:?}", closing_fee);
    msg!("leverage config spread rate: {:?}", spread_rate);
    msg!("leverage config liquidation fee: {:?}", liquidation_fee);
    msg!("leverage config liquidation threshold: {:?}", liquidation_threshold);
    msg!("leverage config liquidation protocol ratio: {:?}", liquidation_protocol_ratio);
    msg!("leverage config slippage rate: {:?}", slippage_rate);
    msg!("leverage config emergency eject period: {:?}", emergency_eject_period);
    msg!("leverage config saver threshold: {:?}", saver_threshold);
    msg!("leverage config saver target reduction: {:?}", saver_target_reduction);

    emit!(EventLeverageConfigSet{
        fee_vault: *fee_vault.key,
        freeze,
        protocol_fee,
        min_leverage,
        max_leverage,
        leverage_step,
        leverage_fee,
        min_leverage_limit,
        max_leverage_limit,
        deleverage_fee,
        min_deleverage_limit,
        max_deleverage_limit,
        closing_fee,
        spread_rate,
        liquidation_fee,
        liquidation_threshold,
        liquidation_protocol_ratio,
        slippage_rate,
        emergency_eject_period,
        saver_threshold,
        saver_target_reduction,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct LeverageConfigSet<'info> {
    #[account()]
    pub protocol: AccountLoader<'info, Protocol>,
    /// CHECK Safe
    #[account()]
    pub fee_vault: AccountInfo<'info>,
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