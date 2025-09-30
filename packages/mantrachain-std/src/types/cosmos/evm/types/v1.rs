use neutron_std_derive::CosmwasmExt;
/// ExtensionOptionDynamicFeeTx is an extension option that specifies the
/// maxPrioPrice for cosmos tx
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
#[proto_message(type_url = "/cosmos.evm.types.v1.ExtensionOptionDynamicFeeTx")]
pub struct ExtensionOptionDynamicFeeTx {
    /// max_priority_price is the same as `max_priority_fee_per_gas` in eip-1559
    /// spec
    #[prost(string, tag = "1")]
    pub max_priority_price: ::prost::alloc::string::String,
}
/// TxResult is the value stored in eth tx indexer
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
#[proto_message(type_url = "/cosmos.evm.types.v1.TxResult")]
pub struct TxResult {
    /// height of the blockchain
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// tx_index of the cosmos transaction
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tx_index: u32,
    /// msg_index in a batch transaction
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub msg_index: u32,
    /// eth_tx_index is the index in the list of valid eth tx in the block,
    /// aka. the transaction list returned by eth_getBlock api.
    #[prost(int32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub eth_tx_index: i32,
    /// failed is true if the eth transaction did not go succeed
    #[prost(bool, tag = "5")]
    pub failed: bool,
    /// gas_used by the transaction. If it exceeds the block gas limit,
    /// it's set to gas limit, which is what's actually deducted by ante handler.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: u64,
    /// cumulative_gas_used specifies the cumulated amount of gas used for all
    /// processed messages within the current batch transaction.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub cumulative_gas_used: u64,
}
/// ExtensionOptionsWeb3Tx is an extension option that specifies the typed chain
/// id, the fee payer as well as its signature data.
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
#[proto_message(type_url = "/cosmos.evm.types.v1.ExtensionOptionsWeb3Tx")]
pub struct ExtensionOptionsWeb3Tx {
    /// typed_data_chain_id is used only in EIP712 Domain and should match
    /// Ethereum network ID in a Web3 provider (e.g. Metamask).
    #[prost(uint64, tag = "1")]
    #[serde(alias = "typed_data_chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub typed_data_chain_id: u64,
    /// fee_payer is an account address for the fee payer. It will be validated
    /// during EIP712 signature checking.
    #[prost(string, tag = "2")]
    pub fee_payer: ::prost::alloc::string::String,
    /// fee_payer_sig is a signature data from the fee paying account,
    /// allows to perform fee delegation when using EIP712 Domain.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub fee_payer_sig: ::prost::alloc::vec::Vec<u8>,
}
