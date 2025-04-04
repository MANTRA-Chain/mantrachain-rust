use cosmwasm_std::{Addr, Deps};

/// Validates a [String] address or returns the default address if the validation fails.
pub fn validate_addr_or_default(deps: &Deps, unvalidated: Option<String>, default: Addr) -> Addr {
    unvalidated
        .map_or_else(
            || Some(default.clone()),
            |recv| deps.api.addr_validate(&recv).ok(),
        )
        .unwrap_or(default)
}
