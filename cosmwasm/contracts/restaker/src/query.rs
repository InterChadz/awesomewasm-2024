use cosmwasm_std::{Binary, Deps, entry_point, Env, StdResult};
use crate::msg::QueryMsg;

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {

    }
}