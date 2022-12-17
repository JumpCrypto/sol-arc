use anchor_lang::prelude::*;
use std::collections::BTreeMap;
use anchor_lang::solana_program::keccak::Hash;
use core_ds::account::MaxSize;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod context;
mod account;
mod component;
mod constant;

use context::*;
use component::*;

#[program]
pub mod tsab {
    use core_ds::state::SerializedComponent;

    use crate::component::ComponentMetadata;

    use super::*;

    pub fn initalize(ctx:Context<Initialize>, components:BTreeMap<[u8;32], Pubkey>) -> Result<()> {
        ctx.accounts.tsab_config.authority = ctx.accounts.payer.key();
        ctx.accounts.tsab_config.components = components;        
        Ok(())
    }

    pub fn mint_metadata(ctx:Context<MintMetdata>, entity_id: u64, metadata:ComponentMetadata) -> Result<()> {
        // Create Entity
        let system_signer_seeds:&[&[u8]] = &[
            b"tsab_signer",
            &[*ctx.bumps.get("tsab_config").unwrap()]
        ];
        let signer_seeds = &[system_signer_seeds];

        let init_entity_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::InitEntity{
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.entity.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                action_bundle: ctx.accounts.tsab_config.to_account_info(),
                action_bundle_registration: ctx.accounts.tsab_registration.to_account_info(),
                core_ds: ctx.accounts.core_ds_program.to_account_info(),

            },
            signer_seeds
        );

        let metadata_component_hash:[u8; 32] = Hash::new(b"metadata").to_bytes();
        let mut components = BTreeMap::new();
        components.insert(
            ctx.accounts.tsab_config.components.get(&metadata_component_hash).unwrap().key(),
            SerializedComponent {
                max_size: ComponentMetadata::get_max_size(),
                data: metadata.try_to_vec().unwrap()
            }
        );

        registry::cpi::init_entity(init_entity_ctx, entity_id, components)?;

        // Create ARCNFT
        let mint_arcnft_ctx = CpiContext::new_with_signer(
            ctx.accounts.registry_program.to_account_info(),
            registry::cpi::accounts::MintARCNFT{
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                registry_config: ctx.accounts.registry_config.to_account_info(),
                entity: ctx.accounts.entity.to_account_info(),
                registry_instance: ctx.accounts.registry_instance.to_account_info(),
                action_bundle: ctx.accounts.tsab_config.to_account_info(),
                action_bundle_registration: ctx.accounts.tsab_registration.to_account_info(),
                core_ds: ctx.accounts.core_ds_program.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                arcnft: ctx.accounts.arcnft.to_account_info(),
            },
            signer_seeds
        );

        registry::cpi::mint_arcnft(mint_arcnft_ctx)?;

        Ok(())
    }
}