# mantra-claimdrop-std

Common types and error definitions for the Mantra claimdrop contract V2.

## Overview

This package provides shared types, message definitions, and error handling for claimdrop (airdrop) contracts in the Mantra ecosystem. It enables code reuse across different claimdrop-related contracts and provides a consistent interface for campaign management, token distribution, and reward claiming.

## Features

- **Message Types**: Complete set of execute, query, and response messages for claimdrop operations
- **Campaign Management**: Types for creating, managing, and closing airdrop campaigns
- **Distribution Types**: Support for linear vesting and lump sum distribution mechanisms
- **Error Handling**: Comprehensive error types for validation and runtime errors
- **Validation**: Built-in validation for campaign parameters, distribution settings, and time constraints

## Usage

Add this package to your `Cargo.toml`:

```toml
[dependencies]
mantra-claimdrop-std = { version = "1.0.0", path = "path/to/mantra-claimdrop-std" }
```

### Message Types

```rust
use mantra_claimdrop_std::msg::{
    ExecuteMsg, QueryMsg, CampaignParams, DistributionType
};

// Create a campaign
let campaign_params = CampaignParams {
    name: "My Airdrop".to_string(),
    description: "Token distribution campaign".to_string(),
    ty: "airdrop".to_string(),
    total_reward: Coin::new(1000000u128, "umantra"),
    distribution_type: vec![DistributionType::LumpSum {
        percentage: Decimal::percent(100),
        start_time: 1640995200, // Unix timestamp
    }],
    start_time: 1640995200,
    end_time: 1672531200,
};

let execute_msg = ExecuteMsg::ManageCampaign {
    action: CampaignAction::CreateCampaign {
        params: Box::new(campaign_params),
    },
};
```

### Error Handling

```rust
use mantra_claimdrop_std::error::ContractError;

fn validate_campaign(params: &CampaignParams) -> Result<(), ContractError> {
    params.validate_campaign_name_description()?;
    params.validate_campaign_type()?;
    params.validate_rewards()?;
    Ok(())
}
```

## Types

### Campaign Types

- `Campaign`: Complete campaign information including status and claimed amounts
- `CampaignParams`: Parameters for creating new campaigns
- `CampaignAction`: Actions for managing campaigns (create/close)

### Distribution Types

- `DistributionType::LinearVesting`: Gradual token release over time with optional cliff
- `DistributionType::LumpSum`: Immediate token release at specified time

### Response Types

- `RewardsResponse`: Information about claimed, pending, and available rewards
- `ClaimedResponse`: Details about claimed tokens per address
- `AllocationsResponse`: Address allocation information
- `BlacklistResponse`: Address blacklist status

## Validation

The package includes comprehensive validation for:

- Campaign name and description length limits
- Time constraints (start/end times, distribution schedules)
- Distribution percentage totals (must equal 100%)
- Reward amounts and denominations
- Cliff duration limits for linear vesting
