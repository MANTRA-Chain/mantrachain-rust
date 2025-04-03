#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "staking")]
mod staking;
#[cfg(feature = "vesting")]
mod vesting;
