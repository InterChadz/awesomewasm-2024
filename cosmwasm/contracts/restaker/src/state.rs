use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_macro::index_list;
use cw_storage_plus::{IndexedMap, Item, Map, MultiIndex};

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
    pub autocompound_cost: u128,   // Always in untrn
    pub denom: String,             // The native stake token of the dst chain
    pub ica_address: Option<Addr>, // When this is set, the chain is ready to be used
    pub ica_error: Option<String>, // When this is set, the ica setup has failed
}

#[cw_serde]
pub struct UserChainRegistration {
    pub local_address: Addr,
    pub chain_id: String,
    pub remote_address: String, // The address on the other chain
    pub validators: Vec<String>,
    pub delegator_delegations_reply_id: u64, // This is used to set up the ICQ query id (see reply.rs)
    pub delegator_delegations_icq_id: Option<u64>, // This is they ID we use to query the ICQ, if this is set the registration is in progress
}

pub const CONFIG: Item<Config> = Item::new("config");

// chain-id -> Chain
pub const SUPPORTED_CHAINS: Map<String, Chain> = Map::new("supported_chains");
pub const ICA_PORT_ID_TO_CHAIN_ID: Map<String, String> = Map::new("ica_port_id_to_chain_id");

// User registration states
pub const NEXT_REPLY_ID: Item<u64> = Item::new("next_reply_id");
pub const REPLY_ID_TO_USER_CHAIN_REGISTRATION: Map<u64, (Addr, String, String)> =
    Map::new("reply_id_to_user_chain_registration");

// Autocompound and delegate msgs state
pub const REPLY_ID_STORAGE: Item<Vec<u8>> = Item::new("reply_queue_id");
// TODO: Adjust this accordingly
pub const REPLY_ID_TO_USER_DELEGATE: Map<u64, (Addr, String, String)> =
    Map::new("reply_id_to_user_delegate");

// user_address -> balance
pub const USER_BALANCES: Map<Addr, Uint128> = Map::new("user_balances"); // Always in untrn

#[index_list(UserChainRegistration)]
pub struct UserChainRegistrationIndexes<'a> {
    pub local_address: MultiIndex<'a, Addr, UserChainRegistration, (Addr, String, String)>,
}

pub fn user_chain_registrations<'a>(
) -> IndexedMap<'a, (Addr, String, String), UserChainRegistration, UserChainRegistrationIndexes<'a>>
{
    let indexes = UserChainRegistrationIndexes {
        local_address: MultiIndex::new(
            |_pk: &[u8], u: &UserChainRegistration| u.local_address.clone(),
            "user_chain_registrations",
            "user_chain_registrations__local_address",
        ),
    };

    IndexedMap::new("user_chain_registrations", indexes)
}
