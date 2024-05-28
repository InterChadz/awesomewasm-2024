use cosmwasm_std::{coins, DepsMut, entry_point, Env, MessageInfo, Response};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::interchain_txs::helpers::get_port_id;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, UserChainRegistrationInput};
use crate::state::{Chain, CONFIG, ICA_PORT_ID_TO_CHAIN_ID, SUPPORTED_CHAINS, USER_CHAIN_REGISTRATIONS, UserChainRegistration};

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response<NeutronMsg>, ContractError> {
    match msg {
        ExecuteMsg::AddSupportedChain { chain_id, connection_id } => add_supported_chain(deps, env, info, chain_id, connection_id),
        ExecuteMsg::RegisterUser { registrations } => register_user(deps, info, registrations),
    }
}

pub fn add_supported_chain(
    deps: DepsMut,
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

    if info.funds.len() != 1 || info.funds[0].denom != "untrn" || info.funds[0].amount.u128() != config.neutron_register_ica_fee {
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

    let register =
        NeutronMsg::register_interchain_account(connection_id.clone(), ica_id.clone(), Some(coins(config.neutron_register_ica_fee, "untrn")));

    Ok(Response::new()
        .add_attribute("action", "add_supported_chain")
        .add_attribute("chain_id", chain_id)
        .add_attribute("connection_id", connection_id)
        .add_attribute("ica_id", ica_id)
        .add_attribute("ica_port_id", ica_port_id)
        .add_message(register)
    )
}

pub fn register_user(
    deps: DepsMut,
    info: MessageInfo,
    registrations: Vec<UserChainRegistrationInput>,
) -> Result<Response<NeutronMsg>, ContractError> {
    for registration in registrations {
        let chain = SUPPORTED_CHAINS.load(deps.storage, registration.clone().chain_id)?;

        let chain_id = registration.clone().chain_id;
        let remote_address = registration.clone().address;

        if USER_CHAIN_REGISTRATIONS.may_load(deps.storage, (info.clone().sender, chain_id.clone(), remote_address.clone())).unwrap().is_some() {
            return Err(ContractError::ChainAlreadyRegisteredForUser {
                remote_address: remote_address.clone(),
                chain_id: chain_id.clone(),
                address: info.clone().sender.to_string(),
            });
        }

        USER_CHAIN_REGISTRATIONS.save(deps.storage, (info.clone().sender, chain_id.clone(), remote_address.clone()), &UserChainRegistration {
            chain_id,
            local_address: info.clone().sender,
            remote_address,
        })?;

        // TODO: Set up the ICQ stuff
    }

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    mod test_add_supported_chain {
        use cosmwasm_std::coins;
        use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::SUPPORTED_CHAINS;

        #[test]
        fn test_add_supported_chain() {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {
                admin: info.sender.to_string(),
                neutron_register_ica_fee: 1000000,
            }).unwrap();

            let msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
            };

            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            assert_eq!(1, res.messages.len());

            let chains = SUPPORTED_CHAINS.range(deps.as_ref().storage, None, None, cosmwasm_std::Order::Ascending)
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            assert_eq!(chains.len(), 1);

            let chain = chains.get(0).unwrap();
            assert_eq!(chain.0, "chain_id");
            assert_eq!(chain.1.connection_id, "connection_id");
        }
    }

    mod test_register_user {
        use cosmwasm_std::coins;
        use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::USER_CHAIN_REGISTRATIONS;

        #[test]
        fn test_register_user() {
            let mut deps = mock_dependencies();
            let creator_info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(deps.as_mut(), mock_env(), creator_info.clone(), InstantiateMsg {
                admin: creator_info.sender.to_string(),
                neutron_register_ica_fee: 1000000,
            }).unwrap();

            let add_supported_chain_msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
            };
            execute(deps.as_mut(), mock_env(), creator_info.clone(), add_supported_chain_msg).unwrap();

            let info = mock_info("user", &coins(1000000, "untrn"));
            let register_user_msg = ExecuteMsg::RegisterUser {
                registrations: vec![
                    crate::msg::UserChainRegistrationInput {
                        chain_id: "chain_id".to_string(),
                        address: "remote_test_address".to_string(),
                    }
                ]
            };
            let res = execute(deps.as_mut(), mock_env(), info.clone(), register_user_msg).unwrap();
            assert_eq!(0, res.messages.len());

            let registrations = USER_CHAIN_REGISTRATIONS.range(deps.as_ref().storage, None, None, cosmwasm_std::Order::Ascending)
                .map(|item| item.unwrap())
                .collect::<Vec<_>>();
            assert_eq!(registrations.len(), 1);

            let registration = registrations.get(0).unwrap();
            assert_eq!(registration.0, (info.clone().sender, "chain_id".to_string(), "remote_test_address".to_string()));
            assert_eq!(registration.1.local_address, info.sender);
            assert_eq!(registration.1.remote_address, "remote_test_address");
        }
    }
}