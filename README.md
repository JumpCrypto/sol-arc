# ARC Data Standard

ARC stands for "Action Registry Core" and is a standard to represent on chain data using the Entity-Component pattern found in traditional game development. 

Code Provided:
- Core Data Standard Program (Code Complete)
- Rust SDK (+WASM) for CoreDS (TODO)
- Admin Registry (Code Complete)
- Rust SDK (+WASM) for Admin Registry (TODO)
- TSAB (Token Standard Action Bundle) similar to Metaplex Metadata (80% Complete)
- Rust SDK (+WASM) for TSAB (TODO)
- xNFT to view ARC NFTs (TODO)


## Abstract
The primary goal for the ARC Data Standard is to provide a unified interface for on chain games on Solana, but due it's versatility and adaptability, it may uniquely be situated to solve some other problems in the Solana NFT ecosystem as well. 

ARC stands for “Action, Registry, Core”, and uses Entity-Component-System (ECS) style architecture often found in video game development under the hood. The key here is to separate *data* from its *execution,* while creating dynamic data structures that boost interoperability and composability. 

The Core DS program is in charge of maintaining data buckets called entities. It’s a relatively small code surface, and could be frozen if needed, without worrying about schema upgrades (as we’ll get into in a little bit). The goal for the Core DS program is to provide a single program where all the Entity accounts can be queried from. 

The Registry programs sit on top of the Core DS program and usually are unique per *community*. These are governance programs whose goal is to validate if actions taken to make changes to Entities were done in accordance to the community’s governance policy.

Finally, most of the logic for using ARC DS lies in the Action Bundles, that can exist either on chain or off chain. These action bundles are groups of change functions that modify entity data buckets.

## Core

The Core DS program attempts to be as simple as possible to as to be frozen after audits. It’s goal is to define and track three types of accounts: Entities, ARCNFTs, and RegistryInstances. 

### Registry Instances

For every registry, or community, there might exist multiple instances. For example, a given game might have a single registry, but have each of it’s servers as different instances. For simple communities, like the pfp-nft community you might only have the one instance per registry, with a different registry per collection. More on registries down below.

```rs
seeds = [
	b"registry",
	registry.key().to_bytes().as_ref(),
	instance.to_be_bytes().as_ref()
]

#[account]
pub struct RegistryInstance {
    pub registry: Pubkey,
    pub instance: u64,
    pub entities: u64,
}
```
Registry instances keep track of the Registry Program (registry) that they belong to, their instance_id (u64), and how many entities have been created.

### Entities
An entity is the magical data bucket that keeps all sort of state through a BTreeMap that maps a Pubkey to a SerializedComponent. It also contains info about the registry and instance it’s correlated to, but we’ll cover what those are later. 

```rs
seeds = [
	b"entity",
  entity_id.to_be_bytes().as_ref(),
  registry_instance.key().as_ref()
]

#[account]
pub struct Entity {
    pub entity_id: u64,		
    pub instance: u64,
    pub registry: Pubkey,
    pub components: BTreeMap<Pubkey, SerializedComponent>,
}
```
1. Entity ID
    1. There is no standard way to allocate entity ids to entities. Games where entities are made quickly and there’s chances of collisions, ids might be given out through random u64 generation. Other instances, where new entity creation is methodical, entities might be incremented continuously (this approach would have the added benefit of automatically indexing all entity ids in a single counter value, which could be used to fetch entities later). 
2. Instance
    1. This is the u64 instance id for a given registry. Different instances for the same registry exist because you might want different game servers all governed by the same community. For example, in an ARC MMO, you might have a server that allows Portals which allow for quick travel, and in another “hard core” server, the community might not allow portals. Both of them have otherwise the same game rules and structure, just certain systems are turned on / off based on which server you’re on. 
