use cosmwasm_std::{
    DepsMut, entry_point, Env, MessageInfo, Response,
};

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{CONFIG, Config};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    CONFIG.save(deps.storage, &Config {
        admin,
        neutron_register_ica_fee: msg.neutron_register_ica_fee,
    }).unwrap();

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::instantiate::instantiate;
    use crate::msg::InstantiateMsg;
    use crate::state::CONFIG;

    #[test]
    fn test_initialization() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(10000, "untrn"));

        let msg = InstantiateMsg {
            admin: "admin".to_string(),
            neutron_register_ica_fee: 1000000,
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.admin, msg.admin);
        assert_eq!(config.neutron_register_ica_fee, msg.neutron_register_ica_fee);
    }
}