use anchor_lang::prelude::*;
use std::collections::BTreeMap;
use crate::state::*;


#[account]
pub struct RegistryInstance {
    pub registry: Pubkey,
    pub instance: u64,
    pub entities: u64,
}

impl MaxSize for RegistryInstance {
    fn get_max_size() -> u64 {
        return 32 + 8 + 8;
    }
}

#[account]
pub struct Entity {
    pub entity_id: u64,
    pub instance: u64,
    pub registry: Pubkey,
    pub components: BTreeMap<Pubkey, SerializedComponent>,
}

impl MaxSize for Entity {
    fn get_max_size() -> u64 {
        // Max size is listed with empty BTreeMap (4) with the expecation that it'll get realloc'd with new components
        return 8+8+32+4; 
    }
}

#[account] 
pub struct ARCNFT {
    pub entity: Pubkey,
    pub mint: Pubkey,
}

impl MaxSize for ARCNFT {
    fn get_max_size() -> u64 {
        return 32 + 32;
    }
}

pub trait MaxSize {
    fn get_max_size() -> u64;
}