use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, entry_point, Env, Response, StdError, StdResult};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::interchain_queries::{check_query_type, get_registered_query, query_kv_result};
use neutron_sdk::interchain_queries::types::QueryType;
use neutron_sdk::sudo::msg::{RequestPacket, SudoMsg};

use crate::icq::keys::{create_all_icq_keys_for_user, ValidatorHistoricalRange};
use crate::icq::reconstruct::UserQueryData;
use crate::state::{ICA_PORT_ID_TO_CHAIN_ID, SUPPORTED_CHAINS};

/// SudoPayload is a type that stores information about a transaction that we try to execute
/// on the host chain. This is a type introduced for our convenience.
#[cw_serde]
pub struct SudoPayload {
    pub message: String,
    pub port_id: String,
}

#[cw_serde]
struct OpenAckVersion {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<NeutronQuery>, env: Env, msg: SudoMsg) -> StdResult<Response<NeutronMsg>> {
    match msg {
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            deps,
            env,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        ),
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, query_id),
        SudoMsg::Error { request, details } => sudo_error(deps, request, details),
        _ => Ok(Response::default()),
    }
}

fn sudo_open_ack(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    port_id: String,
    _channel_id: String,
    _counterparty_channel_id: String,
    counterparty_version: String,
) -> StdResult<Response<NeutronMsg>> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_open_ack, port_id: {:?}", port_id).as_str());

    // The version variable contains a JSON value with multiple fields,
    // including the generated account address.
    let parsed_version: Result<OpenAckVersion, _> =
        serde_json_wasm::from_str(counterparty_version.as_str());

    // Update the storage record associated with the interchain account.
    if let Ok(parsed_version) = parsed_version {
        deps.api
            .debug(format!("WASMDEBUG: parsed_version: {:?}", parsed_version.clone()).as_str());
        let chain_id = ICA_PORT_ID_TO_CHAIN_ID.load(deps.storage, port_id.clone())?;

        SUPPORTED_CHAINS.update(
            deps.storage,
            chain_id.clone(),
            |existing_chain| -> StdResult<_> {
                let mut chain = existing_chain.unwrap();
                let address = Addr::unchecked(parsed_version.clone().address);
                chain.ica_address = Option::from(address);
                Ok(chain)
            },
        )?;
        return Ok(Response::new()
            .add_attribute("action", "sudo_open_ack")
            .add_attribute("port_id", port_id)
            .add_attribute("chain_id", chain_id)
            .add_attribute("address", parsed_version.address));
    }

    Err(StdError::generic_err("Can't parse counterparty_version"))
}

fn sudo_kv_query_result(deps: DepsMut<NeutronQuery>, query_id: u64) -> StdResult<Response<NeutronMsg>> {
    deps.api
        .debug(format!("WASMDEBUG: sudo_kv_query_result, query_id: {:?}", query_id).as_str());

    let resp = get_registered_query(deps.as_ref(), query_id).unwrap();
    check_query_type(resp.registered_query.query_type, QueryType::KV).unwrap();

    let user_query_data: UserQueryData = query_kv_result(deps.as_ref(), query_id).unwrap();
    deps.api
        .debug(format!("WASMDEBUG: user_query_data, delegation len: {}, val len {}, starting_infos len {} historical_rewards len {}",
                       user_query_data.delegations.len(),
                       user_query_data.validators.len(),
                       user_query_data.delegator_starting_infos.len(),
                       user_query_data.validator_historical_rewards.len()
        ).as_str());

    let delegation = user_query_data.delegations.get(0).unwrap();
    let validators = user_query_data.validators.into_iter().map(|v| v.operator_address).collect::<Vec<_>>();
    let validator_historical_range = user_query_data.delegator_starting_infos.into_iter()
        .map(|v| {
            ValidatorHistoricalRange {
                validator: v.validator,
                period: v.previous_period,
            }
        }).collect::<Vec<_>>();
    let icq_keys = create_all_icq_keys_for_user(delegation.clone().delegator_address, validators, Some(validator_historical_range)).unwrap();
    let icq_msg = NeutronMsg::update_interchain_query(query_id, Some(icq_keys), Some(6), None).unwrap();

    Ok(Response::new().add_message(icq_msg).add_attribute("action", "sudo_kv_query_result"))
}

fn sudo_error(
    deps: DepsMut<NeutronQuery>,
    request: RequestPacket,
    details: String,
) -> StdResult<Response<NeutronMsg>> {
    deps.api
        .debug(format!("WASMDEBUG: sudo error: {}", details).as_str());
    deps.api
        .debug(format!("WASMDEBUG: request packet: {:?}", request).as_str());

    let chain_id = ICA_PORT_ID_TO_CHAIN_ID.load(deps.storage, request.source_port.unwrap())?;

    SUPPORTED_CHAINS.update(
        deps.storage,
        chain_id.clone(),
        |existing_chain| -> StdResult<_> {
            let mut chain = existing_chain.unwrap();
            chain.ica_error = Option::from(details);
            Ok(chain)
        },
    )?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    mod test_sudo_open_ack {
        use cosmwasm_std::{Addr, coins};
        use cosmwasm_std::testing::{mock_env, mock_info};
        use neutron_sdk::sudo::msg::SudoMsg;

        use crate::execute::execute;
        use crate::instantiate::instantiate;
        use crate::msg::{ExecuteMsg, InstantiateMsg};
        use crate::state::SUPPORTED_CHAINS;
        use crate::sudo::sudo;
        use crate::testing::helpers::mock_neutron_dependencies;

        #[test]
        fn test_sudo_open_ack() {
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
            let chain = SUPPORTED_CHAINS
                .load(deps.as_ref().storage, "chain_id".to_string())
                .unwrap();
            assert_eq!(chain.ica_address, None);

            let open_ack_msg = SudoMsg::OpenAck {
                port_id: chain.ica_port_id,
                channel_id: "channel_id".to_string(),
                counterparty_channel_id: "counterparty_channel_id".to_string(),
                counterparty_version: r#"{"version":"version","controller_connection_id":"controller_connection_id","host_connection_id":"host_connection_id","address":"icaaddress","encoding":"encoding","tx_type":"tx_type"}"#.to_string(),
            };
            sudo(deps.as_mut(), mock_env(), open_ack_msg).unwrap();

            let chain = SUPPORTED_CHAINS
                .load(deps.as_ref().storage, "chain_id".to_string())
                .unwrap();
            assert_eq!(chain.ica_address.unwrap(), Addr::unchecked("icaaddress"));
        }
    }
}
