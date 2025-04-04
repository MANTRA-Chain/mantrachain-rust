#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "staking")]
pub mod staking;
#[cfg(feature = "vesting")]
pub mod vesting;
