use cosmwasm_std::{Binary, Deps, entry_point, Env, Order, StdResult, to_json_binary};
use cw_storage_plus::Bound;

use crate::msg::{ChainResponse, GetCalculatedRewardResponse, GetUserRegistrationsResponse, QueryMsg, SupportedChainsResponse, UserChainResponse};
use crate::state::{Chain, SUPPORTED_CHAINS, user_chain_registrations};

pub const DEFAULT_LIMIT: u64 = 30;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SupportedChains { limit, start_after } => to_json_binary(&query_supported_chains(deps, limit, start_after)?),
        QueryMsg::UserRegistrations { address, limit, start_after } => to_json_binary(&query_user_registrations(deps, address, limit, start_after)?),
        QueryMsg::CalculateReward { address, chain_id, remote_address } => to_json_binary(&query_calculate_reward(deps, address, chain_id, remote_address)?), 
    }
}

pub fn query_supported_chains(
    deps: Deps,
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
        .map(|(id, chain)| ChainResponse { chain_id: id, connection_id: chain.connection_id, ica_address: chain.ica_address.map(|addr| addr.to_string())})
        .collect();

    Ok(SupportedChainsResponse { chains: supported_chains})
}

pub fn query_user_registrations(
    deps: Deps,
    address: String,
    limit: Option<u64>,
    _start_after: Option<String>, // TODO: Implement
) -> StdResult<GetUserRegistrationsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    //let start = start_after.map(Bound::exclusive); // TODO: Implement From Str for the PK
    
    let local_address = deps.api.addr_validate(&address)?;
    
    let user_chain_registrations = user_chain_registrations()
        .idx.local_address
        .prefix(local_address)
        .range(deps.storage, None, None, Order::Ascending)
        .take(limit as usize)
        .filter_map(|item| {
            let user_chain_registration = item.unwrap().1;
            Some(UserChainResponse { chain_id: user_chain_registration.chain_id, address: user_chain_registration.remote_address, validators: user_chain_registration.validators.clone()})
        })
        .collect::<Vec<UserChainResponse>>();
    
    Ok(GetUserRegistrationsResponse { user_chain_registrations})
}

pub fn query_calculate_reward(
    _deps: Deps,
    _local_address: String,
    _chain_id: String,
    _remote_address: String,
) -> StdResult<GetCalculatedRewardResponse> {
    Ok(GetCalculatedRewardResponse { reward: 42 })
}

#[cfg(test)]
mod tests {
    mod test_query_supported_chains {
        use cosmwasm_std::{coins, from_json};
        use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SupportedChainsResponse};
        use crate::query::query;

        #[test]
        fn test_query_supported_chains() {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg{ 
                admin: info.sender.to_string(),
                neutron_register_ica_fee: 1000000,
            }).unwrap();
            let add_chain_msg = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
            };
            execute(deps.as_mut(), mock_env(), info.clone(), add_chain_msg).unwrap();
            
            let query_msg = QueryMsg::SupportedChains { limit: None, start_after: None };
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
        use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi};

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, GetUserRegistrationsResponse, InstantiateMsg, QueryMsg, UserChainRegistrationInput};
        use crate::query::query;

        #[test]
        fn test_query_user_registrations() {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &coins(1000000, "untrn"));

            instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg{
                admin: info.sender.to_string(),
                neutron_register_ica_fee: 1000000,
            }).unwrap();
            let add_chain_msg1 = ExecuteMsg::AddSupportedChain {
                chain_id: "chain_id".to_string(),
                connection_id: "connection_id".to_string(),
            };
            execute(deps.as_mut(), mock_env(), info.clone(), add_chain_msg1).unwrap();
            let add_chain_msg2 = ExecuteMsg::AddSupportedChain {
                chain_id: "osmosis".to_string(),
                connection_id: "osmosis_connection_id".to_string(),
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
                registrations: vec![UserChainRegistrationInput {
                    chain_id: "chain_id".to_string(),
                    address: cosmos_remote_user_addr.to_string(),
                    validators: vec![cosmos_validator1.to_string(), cosmos_validator2.to_string()],
                }, UserChainRegistrationInput {
                    chain_id: "osmosis".to_string(),
                    address: osmosis_remote_user_addr.to_string(),
                    validators: vec![osmosis_validator1.to_string()],
                }]
            };
            execute(deps.as_mut(), mock_env(), info.clone(), register_user_msg).unwrap();
            
            let query_msg = QueryMsg::UserRegistrations { address: "local_user".to_string(), limit: None, start_after: None };
            let response = query(deps.as_ref(), mock_env(), query_msg).unwrap();
            let res: GetUserRegistrationsResponse = from_json(&response).unwrap();
            assert_eq!(res.user_chain_registrations.len(), 2);
        }
    }
}