use cosmwasm_std::{coins, entry_point, DepsMut, Env, MessageInfo, Response, SubMsg};
use cw0::must_pay;
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::interchain_queries::types::QueryPayload;
use neutron_sdk::interchain_txs::helpers::get_port_id;

use crate::error::ContractError;
use crate::icq::keys::create_all_icq_keys_for_user;
use crate::msg::{ExecuteMsg, UserChainRegistrationInput};
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
        } => add_supported_chain(deps, env, info, chain_id, connection_id),
        ExecuteMsg::RegisterUser { registrations } => register_user(deps, info, registrations),
        ExecuteMsg::TopupUserBalance {} => topup_user_balance(deps, env, info),
        ExecuteMsg::Autocompound {} => autocompound(deps, env, info),
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

    Ok(Response::new())
}

pub fn add_supported_chain(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    chain_id: String,
    connection_id: String,
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

pub fn register_user(
    deps: DepsMut<NeutronQuery>,
    info: MessageInfo,
    registrations: Vec<UserChainRegistrationInput>,
) -> Result<Response<NeutronMsg>, ContractError> {
    let mut icq_msgs: Vec<SubMsg<NeutronMsg>> = Vec::new();

    let mut next_reply_id = NEXT_REPLY_ID.load(deps.storage).unwrap();
    deps.api
        .debug(format!("WASMDEBUG: next_reply_id: {}", next_reply_id).as_str());
    for registration in registrations {
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

        let user_chain_reg = UserChainRegistration {
            chain_id: chain_id.clone(),
            local_address: info.clone().sender,
            remote_address: remote_address.clone(),
            validators: registration.clone().validators,
            delegator_delegations_reply_id: next_reply_id,
            delegator_delegations_icq_id: None, // This will be updated in the reply
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
        let icq_keys = create_all_icq_keys_for_user(
            remote_address.clone(),
            registration.clone().validators,
            None,
        ).unwrap();
        let icq_msg = NeutronMsg::register_interchain_query(
            QueryPayload::KV(icq_keys),
            chain.connection_id,
            5)
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

    Ok(Response::new())
}

pub fn autocompound(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<NeutronMsg>, ContractError> {
    todo!();

    // Iterate over all user_chain_registrations and autocompound
    let registrations = user_chain_registrations()
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.unwrap())
        .collect::<Vec<_>>();

    for ((src_addr, dst_chain_id, dst_addr), _) in registrations {
        // Autocompound for each registration
        // only if the given user has enough topped up balance to cover protocol fees
        let balance = USER_BALANCES
            .load(deps.storage, src_addr)
            .unwrap_or_default();
    }

    Ok(Response::new()) // Return an empty response
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
                },
            )
            .unwrap();

            let new_admin = "new_admin".to_string();
            let new_fee = 2000000;
            let msg = ExecuteMsg::UpdateConfig {
                config: crate::state::Config {
                    admin: Addr::unchecked(&new_admin),
                    neutron_register_ica_fee: new_fee,
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
                },
            )
                .unwrap();

            let msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
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
        use cosmwasm_std::{coins, Order, StdResult};
        use cosmwasm_std::testing::{mock_env, mock_info, MockApi};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::{NEXT_REPLY_ID, user_chain_registrations};
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
                },
            )
                .unwrap();

            let add_supported_chain_msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
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
}
