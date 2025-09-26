use neutron_std_derive::CosmwasmExt;
/// ContinuousFund defines the fields of continuous fund proposal.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.ContinuousFund")]
pub struct ContinuousFund {
    /// Recipient is the address string of the account receiving funds.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    /// Percentage is the percentage of funds to be allocated from Community pool.
    #[prost(string, tag = "2")]
    pub percentage: ::prost::alloc::string::String,
    /// Optional, if expiry is set, removes the state object when expired.
    #[prost(message, optional, tag = "3")]
    pub expiry: ::core::option::Option<crate::shim::Timestamp>,
}
/// Params defines the parameters for the protocolpool module.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.Params")]
pub struct Params {
    /// EnabledDistributionDenoms lists the denoms that are allowed to be distributed.
    /// This is to avoid spending time distributing undesired tokens to continuous funds and budgets.
    #[prost(string, repeated, tag = "1")]
    pub enabled_distribution_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DistributionFrequency is the frequency (in terms of blocks) that funds are distributed out from the
    /// x/protocolpool module.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub distribution_frequency: u64,
}
/// GenesisState defines the protocolpool module's genesis state.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.GenesisState")]
pub struct GenesisState {
    /// ContinuousFunds defines the continuous funds at genesis.
    #[prost(message, repeated, tag = "1")]
    pub continuous_funds: ::prost::alloc::vec::Vec<ContinuousFund>,
    /// Params defines the parameters of this module, currently only contains the
    /// denoms that will be used for continuous fund distributions.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCommunityPoolRequest is the request type for the Query/CommunityPool RPC
/// method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryCommunityPoolRequest")]
#[proto_query(
    path = "/cosmos.protocolpool.v1.Query/CommunityPool",
    response_type = QueryCommunityPoolResponse
)]
pub struct QueryCommunityPoolRequest {}
/// QueryCommunityPoolResponse is the response type for the Query/CommunityPool
/// RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryCommunityPoolResponse")]
pub struct QueryCommunityPoolResponse {
    /// pool defines community pool's coins.
    #[prost(message, repeated, tag = "1")]
    pub pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// QueryContinuousFundRequest is the request type for the Query/ContinuousFund
/// RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryContinuousFundRequest")]
#[proto_query(
    path = "/cosmos.protocolpool.v1.Query/ContinuousFund",
    response_type = QueryContinuousFundResponse
)]
pub struct QueryContinuousFundRequest {
    /// recipient is the recipient address to query unclaimed budget amount for.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
}
/// QueryUnclaimedBudgetResponse is the response type for the Query/ContinuousFund
/// RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryContinuousFundResponse")]
pub struct QueryContinuousFundResponse {
    /// ContinuousFunds is the given continuous fund returned in the query.
    #[prost(message, optional, tag = "1")]
    pub continuous_fund: ::core::option::Option<ContinuousFund>,
}
/// QueryContinuousFundRequest is the request type for the Query/ContinuousFunds
/// RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryContinuousFundsRequest")]
#[proto_query(
    path = "/cosmos.protocolpool.v1.Query/ContinuousFunds",
    response_type = QueryContinuousFundsResponse
)]
pub struct QueryContinuousFundsRequest {}
/// QueryUnclaimedBudgetResponse is the response type for the Query/ContinuousFunds
/// RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryContinuousFundsResponse")]
pub struct QueryContinuousFundsResponse {
    /// ContinuousFunds defines all continuous funds in state.
    #[prost(message, repeated, tag = "1")]
    pub continuous_funds: ::prost::alloc::vec::Vec<ContinuousFund>,
}
/// QueryParamsRequest is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.protocolpool.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgFundCommunityPool allows an account to directly
/// fund the community pool.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgFundCommunityPool")]
pub struct MsgFundCommunityPool {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgFundCommunityPoolResponse defines the Msg/FundCommunityPool response type.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgFundCommunityPoolResponse")]
pub struct MsgFundCommunityPoolResponse {}
/// MsgCommunityPoolSpend defines a message for sending tokens from the community
/// pool to another account. This message is typically executed via a governance
/// proposal with the governance module being the executing authority.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCommunityPoolSpend")]
pub struct MsgCommunityPoolSpend {
    /// Authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgCommunityPoolSpendResponse defines the response to executing a
/// MsgCommunityPoolSpend message.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCommunityPoolSpendResponse")]
pub struct MsgCommunityPoolSpendResponse {}
/// MsgCreateContinuousFund defines a message for adding continuous funds.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCreateContinuousFund")]
pub struct MsgCreateContinuousFund {
    /// Authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Recipient address of the account receiving funds.
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    /// Percentage is the percentage of funds to be allocated from Community pool.
    #[prost(string, tag = "3")]
    pub percentage: ::prost::alloc::string::String,
    /// Optional, if expiry is set, removes the state object when expired.
    #[prost(message, optional, tag = "4")]
    pub expiry: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgCreateContinuousFundResponse defines the response to executing a
/// MsgCreateContinuousFund message.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCreateContinuousFundResponse")]
pub struct MsgCreateContinuousFundResponse {}
/// MsgCancelContinuousFund defines a message to cancel continuous funds for a specific recipient.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCancelContinuousFund")]
pub struct MsgCancelContinuousFund {
    /// Authority is the account address of authority.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Recipient is the account address string of the recipient whose funds are to be cancelled.
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
}
/// MsgCancelContinuousFundResponse defines the response to executing a
/// MsgCancelContinuousFund message.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgCancelContinuousFundResponse")]
pub struct MsgCancelContinuousFundResponse {
    /// CanceledTime is the canceled time.
    #[prost(message, optional, tag = "1")]
    pub canceled_time: ::core::option::Option<crate::shim::Timestamp>,
    /// CanceledHeight defines the canceled block height.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub canceled_height: u64,
    /// Recipient is the account address string of the recipient whose funds are cancelled.
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/protocolpool parameters to update.
    ///
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
#[proto_message(type_url = "/cosmos.protocolpool.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct ProtocolpoolQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ProtocolpoolQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn community_pool(&self) -> Result<QueryCommunityPoolResponse, cosmwasm_std::StdError> {
        QueryCommunityPoolRequest {}.query(self.querier)
    }
    pub fn continuous_fund(
        &self,
        recipient: ::prost::alloc::string::String,
    ) -> Result<QueryContinuousFundResponse, cosmwasm_std::StdError> {
        QueryContinuousFundRequest { recipient }.query(self.querier)
    }
    pub fn continuous_funds(&self) -> Result<QueryContinuousFundsResponse, cosmwasm_std::StdError> {
        QueryContinuousFundsRequest {}.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
