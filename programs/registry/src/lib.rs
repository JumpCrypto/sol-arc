use anchor_lang::prelude::*;
use std::collections::{BTreeSet, BTreeMap};
use core_ds::state::SerializedComponent;

declare_id!("H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3");

pub mod account;
pub mod context;
pub mod constant;
pub mod error;
pub mod event;
pub mod state;

//use account::*;
use context::*;
//use constant::*;
//use error::*;
//use event::*;
//use state::*;

#[program]
pub mod registry {

    use std::collections::BTreeMap;

    use super::*;

    pub fn initalize(ctx:Context<Initialize>, core_ds: Pubkey) -> Result<()> {
        ctx.accounts.registry_config.core_ds = core_ds;
        ctx.accounts.registry_config.components = 0;
        Ok(())
    }

    /**
     * Instance World should normally be regulated by governance; 
     * In this case, we allow anyone to instance a new dominari registry.
     * We also set the Instance Authority for the World to the Payer
     * This authority is the only one that can add action_bundles to a given instance
     */
    pub fn instance_registry(ctx:Context<InstanceRegistry>, instance:u64) -> Result<()> {
        let core_ds = ctx.accounts.core_ds.to_account_info();
        let accounts = core_ds::cpi::accounts::InitRegistryInstance {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            registry_instance: ctx.accounts.registry_instance.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info()
        };
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];

        let register_registry_ctx = CpiContext::new_with_signer(
            core_ds,
            accounts,
            signer_seeds
        );

        core_ds::cpi::init_registry(register_registry_ctx, ctx.program_id.key(), instance)?;
        ctx.accounts.instance_authority.instance = instance;
        ctx.accounts.instance_authority.authority = ctx.accounts.payer.key(); // fancier Worlds might have different governance setup for this
        
        Ok(())
    }

    /**
     * Anyone can register new components as long as they use unique URIs
     */
    pub fn register_component(ctx:Context<RegisterComponent>, schema:String) -> Result<()> {
        ctx.accounts.component.url = schema.clone();
        ctx.accounts.registry_config.components += 1;
        Ok(())
    }

    pub fn register_action_bundle(ctx: Context<RegisterSystem>) -> Result<()> {
        ctx.accounts.action_bundle_registration.action_bundle = ctx.accounts.action_bundle.key();
        ctx.accounts.action_bundle_registration.instances = BTreeSet::new();
        ctx.accounts.action_bundle_registration.instances.insert(ctx.accounts.registry_instance.instance);
        ctx.accounts.action_bundle_registration.can_mint = true;
        Ok(())
    }

    pub fn add_components_to_action_bundle_registration(ctx:Context<AddComponentsToActionBundleRegistration>, components:Vec<Pubkey>) -> Result<()> {
        for comp in components {
            ctx.accounts.action_bundle_registration.components.insert(comp);
        }
        Ok(())
    }

    pub fn add_instances_to_action_bundle_registration(ctx:Context<AddInstancesToActionBundleRegistration>, instances: Vec<u64>) -> Result<()> {
        for instance in instances{
            ctx.accounts.action_bundle_registration.instances.insert(instance);
        }
        Ok(())
    }

    pub fn init_entity(ctx:Context<InitEntity>, entity_id: u64, components: BTreeMap<Pubkey, SerializedComponent>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::InitEntity {
            entity: ctx.accounts.entity.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            registry_instance: ctx.accounts.registry_instance.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info(),
        };  
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::init_entity(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ), entity_id, components)?;
        
        Ok(())
    }

    pub fn mint_arcnft(ctx:Context<MintARCNFT>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::MintARCNFT {
            entity: ctx.accounts.entity.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            registry_instance: ctx.accounts.registry_instance.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            arcnft: ctx.accounts.arcnft.to_account_info(),
        };  
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::mint_arcnft(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ))?;
        
        Ok(())
    }

    pub fn req_add_component(ctx:Context<AddComponents>, components: Vec<(Pubkey,SerializedComponent)>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::AddComponent {
            payer: ctx.accounts.payer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info()
        };
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::add_components(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;
        Ok(())
    }

    pub fn req_remove_component(ctx:Context<RemoveComponent>, components: Vec<Pubkey>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::RemoveComponent {
            benefactor: ctx.accounts.benefactor.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info()
        };
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::remove_component(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;
        Ok(())
    }

    pub fn req_modify_component(ctx:Context<ModifyComponent>, components: Vec<(Pubkey, Vec<u8>)>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::ModifyComponent {
            entity: ctx.accounts.entity.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info()
        };
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::modify_components(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ), components)?;

        Ok(())
    }

    pub fn req_remove_entity(ctx:Context<RemoveEntity>) -> Result<()> {
        let accounts = core_ds::cpi::accounts::RemoveEntity {
            benefactor: ctx.accounts.benefactor.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            entity: ctx.accounts.entity.to_account_info(),
            registry_signer: ctx.accounts.registry_config.to_account_info()
        };
        let registry_signer_seeds:&[&[u8]] = &[
            b"registry_signer",
            &[*ctx.bumps.get("registry_config").unwrap()]
        ];
        let signer_seeds = &[registry_signer_seeds];
        
        core_ds::cpi::remove_entity(CpiContext::new_with_signer(
            ctx.accounts.core_ds.to_account_info(),
            accounts,
            signer_seeds
        ))?;

        Ok(())
    }

}