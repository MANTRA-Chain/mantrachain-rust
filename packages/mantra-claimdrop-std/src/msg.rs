use std::fmt::{Display, Formatter};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{ensure, Coin, Decimal, Timestamp, Uint128};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

use crate::error::ContractError;

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner of the contract. If not set, it is the sender of the Instantiate message.
    pub owner: Option<String>,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Manages campaigns based on the action, defined by [CampaignAction].
    ManageCampaign { action: CampaignAction },
    /// Claims rewards from a campaign
    Claim {
        /// The receiver address of the claimed rewards. If not set, the sender of the message will be the receiver.
        /// This is useful for allowing a contract to do the claim operation on behalf of a user.
        receiver: Option<String>,
        /// The amount to claim. If not set, all available tokens will be claimed.
        amount: Option<Uint128>,
    },
    /// Adds a batch of addresses and their allocations. This can only be done before the campaign has started.
    AddAllocations {
        /// Vector of (address, amount) pairs
        allocations: Vec<(String, Uint128)>,
    },
    /// Replaces an address in the allocation list. This can only be done before the campaign has started.
    ReplaceAddress {
        /// The old address to replace
        old_address: String,
        /// The new address to use
        new_address: String,
    },
    /// Removes an address in the allocation list. This can only be done before the campaign has started.
    RemoveAddress {
        /// The address to remove
        address: String,
    },
    /// Blacklists or unblacklists an address. This can be done at any time.
    BlacklistAddress {
        /// The address to blacklist/unblacklist
        address: String,
        /// Whether to blacklist or unblacklist
        blacklist: bool,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CampaignResponse)]
    /// Get the airdrop campaign
    Campaign {},
    #[returns(RewardsResponse)]
    /// Get the rewards for a specific campaign and receiver address.
    Rewards {
        /// The address to get the rewards for.
        receiver: String,
    },
    #[returns(ClaimedResponse)]
    /// Get the total amount of tokens claimed on the campaign.
    Claimed {
        /// If provided, it will return the tokens claimed by the specified address.
        address: Option<String>,
        /// The address to start querying from. Used for paginating results.
        start_from: Option<String>,
        /// The maximum number of items to return. If not set, the default value is used. Used for paginating results.
        limit: Option<u16>,
    },
    #[returns(AllocationsResponse)]
    /// Get the allocation for an address
    Allocations {
        /// The address to get the allocation for, if provided
        address: Option<String>,
        /// The address to start querying from. Used for paginating results.
        start_after: Option<String>,
        /// The maximum number of items to return. If not set, the default value is used. Used for paginating results.
        limit: Option<u16>,
    },
    #[returns(BlacklistResponse)]
    /// Check if an address is blacklisted
    IsBlacklisted {
        /// The address to check
        address: String,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

pub type CampaignResponse = Campaign;

/// Response to the Rewards query.
#[cw_serde]
pub struct RewardsResponse {
    /// The tokens that have been claimed by the address.
    pub claimed: Vec<Coin>,
    /// The total amount of tokens that is pending to be claimed by the address.
    pub pending: Vec<Coin>,
    /// The tokens that are available to be claimed by the address.
    pub available_to_claim: Vec<Coin>,
}

/// Response to the Claimed query.
#[cw_serde]
pub struct ClaimedResponse {
    /// Contains a vector with a tuple with (address, coin) that have been claimed
    pub claimed: Vec<(String, Coin)>,
}

/// Response to the Allocation query.
#[cw_serde]
pub struct AllocationsResponse {
    /// A vector with a tuple with (address, amount) that have been allocated.
    pub allocations: Vec<(String, Uint128)>,
}

/// Response to the Blacklist query.
#[cw_serde]
pub struct BlacklistResponse {
    /// Whether the address is blacklisted
    pub is_blacklisted: bool,
}

/// The campaign action that can be executed with the [ExecuteMsg::ManageCampaign] message.
#[cw_serde]
pub enum CampaignAction {
    /// Creates a new campaign
    CreateCampaign {
        /// The parameters to create a campaign with
        params: Box<CampaignParams>,
    },
    /// Closes the campaign
    CloseCampaign {},
}

/// Represents a campaign.
#[cw_serde]
pub struct Campaign {
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    /// Campaign type. Value used by front ends.
    #[serde(rename = "type")]
    pub ty: String,
    /// The denom to be distributed as reward by the campaign
    pub reward_denom: String,
    /// The total amount of the reward asset that is intended to be allocated to the campaign
    pub total_reward: Coin,
    /// The amount of the reward asset that has been claimed
    pub claimed: Coin,
    /// The ways the reward is distributed, which are defined by the [DistributionType].
    /// The sum of the percentages must be 100.
    pub distribution_type: Vec<DistributionType>,
    /// The campaign start time (unix timestamp), in seconds
    pub start_time: u64,
    /// The campaign end time (unix timestamp), in seconds
    pub end_time: u64,
    /// The timestamp at which the campaign was closed, in seconds
    pub closed: Option<u64>,
}

impl Display for Campaign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Campaign {{ name: {}, description: {}, type: {}, reward_denom: {}, total_reward: {}, claimed: {}, distribution_type: {:?}, start_time: {}, end_time: {}, closed: {:?} }}",
            self.name,
            self.description,
            self.ty,
            self.reward_denom,
            self.total_reward,
            self.claimed,
            self.distribution_type,
            self.start_time,
            self.end_time,
            self.closed
        )
    }
}

