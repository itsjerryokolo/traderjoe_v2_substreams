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
    #[prost(uint64, tag="2")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="2")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="2")]
    pub evt_index: u64,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(message, optional, tag="5")]
    pub token_x: ::core::option::Option<Token>,
    #[prost(message, optional, tag="6")]
    pub token_y: ::core::option::Option<Token>,
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
    #[prost(uint64, tag="2")]
    pub evt_index: u64,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub lb_pair: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub ignored: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub factory_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub decimals: u64,
    #[prost(uint64, tag="6")]
    pub total_supply: u64,
    #[prost(string, optional, tag="7")]
    pub volume: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub volume_usd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub untracked_volume_usd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="10")]
    pub tx_count: ::core::option::Option<i64>,
    #[prost(string, optional, tag="11")]
    pub total_value_locked: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub total_value_locked_usd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub derived_avax: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub fees_usd: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
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
    #[prost(uint64, tag="11")]
    pub evt_index: u64,
    #[prost(message, optional, tag="12")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="13")]
    pub evt_block_number: u64,
}
/// // Entities
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub avax_price_usd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbFactory {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub pair_count: i64,
    #[prost(string, tag="3")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub volume_avax: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub tx_count: i64,
    #[prost(int64, tag="9")]
    pub token_count: i64,
    #[prost(int64, tag="10")]
    pub user_count: i64,
    #[prost(int64, tag="11")]
    pub flashloan_fee: i64,
    #[prost(message, repeated, tag="12")]
    pub ignored_lb_pairs: ::prost::alloc::vec::Vec<LbPair>,
    #[prost(string, tag="13")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub fees_avax: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderJoeHourData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub factory: ::core::option::Option<LbFactory>,
    #[prost(string, tag="4")]
    pub volume_avax: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="10")]
    pub tx_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderJoeDayData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub factory: ::core::option::Option<LbFactory>,
    #[prost(string, tag="4")]
    pub volume_avax: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="10")]
    pub tx_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHourData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<Token>,
    #[prost(string, tag="4")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub volume_avax: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub tx_count: i64,
    #[prost(string, tag="8")]
    pub total_value_locked: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub price_usd: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub close: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenDayData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<Token>,
    #[prost(string, tag="4")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub volume_avax: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub tx_count: i64,
    #[prost(string, tag="8")]
    pub total_value_locked: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub price_usd: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub close: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SJoeDayData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(string, tag="3")]
    pub amount_x: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount_y: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub collected_avax: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub collected_usd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPair {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub factory: ::core::option::Option<LbFactory>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub base_fee_pct: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub bin_step: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub active_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub total_value_locked_avax: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub token_x_price: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub token_y_price: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub token_x_price_usd: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub token_y_price_usd: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub volume_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub volume_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub tx_count: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub fees_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub fees_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(string, tag="25")]
    pub liquidity_provider_count: ::prost::alloc::string::String,
    #[prost(message, optional, tag="26")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="27")]
    pub block: u64,
    #[prost(uint64, tag="28")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairParameterSet {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(int64, tag="4")]
    pub bin_step: i64,
    #[prost(int64, tag="5")]
    pub base_factor: i64,
    #[prost(int64, tag="6")]
    pub filter_period: i64,
    #[prost(int64, tag="7")]
    pub decay_period: i64,
    #[prost(int64, tag="8")]
    pub reduction_factor: i64,
    #[prost(int64, tag="9")]
    pub variable_fee_control: i64,
    #[prost(int64, tag="10")]
    pub protocol_share: i64,
    #[prost(string, tag="11")]
    pub protocol_share_pct: ::prost::alloc::string::String,
    #[prost(int64, tag="12")]
    pub max_volatility_accumulated: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bin {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="3")]
    pub price_x: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub price_y: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub total_supply: i64,
    #[prost(string, tag="6")]
    pub reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reserve_y: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub bin_id: i64,
    #[prost(string, repeated, tag="9")]
    pub liquidity_providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag="10")]
    pub liquidity_provider_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bins {
    #[prost(message, repeated, tag="1")]
    pub bins: ::prost::alloc::vec::Vec<Bin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairDayData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="4")]
    pub token_x: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_y: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub volume_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub volume_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="13")]
    pub tx_count: i64,
    #[prost(string, tag="14")]
    pub fees_usd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairDayDatas {
    #[prost(message, repeated, tag="1")]
    pub lb_pair_day_datas: ::prost::alloc::vec::Vec<LbPairDayData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairHourData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(message, optional, tag="3")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="4")]
    pub token_x: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_y: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub total_value_locked_usd: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub volume_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub volume_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub volume_usd: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub untracked_volume_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="13")]
    pub tx_count: i64,
    #[prost(string, tag="14")]
    pub fees_usd: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairHourDatas {
    #[prost(message, repeated, tag="1")]
    pub lb_pair_hour_datas: ::prost::alloc::vec::Vec<LbPairHourData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityPosition {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub string: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(int64, tag="4")]
    pub bins_count: i64,
    #[prost(int32, tag="5")]
    pub block: i32,
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityPositions {
    #[prost(message, repeated, tag="1")]
    pub liquidity_positions: ::prost::alloc::vec::Vec<LiquidityPosition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserBinLiquidity {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_position: ::core::option::Option<LiquidityPosition>,
    #[prost(message, optional, tag="3")]
    pub lb_pair_bin_id: ::core::option::Option<Bin>,
    #[prost(message, optional, tag="4")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="5")]
    pub string: ::prost::alloc::string::String,
    #[prost(int64, tag="6")]
    pub bin_id: i64,
    #[prost(int64, tag="7")]
    pub liquidity: i64,
    #[prost(int32, tag="8")]
    pub block: i32,
    #[prost(message, optional, tag="9")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserBinLiquidities {
    #[prost(message, repeated, tag="1")]
    pub user_bin_liquidities: ::prost::alloc::vec::Vec<UserBinLiquidity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeesData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub string: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="4")]
    pub accrued_fees_x: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub accrued_fees_y: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub accrued_fees_l: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub collected_fees_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub collected_fees_y: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeesDatas {
    #[prost(message, repeated, tag="1")]
    pub user_fees_datas: ::prost::alloc::vec::Vec<UserFeesData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeesHourData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(string, tag="3")]
    pub string: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="5")]
    pub accrued_fees_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub accrued_fees_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub accrued_fees_l: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub collected_fees_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub collected_fees_y: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFeesHourDatas {
    #[prost(message, repeated, tag="1")]
    pub user_fees_hour_datas: ::prost::alloc::vec::Vec<UserFeesHourData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub block_number: i32,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swaps {
    #[prost(message, repeated, tag="1")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flash {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(bytes="vec", tag="5")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub origin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="8")]
    pub token: ::core::option::Option<Token>,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub amount_usd: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub fees: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub fees_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="13")]
    pub log_index: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flashes {
    #[prost(message, repeated, tag="1")]
    pub flashes: ::prost::alloc::vec::Vec<Flash>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collect {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(string, tag="5")]
    pub amount_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub amount_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub origin: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub collected_avax: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub collected_usd: ::prost::alloc::string::String,
    #[prost(int64, tag="11")]
    pub log_index: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collects {
    #[prost(message, repeated, tag="1")]
    pub collects: ::prost::alloc::vec::Vec<Collect>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag="4")]
    pub is_batch: bool,
    #[prost(int32, tag="5")]
    pub batch_index: i32,
    #[prost(bool, tag="6")]
    pub is_mint: bool,
    #[prost(bool, tag="7")]
    pub is_burn: bool,
    #[prost(message, optional, tag="8")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(int64, tag="9")]
    pub bin_id: i64,
    #[prost(int64, tag="10")]
    pub amount: i64,
    #[prost(bytes="vec", tag="11")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub origin: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="15")]
    pub log_index: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
// @@protoc_insertion_point(module)
