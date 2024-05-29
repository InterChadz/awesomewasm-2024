use cosmwasm_std::{
    coin, coins, entry_point, BankMsg, DepsMut, Env, MessageInfo, Response, StdError, SubMsg,
};
use cw0::must_pay;
use cw_storage_plus::PrefixBound;
use interchain_queries::v047::register_queries::new_register_delegator_delegations_query_msg;
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::interchain_queries;
use neutron_sdk::interchain_txs::helpers::get_port_id;

use crate::error::ContractError;
use crate::helpers::get_delegate_submsg;
use crate::msg::{ExecuteMsg, UserChainRegistrationInput};
use crate::query::query_calculate_reward;
use crate::state::{
    user_chain_registrations, Chain, Config, UserChainRegistration, CONFIG,
    ICA_PORT_ID_TO_CHAIN_ID, NEXT_REPLY_ID, REPLY_ID_TO_USER_CHAIN_REGISTRATION, SUPPORTED_CHAINS,
    USER_BALANCES,
};

//const STAKING_STORE_KEY: &str = "staking";
//const STAKING_DELEGATION_KEY_PREFIX: u8 = 0x31;

#[entry_point]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { config } => update_config(deps, info, config),
        ExecuteMsg::AddSupportedChain {
            chain_id,
            connection_id,
            denom,
            autocompound_cost,
        } => add_supported_chain(
            deps,
            env,
            info,
            chain_id,
            connection_id,
            denom,
            autocompound_cost,
        ),
        ExecuteMsg::UpdateSupportedChain {
            chain_id,
            connection_id,
            denom,
            autocompound_cost,
        } => update_supported_chain(
            deps,
            env,
            info,
            chain_id,
            connection_id,
            denom,
            autocompound_cost,
        ),
        ExecuteMsg::RegisterUser { registrations } => register_user(env, deps, info, registrations),
        ExecuteMsg::TopupUserBalance {} => topup_user_balance(deps, env, info),
        ExecuteMsg::Autocompound { delegators_amount } => {
            autocompound(deps, env, info, delegators_amount)
        }
    }
}

pub fn update_config(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    new_config: Config,
) -> Result<Response<NeutronMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Validate admin is caller
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::new()
        .add_attribute("action", "update_config")
        .add_attribute("config", format!("{:?}", config)))
}

pub fn add_supported_chain(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    chain_id: String,
    connection_id: String,
    denom: String,
    autocompound_cost: u128,
) -> Result<Response<NeutronMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if (SUPPORTED_CHAINS.may_load(deps.storage, chain_id.clone())?).is_some() {
        return Err(ContractError::ChainAlreadyExists {});
    }

    if info.funds.len() != 1
        || info.funds[0].denom != "untrn"
        || info.funds[0].amount.u128() != config.neutron_register_ica_fee
    {
        return Err(ContractError::NotEnoughFunds {
            required_amount: config.neutron_register_ica_fee.into(),
            actual_amount: info.funds[0].amount.u128(),
        });
    }

    let ica_id = format!("restake-{}", chain_id);
    let ica_port_id = get_port_id(env.contract.address.as_str(), &ica_id);
    let chain = Chain {
        connection_id: connection_id.clone(),
        ica_id: ica_id.clone(),
        ica_port_id: ica_port_id.clone(),
        autocompound_cost,
        denom,
        ica_address: None,
        ica_error: None,
    };

    SUPPORTED_CHAINS.save(deps.storage, chain_id.clone(), &chain)?;
    ICA_PORT_ID_TO_CHAIN_ID.save(deps.storage, ica_port_id.clone(), &chain_id)?;

    let register = NeutronMsg::register_interchain_account(
        connection_id.clone(),
        ica_id.clone(),
        Some(coins(config.neutron_register_ica_fee, "untrn")),
    );

    Ok(Response::new()
        .add_attribute("action", "add_supported_chain")
        .add_attribute("chain_id", chain_id)
        .add_attribute("connection_id", connection_id)
        .add_attribute("ica_id", ica_id)
        .add_attribute("ica_port_id", ica_port_id)
        .add_message(register))
}

fn update_supported_chain(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    chain_id: String,
    connection_id: String,
    denom: String,
    autocompound_cost: u128,
) -> Result<Response<NeutronMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let chain = SUPPORTED_CHAINS.load(deps.storage, chain_id.clone())?;

    let ica_id = format!("restake-{}", chain_id);
    let ica_port_id = get_port_id(env.contract.address.as_str(), &ica_id);
    let chain = Chain {
        connection_id,
        ica_id,
        ica_port_id,
        autocompound_cost,
        denom,
        ica_address: chain.ica_address,
        ica_error: chain.ica_error,
    };

    SUPPORTED_CHAINS.save(deps.storage, chain_id, &chain)?;

    Ok(Response::new())
}

