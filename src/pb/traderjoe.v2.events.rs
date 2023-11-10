// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryEvents {
    #[prost(message, repeated, tag="1")]
    pub fee_parameters_sets: ::prost::alloc::vec::Vec<FeeParametersSet>,
    #[prost(message, repeated, tag="2")]
    pub flash_loan_fee_sets: ::prost::alloc::vec::Vec<FlashLoanFeeSet>,
    #[prost(message, repeated, tag="3")]
    pub lb_pair_createds: ::prost::alloc::vec::Vec<LbPairCreated>,
    #[prost(message, repeated, tag="4")]
    pub lb_pair_ignored_state_changeds: ::prost::alloc::vec::Vec<LbPairIgnoredStateChanged>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeParametersSet {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bin_step: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub base_factor: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub filter_period: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub decay_period: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub reduction_factor: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub variable_fee_control: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub protocol_share: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub max_volatility_accumulator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlashLoanFeeSet {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub old_flash_loan_fee: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub new_flash_loan_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token_x: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token_y: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub bin_step: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub pid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairIgnoredStateChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub lb_pair: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub ignored: bool,
}
/// Template Events
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateEvents {
    #[prost(message, repeated, tag="10")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
    #[prost(message, repeated, tag="7")]
    pub flash_loans: ::prost::alloc::vec::Vec<FlashLoan>,
    #[prost(message, repeated, tag="4")]
    pub composition_fees: ::prost::alloc::vec::Vec<CompositionFee>,
    #[prost(message, repeated, tag="5")]
    pub deposited_to_bins: ::prost::alloc::vec::Vec<DepositedToBin>,
    #[prost(message, repeated, tag="13")]
    pub withdrawn_from_bins: ::prost::alloc::vec::Vec<WithdrawnFromBin>,
    #[prost(message, repeated, tag="6")]
    pub fees_collected: ::prost::alloc::vec::Vec<FeesCollected>,
    #[prost(message, repeated, tag="9")]
    pub protocol_fees_collected: ::prost::alloc::vec::Vec<ProtocolFeesCollected>,
    #[prost(message, repeated, tag="12")]
    pub transfer_singles: ::prost::alloc::vec::Vec<TransferSingle>,
    #[prost(message, repeated, tag="11")]
    pub transfer_batches: ::prost::alloc::vec::Vec<TransferBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositionFee {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub id: u64,
    #[prost(uint64, tag="4")]
    pub fees_x: u64,
    #[prost(uint64, tag="5")]
    pub fees_y: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositedToBin {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub id: u64,
    #[prost(uint64, tag="4")]
    pub amount_x: u64,
    #[prost(uint64, tag="5")]
    pub amount_y: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeesCollected {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub amount_x: u64,
    #[prost(uint64, tag="4")]
    pub amount_y: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlashLoan {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub receiver: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub amount: u64,
    #[prost(uint64, tag="5")]
    pub fee: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolFeesCollected {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub amount_x: u64,
    #[prost(uint64, tag="4")]
    pub amount_y: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub swap_for_y: bool,
    #[prost(uint64, tag="5")]
    pub amount_in: u64,
    #[prost(uint64, tag="6")]
    pub amount_out: u64,
    #[prost(uint64, tag="7")]
    pub volatility_accumulated: u64,
    #[prost(uint64, tag="8")]
    pub fees: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferBatch {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="4")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferSingle {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub id: u64,
    #[prost(uint64, tag="5")]
    pub amount: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawnFromBin {
    #[prost(bytes="vec", tag="1")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub id: u64,
    #[prost(uint64, tag="4")]
    pub amount_x: u64,
    #[prost(uint64, tag="5")]
    pub amount_y: u64,
    #[prost(string, tag="10")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="11")]
    pub evt_index: u32,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
// @@protoc_insertion_point(module)
