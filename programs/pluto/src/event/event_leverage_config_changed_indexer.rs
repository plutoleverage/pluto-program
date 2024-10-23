use anchor_lang::prelude::*;

#[event]
pub struct EventLeverageConfigChangeIndexer {
    pub old_indexer: Pubkey,
    pub indexer: Pubkey,
}