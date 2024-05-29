use crate::state::{
    user_chain_registrations, UserChainRegistration, REPLY_ID_TO_USER_CHAIN_REGISTRATION,
};
use cosmwasm_std::{entry_point, DepsMut, Env, Reply, Response, StdError, StdResult};
use neutron_sdk::bindings::msg::MsgRegisterInterchainQueryResponse;

#[entry_point]
pub fn reply(deps: DepsMut, _: Env, msg: Reply) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", msg).as_str());

    let resp: MsgRegisterInterchainQueryResponse = serde_json_wasm::from_slice(
        msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .data
            .ok_or_else(|| StdError::generic_err("no result"))?
            .as_slice(),
    )
    .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;

    let reply_id_to_reg = REPLY_ID_TO_USER_CHAIN_REGISTRATION
        .may_load(deps.storage, msg.id)
        .unwrap();
    if reply_id_to_reg.is_some() {
        user_chain_registrations().update(
            deps.storage,
            reply_id_to_reg.unwrap(),
            |reg_opt| -> Result<UserChainRegistration, StdError> {
                let mut reg = reg_opt.unwrap();
                reg.delegator_delegations_icq_id = Some(resp.id);
                Ok(reg)
            },
        )?;
        return Ok(Response::default());
    }


    // If not found by now, we error out
    Err(StdError::generic_err(format!(
        "unsupported reply message id {}",
        msg.id,
    )))
}