// TODO remove_supported_chain

pub fn register_user(
    env: Env,
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    registrations: Vec<UserChainRegistrationInput>,
) -> Result<Response<NeutronMsg>, ContractError> {
    let mut icq_msgs: Vec<SubMsg<NeutronMsg>> = Vec::new();

    let mut next_reply_id = NEXT_REPLY_ID.load(deps.storage).unwrap();
    deps.api
        .debug(format!("WASMDEBUG: next_reply_id: {}", next_reply_id).as_str());
    for registration in registrations {
        // TODO_NICE: handle this .load with a may_load
        let chain = SUPPORTED_CHAINS.load(deps.storage, registration.clone().chain_id)?;

        let chain_id = registration.clone().chain_id;
        let remote_address = registration.clone().address;

        if user_chain_registrations()
            .may_load(
                deps.storage,
                (
                    info.clone().sender,
                    chain_id.clone(),
                    remote_address.clone(),
                ),
            )
            .unwrap()
            .is_some()
        {
            return Err(ContractError::ChainAlreadyRegisteredForUser {
                remote_address: remote_address.clone(),
                chain_id: chain_id.clone(),
                address: info.clone().sender.to_string(),
            });
        }

        // Get config to calculate next_compound_height
        let config = CONFIG.load(deps.storage)?;

        let user_chain_reg = UserChainRegistration {
            chain_id: chain_id.clone(),
            local_address: info.clone().sender,
            remote_address: remote_address.clone(),
            validators: registration.clone().validators,
            delegator_delegations_reply_id: next_reply_id,
            delegator_delegations_icq_id: None,
            next_compound_height: env.block.height + config.autocompound_threshold,
        };
        user_chain_registrations().save(
            deps.storage,
            (
                info.clone().sender,
                chain_id.clone(),
                remote_address.clone(),
            ),
            &user_chain_reg,
        )?;
        REPLY_ID_TO_USER_CHAIN_REGISTRATION
            .save(
                deps.storage,
                next_reply_id,
                &(
                    user_chain_reg.local_address,
                    user_chain_reg.chain_id,
                    user_chain_reg.remote_address,
                ),
            )
            .unwrap();

        // ICQ stuff:
        let icq_msg = new_register_delegator_delegations_query_msg(
            chain.connection_id,
            remote_address,
            registration.validators,
            5,
        )
        .unwrap();

        let sub_msg = SubMsg::reply_on_success(icq_msg, next_reply_id);
        icq_msgs.push(sub_msg);

        next_reply_id += 1;

        /*let converted_addr_bytes = decode_and_convert(&remote_address).unwrap();
        let delegation_key = create_delegation_key(converted_addr_bytes).unwrap();

        let kv_key = KVKey {
            path: STAKING_STORE_KEY.to_string(),
            key: Binary(delegation_key),
        };
        let staking_delegation_icq_msg = NeutronMsg::register_interchain_query(
            QueryPayload::KV(vec![kv_key]),
            chain.connection_id,
            5, // 🤷
        ).unwrap();

        icq_msgs.push(staking_delegation_icq_msg);*/
    }

    NEXT_REPLY_ID.save(deps.storage, &next_reply_id).unwrap();

    Ok(Response::new()
        .add_attribute("action", "register_user")
        .add_submessages(icq_msgs))
}

/*fn create_delegation_key(delegator: AddressBytes) -> StdResult<AddressBytes> {
    let mut key: Vec<u8> = vec![STAKING_DELEGATION_KEY_PREFIX];
    key.extend_from_slice(delegator.as_slice());

    Ok(key)
}*/

pub fn topup_user_balance(
    _deps: DepsMut<NeutronQuery>,
    _env: Env,
    info: MessageInfo,
) -> Result<Response<NeutronMsg>, ContractError> {
    must_pay(&info, "untrn")?;

    // Topup the balance for a specific user
    USER_BALANCES.update(
        _deps.storage,
        info.sender,
        |balance| -> Result<_, ContractError> {
            Ok(balance.unwrap_or_default() + info.funds[0].amount)
        },
    )?;

    Ok(Response::new().add_attribute("action", "topup_user_balance"))
}

