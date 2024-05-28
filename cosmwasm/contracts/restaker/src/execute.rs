use cosmwasm_std::{DepsMut, entry_point, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {

    }
}