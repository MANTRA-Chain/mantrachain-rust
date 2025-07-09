use cosmwasm_std::{ConversionOverflowError, Decimal, OverflowError, StdError};
use cw_migrate_error_derive::cw_migrate_invalid_version_error;
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use thiserror::Error;

#[cw_migrate_invalid_version_error]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("{0}")]
    OwnershipError(#[from] OwnershipError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("Invalid distribution percentage, expected: {expected}, actual: {actual}")]
    InvalidDistributionPercentage { expected: Decimal, actual: Decimal },

    #[error("Invalid distribution percentage, cannot be zero")]
    ZeroDistributionPercentage,

    #[error("Invalid campaign parameter: {param} - {reason}")]
    InvalidCampaignParam { param: String, reason: String },

    #[error("Claim amount exceeds the maximum claimable amount")]
    ExceededMaxClaimAmount,

    #[error("Campaign error: {reason}")]
    CampaignError { reason: String },

    #[error("Invalid distribution times, start time: {start_time}, end time: {end_time}")]
    InvalidDistributionTimes { start_time: u64, end_time: u64 },

    #[error("Invalid start distribution time, start time: {start_time}, campaign start time: {campaign_start_time}. The start time needs to be in the future.")]
    InvalidStartDistributionTime {
        start_time: u64,
        campaign_start_time: u64,
    },

    #[error("Invalid end distribution time, end time: {end_time}, campaign_end_time: {campaign_end_time}. The distribution end time needs to be less or equal to the campaign's end time.")]
    InvalidEndDistributionTime {
        end_time: u64,
        campaign_end_time: u64,
    },

    #[error("There's nothing to claim for the given address")]
    NothingToClaim,

    #[error("No allocation found for address: {address}")]
    NoAllocationFound { address: String },

    #[error("The current address already has an allocation: {address}")]
    AllocationAlreadyExists { address: String },

    #[error("Address is blacklisted")]
    AddressBlacklisted,

    #[error("Invalid claim amount: {reason}")]
    InvalidClaimAmount { reason: String },

    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },

    #[error("Batch size limit exceeded: {actual}, maximum allowed: {max}")]
    BatchSizeLimitExceeded { actual: usize, max: usize },
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
