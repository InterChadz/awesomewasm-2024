use cosmwasm_std::{coins, DepsMut, entry_point, Env, MessageInfo, Response};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::interchain_txs::helpers::get_port_id;

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{Chain, CONFIG, ICA_PORT_ID_TO_CHAIN_ID, SUPPORTED_CHAINS};

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response<NeutronMsg>, ContractError> {
    match msg {
        ExecuteMsg::AddSupportedChain { chain_id, connection_id } => add_supported_chain(deps, env,  info, chain_id, connection_id),
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

            instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg{ 
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
}