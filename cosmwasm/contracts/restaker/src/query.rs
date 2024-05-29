use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::Bound;
use interchain_queries::v047::queries::query_delegations;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::interchain_queries;

use crate::helpers::get_due_user_chain_registrations;
use crate::msg::{
    ChainResponse, ConfigResponse, DueUserChainRegistrationsResponse, GetCalculatedRewardResponse,
    GetUserRegistrationsResponse, QueryMsg, SupportedChainsResponse, UserBalanceResponse,
    UserChainResponse,
};
use crate::state::{user_chain_registrations, Chain, CONFIG, SUPPORTED_CHAINS, USER_BALANCES};

pub const DEFAULT_LIMIT: u64 = 30;

#[entry_point]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::SupportedChains { limit, start_after } => {
            to_json_binary(&query_supported_chains(deps, limit, start_after)?)
        }
        QueryMsg::UserRegistrations {
            address,
            limit,
            start_after,
        } => to_json_binary(&query_user_registrations(
            deps,
            address,
            limit,
            start_after,
        )?),
        QueryMsg::CalculateReward {
            address,
            chain_id,
            remote_address,
        } => to_json_binary(&query_calculate_reward(
            deps,
            env,
            address,
            chain_id,
            remote_address,
        )?),
        QueryMsg::UserBalance { address } => to_json_binary(&query_user_balance(deps, address)?),
        QueryMsg::DueUserChainRegistrationsResponse { delegators_amount } => to_json_binary(
            &query_due_user_chain_registrations(deps, env, delegators_amount)?,
        ),
    }
}

pub fn query_config(deps: Deps<NeutronQuery>) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}

pub fn query_supported_chains(
    deps: Deps<NeutronQuery>,
    limit: Option<u64>,
    start_after: Option<String>,
) -> StdResult<SupportedChainsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    let start = start_after.map(Bound::exclusive);

    let supported_chains: Vec<ChainResponse> = SUPPORTED_CHAINS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit as usize)
        .collect::<Result<Vec<(String, Chain)>, _>>()?
        .into_iter()
        .map(|(id, chain)| ChainResponse {
            chain_id: id,
            connection_id: chain.connection_id,
            ica_address: chain.ica_address.map(|addr| addr.to_string()),
        })
        .collect();

    Ok(SupportedChainsResponse {
        chains: supported_chains,
    })
}

pub fn query_user_registrations(
    deps: Deps<NeutronQuery>,
    address: String,
    limit: Option<u64>,
    _start_after: Option<String>, // TODO: Implement
) -> StdResult<GetUserRegistrationsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    //let start = start_after.map(Bound::exclusive); // TODO: Implement From Str for the PK

    let local_address = deps.api.addr_validate(&address)?;

    let user_chain_registrations = user_chain_registrations()
        .idx
        .local_address
        .prefix(local_address)
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit as usize)
        .filter_map(|item| {
            let user_chain_registration = item.unwrap().1;
            Some(UserChainResponse {
                chain_id: user_chain_registration.chain_id,
                remote_address: user_chain_registration.remote_address,
                validators: user_chain_registration.validators.clone(),
                delegator_delegations_reply_id: user_chain_registration
                    .delegator_delegations_reply_id,
                delegator_delegations_icq_id: user_chain_registration.delegator_delegations_icq_id,
            })
        })
        .collect::<Vec<UserChainResponse>>();

    Ok(GetUserRegistrationsResponse {
        user_chain_registrations,
    })
}

pub fn query_calculate_reward(
    deps: Deps<NeutronQuery>,
    env: Env,
    local_address: String,
    chain_id: String,
    remote_address: String,
) -> StdResult<GetCalculatedRewardResponse> {
    let local_address = deps.api.addr_validate(&local_address)?;
    let user_reg =
        user_chain_registrations().load(deps.storage, (local_address, chain_id, remote_address))?;
    let icq_id = user_reg.delegator_delegations_icq_id.unwrap();
    let response = query_delegations(deps, env, icq_id).unwrap();

    let mut total_delegation = 0;
    response.delegations.iter().for_each(|delegation| {
        deps.api.debug(&format!("Delegation: {:?}", delegation));
        total_delegation += delegation.amount.amount.u128();
    });

    Ok(GetCalculatedRewardResponse {
        total_delegation,
        reward: 42,
    })
}

