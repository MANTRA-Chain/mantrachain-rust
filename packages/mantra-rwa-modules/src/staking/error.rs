use cosmwasm_std::{
    CheckedMultiplyFractionError, ConversionOverflowError, OverflowError, StdError,
};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum StakingError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    CheckedMultiplyFractionError(#[from] CheckedMultiplyFractionError),

    #[error(transparent)]
    OverflowError(#[from] OverflowError),

    #[error(transparent)]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("Not enough validators provided. Minimum required: {min_validators}, provided: {provided_validators}")]
    NotEnoughValidators {
        min_validators: usize,
        provided_validators: usize,
    },

    #[error("There are no staking rewards to claim")]
    NothingToClaim,

    #[error("The amount to delegate must be greater than zero")]
    ZeroAmount,

    #[error("The provided denom is not the expected bonded denom. Expected: {expected}, Provided: {provided}")]
    InvalidDenom { expected: String, provided: String },
}
