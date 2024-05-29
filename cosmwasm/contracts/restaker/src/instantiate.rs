use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use neutron_sdk::bindings::query::NeutronQuery;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{Config, CONFIG, NEXT_REPLY_ID};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    CONFIG
        .save(
            deps.storage,
            &Config {
                admin,
                neutron_register_ica_fee: msg.neutron_register_ica_fee,
            },
        )
        .unwrap();

    NEXT_REPLY_ID.save(deps.storage, &1).unwrap();

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_env, mock_info};

    use crate::instantiate::instantiate;
    use crate::msg::InstantiateMsg;
    use crate::state::{CONFIG, NEXT_REPLY_ID};
    use crate::testing::helpers::mock_neutron_dependencies;

    #[test]
    fn test_initialization() {
        let mut deps = mock_neutron_dependencies();

        let info = mock_info("creator", &coins(10000, "untrn"));

        let msg = InstantiateMsg {
            admin: "admin".to_string(),
            neutron_register_ica_fee: 1000000,
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        let config = CONFIG.load(deps.as_ref().storage).unwrap();
        assert_eq!(config.admin, msg.admin);
        assert_eq!(
            config.neutron_register_ica_fee,
            msg.neutron_register_ica_fee
        );

        let next_reply_id = NEXT_REPLY_ID.load(deps.as_ref().storage).unwrap();
        assert_eq!(next_reply_id, 1);
    }
}
