use crate::staking::error::StakingError;
use crate::staking::native::DelegationStrategy;
use crate::staking::MIN_VALIDATORS;
use cosmwasm_std::{ensure, Addr, BlockInfo, Deps, DepsMut, Env, MessageInfo};
use sha2::{Digest, Sha256};

/// Returns a list of validators based on the provided delegation strategy.
pub(crate) fn get_validators(
    deps: Deps,
    env: &Env,
    sender: &Addr,
    delegation_strategy: DelegationStrategy,
) -> Result<Vec<String>, StakingError> {
    let validators = match delegation_strategy {
        DelegationStrategy::Pseudorandom(n) => {
            let n = n.unwrap_or(MIN_VALIDATORS);
            check_validators_size(n)?;

            pick_pseudorandom_validators(deps, &env.block, &sender, n)?
        }
        DelegationStrategy::TopN(n) => {
            check_validators_size(n)?;

            deps.querier
                .query_all_validators()?
                .iter()
                .take(n)
                .map(|v| v.address.clone())
                .collect()
        }
        DelegationStrategy::BottomN(n) => {
            check_validators_size(n)?;

            deps.querier
                .query_all_validators()?
                .iter()
                .rev()
                .take(n)
                .map(|v| v.address.clone())
                .collect()
        }
        DelegationStrategy::Custom(validators) => {
            check_validators_size(validators.len())?;
            validators
        }
    };
    Ok(validators)
}

/// Checks if the number of validators is valid.
#[inline]
fn check_validators_size(n: usize) -> Result<(), StakingError> {
    ensure!(
        n >= MIN_VALIDATORS,
        StakingError::NotEnoughValidators {
            min_validators: MIN_VALIDATORS,
            provided_validators: n
        }
    );
    Ok(())
}

/// Pseudorandomly selects a specified number of validators from the active validator set.
fn pick_pseudorandom_validators(
    deps: Deps,
    block: &BlockInfo,
    sender: &Addr,
    num_validators: usize,
) -> Result<Vec<String>, StakingError> {
    let active_validators = deps.querier.query_all_validators()?;

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

    let selected_indices = &indices[..num_validators];

    let selected_validators = selected_indices
        .iter()
        .map(|&i| active_validators[i].address.clone())
        .collect();

    Ok(selected_validators)
}
