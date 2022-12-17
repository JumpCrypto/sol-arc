use anchor_lang::prelude::*;

use core_ds::account::MaxSize;
use crate::constant::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct ComponentMetadata{
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub is_mutable: bool,    
}

impl MaxSize for ComponentMetadata {
    fn get_max_size() -> u64 {
        return 32 + 32 + METADATA_NAME_MAX_SIZE + METADATA_SYMBOL_MAX_SIZE + METADATA_URI_MAX_SIZE + 1;
    }
}