impl Campaign {
    /// Creates a new campaign from the given parameters
    pub fn from_params(params: CampaignParams) -> Self {
        let reward_denom = params.reward_denom.clone();

        Campaign {
            name: params.name,
            description: params.description,
            ty: params.ty,
            reward_denom: params.reward_denom,
            total_reward: params.total_reward,
            claimed: Coin {
                denom: reward_denom,
                amount: Uint128::zero(),
            },
            distribution_type: params.distribution_type,
            start_time: params.start_time,
            end_time: params.end_time,
            closed: None,
        }
    }

    /// Checks if the campaign has started
    pub fn has_started(&self, current_time: &Timestamp) -> bool {
        current_time.seconds() >= self.start_time
    }

    /// Checks if the campaign has ended
    pub fn has_ended(&self, current_time: &Timestamp) -> bool {
        current_time.seconds() >= self.end_time
    }
}

/// Represents the parameters to create a campaign with.
#[cw_serde]
pub struct CampaignParams {
    /// The campaign name
    pub name: String,
    /// The campaign description
    pub description: String,
    /// Campaign type. Value used by front ends.
    #[serde(rename = "type")]
    pub ty: String,
    /// The denom to be distributed as reward by the campaign
    pub reward_denom: String,
    /// The total amount of the reward asset that is intended to be allocated to the campaign
    pub total_reward: Coin,
    /// The ways the reward is distributed, which are defined by the [DistributionType].
    /// The sum of the percentages must be 100.
    pub distribution_type: Vec<DistributionType>,
    /// The campaign start time (unix timestamp), in seconds
    pub start_time: u64,
    /// The campaign end timestamp (unix timestamp), in seconds
    pub end_time: u64,
}

impl CampaignParams {
    /// Validates the campaign name and description
    pub fn validate_campaign_name_description(&self) -> Result<(), ContractError> {
        ensure!(
            !self.name.is_empty(),
            ContractError::InvalidCampaignParam {
                param: "name".to_string(),
                reason: "cannot be empty".to_string(),
            }
        );

        ensure!(
            self.name.len() <= 200usize,
            ContractError::InvalidCampaignParam {
                param: "name".to_string(),
                reason: "cannot be longer than 200 characters".to_string(),
            }
        );

        ensure!(
            !self.description.is_empty(),
            ContractError::InvalidCampaignParam {
                param: "description".to_string(),
                reason: "cannot be empty".to_string(),
            }
        );

        ensure!(
            self.description.len() <= 2000usize,
            ContractError::InvalidCampaignParam {
                param: "description".to_string(),
                reason: "cannot be longer than 2000 characters".to_string(),
            }
        );

        Ok(())
    }

    /// Validates the campaign type
    pub fn validate_campaign_type(&self) -> Result<(), ContractError> {
        ensure!(
            !self.ty.is_empty(),
            ContractError::InvalidCampaignParam {
                param: "type".to_string(),
                reason: "cannot be empty".to_string(),
            }
        );

        ensure!(
            self.ty.len() <= 200usize,
            ContractError::InvalidCampaignParam {
                param: "type".to_string(),
                reason: "cannot be longer than 200 characters".to_string(),
            }
        );

        Ok(())
    }

    /// Validates the start and end times of a campaign
    pub fn validate_campaign_times(&self, current_time: Timestamp) -> Result<(), ContractError> {
        ensure!(
            self.start_time < self.end_time,
            ContractError::InvalidCampaignParam {
                param: "start_time".to_string(),
                reason: "cannot be greater or equal than end_time".to_string(),
            }
        );
        ensure!(
            self.start_time >= current_time.seconds(),
            ContractError::InvalidCampaignParam {
                param: "start_time".to_string(),
                reason: "cannot be less than the current time".to_string(),
            }
        );

        Ok(())
    }

