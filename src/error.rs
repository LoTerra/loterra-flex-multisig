use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Required threshold cannot be zero")]
    ZeroThreshold {},

    #[error("Not possible to reach required (passing) threshold")]
    UnreachableThreshold {},

    #[error("Group contract invalid address '{addr}'")]
    InvalidGroup { addr: String },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Proposal is not open")]
    NotOpen {},

    #[error("Proposal voting period has expired")]
    Expired {},

    #[error("Proposal must expire before you can close it")]
    NotExpired {},

    #[error("Wrong expiration option")]
    WrongExpiration {},

    #[error("Already voted on this proposal")]
    AlreadyVoted {},

    #[error("Proposal must have passed and not yet been executed")]
    WrongExecuteStatus {},

    #[error("Cannot close completed or passed proposals")]
    WrongCloseStatus {},

    #[error("Creating a proposal require {0}{1} collateral")]
    EmptyFunds(Uint128, String),

    #[error("Only UST is accepted")]
    WrongDenom {},

    #[error("Only send UST")]
    MultipleDenoms {},

    #[error("Attach 100UST to this transaction to valid your proposal")]
    AmountNotMatch {},
}
