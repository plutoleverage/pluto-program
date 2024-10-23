use anchor_lang::prelude::*;

#[event]
pub struct EventEarnConfigChangeIndexer {
    pub old_indexer: Pubkey,
    pub indexer: Pubkey,
}