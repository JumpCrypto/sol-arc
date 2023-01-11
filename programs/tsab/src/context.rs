use anchor_lang::prelude::*;

use anchor_spl::token::Mint;
use core_ds::account::{MaxSize, RegistryInstance};
use core_ds::program::CoreDs;
use registry::account::{RegistryConfig, ActionBundleRegistration};
use registry::program::Registry;
use crate::account::TSABConfig;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Signer<'info>,

    #[account(
        init,
        payer=payer,
        space=8+TSABConfig::get_max_size() as usize,
        seeds=[
            b"tsab_signer"
        ],
        bump,
    )]
    pub tsab_config: Account<'info, TSABConfig>,
}

#[derive(Accounts)]
pub struct MintMetdata<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Signer<'info>,

    //SPL Mint
    pub mint: Account<'info, Mint>,

    //AB Accounts
    //// AB Config/Signer
    pub tsab_config: Account<'info, TSABConfig>,

    // New Accounts created via CPI
    //// Entity
    /// CHECK: Created via CPI
    pub entity: AccountInfo<'info>,
    //// ARCNFT
    /// CHECK: Created via CPI
    pub arcnft: AccountInfo<'info>,
    
    // Registry Accounts
    //// Registry Config/Signer
    pub registry_config: Account<'info, RegistryConfig>,
    //// Registry Program
    pub registry_program: Program<'info, Registry>,
    //// AB Registration
    pub tsab_registration: Account<'info, ActionBundleRegistration>,

    // CoreDS Accounts
    //// CoreDS Program
    pub core_ds_program: Program<'info, CoreDs>,
    //// Registry Instance
    pub registry_instance: Account<'info, RegistryInstance>,
}