pub fn autocompound(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    delegators_amount: u64,
) -> Result<Response<NeutronMsg>, ContractError> {
    let current_height = env.block.height;

    // end bounds for the range query
    let end = Some(PrefixBound::inclusive(current_height));

    // Iterate over all user_chain_registrations with next_compound_height <= current_height

    let registrations = user_chain_registrations()
        .idx
        .next_compound_height
        .prefix_range(deps.storage, None, end, cosmwasm_std::Order::Ascending)
        .map(|item| item.unwrap())
        .take(delegators_amount as usize)
        .collect::<Vec<_>>();

    let mut delegate_submsgs: Vec<SubMsg<NeutronMsg>> = vec![];
    let mut total_fees: u128 = 0;

    for ((src_addr, dst_chain_id, dst_addr), registration) in registrations {
        let mut balance = USER_BALANCES
            .load(deps.storage, src_addr.clone())
            .unwrap_or_default();

        let supported_chain = SUPPORTED_CHAINS
            .load(deps.storage, dst_chain_id.clone())
            .map_err(|_| StdError::not_found("Chain not found"))?;

        // Since a user could have staking position with more than one validator, we iterate over all of them
        for validator in registration.validators {
            // Only if the given user has enough topped up balance to cover protocol fees
            if balance.u128() < supported_chain.autocompound_cost {
                continue;
            }

            // TODO: Is it time to autocompound for that user based on the latest height we did, and the threshold we want? Add this to Config struct

            // Does this user has any rewards to compound?
            let calculate_reward = query_calculate_reward(
                deps.as_ref(),
                env.clone(),
                registration.local_address.to_string(),
                dst_chain_id.clone(),
                dst_addr.clone(),
            )?;

            // If there are not enough rewards to compound, continue
            // TODO_NICE: This could be use a threshold like at least > 0.1 (100000 udenom). Make this configurable.
            if calculate_reward.reward == 0 {
                continue;
            }

            // Here we know that user can autocompound.
            // Get the delegate submsg accordingly.
            let submsg = get_delegate_submsg(
                supported_chain.clone().ica_id,
                supported_chain.clone().connection_id,
                dst_addr.clone(),
                validator, // TODO: We should iter validators, querying the cumulated rewards foreach of them
                calculate_reward.reward,
                supported_chain.clone().denom,
                None, // TODO: timeout by Config struct, or default defined on helpers.rs?
            )?;
            delegate_submsgs.push(submsg);

            // Decrease in memory balance for the current user inside the validators iteration
            balance = balance
                .checked_sub(supported_chain.autocompound_cost.into())
                .unwrap(); // TODO_NICE: Better handle this unwrap
            total_fees += supported_chain.autocompound_cost;
        }

        // Save the new USER_BALANCES for the current user
        USER_BALANCES.save(deps.storage, src_addr, &balance)?;
    }

    // Return a response only if there are any msgs to send, otherwise throw a ContractError.
    if !delegate_submsgs.is_empty() {
        // Bank message to send total_fees to info.sender keeper
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(total_fees, "untrn")],
        };
        Ok(Response::new()
            .add_attribute("action", "autocompound")
            .add_submessages(delegate_submsgs)
            .add_message(bank_msg))
    } else {
        Err(ContractError::NoRewardsToAutocompound {})
    }
}

#[cfg(test)]
mod tests {
    mod test_update_config {
        use cosmwasm_std::testing::{mock_env, mock_info};
        use cosmwasm_std::{coins, Addr};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::CONFIG;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_update_config() {
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

            let new_admin = "new_admin".to_string();
            let new_fee = 2000000;
            let msg = ExecuteMsg::UpdateConfig {
                config: crate::state::Config {
                    admin: Addr::unchecked(&new_admin),
                    neutron_register_ica_fee: new_fee,
                    autocompound_threshold: 100,
                },
            };

            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            assert_eq!(0, res.messages.len());

            let config = CONFIG.load(deps.as_ref().storage).unwrap();
            assert_eq!(config.admin, new_admin);
            assert_eq!(config.neutron_register_ica_fee, new_fee);
        }
    }

    mod test_add_supported_chain {
        use cosmwasm_std::coins;
        use cosmwasm_std::testing::{mock_env, mock_info};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::SUPPORTED_CHAINS;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_add_supported_chain() {
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

            let msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
                denom: "denom".to_string(),
                autocompound_cost: 100000,
            };

            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            assert_eq!(1, res.messages.len());

            let chains = SUPPORTED_CHAINS
                .range(
                    deps.as_ref().storage,
                    None,
                    None,
                    cosmwasm_std::Order::Ascending,
                )
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            assert_eq!(chains.len(), 1);

