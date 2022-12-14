use anchor_lang::prelude::*;

use crate::account::MaxSize;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct SerializedComponent{
    pub max_size: u64,
    pub data: Vec<u8>,
}

impl MaxSize for SerializedComponent {
    fn get_max_size() -> u64 {
        return 8+4;
    }
}