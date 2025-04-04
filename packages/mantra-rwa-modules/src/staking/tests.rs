use super::*;
use crate::staking::error::StakingError;
use crate::staking::helpers::get_validators;
use crate::staking::native::{claim_staking_rewards, delegate, undelegate, DelegationStrategy};
use cosmwasm_std::testing::{
    mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage, StakingQuerier,
};
use cosmwasm_std::{
    coin, coins, Addr, BankMsg, CosmosMsg, DecCoin, Decimal256, DistributionMsg, FullDelegation,
    OwnedDeps, StakingMsg, Uint256, Validator, WasmMsg,
};
use std::str::FromStr;

fn get_delegations(api: &MockApi, sender: &Addr, count: usize) -> Vec<FullDelegation> {
    (1..=count)
        .map(|i| {
            FullDelegation::create(
                sender.clone(),
                api.addr_make(&format!("validator{}", i)).to_string(),
                coin(100, "uom"),
                coin(100, "uom"),
                coins(5, "uom"),
            )
        })
        .collect()
}

fn get_validator_list(api: MockApi, count: usize) -> Vec<Validator> {
    (1..=count)
        .map(|i| {
            Validator::create(
                api.addr_make(&format!("validator{}", i)).to_string(),
                Default::default(),
                Default::default(),
                Default::default(),
            )
        })
        .collect()
}

fn mock_validators(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
    deps.querier
        .staking
        .update("uom", &get_validator_list(deps.api, 5), &[]);
}

fn mock_staking_rewards(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>, sender: &Addr) {
    deps.querier.distribution.set_validators(
        sender.to_string(),
        vec![
            deps.api.addr_make("validator1").to_string(),
            deps.api.addr_make("validator2").to_string(),
            deps.api.addr_make("validator3").to_string(),
            deps.api.addr_make("validator4").to_string(),
        ],
    );

    for i in 1..=4 {
        deps.querier.distribution.set_rewards(
            deps.api.addr_make(&format!("validator{}", i)).to_string(),
            sender.to_string(),
            vec![DecCoin {
                denom: "uom".to_string(),
                amount: Decimal256::one(),
            }],
        );
    }

    deps.querier.staking.update(
        "uom",
        &get_validator_list(deps.api, 5),
        &get_delegations(&deps.api, &sender, 4),
    );
}

fn mocked_validators_addresses(api: MockApi) -> Vec<String> {
    vec![
        api.addr_make("validator1").to_string(),
        api.addr_make("validator2").to_string(),
        api.addr_make("validator3").to_string(),
        api.addr_make("validator4").to_string(),
        api.addr_make("validator5").to_string(),
    ]
}

#[test]
fn test_get_pseudorandom_validators() {
    let mut deps = mock_dependencies();
    mock_validators(&mut deps);
    let mut env = mock_env();
    let sender = deps.api.addr_make("sender");

    let validators_1 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Pseudorandom(Some(4)),
    )
    .unwrap();

    env.block.height += 1000;

    let validators_2 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Pseudorandom(None),
    )
    .unwrap();
    assert_eq!(validators_1.len(), 4);
    assert_eq!(validators_2.len(), 4);
    assert_ne!(validators_1, validators_2);

    let validators_3 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Pseudorandom(Some(10)),
    )
    .unwrap();
    // only 5 validators are available
    assert_eq!(validators_3.len(), 5);

    let err = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Pseudorandom(Some(MIN_VALIDATORS - 1)),
    )
    .unwrap_err();
    match err {
        StakingError::NotEnoughValidators {
            min_validators,
            provided_validators,
        } => {
            assert_eq!(min_validators, MIN_VALIDATORS);
            assert_eq!(provided_validators, MIN_VALIDATORS - 1);
        }
        _ => panic!("Expected NotEnoughValidators error"),
    }
}

#[test]
fn test_get_topn_validators() {
    let mut deps = mock_dependencies();
    mock_validators(&mut deps);
    let env = mock_env();
    let sender = deps.api.addr_make("sender");

    let validators_1 =
        get_validators(deps.as_ref(), &env, &sender, DelegationStrategy::TopN(4)).unwrap();
    let validators_2 =
        get_validators(deps.as_ref(), &env, &sender, DelegationStrategy::TopN(10)).unwrap();
    assert_eq!(
        validators_1,
        mocked_validators_addresses(deps.api)
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<String>>()
    );
    assert_eq!(
        validators_2,
        mocked_validators_addresses(deps.api)
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<String>>()
    );

    let err = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::TopN(MIN_VALIDATORS - 1),
    )
    .unwrap_err();
    match err {
        StakingError::NotEnoughValidators {
            min_validators,
            provided_validators,
        } => {
            assert_eq!(min_validators, MIN_VALIDATORS);
            assert_eq!(provided_validators, MIN_VALIDATORS - 1);
        }
        _ => panic!("Expected NotEnoughValidators error"),
    }
}