pub fn query_user_balance(
    deps: Deps<NeutronQuery>,
    address: String,
) -> StdResult<UserBalanceResponse> {
    let local_address = deps.api.addr_validate(&address)?;

    let balance = USER_BALANCES
        .may_load(deps.storage, local_address)
        .map(|balance| balance.unwrap_or_default())?;

    Ok(UserBalanceResponse {
        balance: balance.into(),
    })
}

pub fn query_due_user_chain_registrations(
    deps: Deps<NeutronQuery>, // Change to DepsMut<NeutronQuery>
    env: Env,
    delegators_amount: u64,
) -> StdResult<DueUserChainRegistrationsResponse> {
    let due_user_chain_registrations =
        get_due_user_chain_registrations(&deps, &env, delegators_amount).unwrap();

    Ok(DueUserChainRegistrationsResponse {
        due_user_chain_registrations,
    })
}

#[cfg(test)]
mod tests {
    mod test_query_supported_chains {
        use cosmwasm_std::testing::{mock_env, mock_info};
        use cosmwasm_std::{coins, from_json};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SupportedChainsResponse};
        use crate::query::query;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_query_supported_chains() {
            let mut deps = mock_neutron_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    admin: info.sender.to_string(),
                    neutron_register_ica_fee: 1000000,
                    autocompound_threshold: 100,
                },
            )
            .unwrap();
            let add_chain_msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
                denom: "denom".to_string(),
                autocompound_cost: 100000,
            };
            execute(deps.as_mut(), mock_env(), info.clone(), add_chain_msg).unwrap();

            let query_msg = QueryMsg::SupportedChains {
                limit: None,
                start_after: None,
            };
            let response = query(deps.as_ref(), mock_env(), query_msg).unwrap();
            let res: SupportedChainsResponse = from_json(&response).unwrap();
            assert_eq!(res.chains.len(), 1);
            assert_eq!(res.chains[0].chain_id, "chain_id");
            assert_eq!(res.chains[0].connection_id, "connection_id");
            assert_eq!(res.chains[0].ica_address, None);
        }
    }

    mod test_query_user_registrations {
        use cosmwasm_std::testing::{mock_env, mock_info, MockApi};
        use cosmwasm_std::{coins, from_json};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{
            ExecuteMsg, GetUserRegistrationsResponse, InstantiateMsg, QueryMsg,
            UserChainRegistrationInput,
        };
        use crate::query::query;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_query_user_registrations() {
            let mut deps = mock_neutron_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    admin: info.sender.to_string(),
                    neutron_register_ica_fee: 1000000,
                    autocompound_threshold: 100,
                },
            )
            .unwrap();
            let add_chain_msg1 = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
                denom: "denom".to_string(),
                autocompound_cost: 100000,
            };
            execute(deps.as_mut(), mock_env(), info.clone(), add_chain_msg1).unwrap();
            let add_chain_msg2 = ExecuteMsg::AddSupportedChain {
                chain_id: "osmosis".to_string(),
                connection_id: "osmosis_connection_id".to_string(),
                denom: "uosmo".to_string(),
                autocompound_cost: 100000,
            };
            execute(deps.as_mut(), mock_env(), info.clone(), add_chain_msg2).unwrap();
            let info = mock_info("local_user", &coins(1000000, "untrn"));
            let cosmos_mock_api = MockApi::default().with_prefix("cosmos");
            let cosmos_remote_user_addr = cosmos_mock_api.addr_make("remote_user");
            let cosmos_validator1 = cosmos_mock_api.addr_make("validator1");
            let cosmos_validator2 = cosmos_mock_api.addr_make("validator2");
            let osmosis_remote_user_addr = cosmos_mock_api.addr_make("osmo");
            let osmosis_validator1 = cosmos_mock_api.addr_make("osmo_validator1");

            let register_user_msg = ExecuteMsg::RegisterUser {
                registrations: vec![
                    UserChainRegistrationInput {
                        chain_id: "chain_id".to_string(),
                        address: cosmos_remote_user_addr.to_string(),
                        validators: vec![
                            cosmos_validator1.to_string(),
                            cosmos_validator2.to_string(),
                        ],
                    },
                    UserChainRegistrationInput {
                        chain_id: "osmosis".to_string(),
                        address: osmosis_remote_user_addr.to_string(),
                        validators: vec![osmosis_validator1.to_string()],
                    },
                ],
            };
            execute(deps.as_mut(), mock_env(), info.clone(), register_user_msg).unwrap();

            let query_msg = QueryMsg::UserRegistrations {
                address: "local_user".to_string(),
                limit: None,
                start_after: None,
            };
            let response = query(deps.as_ref(), mock_env(), query_msg).unwrap();
            let res: GetUserRegistrationsResponse = from_json(&response).unwrap();
            assert_eq!(res.user_chain_registrations.len(), 2);
        }
    }

    mod test_query_due_user_chain_registrations {
        use std::vec;

        use cosmwasm_std::testing::{mock_env, mock_info, MockApi};
        use cosmwasm_std::{coins, from_json};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{DueUserChainRegistrationsResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
        use crate::query::query;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_query_due_user_chain_registrations() {
            let mut deps = mock_neutron_dependencies();
            let creator_info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(
                deps.as_mut(),
                mock_env(),
                creator_info.clone(),
                InstantiateMsg {
                    admin: creator_info.sender.to_string(),
                    neutron_register_ica_fee: 1000000,
                    autocompound_threshold: 100,
                },
            )
            .unwrap();

            let add_supported_chain_msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
                denom: "denom".to_string(),
                autocompound_cost: 100000,
            };
            execute(
                deps.as_mut(),
                mock_env(),
                creator_info.clone(),
                add_supported_chain_msg,
            )
            .unwrap();

            let mock_api = MockApi::default().with_prefix("cosmos");
            let remote_user_addr = mock_api.addr_make("remote_user");

            let info = mock_info("local_user", &coins(1000000, "untrn"));
            let validator1 = mock_api.addr_make("validator1");
            let validator2 = mock_api.addr_make("validator2");
            let register_user_msg = ExecuteMsg::RegisterUser {
                registrations: vec![crate::msg::UserChainRegistrationInput {
                    chain_id: "chain_id".to_string(),
                    address: remote_user_addr.to_string(),
                    validators: vec![
                        validator1.clone().to_string(),
                        validator2.clone().to_string(),
                    ],
                }],
            };

            // Register a user at height 1000
            let mut mock_env = mock_env();
            mock_env.block.height = 1000;
            let res = execute(
                deps.as_mut(),
                mock_env.clone(),
                info.clone(),
                register_user_msg,
            )
            .unwrap();
            assert_eq!(1, res.messages.len());

            // Increase 99 blocks, we still should not be able to compound
            mock_env.block.height = 1099;
            let query_msg = QueryMsg::DueUserChainRegistrationsResponse {
                delegators_amount: 1,
            };
            let response = query(deps.as_ref(), mock_env.clone(), query_msg).unwrap();
            let res = from_json::<DueUserChainRegistrationsResponse>(&response).unwrap();
            assert_eq!(res.due_user_chain_registrations.len(), 0);

            // Compound time!
            mock_env.block.height = 1100; // init + autocompound_threshold set as 100
            let query_msg = QueryMsg::DueUserChainRegistrationsResponse {
                delegators_amount: 1,
            };
            let response = query(deps.as_ref(), mock_env, query_msg).unwrap();
            let res = from_json::<DueUserChainRegistrationsResponse>(&response).unwrap();
            assert_eq!(res.due_user_chain_registrations.len(), 1);
        }
    }
}
