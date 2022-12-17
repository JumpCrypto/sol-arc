use std::collections::BTreeMap;
use anchor_lang::prelude::*;
use core_ds::account::MaxSize;

#[account]
pub struct TSABConfig {
    pub authority: Pubkey,
    // Sha256(component_name) -> Component Pubkey
    pub components: BTreeMap<[u8;32], Pubkey>
}

impl MaxSize for TSABConfig {
    fn get_max_size() -> u64 {
        return 32+4;
    }
}