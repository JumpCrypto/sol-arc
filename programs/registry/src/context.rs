use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::account::*;
use crate::constant::*;

use core_ds::{
    self,
    account::*,
    program::CoreDs,
    state::SerializedComponent
};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[b"registry_signer"],
        bump,
        space=8+RegistryConfig::get_max_size() as usize
    )]
    pub registry_config: Account<'info, RegistryConfig>,
}

#[derive(Accounts)] 
pub struct InstanceRegistry<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,

    /// CHECK: Initialized via CPI
    #[account(mut)]
    pub registry_instance: AccountInfo<'info>,
    pub core_ds: Program<'info, CoreDs>,

    // Instance Authority is in charge of allowing new action_bundles onto this instance
    #[account(
        init,
        payer=payer,
        seeds=[
            b"instance_authority",
            registry_instance.key().as_ref()
        ],
        bump,
        space=8+InstanceAuthority::get_max_size() as usize,
    )]
    pub instance_authority: Account<'info, InstanceAuthority>

}

#[derive(Accounts)]
#[instruction(schema:String)]
pub struct RegisterComponent<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer=payer,
        seeds=[
            schema.as_bytes(),
        ],
        bump,
        space=8+(STRING_MAX_SIZE as usize)
    )]
    pub component: Account<'info, ComponentSchema>,

    #[account(
        mut,
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,
}

#[derive(Accounts)]
pub struct RegisterSystem <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CoreDS Registry Instance Account
    /// Make sure that its a Registry instance that belongs to *this* Registry
    #[account(
        constraint = registry_instance.registry.key() == program_id.key()
    )]
    pub registry_instance: Account<'info, RegistryInstance>,

    /// Make sure the instance authority is of the registry instance that's passed in
    #[account(
        constraint = instance_authority.instance == registry_instance.instance
    )]
    pub instance_authority: Account<'info, InstanceAuthority>,
    
    #[account(
        init,
        payer=payer,
        seeds=[
            b"action_bundle_registration",
            registry_instance.key().as_ref(),
            action_bundle.key().as_ref()
        ],
        bump,
        space=8+ActionBundleRegistration::get_max_size() as usize
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    /// CHECK: This can be any pubkey, but likely will be pubkey of 
    /// PDA Signer from System
    pub action_bundle: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>)]
pub struct AddComponentsToActionBundleRegistration <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CoreDS Registry Instance Account
    /// Make sure that its a Registry instance that belongs to *this* Registry
    #[account(
        constraint = registry_instance.registry.key() == program_id.key()
    )]
    pub registry_instance: Account<'info, RegistryInstance>,

    /// Make sure the instance authority is of the Registry instance that's passed in
    #[account(
        constraint = instance_authority.instance == registry_instance.instance
    )]
    pub instance_authority: Account<'info, InstanceAuthority>,
    
    #[account(
        mut,
        realloc = action_bundle_registration.to_account_info().data_len() + (components.len()*32),
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            b"action_bundle_registration",
            registry_instance.key().as_ref(),
            action_bundle.key().as_ref()
        ],
        bump,
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    /// CHECK: This can be any pubkey, but likely will be pubkey of 
    /// PDA Signer from System
    pub action_bundle: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(instances: Vec<u64>)]
pub struct AddInstancesToActionBundleRegistration <'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CoreDS Registry Instance Account
    /// Make sure that its a Registry instance that belongs to *this* Registry
    #[account(
        constraint = registry_instance.registry.key() == program_id.key()
    )]
    pub registry_instance: Account<'info, RegistryInstance>,

    /// Make sure the instance authority is of the Registry instance that's passed in
    #[account(
        constraint = instance_authority.instance == registry_instance.instance
    )]
    pub instance_authority: Account<'info, InstanceAuthority>,
    
    #[account(
        mut,
        realloc = action_bundle_registration.to_account_info().data_len() + (instances.len()*8),
        realloc::payer = payer,
        realloc::zero = false,
        seeds=[
            b"action_bundle_registration",
            registry_instance.key().as_ref(),
            action_bundle.key().as_ref()
        ],
        bump,
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    /// CHECK: This can be any pubkey, but likely will be pubkey of 
    /// PDA Signer from System
    pub action_bundle: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(entity_id:u64, components: BTreeMap<Pubkey, SerializedComponent>)]
