use anchor_lang::prelude::*;

#[event]
pub struct EventProtocolSet{
    pub freeze: bool,
    pub freeze_earn: bool,
    pub freeze_lend: bool,
    pub freeze_leverage: bool,
}