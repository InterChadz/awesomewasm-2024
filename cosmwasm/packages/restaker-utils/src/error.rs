use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ArkiteError {
    #[error("{0}")]
    Std(#[from] StdError),
}