pub struct InitEntity<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    /// CHECK: Initalized via CPI
    #[account(mut)]
    pub entity: AccountInfo<'info>,
    
    #[account(
        constraint = registry_instance.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&registry_instance.instance)
    )]
    pub registry_instance: Account<'info, RegistryInstance>,
    pub action_bundle: Signer<'info>,
    // All action_bundles can make any entities they want
    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key() && check_sys_registry(&components.keys().cloned().collect(), &action_bundle_registration.components)
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,
    pub core_ds: Program<'info, CoreDs>,     
}

#[derive(Accounts)]
pub struct MintARCNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub entity: Box<Account<'info, Entity>>,
    pub mint: Account<'info, Mint>,
    
    pub arcnft: AccountInfo<'info>,

    #[account(
        constraint = registry_instance.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&registry_instance.instance)
    )]
    pub registry_instance: Account<'info, RegistryInstance>,

    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key()
    )]
    pub action_bundle: Signer<'info>,

    #[account(
        constraint = action_bundle_registration.can_mint == true
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,
    pub core_ds: Program<'info, CoreDs>,     
}

#[derive(Accounts)]
#[instruction(components: Vec<(Pubkey, SerializedComponent)>)]
pub struct AddComponents<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,

    #[account(
        mut,
        constraint = entity.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&entity.instance)
    )]
    pub entity: Box<Account<'info, Entity>>,
    
    pub action_bundle: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key() && check_sys_registry(&components.iter().map(|tuple| tuple.0.clone() ).collect(), &action_bundle_registration.components)
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    pub core_ds: Program<'info, CoreDs>, 
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>)]
pub struct RemoveComponent<'info>{
    #[account(mut)]
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,

    #[account(
        mut,
        constraint = entity.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&entity.instance)
    )]
    pub entity: Account<'info, Entity>,
    
    pub action_bundle: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key() && check_sys_registry(&components, &action_bundle_registration.components)
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    pub core_ds: Program<'info, CoreDs>, 
}

#[derive(Accounts)]
#[instruction(components: Vec<Pubkey>, data:Vec<Vec<u8>>)]
pub struct ModifyComponent<'info>{
    //Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,

    #[account(
        mut,
        constraint = entity.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&entity.instance)
    )]
    pub entity: Account<'info, Entity>,
    
    pub action_bundle: Signer<'info>,
    
    // System is allowed to modify the component it's adding
    // System is a signer
    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key() && check_sys_registry(&components, &action_bundle_registration.components)
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    pub core_ds: Program<'info, CoreDs>, 
}

#[derive(Accounts)]
pub struct RemoveEntity<'info>{
    #[account(mut)]
    pub benefactor: Signer<'info>,
    pub system_program: Program<'info, System>,

    //Used to Sign Tx for the CPI
    #[account(
        seeds=[b"registry_signer"],
        bump,
    )]
    pub registry_config: Account<'info, RegistryConfig>,

    #[account(
        mut,
        constraint = entity.registry.key() == program_id.key() && action_bundle_registration.instances.contains(&entity.instance) && entity.components.len() == 0
    )]
    pub entity: Account<'info, Entity>,
    
    pub action_bundle: Signer<'info>,
    
    // ANY registered action_bundle can close an empty entity
    #[account(
        constraint = action_bundle_registration.action_bundle.key() == action_bundle.key()
    )]
    pub action_bundle_registration: Account<'info, ActionBundleRegistration>,

    pub core_ds: Program<'info, CoreDs>, 
}


/*************************************************UTIL Functions */

pub fn check_sys_registry(components: &Vec<Pubkey>, action_bundle_components: &BTreeSet<Pubkey>) -> bool {
    for comp in components {
        if !action_bundle_components.contains(comp) {
            return false;
        }
    }
    return true;
}