use neutron_std_derive::CosmwasmExt;
/// EventFeeMarket is the event type for the feemarket module
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.EventFeeMarket")]
pub struct EventFeeMarket {
    /// base_fee for EIP-1559 blocks
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// EventBlockGas defines an Ethereum block gas event
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.EventBlockGas")]
pub struct EventBlockGas {
    /// height of the block
    #[prost(string, tag = "1")]
    pub height: ::prost::alloc::string::String,
    /// amount of gas wanted by the block
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// Params defines the EVM module parameters
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.Params")]
pub struct Params {
    /// no_base_fee forces the EIP-1559 base fee to 0 (needed for 0 price calls)
    #[prost(bool, tag = "1")]
    pub no_base_fee: bool,
    /// base_fee_change_denominator bounds the amount the base fee can change
    /// between blocks.
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub base_fee_change_denominator: u32,
    /// elasticity_multiplier bounds the maximum gas limit an EIP-1559 block may
    /// have.
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub elasticity_multiplier: u32,
    /// enable_height defines at which block height the base fee calculation is
    /// enabled.
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub enable_height: i64,
    /// base_fee for EIP-1559 blocks.
    #[prost(string, tag = "6")]
    pub base_fee: ::prost::alloc::string::String,
    /// min_gas_price defines the minimum gas price value for cosmos and eth
    /// transactions
    #[prost(string, tag = "7")]
    pub min_gas_price: ::prost::alloc::string::String,
    /// min_gas_multiplier bounds the minimum gas used to be charged
    /// to senders based on gas limit
    #[prost(string, tag = "8")]
    pub min_gas_multiplier: ::prost::alloc::string::String,
}
/// GenesisState defines the feemarket module's genesis state.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the feemarket module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// block_gas is the amount of gas wanted on the last block before the upgrade.
    /// Zero by default.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_gas: u64,
}
/// QueryParamsRequest defines the request type for querying x/vm parameters.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.evm.feemarket.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/vm parameters.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryBaseFeeRequest")]
#[proto_query(
    path = "/cosmos.evm.feemarket.v1.Query/BaseFee",
    response_type = QueryBaseFeeResponse
)]
pub struct QueryBaseFeeRequest {}
/// QueryBaseFeeResponse returns the EIP1559 base fee.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryBaseFeeResponse")]
pub struct QueryBaseFeeResponse {
    /// base_fee is the EIP1559 base fee
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// QueryBlockGasRequest defines the request type for querying the EIP1559 base
/// fee.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryBlockGasRequest")]
#[proto_query(
    path = "/cosmos.evm.feemarket.v1.Query/BlockGas",
    response_type = QueryBlockGasResponse
)]
pub struct QueryBlockGasRequest {}
/// QueryBlockGasResponse returns block gas used for a given height.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.QueryBlockGasResponse")]
pub struct QueryBlockGasResponse {
    /// gas is the returned block gas
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas: i64,
}
/// MsgUpdateParams defines a Msg for updating the x/feemarket module parameters.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/feemarket parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
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
#[proto_message(type_url = "/cosmos.evm.feemarket.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct FeemarketQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeemarketQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn base_fee(&self) -> Result<QueryBaseFeeResponse, cosmwasm_std::StdError> {
        QueryBaseFeeRequest {}.query(self.querier)
    }
    pub fn block_gas(&self) -> Result<QueryBlockGasResponse, cosmwasm_std::StdError> {
        QueryBlockGasRequest {}.query(self.querier)
    }
}