            let chain = chains.get(0).unwrap();
            assert_eq!(chain.0, "chain_id");
            assert_eq!(chain.1.connection_id, "connection_id");
        }
    }

    mod test_register_user {
        use cosmwasm_std::testing::{mock_env, mock_info, MockApi};
        use cosmwasm_std::{coins, Order, StdResult};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::{user_chain_registrations, NEXT_REPLY_ID};
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_register_user() {
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
            let res = execute(deps.as_mut(), mock_env(), info.clone(), register_user_msg).unwrap();
            assert_eq!(1, res.messages.len());

            let registrations = user_chain_registrations()
                .range(deps.as_ref().storage, None, None, Order::Ascending)
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            assert_eq!(registrations.len(), 1);

            let registration = registrations.get(0).unwrap();
            assert_eq!(
                registration.0,
                (
                    info.clone().sender,
                    "chain_id".to_string(),
                    remote_user_addr.to_string()
                )
            );
            assert_eq!(registration.1.local_address, info.sender);
            assert_eq!(registration.1.remote_address, remote_user_addr);
            assert_eq!(
                registration.1.validators,
                vec![validator1.to_string(), validator2.to_string()]
            );

            let user_registrations_by_local_address: Vec<_> = user_chain_registrations()
                .idx
                .local_address
                .prefix(info.sender)
                .range(deps.as_ref().storage, None, None, Order::Ascending)
                .collect::<StdResult<_>>()
                .unwrap();
            assert_eq!(user_registrations_by_local_address.len(), 1);
            assert_eq!(
                user_registrations_by_local_address.get(0).unwrap().1,
                registration.1
            );

            let next_reply_id = NEXT_REPLY_ID.load(deps.as_ref().storage).unwrap();
            assert_eq!(next_reply_id, 2);
        }
    }

    mod test_topup_user_balance {
        use crate::execute::execute;
        use crate::msg::ExecuteMsg;
        use crate::state::USER_BALANCES;
        use crate::testing::helpers::mock_neutron_dependencies;
        use cosmwasm_std::testing::{mock_env, mock_info};
        use cosmwasm_std::{coins, Uint128};

        #[test]
        fn test_topup_user_balance() {
            let mut deps = mock_neutron_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                ExecuteMsg::TopupUserBalance {},
            )
            .unwrap();
            assert_eq!(0, res.messages.len());

            let balance = USER_BALANCES
                .load(deps.as_ref().storage, info.sender)
                .unwrap();
            assert_eq!(balance, Uint128::new(1000000));
        }
    }

    // mod test_autocompound {
    //     use cosmwasm_std::testing::{mock_env, mock_info};
    //     use cosmwasm_std::{coins, Uint128};

    //     use crate::execute::execute;
    //     use crate::instantiate::instantiate;
    //     use crate::msg::{ExecuteMsg, InstantiateMsg};
    //     use crate::state::{Chain, SUPPORTED_CHAINS, USER_BALANCES};
    //     use crate::testing::helpers::mock_neutron_dependencies;

    //     #[test]
    //     fn test_autocompound() {
    //         let mut deps = mock_neutron_dependencies();
    //         let creator_info = mock_info("creator", &coins(1000000, "untrn"));

    //         instantiate(
    //             deps.as_mut(),
    //             mock_env(),
    //             creator_info.clone(),
    //             InstantiateMsg {
    //                 admin: creator_info.sender.to_string(),
    //                 neutron_register_ica_fee: 1000000,
    //                 autocompound_threshold: 100,
    //             },
    //         )
    //         .unwrap();

    //         let add_supported_chain_msg = ExecuteMsg::AddSupportedChain {
    //             chain_id: "chain_id".to_string(),
    //             connection_id: "connection_id".to_string(),
    //             denom: "denom".to_string(),
    //             autocompound_cost: 100000,
    //         };
    //         execute(
    //             deps.as_mut(),
    //             mock_env(),
    //             creator_info.clone(),
    //             add_supported_chain_msg,
    //         )
    //         .unwrap();

    //         let local_user_info = mock_info("local_user", &coins(1000000, "untrn"));

    //         let chain = Chain {
    //             connection_id: "connection_id".to_string(),
    //             ica_id: "ica_id".to_string(),
    //             ica_port_id: "ica_port_id".to_string(),
    //             autocompound_cost: 100000,
    //             denom: "denom".to_string(),
    //             ica_address: None,
    //             ica_error: None,
    //         };
    //         SUPPORTED_CHAINS
    //             .save(deps.as_mut().storage, "chain_id".to_string(), &chain)
    //             .unwrap();

    //         USER_BALANCES
    //             .save(
    //                 deps.as_mut().storage,
    //                 local_user_info.sender,
    //                 &Uint128::new(1000000),
    //             )
    //             .unwrap();

    //         // TOOD: Mock rewards on dst_chain
    //         // let remote_user_info = mock_info("remote_user", &coins(1000000, "untrn"));

    //         let keeper_info = mock_info("keeper", &vec![]);
    //         let res = execute(
    //             deps.as_mut(),
    //             mock_env(),
    //             keeper_info.clone(),
    //             ExecuteMsg::Autocompound {
    //                 delegators_amount: 1,
    //             },
    //         )
    //         .unwrap();
    //         assert_eq!(0, res.messages.len());
    //     }
    // }
}
