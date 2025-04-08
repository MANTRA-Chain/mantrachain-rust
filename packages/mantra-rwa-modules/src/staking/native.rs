use crate::staking::error::StakingError;
use crate::staking::helpers;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, coins, ensure, Addr, Attribute, BankMsg, Coin, CosmosMsg, Decimal256, Deps,
    DistributionMsg, Env, StakingMsg, Uint128,
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
/// # Arguments
/// * `deps` - The dependencies for the contract.
/// * `env` - The environment for the contract.
/// * `sender` - The address of the sender.
/// * `to_delegate` - The amount of tokens to delegate.
/// * `delegation_strategy` - The strategy for selecting validators.
///
/// # Returns
/// A tuple with a vector of [CosmosMsg] for delegating native tokens and a vector of [Attribute].
///
/// # Example
/// ```rust
/// use crate::staking::error::StakingError;
/// use crate::staking::native::delegate;
/// use crate::staking::native::{delegate, DelegationStrategy};
/// use cosmwasm_std::{coin, Addr, Attribute, CosmosMsg, Deps, Env};
///
/// let to_delegate = coin(1000, "uom");
/// let delegation_strategy = DelegationStrategy::Pseudorandom(None);
///
/// let (delegate_messages, attributes): (Vec<CosmosMsg>, Vec<Attribute>) =
///     delegate(deps, &env, &sender, to_delegate, delegation_strategy)?;
/// assert_eq!(delegate_messages.len(), 4);
///
/// Ok(Response::default()
///     .add_messages(delegate_messages)
///     .add_attributes(attributes))
/// ```
pub fn delegate(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    to_delegate: Coin,
    delegation_strategy: DelegationStrategy,
) -> Result<(Vec<CosmosMsg>, Vec<Attribute>), StakingError> {
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
    let mut attributes = vec![Attribute::new("action", "delegate")];

    if amount_per_validator > Uint128::zero() {
        for (index, validator) in validators.iter().enumerate() {
            let amount = if index == validators.len() - 1 {
                amount_per_validator.checked_add(dust)?
            } else {
                amount_per_validator
            };

            let delegation = coin(amount.u128(), bonded_denom.clone());

            let msg = CosmosMsg::Staking(StakingMsg::Delegate {
                validator: validator.to_string(),
                amount: coin(amount.u128(), bonded_denom.clone()),
            });

            messages.push(msg);
            attributes.push(Attribute::new(
                "delegation",
                &format!("validator: {:?} -> {:?}", validator, delegation),
            ));
        }
    }

    Ok((messages, attributes))
}

/// Provides messages for undelegating native tokens from a validator.
///
/// # Arguments
/// * `validator` - The address of the validator.
/// * `amount` - The amount of tokens to undelegate.
///
/// # Returns
/// A tuple with a [CosmosMsg] for undelegating native tokens and a vector of [Attribute].
///
/// # Example
/// ```rust
/// use crate::staking::native::undelegate;
/// use cosmwasm_std::{coin, Attribute, CosmosMsg};
///
/// let (undelegate_message, attributes): (Vec<CosmosMsg>, Vec<Attribute>) =
///     undelegate(deps, "validator_address", coin(1000, "uom"))?;
/// assert_eq!(
///     undelegate_message,
///     CosmosMsg::Staking(StakingMsg::Undelegate {
///         validator: "validator_address".to_string(),
///         amount: coin(1000, "uom"),
///     })
/// );
///
/// Ok(Response::default()
///     .add_message(undelegate_message)
///     .add_attributes(attributes))
/// ```
pub fn undelegate(
    validator: &str,
    amount: Coin,
) -> Result<(CosmosMsg, Vec<Attribute>), StakingError> {
    let attributes = vec![
        Attribute::new("action", "undelegate"),
        Attribute::new("validator", validator),
        Attribute::new("amount", &amount.to_string()),
    ];

    Ok((
        CosmosMsg::Staking(StakingMsg::Undelegate {
            validator: validator.to_string(),
            amount,
        }),
        attributes,
    ))
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
/// A tuple with a vector of [CosmosMsg] for claiming staking rewards and a vector of [Attribute].
///
/// # Example
/// ```rust
/// use crate::staking::native::claim_staking_rewards;
/// use cosmwasm_std::{coin, Attribute, CosmosMsg};
///
/// let (claim_messages, attributes): (Vec<CosmosMsg>, Vec<Attribute>) =
///     claim_staking_rewards(deps, "delegator_address", Some("recipient_address"))?;
/// assert_eq!(claim_messages.len(), 2);
///
/// Ok(Response::default()
///     .add_messages(claim_messages)
///     .add_attributes(attributes))
/// ```
pub fn claim_staking_rewards(
    deps: Deps,
    delegator: &str,
    recipient: Option<String>,
) -> Result<(Vec<CosmosMsg>, Vec<Attribute>), StakingError> {
    let total_rewards = deps.querier.query_delegation_total_rewards(delegator)?;

    ensure!(
        !total_rewards.total.is_empty() && total_rewards.total[0].amount > Decimal256::zero(),
        StakingError::NothingToClaim
    );

    let delegations: Vec<String> = deps
        .querier
        .query_all_delegations(delegator)?
        .iter()
        .map(|d| d.validator.clone())
        .collect();

    let mut messages = vec![];

    ensure!(
        total_rewards.rewards.len() == delegations.len(),
        StakingError::NothingToClaim
    );

    for validator in delegations.clone() {
        let msg = CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward { validator });
        messages.push(msg);
    }

    let mut attributes = vec![Attribute::new("action", "claim_staking_rewards")];

    let bonded_denom = deps.querier.query_bonded_denom()?;
    let mut total_amount = Uint128::zero();
    // do not use `total_rewards.total` but the individual rewards instead, as it can provide a
    // wrong value when converting to Uint when the sum of the individual rewards decimals round up.
    for r in total_rewards.rewards {
        attributes.push(Attribute::new("validator", &r.validator_address));
        for reward in r.reward {
            let amount: Uint128 = reward.amount.to_uint_floor().try_into()?;
            total_amount = total_amount.checked_add(amount)?;
            attributes.push(Attribute::new(
                "reward",
                &format!("{}{}", amount, bonded_denom),
            ));
        }
    }

    attributes.push(Attribute::new(
        "total_reward",
        &format!("{}{}", total_amount, bonded_denom),
    ));

    if let Some(recipient) = recipient {
        if total_amount > Uint128::zero() {
            attributes.push(Attribute::new("recipient", &recipient));

            let msg = CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.clone(),
                amount: coins(total_amount.u128(), bonded_denom),
            });
            messages.push(msg);
        }
    }

    Ok((messages, attributes))
}
