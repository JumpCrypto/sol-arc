use anchor_lang::prelude::*;
use std::collections::BTreeMap;

declare_id!("GN5Ww5qa8ej4evFCJxMhV6AFEPKhD1Drdu8qYYptVgDJ");

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
use state::*;

#[program]
pub mod core_ds {

    use super::*;

    pub fn init_registry(ctx:Context<InitRegistryInstance>, registry:Pubkey, instance: u64) -> Result<()> {
        ctx.accounts.registry_instance.registry = registry;
        ctx.accounts.registry_instance.instance = instance;
        
        Ok(())
    }

    pub fn init_entity(ctx:Context<InitEntity>, entity_id:u64, components: BTreeMap<Pubkey,SerializedComponent>) -> Result<()> {
        // Increment registry Instance Entities
        ctx.accounts.registry_instance.entities += 1;

        // Set Entity Data
        ctx.accounts.entity.entity_id = entity_id;
        ctx.accounts.entity.registry = ctx.accounts.registry_instance.registry.key();
        ctx.accounts.entity.instance = ctx.accounts.registry_instance.instance;
        ctx.accounts.entity.components = components;
        
        Ok(())
    }
    
    pub fn mint_arcnft(ctx:Context<MintARCNFT>) -> Result<()> {
        ctx.accounts.arcnft.entity = ctx.accounts.entity.key();
        ctx.accounts.arcnft.mint = ctx.accounts.mint.key();
        Ok(())
    }

    pub fn add_components(ctx:Context<AddComponent>, components:Vec<(Pubkey,SerializedComponent)>) -> Result<()> {
        for comp in components {
            ctx.accounts.entity.components.insert(comp.0, comp.1);
        }
        
        Ok(())
    }

    pub fn remove_component(ctx:Context<RemoveComponent>, removed_components: Vec<Pubkey>) -> Result<()> {
        for comp in removed_components {
            ctx.accounts.entity.components.remove(&comp);
        }   

        Ok(())
    }

    pub fn modify_components(ctx:Context<ModifyComponent>, components: Vec<(Pubkey, Vec<u8>)>) -> Result<()> {
        for comp in components.iter() {
            let mut new_comp = ctx.accounts.entity.components.get(&comp.0).unwrap().clone();
            new_comp.data = comp.1.clone();
            ctx.accounts.entity.components.insert(comp.0.clone(), new_comp);
        }

        Ok(())
    }

    pub fn remove_entity(_ctx:Context<RemoveEntity>) -> Result<()> {
        Ok(())
    }

}

/*
    Entity Mint that's also a SPL Token
        // Initalize SPL Token
        let mint_ix = spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            &ctx.accounts.mint.key(), 
            &ctx.accounts.entity_owner.key(), 
            Some(&ctx.accounts.entity_owner.key()), 
            1
        )?;
        invoke(
            &mint_ix,
            &[ctx.accounts.mint.to_account_info()],
        )?;

        // Create ATA Account
        let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
            &ctx.accounts.payer.key(), 
            &ctx.accounts.entity_owner.key(),
            &ctx.accounts.mint.key()
        );

        invoke(
            &create_ata_ix,
            &[  
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mint_ata.to_account_info(),
                ctx.accounts.entity_owner.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.spl_token_program.to_account_info()
            ]
        )?;

        // Mint SPL Token (1) to Mint ATA
        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.spl_token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.mint_ata.to_account_info(),
                    authority: ctx.accounts.entity_owner.to_account_info()
                }
            ),
            1
        )?;
 */