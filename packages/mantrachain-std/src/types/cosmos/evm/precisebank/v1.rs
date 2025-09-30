use neutron_std_derive::CosmwasmExt;
/// GenesisState defines the precisebank module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.GenesisState")]
pub struct GenesisState {
    /// balances is a list of all the balances in the precisebank module.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<FractionalBalance>,
    /// remainder is an internal value of how much extra fractional digits are
    /// still backed by the reserve, but not assigned to any account.
    #[prost(string, tag = "2")]
    pub remainder: ::prost::alloc::string::String,
}
/// FractionalBalance defines the fractional portion of an account balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.FractionalBalance")]
pub struct FractionalBalance {
    /// address is the address of the balance holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// amount indicates amount of only the fractional balance owned by the
    /// address. FractionalBalance currently only supports tracking 1 single asset,
    /// e.g. fractional balances of uatom.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// QueryRemainderRequest defines the request type for Query/Remainder method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.QueryRemainderRequest")]
#[proto_query(
    path = "/cosmos.evm.precisebank.v1.Query/Remainder",
    response_type = QueryRemainderResponse
)]
pub struct QueryRemainderRequest {}
/// QueryRemainderResponse defines the response type for Query/Remainder method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.QueryRemainderResponse")]
pub struct QueryRemainderResponse {
    /// remainder is the amount backed by the reserve, but not yet owned by any
    /// account, i.e. not in circulation.
    #[prost(message, optional, tag = "1")]
    pub remainder: ::core::option::Option<super::super::super::base::v1beta1::Coin>,
}
/// QueryFractionalBalanceRequest defines the request type for
/// Query/FractionalBalance method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.QueryFractionalBalanceRequest")]
#[proto_query(
    path = "/cosmos.evm.precisebank.v1.Query/FractionalBalance",
    response_type = QueryFractionalBalanceResponse
)]
pub struct QueryFractionalBalanceRequest {
    /// address is the account address to query  fractional balance for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryFractionalBalanceResponse defines the response type for
/// Query/FractionalBalance method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.evm.precisebank.v1.QueryFractionalBalanceResponse")]
pub struct QueryFractionalBalanceResponse {
    /// fractional_balance is the fractional balance of the address.
    #[prost(message, optional, tag = "1")]
    pub fractional_balance: ::core::option::Option<super::super::super::base::v1beta1::Coin>,
}
pub struct PrecisebankQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PrecisebankQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn remainder(&self) -> Result<QueryRemainderResponse, cosmwasm_std::StdError> {
        QueryRemainderRequest {}.query(self.querier)
    }
    pub fn fractional_balance(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryFractionalBalanceResponse, cosmwasm_std::StdError> {
        QueryFractionalBalanceRequest { address }.query(self.querier)
    }
}
