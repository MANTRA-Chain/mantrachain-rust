use crate::staking::error::StakingError;
use crate::staking::native::DelegationStrategy;
use crate::staking::MIN_VALIDATORS;
use cosmwasm_std::{ensure, Addr, BlockInfo, Deps, Env, Validator};
use sha2::{Digest, Sha256};

/// Returns a list of validators based on the provided delegation strategy.
///
/// # Arguments
/// * `deps` - The dependencies for the contract.
/// * `env` - The environment for the contract.
/// * `sender` - The address of the sender.
/// * `delegation_strategy` - The strategy for selecting validators.
///
/// # Returns
/// A vector of validator addresses.
///
/// # Example
/// ```rust
/// use crate::staking::error::StakingError;
/// use crate::staking::helpers::get_validators;
/// use crate::staking::native::DelegationStrategy;
/// use cosmwasm_std::{Addr, Deps, Env};
///
/// let delegation_strategy = DelegationStrategy::Pseudorandom();
/// let validators = get_validators(deps, &env, &sender, delegation_strategy)?;
/// assert_eq!(validators.len(), 4);
/// ```
pub(crate) fn get_validators(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    delegation_strategy: DelegationStrategy,
) -> Result<Vec<String>, StakingError> {
    let active_validators = deps.querier.query_all_validators()?;

    let validators = match delegation_strategy {
        DelegationStrategy::Pseudorandom(n) => {
            let n = n.unwrap_or(MIN_VALIDATORS);
            check_validators_size(active_validators.len(), n)?;

            select_pseudorandom_validators(&env.block, &sender, n, &active_validators)?
        }
        DelegationStrategy::TopN(n) => {
            check_validators_size(active_validators.len(), n)?;

            active_validators
                .iter()
                .take(n)
                .map(|v| v.address.clone())
                .collect()
        }
        DelegationStrategy::BottomN(n) => {
            check_validators_size(active_validators.len(), n)?;

            active_validators
                .iter()
                .rev()
                .take(n)
                .map(|v| v.address.clone())
                .collect()
        }
        DelegationStrategy::Custom(validators) => {
            check_validators_size(active_validators.len(), validators.len())?;
            validators
        }
    };
    Ok(validators)
}

/// Checks if the number of validators is valid.
///
/// # Arguments
/// * `validators_n` - The number of validators to compare against.
/// * `n` - The number of validators intending to delegate to.
///
/// # Returns
/// `()`, if the number of validators is valid.
#[inline]
fn check_validators_size(validators_n: usize, n: usize) -> Result<(), StakingError> {
    if validators_n >= MIN_VALIDATORS {
        ensure!(
            n >= MIN_VALIDATORS,
            StakingError::NotEnoughValidators {
                min_validators: MIN_VALIDATORS,
                provided_validators: n
            }
        );
    }

    Ok(())
}

/// Pseudorandomly selects a specified number of validators from the active validator set.
///
/// # Arguments
/// * `block` - The block information.
/// * `sender` - The address of the sender.
/// * `num_validators` - The number of validators to select.
/// * `active_validators` - A vector of the validator set.
///
/// # Returns
/// A vector of selected validator addresses.
///
/// # Example
/// ```rust
/// use crate::staking::helpers::select_pseudorandom_validators;
/// use cosmwasm_std::{Addr, Env, Deps};
/// use crate::staking::error::StakingError;
///
/// let sender = Addr::unchecked("sender");
/// let num_validators = 4;
/// let selected_validators = select_pseudorandom_validators(deps, &env.block, &sender, num_validators)?;
/// assert_eq!(selected_validators.len(), 4);
/// }
/// ```
fn select_pseudorandom_validators(
    block: &BlockInfo,
    sender: &Addr,
    num_validators: usize,
    active_validators: &[Validator],
) -> Result<Vec<String>, StakingError> {
    let seed = format!("{}{}", block.height, sender);
    let mut hasher = Sha256::new();
    hasher.update(seed);

    let hash = hasher.finalize();
    let hash_bytes: Vec<u8> = hash.to_vec();

    let mut indices: Vec<usize> = (0..active_validators.len()).collect();
    indices.sort_by_key(|&i| {
        let start = i * 4;
        let end = start + 4;
        u32::from_le_bytes(hash_bytes[start..end].try_into().unwrap())
    });

    let take = indices.len().min(num_validators);
    let selected_indices = &indices[..take];

    let selected_validators = selected_indices
        .iter()
        .map(|&i| active_validators[i].address.clone())
        .collect();

    Ok(selected_validators)
}
