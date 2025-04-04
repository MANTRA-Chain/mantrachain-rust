pub mod error;
mod helpers;
pub mod native;
#[cfg(test)]
mod tests;

/// The minimum number of validators required for delegation.
const MIN_VALIDATORS: usize = 4usize;
