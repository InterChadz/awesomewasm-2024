use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}
