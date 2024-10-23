use anchor_lang::prelude::*;

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LeverageAction {
    Idle,
    Open,
    AddCollateral,
    AddPosition,
    Close,
    Safe,
    Eject,
    Liquidate,
    Deleverage,
    TakeProfit,
}