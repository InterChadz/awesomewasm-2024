use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{entry_point, DepsMut, Empty, Env, MessageInfo, Response};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg(test)]
mod tests {}
