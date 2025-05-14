# Changelog

## v3.0.0

- Fix spread vs slippage nomenclature.

## v2.2.0

- Added:
  - Tokenfactory params responses.
  - usage of `mantrachain_std` instead of `osmosis-std` for the TokenfactoryQuerier.

## v2.1.6

- Added:
  - PoolStatus to PoolInfo struct.
  - `pool_indentifier` param to FeatureToggle struct, so features can be enabled/disabled per pool.

- Removed:
  - FeatureToggle from Config struct.

## v2.1.5

- Added:
  - BeforeSendHook msg to tokenfactory helpers.

## v2.1.4

- Removed:
  - Unused fields on Farm struct.

## v2.1.3

- Added:
  - `until_epoch` param to rewards query in farm manager.

## v2.1.2

- Added:
  - Optional `until_epoch` param in claim function of farm manager.

## v2.1.1

- Use cosmwasm2_2 feature flag.

## v2.1.0

- Bump cosmwasm-std and other packages versions.

## v2.0.0

- Added:
  - Expose spread and fees in both `SimulateSwapOperationsResponse` and `ReverseSimulateSwapOperationsResponse`.

## v1.0.2

- Added:
  - Expose extra fees amount in `ReverseSimulationResponse`.
