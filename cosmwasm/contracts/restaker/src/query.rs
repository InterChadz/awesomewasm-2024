use std::str::FromStr;
use cosmwasm_std::{Binary, Decimal256, Deps, entry_point, Env, Order, StdError, StdResult, to_json_binary, Uint128, Uint256};
use cw_storage_plus::Bound;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::interchain_queries::{check_query_type, get_registered_query, query_kv_result};
use neutron_sdk::interchain_queries::types::QueryType;
use restaker_utils::rewards::calculate_delegation_rewards;
use restaker_utils::types::DelegatorStartingInfo as UtilsDelegatorStartingInfo;
use restaker_utils::types::ValidatorHistoricalRewards as UtilsValidatorHistoricalRewards;

use crate::icq::reconstruct::UserQueryData;
use crate::helpers::get_due_user_chain_registrations;
use crate::msg::{ChainResponse, ConfigResponse, DueUserChainRegistrationsResponse, GetCalculatedRewardResponse, GetUserRegistrationsResponse, QueryMsg, RewardResponse, SupportedChainsResponse, UserBalanceResponse, UserChainResponse};
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
        QueryMsg::UserQuery { address, chain_id, remote_address } => to_json_binary(&query_user_query(deps, address, chain_id, remote_address)?),
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

    let resp = get_registered_query(deps, icq_id).unwrap();
    check_query_type(resp.registered_query.query_type, QueryType::KV).unwrap();

    let user_query_data: UserQueryData = query_kv_result(deps, icq_id).unwrap();
    deps.api
        .debug(format!("WASMDEBUG: user_query_data, delegation len: {}, val len {}, starting_infos len {} historical_rewards len {}",
                       user_query_data.delegations.len(),
                       user_query_data.validators.len(),
                       user_query_data.delegator_starting_infos.len(),
                       user_query_data.validator_historical_rewards.len()
        ).as_str());

    let rewards = calculate_rewards(env, deps, user_query_data)?;

    Ok(GetCalculatedRewardResponse {
        rewards,
    })
}

fn calculate_rewards(env: Env, deps: Deps<NeutronQuery>, user_query_data: UserQueryData) -> Result<Vec<RewardResponse>, StdError> {
    let mut rewards: Vec<RewardResponse> = vec![];
    for delegation in user_query_data.delegations.iter() {
        let delegator_starting_info = user_query_data.delegator_starting_infos.iter().find(|dsi| dsi.validator == delegation.validator_address).unwrap();
        let shares_as_dec = Decimal256::from_atomics(
            Uint256::from_str(&delegation.shares)?,
            0, //DECIMAL_PLACES,
        ).unwrap();
        let validator = user_query_data.validators.iter().find(|v| v.operator_address == delegation.validator_address).unwrap();
        let validator_shares_as_dec = Decimal256::from_atomics(
            Uint256::from_str(&validator.all_shares)?,
            0, //DECIMAL_PLACES,
        ).unwrap();
        let validator_tokens_as_u128 = validator.tokens.parse::<u128>().unwrap();
        let historic_rewards = user_query_data.validator_historical_rewards.iter().find(|vhr| vhr.validator == delegation.validator_address).unwrap();
        let validator_current_rewards = user_query_data.validator_current_rewards.iter().find(|vcr| vcr.validator == delegation.validator_address).unwrap();
        let calculated_rewards = calculate_delegation_rewards(
            env.clone(),
            deps.into_empty(),
            UtilsDelegatorStartingInfo {
                height: delegator_starting_info.clone().height,
                stake: delegator_starting_info.clone().stake,
                previous_period: delegator_starting_info.clone().previous_period,
            },
            shares_as_dec,
            validator_shares_as_dec,
            Uint128::from(validator_tokens_as_u128),
            UtilsValidatorHistoricalRewards {
                cumulative_reward_ratio: historic_rewards.cumulative_reward_ratio.clone(),
                reference_count: historic_rewards.reference_count,
            },
            UtilsValidatorHistoricalRewards {
                cumulative_reward_ratio: validator_current_rewards.rewards.clone(),
                reference_count: 0,
            }
        ).unwrap();

        rewards.push(RewardResponse {
            validator: delegation.validator_address.clone(),
            reward: calculated_rewards,
        });
    }
    Ok(rewards)
}

