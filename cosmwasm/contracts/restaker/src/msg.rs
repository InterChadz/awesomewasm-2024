use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use crate::icq::reconstruct::UserQueryData;

use crate::state::{Config, UserChainRegistration};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub neutron_register_ica_fee: u128,
    pub autocompound_threshold: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        config: Config,
    },
    AddSupportedChain {
        chain_id: String,
        connection_id: String,
        denom: String,           // The native staking token of a dst chain
        autocompound_cost: u128, // Always in untrn, this is the fee paid to the keepers for autocompounding
    },
    UpdateSupportedChain {
        chain_id: String,
        connection_id: String,
        denom: String,           // The native staking token of a dst chain
        autocompound_cost: u128, // Always in untrn, this is the fee paid to the keepers for autocompounding
    },
    RegisterUser {
        registrations: Vec<UserChainRegistrationInput>,
    },
    TopupUserBalance {
        // recipient: String, // TODO: nice to have thing
    },
    Autocompound {
        delegators_amount: u64,
    },
}

#[cw_serde]
pub struct UserChainRegistrationInput {
    pub chain_id: String,
    pub address: String,
    pub validators: Vec<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SupportedChainsResponse)]
    SupportedChains {
        limit: Option<u64>,
        start_after: Option<String>,
    },
    #[returns(GetUserRegistrationsResponse)]
    UserRegistrations {
        address: String,
        limit: Option<u64>,
        start_after: Option<String>,
    },
    #[returns(GetCalculatedRewardResponse)]
    CalculateReward {
        address: String,
        chain_id: String,
        remote_address: String,
    },
    #[returns(UserQueryData)]
    UserQuery {
        address: String,
        chain_id: String,
        remote_address: String,
    },
    #[returns(UserBalanceResponse)]
    UserBalance { address: String },
    #[returns(DueUserChainRegistrationsResponse)]
    DueUserChainRegistrations { delegators_amount: u64 },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
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
    pub remote_address: String,
    pub validators: Vec<String>,

    // Mostly for debugging, honestly
    pub delegator_delegations_reply_id: u64,
    pub delegator_delegations_icq_id: Option<u64>,
}

#[cw_serde]
pub struct GetUserRegistrationsResponse {
    pub user_chain_registrations: Vec<UserChainResponse>,
}

#[cw_serde]
pub struct RewardResponse {
    pub validator: String,
    pub reward: Vec<Coin>,
}

#[cw_serde]
pub struct GetCalculatedRewardResponse {
    pub rewards: Vec<RewardResponse>,
}

#[cw_serde]
pub struct UserBalanceResponse {
    pub balance: u128,
}

#[cw_serde]
pub struct DueUserChainRegistrationsResponse {
    pub due_user_chain_registrations: Vec<UserChainRegistration>,
}
