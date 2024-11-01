use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not registered in SUI registry")]
    NotRegistered {},

    #[error("Proposal is not active")]
    ProposalNotActive {},

    #[error("Already voted on this proposal")]
    AlreadyVoted {},

    #[error("Axelar verification failed")]
    AxelarVerificationFailed {},
}