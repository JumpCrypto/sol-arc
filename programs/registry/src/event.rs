use anchor_lang::prelude::*;

#[event]
pub struct NewRegistryInstance {
    pub registry_instance: Pubkey,
    pub instance_authority: Pubkey
}

#[event]
pub struct NewComponentRegistered {
    pub component: Pubkey,
    pub schema: String
}

#[event]
pub struct NewSystemRegistration {
    pub registry_instance: Pubkey,
    pub action_bundle: Pubkey,
    pub action_bundle_registration: Pubkey
}