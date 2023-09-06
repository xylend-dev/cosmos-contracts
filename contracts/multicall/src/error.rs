use cosmwasm_std::StdError;
use ibc_tracking::IbcTrackingError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    IbcTrackingError(#[from] IbcTrackingError),

    #[error("Failed to serialize call into CosmosMsg")]
    SerializationError {},

    #[error("Failed to serialize call into proto msg")]
    ProtoSerializationError {},

    #[error("Invalid reply id")]
    InvalidReplyId {},

    #[error("Invalid memo, serialization failed")]
    InvalidMemo {},

    #[error("Contract locked: {msg}")]
    ContractLocked { msg: String },

    #[error("Calls list is empty")]
    EmptyCallsList {},

    #[error("Fallback address must be set for Ibc tracking")]
    FallbackAddressMustBeSetForIbcTracking {},

    #[error("Invalid call action argument: {msg}")]
    InvalidCallActionArgument { msg: String },

    #[error("Can be called only by contract itself")]
    CanBeCalledOnlyByContractItself {},

    #[error("Fetched token balance is zero. Token: {token}")]
    ZeroBalanceFetched { token: String },

    #[error("Replacer field not found. Replacer: {replacer}")]
    ReplacerFieldNotFound { replacer: String },
}