#[test]
fn test_get_bottomn_validators() {
    let mut deps = mock_dependencies();
    mock_validators(&mut deps);
    let env = mock_env();
    let sender = deps.api.addr_make("sender");

    let validators_1 =
        get_validators(deps.as_ref(), &env, &sender, DelegationStrategy::BottomN(4)).unwrap();
    let validators_2 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::BottomN(10),
    )
    .unwrap();
    assert_eq!(
        validators_1,
        mocked_validators_addresses(deps.api)
            .iter()
            .rev()
            .take(4)
            .cloned()
            .collect::<Vec<String>>()
    );
    assert_eq!(
        validators_2,
        mocked_validators_addresses(deps.api)
            .iter()
            .rev()
            .take(5)
            .cloned()
            .collect::<Vec<String>>()
    );

    let err = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::BottomN(MIN_VALIDATORS - 1),
    )
    .unwrap_err();
    match err {
        StakingError::NotEnoughValidators {
            min_validators,
            provided_validators,
        } => {
            assert_eq!(min_validators, MIN_VALIDATORS);
            assert_eq!(provided_validators, MIN_VALIDATORS - 1);
        }
        _ => panic!("Expected NotEnoughValidators error"),
    }
}

#[test]
fn test_get_custom_validators() {
    let mut deps = mock_dependencies();
    mock_validators(&mut deps);
    let env = mock_env();
    let sender = deps.api.addr_make("sender");

    let validators_1 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Custom(
            mocked_validators_addresses(deps.api)
                .iter()
                .take(4)
                .cloned()
                .collect::<Vec<String>>(),
        ),
    )
    .unwrap();
    let validators_2 = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Custom(
            mocked_validators_addresses(deps.api)
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<String>>(),
        ),
    )
    .unwrap();
    assert_eq!(
        validators_1,
        mocked_validators_addresses(deps.api)
            .iter()
            .take(4)
            .cloned()
            .collect::<Vec<String>>()
    );
    assert_eq!(
        validators_2,
        mocked_validators_addresses(deps.api)
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<String>>()
    );

    let err = get_validators(
        deps.as_ref(),
        &env,
        &sender,
        DelegationStrategy::Custom(vec!["validator1".to_string(), "validator2".to_string()]),
    )
    .unwrap_err();
    match err {
        StakingError::NotEnoughValidators { .. } => {}
        _ => panic!("Expected NotEnoughValidators error"),
    }
}

#[test]
fn test_delegate() {
    let mut deps = mock_dependencies();
    mock_validators(&mut deps);
    let env = mock_env();
    let sender = deps.api.addr_make("sender");

    let to_delegate = coin(1000, "uom");
    let delegation_strategy = DelegationStrategy::TopN(4);

    let delegate_messages = delegate(
        deps.as_ref(),
        &env,
        &sender,
        to_delegate,
        delegation_strategy,
    )
    .unwrap();
    assert_eq!(
        delegate_messages,
        vec![
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator1").to_string(),
                amount: coin(250u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator2").to_string(),
                amount: coin(250u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator3").to_string(),
                amount: coin(250u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator4").to_string(),
                amount: coin(250u128, "uom"),
            })
        ]
    );

    let to_delegate = coin(125, "uom");
    let delegation_strategy = DelegationStrategy::TopN(4);

    let delegate_messages = delegate(
        deps.as_ref(),
        &env,
        &sender,
        to_delegate,
        delegation_strategy,
    )
    .unwrap();

    assert_eq!(
        delegate_messages,
        vec![
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator1").to_string(),
                amount: coin(31u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator2").to_string(),
                amount: coin(31u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator3").to_string(),
                amount: coin(31u128, "uom"),
            }),
            CosmosMsg::Staking(StakingMsg::Delegate {
                validator: deps.api.addr_make("validator4").to_string(),
                // amount + dust
                amount: coin(32u128, "uom"),
            })
        ]
    )
}
#[test]
fn test_undelegate() {
    let mut deps = mock_dependencies();
    let amount = coin(1000, "uom");

    let undelegate_msg = undelegate(deps.api.addr_make("validator1").as_str(), amount).unwrap();

    assert_eq!(
        undelegate_msg,
        CosmosMsg::Staking(StakingMsg::Undelegate {
            validator: deps.api.addr_make("validator1").to_string(),
            amount: coin(1000u128, "uom"),
        })
    );
}

#[test]
fn test_claim_staking_rewards() {
    let mut deps = mock_dependencies();
    let sender = deps.api.addr_make("sender");
    let recipient = deps.api.addr_make("recipient");

    let err = claim_staking_rewards(deps.as_ref(), sender.as_str(), None).unwrap_err();
    match err {
        StakingError::NothingToClaim => {}
        _ => panic!("Expected NothingToClaim error"),
    }

    mock_staking_rewards(&mut deps, &sender);

    let claim_messages = claim_staking_rewards(deps.as_ref(), sender.as_str(), None).unwrap();
    assert_eq!(
        claim_messages,
        vec![
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator1").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator2").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator3").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator4").to_string()
            }),
        ]
    );

    let claim_messages =
        claim_staking_rewards(deps.as_ref(), sender.as_str(), Some(recipient.to_string())).unwrap();
    assert_eq!(
        claim_messages,
        vec![
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator1").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator2").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator3").to_string()
            }),
            CosmosMsg::Distribution(DistributionMsg::WithdrawDelegatorReward {
                validator: deps.api.addr_make("validator4").to_string()
            }),
            CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.to_string(),
                amount: vec![coin(4, "uom")],
            }),
        ]
    );
}
