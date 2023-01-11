use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use std::collections::BTreeMap;

use crate::account::*;
use crate::state::*;
use crate::constant::*;

#[derive(Accounts)]
#[instruction(registry:Pubkey, instance:u64)]
pub struct InitRegistryInstance <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        space=8+RegistryInstance::get_max_size() as usize,
        seeds=[
            SEEDS_REGISTRYINSTANCE_PREFIX,
            registry.key().to_bytes().as_ref(),
            instance.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub registry_instance: Account<'info, RegistryInstance>,

    // Only the Registry can implment new instances of itself. It's left up to the registry on how to implement this.
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER
        ],
        bump,
        seeds::program = registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(entity_id:u64, components: BTreeMap<Pubkey,SerializedComponent>)]
pub struct InitEntity<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub registry_instance: Account<'info, RegistryInstance>,

    #[account(
        init,
        payer=payer,
        space=8+Entity::get_max_size() as usize+compute_comp_arr_max_size(&components.values().cloned().collect()), //It is expected this will get Realloc'd every time a component is added
        seeds = [
            SEEDS_ENTITY_PREFIX,
            entity_id.to_be_bytes().as_ref(),
            registry_instance.key().as_ref()
        ],
        bump,
    )]
    pub entity: Box<Account<'info, Entity>>,

    // Only the Entity's Registry can make changes to the Entity
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER,
        ],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
pub struct MintARCNFT<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub registry_instance: Account<'info, RegistryInstance>,
    pub entity: Box<Account<'info, Entity>>,
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer=payer,
        space=8+ARCNFT::get_max_size() as usize,
        seeds=[
            b"arcnft",
            mint.key().as_ref(),
            entity.key().as_ref()
        ],
        bump,
    )]
    pub arcnft: Account<'info, ARCNFT>,   

    // Only the Entity's Registry can make mint the NFT
    #[account(
        seeds = [
            SEEDS_ARCNFT_PREFIX
        ],
        bump,
        seeds::program = registry_instance.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(components:Vec<(Pubkey,SerializedComponent)>)]
pub struct AddComponent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() + compute_comp_arr_max_size(&components.iter().map(|tuple| tuple.1.clone() ).collect()),
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub entity: Account<'info, Entity>,

    // Only the Entity's registry can make changes to the Entity
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER
        ],
        bump,
        seeds::program = entity.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(removed_components: Vec<Pubkey>)]
pub struct RemoveComponent<'info> {
    #[account(mut)]
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = entity.to_account_info().data_len() - get_removed_size(&entity.components, &removed_components),
        realloc::payer = benefactor,
        realloc::zero = false,
    )]
    pub entity: Account<'info, Entity>,

    // Only the Entity's registry can make changes to the Entity
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER,
        ],
        bump,
        seeds::program = entity.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
#[instruction(components: Vec<(Pubkey, Vec<u8>)>)]
pub struct ModifyComponent<'info> {
    #[account(mut)]
    pub entity: Account<'info, Entity>,

    // Only the Entity's registry can make changes to the Entity
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER,
        ],
        bump,
        seeds::program = entity.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

#[derive(Accounts)]
pub struct RemoveEntity<'info>{
    #[account(mut)]
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    #[account(
        mut,
        constraint = entity.components.len() == 0, // Can only delete empty Entities
        close = benefactor
    )]
    pub entity: Account<'info, Entity>,

    // Only the Entity's registry can make changes to the Entity
    #[account(
        seeds = [
            SEEDS_REGISTRYSIGNER,
        ],
        bump,
        seeds::program = entity.registry.key()
    )]
    pub registry_signer: Signer<'info>
}

/************************************************ Utility Functions */
pub fn compute_comp_arr_max_size(components: &Vec<SerializedComponent>) -> usize {
    let mut max_size:usize = 0;
    for comp in components {
        max_size += comp.max_size as usize + SERIALIZED_COMPONENT_EXTRA_SPACE as usize;
    }
    return max_size;
}

pub fn get_removed_size(components: &BTreeMap<Pubkey, SerializedComponent>, removed_components: &Vec<Pubkey>) -> usize {
    let mut removed_size:usize = 0;
    for comp in removed_components {
        removed_size += components.get(comp).unwrap().max_size as usize + SERIALIZED_COMPONENT_EXTRA_SPACE as usize;
    }
    return removed_size;
}