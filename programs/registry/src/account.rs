use anchor_lang::prelude::*;
use core_ds::account::MaxSize;
use std::collections::BTreeSet;

use crate::constant::STRING_MAX_SIZE;

//use crate::state::*;

#[account]
pub struct RegistryConfig{
    pub core_ds: Pubkey,
    pub components: u64,
}

impl MaxSize for RegistryConfig {
    fn get_max_size() -> u64 {
        return 32+8;
    }
}

#[account]
pub struct InstanceAuthority{
    pub instance: u64,
    pub authority: Pubkey
}

impl MaxSize for InstanceAuthority {
    fn get_max_size() -> u64 {
        return 8+32;
    }
}

#[account]
pub struct ComponentSchema{
    pub url: String,
}

impl MaxSize for ComponentSchema {
    fn get_max_size() -> u64 {
        return STRING_MAX_SIZE;
    }
}

#[account]
pub struct ActionBundleRegistration{
    pub action_bundle: Pubkey,
    pub instances: BTreeSet<u64>,
    pub can_mint: bool,
    pub components: BTreeSet<Pubkey>, //PDA of the Component Schema
}

impl MaxSize for ActionBundleRegistration {
    fn get_max_size() -> u64 {
        return 32+8+1+4;
    }
}