    /// Ensures the distribution type parameters are correct
    pub fn validate_campaign_distribution(&self) -> Result<(), ContractError> {
        let mut total_percentage = Decimal::zero();

        ensure!(
            !self.distribution_type.is_empty() && self.distribution_type.len() <= 2,
            ContractError::InvalidCampaignParam {
                param: "distribution_type".to_string(),
                reason: "invalid number of distribution types, should be at least 1, maximum 2"
                    .to_string(),
            }
        );

        for dist in self.distribution_type.iter() {
            let (percentage, start_time, end_time, cliff_duration) = match dist {
                DistributionType::LinearVesting {
                    percentage,
                    start_time,
                    end_time,
                    cliff_duration,
                } => (percentage, start_time, Some(end_time), cliff_duration),
                DistributionType::LumpSum {
                    percentage,
                    start_time,
                } => (percentage, start_time, None, &None),
            };

            ensure!(
                percentage != Decimal::zero(),
                ContractError::ZeroDistributionPercentage
            );

            total_percentage = total_percentage.checked_add(*percentage)?;

            ensure!(
                *start_time >= self.start_time,
                ContractError::InvalidStartDistributionTime {
                    start_time: *start_time,
                    campaign_start_time: self.start_time,
                }
            );

            // validate the end time. Applies for the linear vesting distribution type only
            if let Some(end_time) = end_time {
                ensure!(
                    end_time > start_time,
                    ContractError::InvalidDistributionTimes {
                        start_time: *start_time,
                        end_time: *end_time,
                    }
                );

                ensure!(
                    *end_time <= self.end_time,
                    ContractError::InvalidEndDistributionTime {
                        end_time: *end_time,
                        campaign_end_time: self.end_time,
                    }
                );
            }

            // validate the cliff duration
            if let Some(cliff_duration) = cliff_duration {
                ensure!(
                    *cliff_duration > 0u64,
                    ContractError::InvalidCampaignParam {
                        param: "cliff_duration".to_string(),
                        reason: "cannot be zero".to_string(),
                    }
                );

                ensure!(
                    // it is safe to unwrap because this cliff validation only applies for linear vesting,
                    // which contains an end_time
                    *cliff_duration < end_time.unwrap() - start_time,
                    ContractError::InvalidCampaignParam {
                        param: "cliff_duration".to_string(),
                        reason: "cannot be greater or equal than the distribution duration"
                            .to_string(),
                    }
                );
            }
        }

        ensure!(
            total_percentage == Decimal::percent(100),
            ContractError::InvalidDistributionPercentage {
                expected: Decimal::percent(100),
                actual: total_percentage,
            }
        );

        Ok(())
    }

    /// Validates the total reward amount and denom
    pub fn validate_rewards(&self) -> Result<(), ContractError> {
        ensure!(
            self.total_reward.amount > Uint128::zero(),
            ContractError::InvalidCampaignParam {
                param: "total_reward".to_string(),
                reason: "cannot be zero".to_string()
            }
        );

        ensure!(
            self.total_reward.denom == self.reward_denom,
            ContractError::InvalidCampaignParam {
                param: "reward_denom".to_string(),
                reason: "reward denom mismatch".to_string()
            }
        );

        Ok(())
    }
}

#[cw_serde]
pub enum DistributionType {
    /// The distribution is done in a linear vesting schedule
    LinearVesting {
        /// The percentage of the total reward to be distributed with a linear vesting schedule
        percentage: Decimal,
        /// The unix timestamp when this distribution type starts, in seconds
        start_time: u64,
        /// The unix timestamp when this distribution type ends, in seconds
        end_time: u64,
        /// The duration of the cliff, in seconds
        cliff_duration: Option<u64>,
    },
    /// The distribution is done in a single lump sum, i.e. no vesting period
    LumpSum {
        percentage: Decimal,
        /// The unix timestamp when this distribution type starts, in seconds
        start_time: u64,
    },
}

impl DistributionType {
    pub fn has_started(&self, current_time: &Timestamp) -> bool {
        let start_time = match self {
            DistributionType::LinearVesting { start_time, .. } => start_time,
            DistributionType::LumpSum { start_time, .. } => start_time,
        };

        current_time.seconds() >= *start_time
    }
}
