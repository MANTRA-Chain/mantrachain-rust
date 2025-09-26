use neutron_std_derive::CosmwasmExt;
/// EventEthereumTx defines the event for an Ethereum transaction
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EventEthereumTx")]
pub struct EventEthereumTx {
    /// amount
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    /// eth_hash is the Ethereum hash of the transaction
    #[prost(string, tag = "2")]
    pub eth_hash: ::prost::alloc::string::String,
    /// index of the transaction in the block
    #[prost(string, tag = "3")]
    pub index: ::prost::alloc::string::String,
    /// gas_used is the amount of gas used by the transaction
    #[prost(string, tag = "4")]
    pub gas_used: ::prost::alloc::string::String,
    /// hash is the CometBFT hash of the transaction
    #[prost(string, tag = "5")]
    pub hash: ::prost::alloc::string::String,
    /// recipient of the transaction
    #[prost(string, tag = "6")]
    pub recipient: ::prost::alloc::string::String,
    /// eth_tx_failed contains a VM error should it occur
    #[prost(string, tag = "7")]
    pub eth_tx_failed: ::prost::alloc::string::String,
}
/// EventTxLog defines the event for an Ethereum transaction log
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EventTxLog")]
pub struct EventTxLog {
    /// tx_logs is an array of transaction logs
    #[prost(string, repeated, tag = "1")]
    pub tx_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventMessage
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EventMessage")]
pub struct EventMessage {
    /// module which emits the event
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// sender of the message
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// tx_type is the type of the message
    #[prost(string, tag = "3")]
    pub tx_type: ::prost::alloc::string::String,
}
/// EventBlockBloom defines an Ethereum block bloom filter event
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EventBlockBloom")]
pub struct EventBlockBloom {
    /// bloom is the bloom filter of the block
    #[prost(string, tag = "1")]
    pub bloom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.Params")]
pub struct Params {
    /// evm_denom represents the token denomination used to run the EVM state
    /// transitions.
    #[prost(string, tag = "1")]
    pub evm_denom: ::prost::alloc::string::String,
    /// extra_eips defines the additional EIPs for the vm.Config
    #[prost(int64, repeated, packed = "false", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub extra_eips: ::prost::alloc::vec::Vec<i64>,
    /// allow_unprotected_txs defines if replay-protected (i.e non EIP155
    /// signed) transactions can be executed on the state machine.
    #[prost(bool, tag = "5")]
    pub allow_unprotected_txs: bool,
    /// evm_channels is the list of channel identifiers from EVM compatible chains
    #[prost(string, repeated, tag = "7")]
    pub evm_channels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// access_control defines the permission policy of the EVM
    #[prost(message, optional, tag = "8")]
    pub access_control: ::core::option::Option<AccessControl>,
    /// active_static_precompiles defines the slice of hex addresses of the
    /// precompiled contracts that are active
    #[prost(string, repeated, tag = "9")]
    pub active_static_precompiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub history_serve_window: u64,
}
/// AccessControl defines the permission policy of the EVM
/// for creating and calling contracts
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.AccessControl")]
pub struct AccessControl {
    /// create defines the permission policy for creating contracts
    #[prost(message, optional, tag = "1")]
    pub create: ::core::option::Option<AccessControlType>,
    /// call defines the permission policy for calling contracts
    #[prost(message, optional, tag = "2")]
    pub call: ::core::option::Option<AccessControlType>,
}
/// AccessControlType defines the permission type for policies
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.AccessControlType")]
pub struct AccessControlType {
    /// access_type defines which type of permission is required for the operation
    #[prost(enumeration = "AccessType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub access_type: i32,
    /// access_control_list defines defines different things depending on the
    /// AccessType:
    /// - ACCESS_TYPE_PERMISSIONLESS: list of addresses that are blocked from
    /// performing the operation
    /// - ACCESS_TYPE_RESTRICTED: ignored
    /// - ACCESS_TYPE_PERMISSIONED: list of addresses that are allowed to perform
    /// the operation
    #[prost(string, repeated, tag = "2")]
    pub access_control_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ChainConfig defines the Ethereum ChainConfig parameters using *sdk.Int values
/// instead of *big.Int.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.ChainConfig")]
pub struct ChainConfig {
    /// homestead_block switch (nil no fork, 0 = already homestead)
    #[prost(string, tag = "1")]
    pub homestead_block: ::prost::alloc::string::String,
    /// dao_fork_block corresponds to TheDAO hard-fork switch block (nil no fork)
    #[prost(string, tag = "2")]
    pub dao_fork_block: ::prost::alloc::string::String,
    /// dao_fork_support defines whether the nodes supports or opposes the DAO
    /// hard-fork
    #[prost(bool, tag = "3")]
    pub dao_fork_support: bool,
    /// eip150_block: EIP150 implements the Gas price changes
    /// (<https://github.com/ethereum/EIPs/issues/150>) EIP150 HF block (nil no fork)
    #[prost(string, tag = "4")]
    pub eip150_block: ::prost::alloc::string::String,
    /// eip155_block: EIP155Block HF block
    #[prost(string, tag = "6")]
    pub eip155_block: ::prost::alloc::string::String,
    /// eip158_block: EIP158 HF block
    #[prost(string, tag = "7")]
    pub eip158_block: ::prost::alloc::string::String,
    /// byzantium_block: Byzantium switch block (nil no fork, 0 = already on
    /// byzantium)
    #[prost(string, tag = "8")]
    pub byzantium_block: ::prost::alloc::string::String,
    /// constantinople_block: Constantinople switch block (nil no fork, 0 = already
    /// activated)
    #[prost(string, tag = "9")]
    pub constantinople_block: ::prost::alloc::string::String,
    /// petersburg_block: Petersburg switch block (nil same as Constantinople)
    #[prost(string, tag = "10")]
    pub petersburg_block: ::prost::alloc::string::String,
    /// istanbul_block: Istanbul switch block (nil no fork, 0 = already on
    /// istanbul)
    #[prost(string, tag = "11")]
    pub istanbul_block: ::prost::alloc::string::String,
    /// muir_glacier_block: Eip-2384 (bomb delay) switch block (nil no fork, 0 =
    /// already activated)
    #[prost(string, tag = "12")]
    pub muir_glacier_block: ::prost::alloc::string::String,
    /// berlin_block: Berlin switch block (nil = no fork, 0 = already on berlin)
    #[prost(string, tag = "13")]
    pub berlin_block: ::prost::alloc::string::String,
    /// london_block: London switch block (nil = no fork, 0 = already on london)
    #[prost(string, tag = "17")]
    pub london_block: ::prost::alloc::string::String,
    /// arrow_glacier_block: Eip-4345 (bomb delay) switch block (nil = no fork, 0 =
    /// already activated)
    #[prost(string, tag = "18")]
    pub arrow_glacier_block: ::prost::alloc::string::String,
    /// gray_glacier_block: EIP-5133 (bomb delay) switch block (nil = no fork, 0 =
    /// already activated)
    #[prost(string, tag = "20")]
    pub gray_glacier_block: ::prost::alloc::string::String,
    /// merge_netsplit_block: Virtual fork after The Merge to use as a network
    /// splitter
    #[prost(string, tag = "21")]
    pub merge_netsplit_block: ::prost::alloc::string::String,
    /// chain_id is the id of the chain (EIP-155)
    #[prost(uint64, tag = "24")]
    #[serde(alias = "chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub chain_id: u64,
    /// denom is the denomination used on the EVM
    #[prost(string, tag = "25")]
    pub denom: ::prost::alloc::string::String,
    /// decimals is the real decimal precision of the denomination used on the EVM
    #[prost(uint64, tag = "26")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub decimals: u64,
    /// shanghai_time: Shanghai switch time (nil = no fork, 0 = already on
    /// shanghai)
    #[prost(string, tag = "27")]
    pub shanghai_time: ::prost::alloc::string::String,
    /// cancun_time: Cancun switch time (nil = no fork, 0 = already on cancun)
    #[prost(string, tag = "28")]
    pub cancun_time: ::prost::alloc::string::String,
    /// prague_time: Prague switch time (nil = no fork, 0 = already on prague)
    #[prost(string, tag = "29")]
    pub prague_time: ::prost::alloc::string::String,
    /// verkle_time: Verkle switch time (nil = no fork, 0 = already on verkle)
    #[prost(string, tag = "30")]
    pub verkle_time: ::prost::alloc::string::String,
    /// osaka_time: Osaka switch time (nil = no fork, 0 = already on osaka)
    #[prost(string, tag = "31")]
    pub osaka_time: ::prost::alloc::string::String,
}
/// State represents a single Storage key value pair item.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.State")]
pub struct State {
    /// key is the stored key
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// value is the stored value for the given key
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// TransactionLogs define the logs generated from a transaction execution
/// with a given hash. It it used for import/export data as transactions are not
/// persisted on blockchain state after an upgrade.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.TransactionLogs")]
pub struct TransactionLogs {
    /// hash of the transaction
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// logs is an array of Logs for the given transaction hash
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
}
/// Log represents an protobuf compatible Ethereum Log that defines a contract
/// log event. These events are generated by the LOG opcode and stored/indexed by
/// the node.
///
/// NOTE: address, topics and data are consensus fields. The rest of the fields
/// are derived, i.e. filled in by the nodes, but not secured by consensus.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.Log")]
pub struct Log {
    /// address of the contract that generated the event
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// topics is a list of topics provided by the contract.
    #[prost(string, repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// data which is supplied by the contract, usually ABI-encoded
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block_number of the block in which the transaction was included
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_number: u64,
    /// tx_hash is the transaction hash
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
    /// tx_index of the transaction in the block
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tx_index: u64,
    /// block_hash of the block in which the transaction was included
    #[prost(string, tag = "7")]
    pub block_hash: ::prost::alloc::string::String,
    /// index of the log in the block
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
    /// removed is true if this log was reverted due to a chain
    /// reorganisation. You must pay attention to this field if you receive logs
    /// through a filter query.
    #[prost(bool, tag = "9")]
    pub removed: bool,
}
/// TxResult stores results of Tx execution.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.TxResult")]
pub struct TxResult {
    /// contract_address contains the ethereum address of the created contract (if
    /// any). If the state transition is an evm.Call, the contract address will be
    /// empty.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// bloom represents the bloom filter bytes
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub bloom: ::prost::alloc::vec::Vec<u8>,
    /// tx_logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, optional, tag = "3")]
    pub tx_logs: ::core::option::Option<TransactionLogs>,
    /// ret defines the bytes from the execution.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// reverted flag is set to true when the call has been reverted
    #[prost(bool, tag = "5")]
    pub reverted: bool,
    /// gas_used notes the amount of gas consumed while execution
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: u64,
}
/// AccessTuple is the element type of an access list.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.AccessTuple")]
pub struct AccessTuple {
    /// address is a hex formatted ethereum address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// storage_keys are hex formatted hashes of the storage keys
    #[prost(string, repeated, tag = "2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TraceConfig holds extra parameters to trace functions.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.TraceConfig")]
pub struct TraceConfig {
    /// tracer is a custom javascript tracer
    #[prost(string, tag = "1")]
    pub tracer: ::prost::alloc::string::String,
    /// timeout overrides the default timeout of 5 seconds for JavaScript-based
    /// tracing calls
    #[prost(string, tag = "2")]
    pub timeout: ::prost::alloc::string::String,
    /// reexec defines the number of blocks the tracer is willing to go back
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reexec: u64,
    /// disable_stack switches stack capture
    #[prost(bool, tag = "5")]
    pub disable_stack: bool,
    /// disable_storage switches storage capture
    #[prost(bool, tag = "6")]
    pub disable_storage: bool,
    /// debug can be used to print output during capture end
    #[prost(bool, tag = "8")]
    pub debug: bool,
    /// limit defines the maximum length of output, but zero means unlimited
    #[prost(int32, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub limit: i32,
    /// overrides can be used to execute a trace using future fork rules
    #[prost(message, optional, tag = "10")]
    pub overrides: ::core::option::Option<ChainConfig>,
    /// enable_memory switches memory capture
    #[prost(bool, tag = "11")]
    pub enable_memory: bool,
    /// enable_return_data switches the capture of return data
    #[prost(bool, tag = "12")]
    pub enable_return_data: bool,
    /// tracer_json_config configures the tracer using a JSON string
    #[prost(string, tag = "13")]
    pub tracer_json_config: ::prost::alloc::string::String,
}
/// Preinstall defines a contract that is preinstalled on-chain with a specific
/// contract address and bytecode
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.Preinstall")]
pub struct Preinstall {
    /// name of the preinstall contract
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// address in hex format of the preinstall contract
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// code in hex format for the preinstall contract
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
}
/// AccessType defines the types of permissions for the operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AccessType {
    /// ACCESS_TYPE_PERMISSIONLESS does not restrict the operation to anyone
    Permissionless = 0,
    /// ACCESS_TYPE_RESTRICTED restrict the operation to anyone
    Restricted = 1,
    /// ACCESS_TYPE_PERMISSIONED only allows the operation for specific addresses
    Permissioned = 2,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Permissionless => "ACCESS_TYPE_PERMISSIONLESS",
            AccessType::Restricted => "ACCESS_TYPE_RESTRICTED",
            AccessType::Permissioned => "ACCESS_TYPE_PERMISSIONED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TYPE_PERMISSIONLESS" => Some(Self::Permissionless),
            "ACCESS_TYPE_RESTRICTED" => Some(Self::Restricted),
            "ACCESS_TYPE_PERMISSIONED" => Some(Self::Permissioned),
            _ => None,
        }
    }
}
/// GenesisState defines the evm module's genesis state.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.GenesisState")]
pub struct GenesisState {
    /// accounts is an array containing the ethereum genesis accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccount>,
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
    /// preinstalls defines a set of predefined contracts
    #[prost(message, repeated, tag = "3")]
    pub preinstalls: ::prost::alloc::vec::Vec<Preinstall>,
}
/// GenesisAccount defines an account to be initialized in the genesis state.
/// Its main difference between with Geth's GenesisAccount is that it uses a
/// custom storage type and that it doesn't contain the private key field.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.GenesisAccount")]
pub struct GenesisAccount {
    /// address defines an ethereum hex formated address of an account
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// code defines the hex bytes of the account code.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// storage defines the set of state key values for the account.
    #[prost(message, repeated, tag = "3")]
    pub storage: ::prost::alloc::vec::Vec<State>,
}
/// MsgEthereumTx encapsulates an Ethereum transaction as an SDK message.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgEthereumTx")]
pub struct MsgEthereumTx {
    /// from is the bytes of ethereum signer address. This address value is checked
    /// against the address derived from the signature (V, R, S) using the
    /// secp256k1 elliptic curve
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub from: ::prost::alloc::vec::Vec<u8>,
    /// raw is the raw ethereum transaction
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
/// ExtensionOptionsEthereumTx is an extension option for ethereum transactions
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.ExtensionOptionsEthereumTx")]
pub struct ExtensionOptionsEthereumTx {}
/// MsgEthereumTxResponse defines the Msg/EthereumTx response type.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgEthereumTxResponse")]
pub struct MsgEthereumTxResponse {
    /// hash of the ethereum transaction in hex format. This hash differs from the
    /// CometBFT sha256 hash of the transaction bytes. See
    /// <https://github.com/tendermint/tendermint/issues/6539> for reference
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// logs contains the transaction hash and the proto-compatible ethereum
    /// logs.
    #[prost(message, repeated, tag = "2")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// ret is the returned data from evm function (result or data supplied with
    /// revert opcode)
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// vm_error is the error returned by vm execution
    #[prost(string, tag = "4")]
    pub vm_error: ::prost::alloc::string::String,
    /// gas_used specifies how much gas was consumed by the transaction
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_used: u64,
}
/// MsgUpdateParams defines a Msg for updating the x/vm module parameters.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgUpdateParams")]
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgRegisterPreinstalls defines a Msg for creating preinstalls in evm state.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgRegisterPreinstalls")]
pub struct MsgRegisterPreinstalls {
    /// authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// preinstalls defines the preinstalls to create.
    #[prost(message, repeated, tag = "2")]
    pub preinstalls: ::prost::alloc::vec::Vec<Preinstall>,
}
/// MsgRegisterPreinstallsResponse defines the response structure for executing a
/// MsgRegisterPreinstalls message.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.MsgRegisterPreinstallsResponse")]
pub struct MsgRegisterPreinstallsResponse {}
/// QueryConfigRequest defines the request type for querying the config
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryConfigRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/Config",
    response_type = QueryConfigResponse
)]
pub struct QueryConfigRequest {}
/// QueryConfigResponse returns the EVM config.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryConfigResponse")]
pub struct QueryConfigResponse {
    /// config is the evm configuration
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<ChainConfig>,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryAccountRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/Account",
    response_type = QueryAccountResponse
)]
pub struct QueryAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryAccountResponse")]
pub struct QueryAccountResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
    /// code_hash is the hex-formatted code bytes from the EOA.
    #[prost(string, tag = "2")]
    pub code_hash: ::prost::alloc::string::String,
    /// nonce is the account's sequence number.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nonce: u64,
}
/// QueryCosmosAccountRequest is the request type for the Query/CosmosAccount RPC
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryCosmosAccountRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/CosmosAccount",
    response_type = QueryCosmosAccountResponse
)]
pub struct QueryCosmosAccountRequest {
    /// address is the ethereum hex address to query the account for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCosmosAccountResponse is the response type for the Query/CosmosAccount
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryCosmosAccountResponse")]
pub struct QueryCosmosAccountResponse {
    /// cosmos_address is the cosmos address of the account.
    #[prost(string, tag = "1")]
    pub cosmos_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub account_number: u64,
}
/// QueryValidatorAccountRequest is the request type for the
/// Query/ValidatorAccount RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryValidatorAccountRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/ValidatorAccount",
    response_type = QueryValidatorAccountResponse
)]
pub struct QueryValidatorAccountRequest {
    /// cons_address is the validator cons address to query the account for.
    #[prost(string, tag = "1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QueryValidatorAccountResponse is the response type for the
/// Query/ValidatorAccount RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryValidatorAccountResponse")]
pub struct QueryValidatorAccountResponse {
    /// account_address is the cosmos address of the account in bech32 format.
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
    /// sequence is the account's sequence number.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
    /// account_number is the account number
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub account_number: u64,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryBalanceRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// address is the ethereum hex address to query the balance for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// balance is the balance of the EVM denomination.
    #[prost(string, tag = "1")]
    pub balance: ::prost::alloc::string::String,
}
/// QueryStorageRequest is the request type for the Query/Storage RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryStorageRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/Storage",
    response_type = QueryStorageResponse
)]
pub struct QueryStorageRequest {
    /// address is the ethereum hex address to query the storage state for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// key defines the key of the storage state
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryStorageResponse is the response type for the Query/Storage RPC
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryStorageResponse")]
pub struct QueryStorageResponse {
    /// value defines the storage state value hash associated with the given key.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// QueryCodeRequest is the request type for the Query/Code RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryCodeRequest")]
#[proto_query(path = "/cosmos.evm.vm.v1.Query/Code", response_type = QueryCodeResponse)]
pub struct QueryCodeRequest {
    /// address is the ethereum hex address to query the code for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryCodeResponse is the response type for the Query/Code RPC
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryCodeResponse")]
pub struct QueryCodeResponse {
    /// code represents the code bytes from an ethereum address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub code: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTxLogsRequest is the request type for the Query/TxLogs RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTxLogsRequest")]
pub struct QueryTxLogsRequest {
    /// hash is the ethereum transaction hex hash to query the logs for.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageRequest>,
}
/// QueryTxLogsResponse is the response type for the Query/TxLogs RPC method.
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTxLogsResponse")]
pub struct QueryTxLogsResponse {
    /// logs represents the ethereum logs generated from the given transaction.
    #[prost(message, repeated, tag = "1")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/Params",
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// EthCallRequest defines EthCall request
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EthCallRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/EthCall",
    response_type = MsgEthereumTxResponse
)]
pub struct EthCallRequest {
    /// args uses the same json format as the json rpc api.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub args: ::prost::alloc::vec::Vec<u8>,
    /// gas_cap defines the default gas cap to be used
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas_cap: u64,
    /// proposer_address of the requested block in hex format
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "4")]
    #[serde(alias = "chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub chain_id: i64,
}
/// EstimateGasResponse defines EstimateGas response
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.EstimateGasResponse")]
pub struct EstimateGasResponse {
    /// gas returns the estimated gas
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub gas: u64,
    /// ret is the returned data from evm function (result or data supplied with
    /// revert opcode)
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub ret: ::prost::alloc::vec::Vec<u8>,
    /// vm_error is the error returned by vm execution
    #[prost(string, tag = "3")]
    pub vm_error: ::prost::alloc::string::String,
}
/// QueryTraceTxRequest defines TraceTx request
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTraceTxRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/TraceTx",
    response_type = QueryTraceTxResponse
)]
pub struct QueryTraceTxRequest {
    /// msg is the MsgEthereumTx for the requested transaction
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// predecessors is an array of transactions included in the same block
    /// need to be replayed first to get correct context for tracing.
    #[prost(message, repeated, tag = "4")]
    pub predecessors: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// block_number of requested transaction
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_number: i64,
    /// block_hash of requested transaction
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of requested transaction
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<crate::shim::Timestamp>,
    /// proposer_address is the proposer of the requested block
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "9")]
    #[serde(alias = "chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub chain_id: i64,
    /// block_max_gas of the block of the requested transaction
    #[prost(int64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_max_gas: i64,
}
/// QueryTraceTxResponse defines TraceTx response
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTraceTxResponse")]
pub struct QueryTraceTxResponse {
    /// data is the response serialized in bytes
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTraceBlockRequest defines TraceTx request
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTraceBlockRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/TraceBlock",
    response_type = QueryTraceBlockResponse
)]
pub struct QueryTraceBlockRequest {
    /// txs is an array of messages in the block
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<MsgEthereumTx>,
    /// trace_config holds extra parameters to trace functions.
    #[prost(message, optional, tag = "3")]
    pub trace_config: ::core::option::Option<TraceConfig>,
    /// block_number of the traced block
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_number: i64,
    /// block_hash (hex) of the traced block
    #[prost(string, tag = "6")]
    pub block_hash: ::prost::alloc::string::String,
    /// block_time of the traced block
    #[prost(message, optional, tag = "7")]
    pub block_time: ::core::option::Option<crate::shim::Timestamp>,
    /// proposer_address is the address of the requested block
    #[prost(bytes = "vec", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    /// chain_id is the eip155 chain id parsed from the requested block header
    #[prost(int64, tag = "9")]
    #[serde(alias = "chainID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub chain_id: i64,
    /// block_max_gas of the traced block
    #[prost(int64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_max_gas: i64,
}
/// QueryTraceBlockResponse defines TraceBlock response
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryTraceBlockResponse")]
pub struct QueryTraceBlockResponse {
    /// data is the response serialized in bytes
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryBaseFeeRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/BaseFee",
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryBaseFeeResponse")]
pub struct QueryBaseFeeResponse {
    /// base_fee is the EIP1559 base fee
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// QueryGlobalMinGasPriceRequest defines the request type for querying the
/// GlobalMinGasPrice
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryGlobalMinGasPriceRequest")]
#[proto_query(
    path = "/cosmos.evm.vm.v1.Query/GlobalMinGasPrice",
    response_type = QueryGlobalMinGasPriceResponse
)]
pub struct QueryGlobalMinGasPriceRequest {}
/// QueryGlobalMinGasPriceResponse returns the GlobalMinGasPrice
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
#[proto_message(type_url = "/cosmos.evm.vm.v1.QueryGlobalMinGasPriceResponse")]
pub struct QueryGlobalMinGasPriceResponse {
    /// min_gas_price is the feemarket's min_gas_price
    #[prost(string, tag = "1")]
    pub min_gas_price: ::prost::alloc::string::String,
}
pub struct VmQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> VmQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn account(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryAccountResponse, cosmwasm_std::StdError> {
        QueryAccountRequest { address }.query(self.querier)
    }
    pub fn cosmos_account(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryCosmosAccountResponse, cosmwasm_std::StdError> {
        QueryCosmosAccountRequest { address }.query(self.querier)
    }
    pub fn validator_account(
        &self,
        cons_address: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorAccountResponse, cosmwasm_std::StdError> {
        QueryValidatorAccountRequest { cons_address }.query(self.querier)
    }
    pub fn balance(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest { address }.query(self.querier)
    }
    pub fn storage(
        &self,
        address: ::prost::alloc::string::String,
        key: ::prost::alloc::string::String,
    ) -> Result<QueryStorageResponse, cosmwasm_std::StdError> {
        QueryStorageRequest { address, key }.query(self.querier)
    }
    pub fn code(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryCodeResponse, cosmwasm_std::StdError> {
        QueryCodeRequest { address }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn eth_call(
        &self,
        args: ::prost::alloc::vec::Vec<u8>,
        gas_cap: u64,
        proposer_address: ::prost::alloc::vec::Vec<u8>,
        chain_id: i64,
    ) -> Result<MsgEthereumTxResponse, cosmwasm_std::StdError> {
        EthCallRequest {
            args,
            gas_cap,
            proposer_address,
            chain_id,
        }
        .query(self.querier)
    }
    pub fn estimate_gas(
        &self,
        args: ::prost::alloc::vec::Vec<u8>,
        gas_cap: u64,
        proposer_address: ::prost::alloc::vec::Vec<u8>,
        chain_id: i64,
    ) -> Result<EstimateGasResponse, cosmwasm_std::StdError> {
        EthCallRequest {
            args,
            gas_cap,
            proposer_address,
            chain_id,
        }
        .query(self.querier)
    }
    pub fn trace_tx(
        &self,
        msg: ::core::option::Option<MsgEthereumTx>,
        trace_config: ::core::option::Option<TraceConfig>,
        predecessors: ::prost::alloc::vec::Vec<MsgEthereumTx>,
        block_number: i64,
        block_hash: ::prost::alloc::string::String,
        block_time: ::core::option::Option<crate::shim::Timestamp>,
        proposer_address: ::prost::alloc::vec::Vec<u8>,
        chain_id: i64,
        block_max_gas: i64,
    ) -> Result<QueryTraceTxResponse, cosmwasm_std::StdError> {
        QueryTraceTxRequest {
            msg,
            trace_config,
            predecessors,
            block_number,
            block_hash,
            block_time,
            proposer_address,
            chain_id,
            block_max_gas,
        }
        .query(self.querier)
    }
    pub fn trace_block(
        &self,
        txs: ::prost::alloc::vec::Vec<MsgEthereumTx>,
        trace_config: ::core::option::Option<TraceConfig>,
        block_number: i64,
        block_hash: ::prost::alloc::string::String,
        block_time: ::core::option::Option<crate::shim::Timestamp>,
        proposer_address: ::prost::alloc::vec::Vec<u8>,
        chain_id: i64,
        block_max_gas: i64,
    ) -> Result<QueryTraceBlockResponse, cosmwasm_std::StdError> {
        QueryTraceBlockRequest {
            txs,
            trace_config,
            block_number,
            block_hash,
            block_time,
            proposer_address,
            chain_id,
            block_max_gas,
        }
        .query(self.querier)
    }
    pub fn base_fee(&self) -> Result<QueryBaseFeeResponse, cosmwasm_std::StdError> {
        QueryBaseFeeRequest {}.query(self.querier)
    }
    pub fn config(&self) -> Result<QueryConfigResponse, cosmwasm_std::StdError> {
        QueryConfigRequest {}.query(self.querier)
    }
    pub fn global_min_gas_price(
        &self,
    ) -> Result<QueryGlobalMinGasPriceResponse, cosmwasm_std::StdError> {
        QueryGlobalMinGasPriceRequest {}.query(self.querier)
    }
}
