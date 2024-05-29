use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub neutron_register_ica_fee: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        config: Config,
    },
    AddSupportedChain {
        chain_id: String,
        connection_id: String,
    },
    RegisterUser {
        registrations: Vec<UserChainRegistrationInput>,
    },
    TopupUserBalance {
        // recipient: String, // TODO: nice to have thing
    },
    Autocompound {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SupportedChainsResponse)]
    SupportedChains {
        limit: Option<u64>,
        start_after: Option<String>,
    },
    #[returns(UserRegistrationsResponse)]
    UserRegistrations {
        address: String,
        limit: Option<u64>,
        start_after: Option<String>,
    },
    #[returns(CalculatedRewardResponse)]
    CalculateReward {
        address: String,
        chain_id: String,
        remote_address: String,
    },
}

#[cw_serde]
pub struct UserChainRegistrationInput {
    pub chain_id: String,
    pub address: String,
    pub validators: Vec<String>,
}

#[cw_serde]
pub struct ChainResponse {
    pub chain_id: String,
    pub connection_id: String,
    pub ica_address: Option<String>, // When this is set, the chain is ready to be used, until then dont use it
}

#[cw_serde]
pub struct SupportedChainsResponse {
    pub chains: Vec<ChainResponse>,
}

#[cw_serde]
pub struct UserChainResponse {
    pub chain_id: String,
    pub address: String,
    pub validators: Vec<String>,

    // Mostly for debugging, honestly
    pub delegator_delegations_reply_id: u64,
    pub delegator_delegations_icq_id: Option<u64>,
}

#[cw_serde]
pub struct UserRegistrationsResponse {
    pub user_chain_registrations: Vec<UserChainResponse>,
}

#[cw_serde]
pub struct CalculatedRewardResponse {
    pub total_delegation: u128,
    pub reward: u128,
}