pub fn query_user_query(
    deps: Deps<NeutronQuery>,
    local_address: String,
    chain_id: String,
    remote_address: String,
) -> StdResult<UserQueryData> {
    let local_address = deps.api.addr_validate(&local_address)?;
    let user_reg =
        user_chain_registrations().load(deps.storage, (local_address, chain_id, remote_address))?;
    let icq_id = user_reg.delegator_delegations_icq_id.unwrap();

    let user_query_data: UserQueryData = query_kv_result(deps, icq_id).unwrap();
    deps.api
        .debug(format!("WASMDEBUG: user_query_data, delegation len: {}, val len {}, starting_infos len {} historical_rewards len {}",
                       user_query_data.delegations.len(),
                       user_query_data.validators.len(),
                       user_query_data.delegator_starting_infos.len(),
                       user_query_data.validator_historical_rewards.len()
        ).as_str());

    Ok(user_query_data)
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
        use cosmwasm_std::{coins, from_json};
        use cosmwasm_std::testing::{mock_env, mock_info};

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
        use cosmwasm_std::{coins, from_json};
        use cosmwasm_std::testing::{mock_env, mock_info, MockApi};

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

    mod test_calculate_rewards {
        use cosmwasm_std::{Coin, Uint128};
        use cosmwasm_std::testing::mock_env;
        use crate::icq::reconstruct::{Delegation, DelegatorStartingInfoWithValidator, UserQueryData, Validator, ValidatorCurrentRewards, ValidatorHistoricalRewards};
        use crate::query::calculate_rewards;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_calculate_rewards() {
            /*
              Use the following yaml to populate the UserQueryData struct
                delegations:
  - delegator_address: cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
    shares: "1000000000000000000000000000000"
    validator_address: cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
  delegator_starting_infos:
  - delegator: cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw
    height: 7333
    previous_period: 11
    stake: "1000000000000000000000000000000"
    validator: cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
  validator_current_rewards:
  - period: 12
    rewards:
    - amount: "2444866473546000000000000000"
      denom: uatom
    validator: cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
  validator_historical_rewards:
  - cumulative_reward_ratio:
    - amount: "480297754365783730"
      denom: uatom
    reference_count: 2
    validator: cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
  validators:
  - all_shares: "1007000000000000000000000000000"
    operator_address: cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn
    tokens: "1007000000000"

             */
           let user_query_data = UserQueryData {
               delegations: vec![
                   Delegation {
                       delegator_address: "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw".to_string(),
                       shares: "1000000000000000000000000000000".to_string(),
                       validator_address: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string()
                   }
               ],
                validators: vec![
                     Validator {
                          operator_address: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string(),
                          tokens: "1007000000000".to_string(),
                          all_shares: "1007000000000000000000000000000".to_string()
                     }
                ],
                delegator_starting_infos: vec![
                    DelegatorStartingInfoWithValidator {
                        previous_period: 11,
                        stake: "1000000000000000000000000000000".to_string(),
                        height: 7333,
                        delegator: "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw".to_string(),
                        validator: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string()
                    }
                ],
                validator_historical_rewards: vec![
                    ValidatorHistoricalRewards {
                        validator: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string(),
                        cumulative_reward_ratio: vec![
                            Coin {
                                denom: "uatom".to_string(),
                                amount: "480297754365783730".parse().unwrap(),
                            }
                        ],
                        reference_count: 2
                    }
                ],
                validator_current_rewards: vec![
                    ValidatorCurrentRewards {
                        validator: "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn".to_string(),
                        period: 12,
                        rewards: vec![
                            Coin {
                                denom: "uatom".to_string(),
                                amount: "2444866473546000000000000000".parse().unwrap(),
                            }
                        ]
                    }
                ],
            };

            let deps = mock_neutron_dependencies();

            let rewards = calculate_rewards(mock_env(), deps.as_ref(), user_query_data).unwrap();
            assert_eq!(rewards.len(), 1);
            assert_eq!(rewards[0].validator, "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn");
            assert_eq!(rewards[0].reward.len(), 1);
            assert_eq!(rewards[0].reward[0].denom, "uatom");
            assert_eq!(rewards[0].reward[0].amount, Uint128::new(2444866473));
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
