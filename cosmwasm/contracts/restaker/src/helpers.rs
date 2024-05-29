use cosmos_sdk_proto::cosmos::{base::v1beta1::Coin, staking::v1beta1::MsgDelegate};
use cosmos_sdk_proto::traits::Message;
use cosmwasm_std::{Binary, StdError};
use neutron_sdk::bindings::{
    msg::{IbcFee, NeutronMsg},
    types::ProtobufAny,
};

use crate::error::ContractError;

const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2; // 2 weeks TODO: this is a lot, how much? Or we just deprecate this and we always pass it from above.

pub fn get_delegate_submsg(
    interchain_account_id: String,
    connection_id: String,
    delegator: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> Result<NeutronMsg, ContractError> {
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
    let delegate_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(buf),
    };

    // specify fees to refund relayers for submission of ack and timeout messages
    //
    // The contract MUST HAVE recv_fee + ack_fee + timeout_fee coins on its balance!
    // See more info about fees here: https://docs.neutron.org/neutron/modules/interchain-txs/messages#msgsubmittx
    // and here: https://docs.neutron.org/neutron/modules/feerefunder/overview
    // TODO_NICE: Relayers should be paid as the Keeper network peers!
    let fee = IbcFee {
        recv_fee: vec![],    // must be empty
        ack_fee: vec![],     // ack_fee: vec![CosmosCoin::new(100000u128, "untrn")],
        timeout_fee: vec![], // timeout_fee: vec![CosmosCoin::new(100000u128, "untrn")],
    };

    // Form the neutron SubmitTx message containing the binary Delegate message.
    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![delegate_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    Ok(cosmos_msg)
}
