use cosmos_sdk_proto::cosmos::{base::v1beta1::Coin, staking::v1beta1::MsgDelegate};
use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, StdError, Storage, SubMsg};
use neutron_sdk::bindings::{
    msg::{IbcFee, NeutronMsg},
    query::NeutronQuery,
    types::ProtobufAny,
};
use serde_json_wasm::to_vec;

use crate::{error::ContractError, state::REPLY_ID_STORAGE, sudo::SudoPayload};

const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2; // 2 weeks TODO: this is a lot, how much? Or we just deprecate this and we always pass it from above.

pub fn get_delegate_submsg(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    interchain_account_id: String,
    connection_id: String,
    port_id: String,
    delegator: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> Result<SubMsg<NeutronMsg>, ContractError> {
    // Get the delegator address from the storage & form the Delegate message.

    let delegate_msg = MsgDelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(Coin {
            denom,
            amount: amount.to_string(),
        }),
    };

    // Serialize the Delegate message.
    let mut buf = Vec::new();
    buf.reserve(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(ContractError::Std(StdError::generic_err(format!(
            "Error encoding Delegate message: {}",
            e
        ))));
    }

    // Put the serialized Delegate message to a types.Any protobuf message.
    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(buf),
    };

    // // specify fees to refund relayers for submission of ack and timeout messages
    // //
    // // The contract MUST HAVE recv_fee + ack_fee + timeout_fee coins on its balance!
    // // See more info about fees here: https://docs.neutron.org/neutron/modules/interchain-txs/messages#msgsubmittx
    // // and here: https://docs.neutron.org/neutron/modules/feerefunder/overview
    let fee = IbcFee {
        recv_fee: vec![],    // must be empty
        ack_fee: vec![],     // ack_fee: vec![CosmosCoin::new(100000u128, "untrn")],
        timeout_fee: vec![], // timeout_fee: vec![CosmosCoin::new(100000u128, "untrn")],
    };

    // Form the neutron SubmitTx message containing the binary Delegate message.
    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![any_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    // We use a submessage here because we need the process message reply to save
    // the outgoing IBC packet identifier for later.
    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id,
            // Here you can store some information about the transaction to help you parse
            // the acknowledgement later.
            message: "interchain_delegate".to_string(),
        },
    )?;

    Ok(submsg)
    // Ok(Response::default().add_submessages(vec![submsg]))
}

fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    mut deps: DepsMut<NeutronQuery>,
    msg: C,
    payload: SudoPayload,
) -> Result<SubMsg<T>, ContractError> {
    save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
}

pub fn save_reply_payload(
    store: &mut dyn Storage,
    payload: SudoPayload,
) -> Result<(), ContractError> {
    REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
}
