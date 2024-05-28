use crate::msg::QueryMsg;
use cosmwasm_std::{entry_point, Binary, Deps, Env, StdResult};

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}
