use crate::staking::error::StakingError;
use crate::staking::{helpers, MIN_VALIDATORS};
use cosmwasm_std::{
    coin, ensure, Addr, BankMsg, Coin, CosmosMsg, Decimal256, Deps, DepsMut, DistributionMsg, Env,
    MessageInfo, StakingMsg, Uint128,
};

/// Delegation strategies for selecting validators.
pub enum DelegationStrategy {
    Pseudorandom(Option<usize>),
    TopN(usize),
    BottomN(usize),
    Custom(Vec<String>),
}

/// Provides messages for delegating native tokens to validators based on the provided delegation strategy.
/// todo example docs
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

    let dust = to_delegate.amount.saturating_sub(amount_per_validator);

    let mut messages = vec![];

    if amount_per_validator > Uint128::zero() {
        for (index, validator) in validators.iter().enumerate() {
            let validator = deps.api.addr_validate(validator)?;

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
pub(crate) fn undelegate(
    deps: Deps,
    validator: &str,
    amount: Coin,
) -> Result<CosmosMsg, StakingError> {
    Ok(CosmosMsg::Staking(StakingMsg::Undelegate {
        validator: deps.api.addr_validate(validator)?.to_string(),
        amount,
    }))
}

/// Claims staking rewards from all validators.
pub(crate) fn claim_staking_rewards(
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

    for validator in delegations.clone() {
        let msg = CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward { validator });
        messages.push(msg);
    }

    if let Some(recipient) = recipient {
        for reward in rewards.total {
            let amount: Uint128 = reward.amount.atomics().try_into()?;

            let msg = CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.clone(),
                amount: vec![coin(amount.u128(), reward.denom.to_string())],
            });

            messages.push(msg);
        }
    }

    Ok(messages)
}