3. Registry
    1. This is the program that lays out all the governance rules. This could be as simple as an Admin Registry (provided) that gives command of the rules to the person who instantiates the code, to a very complex token governed registry that gate keeps what action bundles can make changes to what components on which entities. More on this in the registry section.
4. Components
    1. This maps registered component pubkeys with a *SerializedComponent*. Basically, for any given component name (usually a url pointing to it’s unique schema registered with a Registry — more on this below) it maps to a bucket of bytes. This means that the Core DS program never really cares what data goes in and out of an entity, it leaves Registries to mark the data with their own stickers and deal with them as such.

### ARC NFT

ARC NFTs are Entities entangled with a sol mint. This allows them to be transferred and traded just like SPL Tokens, while tying Entity data to that SPL token. 

```rs
seeds = [
            b"arcnft",
            mint.key().as_ref(),
            entity.key().as_ref()
        ]

#[account] 
pub struct ARCNFT {
    pub entity: Pubkey,
    pub mint: Pubkey,
}
```


## Registries
A Registry is a *community* or *set of communities* that interact with the same set of components in roughly the same way. There isn’t a good formal definition I can give for registries, but it might make sense when thinking of them through examples. 

→ For games, Registries can encompass a full game, where the *instances* of the registry encompass each server of that game. Some games might have *lots* of instances; you might create a new instance every time you play the game with a friend, or maybe only a few instances, as official persistent servers. 

→ For PFP-Style/Metaplex Metadata NFTs, there might exist a registry *per IP* with an instance per collection drop. This way, the governance of that IP is handled by the same registry for all the different collections they drop. Or they might have just one registry per collection, and isolate their governance per drop.

One benefit is by isolating governance by community, each community can setup their own system, and don’t need to rely on one over arching system, while still enjoying the benefits of one over arching data storage contract. This significantly boosts interoperability with wallets and explorers by standardizing data formats while still having a very flexible system for on chain data.

A registry is in charge primarily of hosting two types of accounts; ComponentSchema and ActionBundleRegistration.

```rs
seeds = [
	schema_url.as_bytes()
]

#[account]
pub struct ComponentSchema{
    pub url: String,
}
```
The ComponentSchema account simply registers a schema url for a component and issues it a pubkey that identifies it within the Registry. When the registry is adding/modifying/removing components from entities, it uses this pubkey, just as it uses it when deciding which ActionBundles to give what kind of access.
```rs
seeds = [
            b"action_bundle_registration",
            registry_instance.key().as_ref(),
            action_bundle.key().as_ref()
        ]

#[account]
pub struct ActionBundleRegistration{
    pub action_bundle: Pubkey,
    pub instance: BTreeSet<u64>,
    pub can_mint: bool,
    pub components: BTreeSet<Pubkey>, //PDA of the Component Schema
}
```
ActionBundleRegistration keeps track of what Pubkeys can make changes to what components. It also specifies in which instances that action bundle can make changes. It also tracks if that action bundle has the ability to mint ARC NFTs and what specific components it can edit. 

## Action Bundles
Action Bundles are where all serialization and deserialization logic takes place for SerializedComponents. Action Bundles validate the state change based on community rules, Registries validate that the Action Bundle approving the change was approved by the community, and finally Core DS handles the data storage itself. You can think of the Action Bundle as a Bank Client, the Registry as a Banker, and the Core DS program as the Bank Vault in how their responsibilities tie together. 

## Use Cases
1. Games
2. PFP-Style/Metaplex Metadat NFTs
3. Off Chain Oracles

## FAQ

1. Can a Entity exist across multiple registries/instances? 
    1. No. This is because the *rules* governing the data inside an entity are hard locked to the governance of that entity. 
2. Can ComponentSchema Pubkeys be u64 schema_ids instead to save space?
    1. Possibly :- the deterministic nature of PDAs means we don’t need to worry about collisions, but theoretically, this job could be given up to the client to find a non collision u64 and submit that when registering a ComponentSchema (like we do for Entities).