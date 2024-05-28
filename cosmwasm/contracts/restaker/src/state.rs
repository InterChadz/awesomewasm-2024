use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub neutron_register_ica_fee: u128, // Always in untrn
}

#[cw_serde]
pub struct Chain {
    pub connection_id: String,
    pub ica_id: String,
    pub ica_port_id: String,
    pub ica_address: Option<Addr>, // When this is set, the chain is ready to be used
    pub ica_error: Option<String>, // When this is set, the ica setup has failed
}

#[cw_serde]
pub struct UserChainRegistration {
    pub local_address: Addr,
    pub chain_id: String,
    pub remote_address: String, // The address on the other chain
}

pub const CONFIG: Item<Config> = Item::new("config");

// chain-id -> Chain
pub const SUPPORTED_CHAINS: Map<String, Chain> = Map::new("supported_chains");
pub const ICA_PORT_ID_TO_CHAIN_ID: Map<String, String> = Map::new("ica_port_id_to_chain_id");

// (local_address, chain_id, remote_address) -> UserChainRegistration (e.g. neutron1.., cosmos-hub-1, cosmos1... -> UserChainRegistration)
pub const USER_CHAIN_REGISTRATIONS: Map<(Addr, String, String), UserChainRegistration> = Map::new("user_chain_registrations");
