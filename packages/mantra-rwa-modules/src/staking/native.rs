use crate::staking::error::StakingError;
use crate::staking::{helpers, MIN_VALIDATORS};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, ensure, Addr, BankMsg, Coin, CosmosMsg, Decimal256, Deps, DistributionMsg, Env,
    StakingMsg, Uint128, Uint256,
};
use std::fmt::Display;

/// Delegation strategies for selecting validators.
#[cw_serde]
pub enum DelegationStrategy {
    /// Selects n validators pseudorandomly.
    Pseudorandom(Option<usize>),
    /// Selects the top n validators.
    TopN(usize),
    /// Selects the bottom n validators.
    BottomN(usize),
    /// Allows custom selection of validators.
    Custom(Vec<String>),
}

impl Display for DelegationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DelegationStrategy::Pseudorandom(n) => write!(f, "pseudorandom({:?})", n),
            DelegationStrategy::TopN(n) => write!(f, "top_n({})", n),
            DelegationStrategy::BottomN(n) => write!(f, "bottom_n({})", n),
            DelegationStrategy::Custom(validators) => write!(f, "custom({:?})", validators),
        }
    }
}

/// Provides cosmos messages for delegating native tokens to validators based on the provided
/// delegation strategy.
///
///
/// # Arguments
///
/// * `deps` - The dependencies for the contract.
/// * `env` - The environment for the contract.
/// * `sender` - The address of the sender.
/// * `to_delegate` - The amount of tokens to delegate.
/// * `delegation_strategy` - The strategy for selecting validators.
///
/// # Returns
///
/// A vector of cosmos messages for delegating native tokens.
///
/// # Example
/// ```rust
/// use crate::staking::native::delegate;
/// use cosmwasm_std::{coin, Addr, CosmosMsg, Deps, Env};
/// use crate::staking::native::{DelegationStrategy, delegate};
/// use crate::staking::error::StakingError;
///
/// let to_delegate = coin(1000, "uom");
/// let delegation_strategy = DelegationStrategy::Pseudorandom(None);
///
/// let delegate_messages: Vec<CosmosMsg> = delegate(deps, &env, &sender, to_delegate, delegation_strategy)?;
/// assert_eq!(delegate_messages.len(), 4);
///
/// Ok(Response::default().add_messages(delegate_messages))
/// ```
///
pub fn delegate(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    to_delegate: Coin,
    delegation_strategy: DelegationStrategy,
) -> Result<Vec<CosmosMsg>, StakingError> {
    ensure!(
        to_delegate.amount > Uint128::zero(),
        StakingError::ZeroAmount
    );

    let bonded_denom = deps.querier.query_bonded_denom()?;
    ensure!(
        to_delegate.denom == bonded_denom,
        StakingError::InvalidDenom {
            expected: bonded_denom,
            provided: to_delegate.denom,
        }
    );

    let validators = helpers::get_validators(deps, env, sender, delegation_strategy)?;

    let amount_per_validator = to_delegate
        .amount
        .checked_div_floor((validators.len() as u128, 1u128))?;

    let dust = to_delegate
        .amount
        .saturating_sub(amount_per_validator.checked_mul(Uint128::from(validators.len() as u128))?);

    let mut messages = vec![];

    if amount_per_validator > Uint128::zero() {
        for (index, validator) in validators.iter().enumerate() {
            let amount = if index == validators.len() - 1 {
                amount_per_validator.checked_add(dust)?
            } else {
                amount_per_validator
            };

            let msg = CosmosMsg::Staking(StakingMsg::Delegate {
                validator: validator.to_string(),
                amount: coin(amount.u128(), bonded_denom.clone()),
            });

            messages.push(msg);
        }
    }

    Ok(messages)
}

/// Provides messages for undelegating native tokens from a validator.
///
/// # Arguments
///
/// * `deps` - The dependencies for the contract.
/// * `validator` - The address of the validator.
/// * `amount` - The amount of tokens to undelegate.
///
/// # Returns
///
/// A cosmos message for undelegating native tokens.
///
/// # Example
/// ```rust
/// use crate::staking::native::undelegate;
/// use cosmwasm_std::{coin, CosmosMsg};
///
/// let undelegate_message: CosmosMsg = undelegate(deps, "validator_address", coin(1000, "uom"))?;
/// assert_eq!(
///     undelegate_message,
///     CosmosMsg::Staking(StakingMsg::Undelegate {
///         validator: "validator_address".to_string(),
///         amount: coin(1000, "uom"),
///     })
/// );
/// ```
pub fn undelegate(validator: &str, amount: Coin) -> Result<CosmosMsg, StakingError> {
    Ok(CosmosMsg::Staking(StakingMsg::Undelegate {
        validator: validator.to_string(),
        amount,
    }))
}

/// Claims staking rewards from all validators.
///
/// # Arguments
/// * `deps` - The dependencies for the contract.
/// * `delegator` - The address of the delegator.
/// * `recipient` - The address of the recipient, where the rewards will be sent to. If `None`,
/// the rewards won't be sent anywhere, only claimed.
///
/// # Returns
///
/// A vector of cosmos messages for claiming staking rewards.
///
/// # Example
///
/// ```rust
/// use crate::staking::native::claim_staking_rewards;
/// use cosmwasm_std::{coin, CosmosMsg};
///
/// let claim_messages: Vec<CosmosMsg> = claim_staking_rewards(deps, "delegator_address", Some("recipient_address"))?;
/// assert_eq!(claim_messages.len(), 2);
///
/// Ok(Response::default().add_messages(claim_messages))
/// ```
pub fn claim_staking_rewards(
    deps: Deps,
    delegator: &str,
    recipient: Option<String>,
) -> Result<Vec<CosmosMsg>, StakingError> {
    let rewards = deps.querier.query_delegation_total_rewards(delegator)?;

    ensure!(
        !rewards.total.is_empty() && rewards.total[0].amount > Decimal256::zero(),
        StakingError::NothingToClaim
    );

    let delegations: Vec<String> = deps
        .querier
        .query_all_delegations(delegator)?
        .iter()
        .map(|d| d.validator.clone())
        .collect();

    let mut messages = vec![];

    println!(">>> rewards = {:?}", rewards);
    println!(">>> delegations = {:?}", delegations);

    ensure!(
        rewards.rewards.len() == delegations.len(),
        StakingError::NothingToClaim
    );

    for validator in delegations.clone() {
        let msg = CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward { validator });
        messages.push(msg);
    }

    if let Some(recipient) = recipient {
        for reward in rewards.total {
            //todo testing this on chain, we can see that sometimes the value being sent
            // to the recipient is 1uom above the actual value, which makes the tx fail.
            // doesn't happen always, investigate further when it happens and how to mitigate it
            // ideally, we can do this without a callback to get the total amount of rewards claimed.
            // perhaps see how this is being handled (Decimal256) at the chain level.
            let amount: Uint128 = reward.amount.to_uint_floor().try_into()?;

            if amount == Uint128::zero() {
                continue;
            }

            let msg = CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.clone(),
                amount: vec![coin(amount.u128(), reward.denom.to_string())],
            });

            messages.push(msg);
        }
    }

    Ok(messages)
}
