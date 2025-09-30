use neutron_std_derive::CosmwasmExt;
/// TokenPair defines an instance that records a pairing consisting of a native
/// Cosmos Coin and an ERC20 token address.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.TokenPair")]
pub struct TokenPair {
    /// erc20_address is the hex address of ERC20 contract token
    #[prost(string, tag = "1")]
    pub erc20_address: ::prost::alloc::string::String,
    /// denom defines the cosmos base denomination to be mapped to
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// enabled defines the token mapping enable status
    #[prost(bool, tag = "3")]
    pub enabled: bool,
    /// contract_owner is the an ENUM specifying the type of ERC20 owner (0
    /// invalid, 1 ModuleAccount, 2 external address)
    #[prost(enumeration = "Owner", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub contract_owner: i32,
}
/// Allowance is a token allowance only for erc20 precompile
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.Allowance")]
pub struct Allowance {
    /// erc20_address is the hex address of ERC20 contract
    #[prost(string, tag = "1")]
    pub erc20_address: ::prost::alloc::string::String,
    /// owner is the hex address of the owner account
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// spender is the hex address that is allowed to spend the allowance
    #[prost(string, tag = "3")]
    pub spender: ::prost::alloc::string::String,
    /// value specifies the maximum amount of tokens that can be spent
    /// by this token allowance and will be updated as tokens are spent.
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
}
/// Deprecated: RegisterCoinProposal is a gov Content type to register a token
/// pair for a native Cosmos coin. We're keeping it to remove the existing
/// proposals from store. After that, remove this message.
///
/// NOTE: Keep this message for backwards compatibility on proposals query
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.RegisterCoinProposal")]
pub struct RegisterCoinProposal {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// metadata slice of the native Cosmos coins
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::super::bank::v1beta1::Metadata>,
}
/// Deprecated: ProposalMetadata is used to parse a slice of denom metadata and
/// generate the RegisterCoinProposal content. We're keeping it to remove the
/// existing proposals from store. After that, remove this message.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.ProposalMetadata")]
pub struct ProposalMetadata {
    /// metadata slice of the native Cosmos coins
    #[prost(message, repeated, tag = "1")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::super::bank::v1beta1::Metadata>,
}
/// Deprecated: RegisterERC20Proposal is a gov Content type to register a token
/// pair for an ERC20 token.
///
/// NOTE: Keep this message for backwards compatibility on proposals query
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.RegisterERC20Proposal")]
pub struct RegisterErc20Proposal {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// erc20addresses is a slice of  ERC20 token contract addresses
    #[prost(string, repeated, tag = "3")]
    pub erc20addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Deprecated: ToggleTokenConversionProposal is a gov Content type to toggle the
/// conversion of a token pair.
///
/// NOTE: Keep this message for backwards compatibility on proposals query
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.ToggleTokenConversionProposal")]
pub struct ToggleTokenConversionProposal {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// token identifier can be either the hex contract address of the ERC20 or the
    /// Cosmos base denomination
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
}
/// Owner enumerates the ownership of a ERC20 contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Owner {
    /// OWNER_UNSPECIFIED defines an invalid/undefined owner.
    Unspecified = 0,
    /// OWNER_MODULE - erc20 is owned by the erc20 module account.
    Module = 1,
    /// OWNER_EXTERNAL - erc20 is owned by an external account.
    External = 2,
}
impl Owner {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Owner::Unspecified => "OWNER_UNSPECIFIED",
            Owner::Module => "OWNER_MODULE",
            Owner::External => "OWNER_EXTERNAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OWNER_UNSPECIFIED" => Some(Self::Unspecified),
            "OWNER_MODULE" => Some(Self::Module),
            "OWNER_EXTERNAL" => Some(Self::External),
            _ => None,
        }
    }
}
/// EventRegisterPair is an event emitted when a coin is registered.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.EventRegisterPair")]
pub struct EventRegisterPair {
    /// denom is the coin's denomination.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag = "2")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventToggleTokenConversion is an event emitted when a coin's token conversion
/// is toggled.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.EventToggleTokenConversion")]
pub struct EventToggleTokenConversion {
    /// denom is the coin's denomination.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag = "2")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventConvertCoin is an event emitted when a coin is converted.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.EventConvertCoin")]
pub struct EventConvertCoin {
    /// sender is the sender's address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver's address.
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    /// amount is the amount of coins to be converted.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// denom is the coin's denomination.
    #[prost(string, tag = "4")]
    pub denom: ::prost::alloc::string::String,
    /// erc20_address is the ERC20 contract address.
    #[prost(string, tag = "5")]
    pub erc20_address: ::prost::alloc::string::String,
}
/// EventConvertERC20 is an event emitted when an ERC20 is converted.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.EventConvertERC20")]
pub struct EventConvertErc20 {
    /// sender is the sender's address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver's address.
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    /// amount is the amount of coins to be converted.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// denom is the coin's denomination.
    #[prost(string, tag = "4")]
    pub denom: ::prost::alloc::string::String,
    /// contract_address of an ERC20 token contract, that is registered in a token
    /// pair
    #[prost(string, tag = "5")]
    pub contract_address: ::prost::alloc::string::String,
}
/// GenesisState defines the module's genesis state.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.GenesisState")]
pub struct GenesisState {
    /// params are the erc20 module parameters at genesis
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// token_pairs is a slice of the registered token pairs at genesis
    #[prost(message, repeated, tag = "2")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
    /// allowances is a slice of the registered allowances at genesis
    #[prost(message, repeated, tag = "3")]
    pub allowances: ::prost::alloc::vec::Vec<Allowance>,
    /// native_precompiles is a slice of registered native precompiles at genesis
    #[prost(string, repeated, tag = "4")]
    pub native_precompiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// dynamic_precompiles is a slice of registered dynamic precompiles at genesis
    #[prost(string, repeated, tag = "5")]
    pub dynamic_precompiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the erc20 module params
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.Params")]
pub struct Params {
    /// enable_erc20 is the parameter to enable the conversion of Cosmos coins <-->
    /// ERC20 tokens.
    #[prost(bool, tag = "1")]
    pub enable_erc20: bool,
    /// permissionless_registration is the parameter that allows ERC20s to be
    /// permissionlessly registered to be converted to bank tokens and vice versa
    #[prost(bool, tag = "5")]
    pub permissionless_registration: bool,
}
/// QueryTokenPairsRequest is the request type for the Query/TokenPairs RPC
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryTokenPairsRequest")]
#[proto_query(
    path = "/cosmos.evm.erc20.v1.Query/TokenPairs",
    response_type = QueryTokenPairsResponse
)]
pub struct QueryTokenPairsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageRequest>,
}
/// QueryTokenPairsResponse is the response type for the Query/TokenPairs RPC
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryTokenPairsResponse")]
pub struct QueryTokenPairsResponse {
    /// token_pairs is a slice of registered token pairs for the erc20 module
    #[prost(message, repeated, tag = "1")]
    pub token_pairs: ::prost::alloc::vec::Vec<TokenPair>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageResponse>,
}
/// QueryTokenPairRequest is the request type for the Query/TokenPair RPC method.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryTokenPairRequest")]
#[proto_query(
    path = "/cosmos.evm.erc20.v1.Query/TokenPair",
    response_type = QueryTokenPairResponse
)]
pub struct QueryTokenPairRequest {
    /// token identifier can be either the hex contract address of the ERC20 or the
    /// Cosmos base denomination
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
/// QueryTokenPairResponse is the response type for the Query/TokenPair RPC
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryTokenPairResponse")]
pub struct QueryTokenPairResponse {
    /// token_pairs returns the info about a registered token pair for the erc20
    /// module
    #[prost(message, optional, tag = "1")]
    pub token_pair: ::core::option::Option<TokenPair>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.evm.erc20.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params are the erc20 module parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgConvertERC20 defines a Msg to convert a ERC20 token to a native Cosmos
/// coin.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgConvertERC20")]
pub struct MsgConvertErc20 {
    /// contract_address of an ERC20 token contract, that is registered in a token
    /// pair
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// amount of ERC20 tokens to convert
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// receiver is the bech32 address to receive native Cosmos coins
    #[prost(string, tag = "3")]
    pub receiver: ::prost::alloc::string::String,
    /// sender is the hex address from the owner of the given ERC20 tokens
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgConvertERC20Response returns no fields
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgConvertERC20Response")]
pub struct MsgConvertErc20Response {}
/// MsgConvertCoin defines a Msg to convert a native Cosmos coin to a ERC20 token
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgConvertCoin")]
pub struct MsgConvertCoin {
    /// coin is a Cosmos coin whose denomination is registered in a token pair. The
    /// coin amount defines the amount of coins to convert.
    #[prost(message, optional, tag = "1")]
    pub coin: ::core::option::Option<super::super::super::base::v1beta1::Coin>,
    /// receiver is the hex address to receive ERC20 token
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    /// sender is the cosmos bech32 address from the owner of the given Cosmos
    /// coins
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
/// MsgConvertCoinResponse returns no fields
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgConvertCoinResponse")]
pub struct MsgConvertCoinResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type for Erc20 parameters.
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/vm parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgRegisterERC20 is the Msg/RegisterERC20 request type for registering
/// an Erc20 contract token pair.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgRegisterERC20")]
pub struct MsgRegisterErc20 {
    /// signer is the address registering the erc20 pairs
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// erc20addresses is a slice of ERC20 token contract hex addresses
    #[prost(string, repeated, tag = "2")]
    pub erc20addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgRegisterERC20Response defines the response structure for executing a
/// MsgRegisterERC20 message.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgRegisterERC20Response")]
pub struct MsgRegisterErc20Response {}
/// MsgToggleConversion is the Msg/MsgToggleConversion request type for toggling
/// an Erc20 contract conversion capability.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgToggleConversion")]
pub struct MsgToggleConversion {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// token identifier can be either the hex contract address of the ERC20 or the
    /// Cosmos base denomination
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
/// MsgToggleConversionResponse defines the response structure for executing a
/// ToggleConversion message.
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
#[proto_message(type_url = "/cosmos.evm.erc20.v1.MsgToggleConversionResponse")]
pub struct MsgToggleConversionResponse {}
pub struct Erc20Querier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> Erc20Querier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn token_pairs(
        &self,
        pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryTokenPairsResponse, cosmwasm_std::StdError> {
        QueryTokenPairsRequest { pagination }.query(self.querier)
    }
    pub fn token_pair(
        &self,
        token: ::prost::alloc::string::String,
    ) -> Result<QueryTokenPairResponse, cosmwasm_std::StdError> {
        QueryTokenPairRequest { token }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
