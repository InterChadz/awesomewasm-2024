use cosmwasm_std::StdError;
use cw0::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("chain already exists")]
    ChainAlreadyExists {},

    #[error("chain not found")]
    ChainNotFound {},

    #[error("not enough funds, required amount is {required_amount}untrn, actual amount is {actual_amount}untrn")]
    NotEnoughFunds {
        required_amount: u128,
        actual_amount: u128,
    },

    #[error("chain {chain_id} already registered for user with local {address} using remote address {remote_address}")]
    ChainAlreadyRegisteredForUser {
        chain_id: String,
        address: String,
        remote_address: String,
    },

    #[error("No rewards to autocompound")]
    NoRewardsToAutocompound {},
}
