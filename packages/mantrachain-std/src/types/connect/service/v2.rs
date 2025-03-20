use osmosis_std_derive::CosmwasmExt;
/// QueryPricesRequest defines the request type for the the Prices method.
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
#[proto_message(type_url = "/connect.service.v2.QueryPricesRequest")]
pub struct QueryPricesRequest {}
/// QueryPricesResponse defines the response type for the Prices method.
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
#[proto_message(type_url = "/connect.service.v2.QueryPricesResponse")]
pub struct QueryPricesResponse {
    /// Prices defines the list of prices.
    #[prost(map = "string, string", tag = "1")]
    pub prices:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Timestamp defines the timestamp of the prices.
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    /// Version defines the version of the oracle service that provided the prices.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// QueryMarketMapRequest defines the request type for the MarketMap method.
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
#[proto_message(type_url = "/connect.service.v2.QueryMarketMapRequest")]
pub struct QueryMarketMapRequest {}
/// QueryMarketMapResponse defines the response type for the MarketMap method.
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
#[proto_message(type_url = "/connect.service.v2.QueryMarketMapResponse")]
pub struct QueryMarketMapResponse {
    /// MarketMap defines the current market map configuration.
    #[prost(message, optional, tag = "1")]
    pub market_map: ::core::option::Option<super::super::marketmap::v2::MarketMap>,
}
/// QueryVersionRequest defines the request type for the Version method.
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
#[proto_message(type_url = "/connect.service.v2.QueryVersionRequest")]
pub struct QueryVersionRequest {}
/// QueryVersionResponse defines the response type for the Version method.
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
#[proto_message(type_url = "/connect.service.v2.QueryVersionResponse")]
pub struct QueryVersionResponse {
    /// Version defines the current version of the oracle service.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
