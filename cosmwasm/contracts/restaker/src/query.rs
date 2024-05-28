use cosmwasm_std::{Binary, Deps, entry_point, Env, StdResult, to_json_binary};
use cw_storage_plus::Bound;

use crate::msg::{ChainResponse, QueryMsg, SupportedChainsResponse};
use crate::state::{Chain, SUPPORTED_CHAINS};

pub const DEFAULT_LIMIT: u64 = 30;

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SupportedChains { limit, start_after } => to_json_binary(&query_supported_chains(_deps, limit, start_after)?),
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
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit as usize)
        .collect::<Result<Vec<(String, Chain)>, _>>()?
        .into_iter()
        .map(|(id, chain)| ChainResponse { chain_id: id, connection_id: chain.connection_id, ica_address: chain.ica_address.map(|addr| addr.to_string())})
        .collect();

    Ok(SupportedChainsResponse { chains: supported_chains})
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
}