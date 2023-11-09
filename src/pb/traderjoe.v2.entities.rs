// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bin_step: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(message, optional, tag="12")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag="13")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag="1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundle {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub avax_price_usd: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bundles {
    #[prost(message, repeated, tag="1")]
    pub bundles: ::prost::alloc::vec::Vec<Bundle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbFactory {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub pair_count: i64,
    #[prost(double, tag="3")]
    pub volume_usd: f64,
    #[prost(double, tag="4")]
    pub volume_avax: f64,
    #[prost(double, tag="5")]
    pub untracked_volume_usd: f64,
    #[prost(double, tag="6")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="7")]
    pub total_value_locked_avax: f64,
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
    #[prost(double, tag="13")]
    pub fees_usd: f64,
    #[prost(double, tag="14")]
    pub fees_avax: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbFactories {
    #[prost(message, repeated, tag="1")]
    pub lb_factories: ::prost::alloc::vec::Vec<LbFactory>,
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
    #[prost(double, tag="4")]
    pub volume_avax: f64,
    #[prost(double, tag="5")]
    pub volume_usd: f64,
    #[prost(double, tag="6")]
    pub untracked_volume_usd: f64,
    #[prost(double, tag="7")]
    pub total_value_locked_avax: f64,
    #[prost(double, tag="8")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="9")]
    pub fees_usd: f64,
    #[prost(int64, tag="10")]
    pub tx_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderJoeHourDatas {
    #[prost(message, repeated, tag="1")]
    pub trader_joe_hour_datas: ::prost::alloc::vec::Vec<TraderJoeHourData>,
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
    #[prost(double, tag="4")]
    pub volume_avax: f64,
    #[prost(double, tag="5")]
    pub volume_usd: f64,
    #[prost(double, tag="6")]
    pub untracked_volume_usd: f64,
    #[prost(double, tag="7")]
    pub total_value_locked_avax: f64,
    #[prost(double, tag="8")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="9")]
    pub fees_usd: f64,
    #[prost(int64, tag="10")]
    pub tx_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraderJoeDayDatas {
    #[prost(message, repeated, tag="1")]
    pub trader_joe_day_datas: ::prost::alloc::vec::Vec<TraderJoeDayData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub factory: ::core::option::Option<LbFactory>,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub decimals: i64,
    #[prost(int64, tag="6")]
    pub total_supply: i64,
    #[prost(double, tag="7")]
    pub volume: f64,
    #[prost(double, tag="8")]
    pub volume_usd: f64,
    #[prost(double, tag="9")]
    pub untracked_volume_usd: f64,
    #[prost(int64, tag="10")]
    pub tx_count: i64,
    #[prost(double, tag="11")]
    pub total_value_locked: f64,
    #[prost(double, tag="12")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="13")]
    pub derived_avax: f64,
    #[prost(double, tag="14")]
    pub fees_usd: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
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
    #[prost(double, tag="4")]
    pub volume: f64,
    #[prost(double, tag="5")]
    pub volume_avax: f64,
    #[prost(double, tag="6")]
    pub volume_usd: f64,
    #[prost(int64, tag="7")]
    pub tx_count: i64,
    #[prost(double, tag="8")]
    pub total_value_locked: f64,
    #[prost(double, tag="9")]
    pub total_value_locked_avax: f64,
    #[prost(double, tag="10")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="11")]
    pub price_usd: f64,
    #[prost(double, tag="12")]
    pub fees_usd: f64,
    #[prost(double, tag="13")]
    pub open: f64,
    #[prost(double, tag="14")]
    pub high: f64,
    #[prost(double, tag="15")]
    pub low: f64,
    #[prost(double, tag="16")]
    pub close: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHourDatas {
    #[prost(message, repeated, tag="1")]
    pub token_hour_datas: ::prost::alloc::vec::Vec<TokenHourData>,
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
    #[prost(double, tag="4")]
    pub volume: f64,
    #[prost(double, tag="5")]
    pub volume_avax: f64,
    #[prost(double, tag="6")]
    pub volume_usd: f64,
    #[prost(int64, tag="7")]
    pub tx_count: i64,
    #[prost(double, tag="8")]
    pub total_value_locked: f64,
    #[prost(double, tag="9")]
    pub total_value_locked_avax: f64,
    #[prost(double, tag="10")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="11")]
    pub price_usd: f64,
    #[prost(double, tag="12")]
    pub fees_usd: f64,
    #[prost(double, tag="13")]
    pub open: f64,
    #[prost(double, tag="14")]
    pub high: f64,
    #[prost(double, tag="15")]
    pub low: f64,
    #[prost(double, tag="16")]
    pub close: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenDayDatas {
    #[prost(message, repeated, tag="1")]
    pub token_day_datas: ::prost::alloc::vec::Vec<TokenDayData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SJoeDayData {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub date: i32,
    #[prost(double, tag="3")]
    pub amount_x: f64,
    #[prost(double, tag="4")]
    pub amount_y: f64,
    #[prost(double, tag="5")]
    pub collected_avax: f64,
    #[prost(double, tag="6")]
    pub collected_usd: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SJoeDayDatas {
    #[prost(message, repeated, tag="1")]
    pub sjoe_day_datas: ::prost::alloc::vec::Vec<SJoeDayData>,
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
    #[prost(double, tag="4")]
    pub base_fee_pct: f64,
    #[prost(message, optional, tag="5")]
    pub token_x: ::core::option::Option<Token>,
    #[prost(message, optional, tag="6")]
    pub token_y: ::core::option::Option<Token>,
    #[prost(int64, tag="7")]
    pub bin_step: i64,
    #[prost(int64, tag="8")]
    pub active_id: i64,
    #[prost(double, tag="9")]
    pub reserve_x: f64,
    #[prost(double, tag="10")]
    pub reserve_y: f64,
    #[prost(double, tag="11")]
    pub total_value_locked_avax: f64,
    #[prost(double, tag="12")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="13")]
    pub token_x_price: f64,
    #[prost(double, tag="14")]
    pub token_y_price: f64,
    #[prost(double, tag="15")]
    pub token_x_price_usd: f64,
    #[prost(double, tag="16")]
    pub token_y_price_usd: f64,
    #[prost(double, tag="17")]
    pub volume_token_x: f64,
    #[prost(double, tag="18")]
    pub volume_token_y: f64,
    #[prost(double, tag="19")]
    pub volume_usd: f64,
    #[prost(double, tag="20")]
    pub untracked_volume_usd: f64,
    #[prost(int64, tag="21")]
    pub tx_count: i64,
    #[prost(double, tag="22")]
    pub fees_token_x: f64,
    #[prost(double, tag="23")]
    pub fees_token_y: f64,
    #[prost(double, tag="24")]
    pub fees_usd: f64,
    #[prost(int64, tag="25")]
    pub liquidity_provider_count: i64,
    #[prost(message, optional, tag="26")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag="27")]
    pub block: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairs {
    #[prost(message, repeated, tag="1")]
    pub lb_pairs: ::prost::alloc::vec::Vec<LbPair>,
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
    #[prost(double, tag="11")]
    pub protocol_share_pct: f64,
    #[prost(int64, tag="12")]
    pub max_volatility_accumulated: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairParameterSets {
    #[prost(message, repeated, tag="1")]
    pub lb_pair_parameter_sets: ::prost::alloc::vec::Vec<LbPairParameterSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bin {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub lb_pair: ::core::option::Option<LbPair>,
    #[prost(double, tag="3")]
    pub price_x: f64,
    #[prost(double, tag="4")]
    pub price_y: f64,
    #[prost(int64, tag="5")]
    pub total_supply: i64,
    #[prost(double, tag="6")]
    pub reserve_x: f64,
    #[prost(double, tag="7")]
    pub reserve_y: f64,
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
    #[prost(message, optional, tag="4")]
    pub token_x: ::core::option::Option<Token>,
    #[prost(message, optional, tag="5")]
    pub token_y: ::core::option::Option<Token>,
    #[prost(double, tag="6")]
    pub reserve_x: f64,
    #[prost(double, tag="7")]
    pub reserve_y: f64,
    #[prost(double, tag="8")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="9")]
    pub volume_token_x: f64,
    #[prost(double, tag="10")]
    pub volume_token_y: f64,
    #[prost(double, tag="11")]
    pub volume_usd: f64,
    #[prost(double, tag="12")]
    pub untracked_volume_usd: f64,
    #[prost(int64, tag="13")]
    pub tx_count: i64,
    #[prost(double, tag="14")]
    pub fees_usd: f64,
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
    #[prost(message, optional, tag="4")]
    pub token_x: ::core::option::Option<Token>,
    #[prost(message, optional, tag="5")]
    pub token_y: ::core::option::Option<Token>,
    #[prost(double, tag="6")]
    pub reserve_x: f64,
    #[prost(double, tag="7")]
    pub reserve_y: f64,
    #[prost(double, tag="8")]
    pub total_value_locked_usd: f64,
    #[prost(double, tag="9")]
    pub volume_token_x: f64,
    #[prost(double, tag="10")]
    pub volume_token_y: f64,
    #[prost(double, tag="11")]
    pub volume_usd: f64,
    #[prost(double, tag="12")]
    pub untracked_volume_usd: f64,
    #[prost(int64, tag="13")]
    pub tx_count: i64,
    #[prost(double, tag="14")]
    pub fees_usd: f64,
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
    #[prost(double, tag="4")]
    pub accrued_fees_x: f64,
    #[prost(double, tag="5")]
    pub accrued_fees_y: f64,
    #[prost(double, tag="6")]
    pub accrued_fees_l: f64,
    #[prost(double, tag="7")]
    pub collected_fees_x: f64,
    #[prost(double, tag="8")]
    pub collected_fees_y: f64,
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
    #[prost(double, tag="5")]
    pub accrued_fees_x: f64,
    #[prost(double, tag="6")]
    pub accrued_fees_y: f64,
    #[prost(double, tag="7")]
    pub accrued_fees_l: f64,
    #[prost(double, tag="8")]
    pub collected_fees_x: f64,
    #[prost(double, tag="9")]
    pub collected_fees_y: f64,
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
pub struct Swap {
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
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub origin: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub active_id: i64,
    #[prost(double, tag="9")]
    pub amount_x_in: f64,
    #[prost(double, tag="10")]
    pub amount_y_in: f64,
    #[prost(double, tag="11")]
    pub amount_x_out: f64,
    #[prost(double, tag="12")]
    pub amount_y_out: f64,
    #[prost(double, tag="13")]
    pub amount_usd: f64,
    #[prost(double, tag="14")]
    pub fees_token_x: f64,
    #[prost(double, tag="15")]
    pub fees_token_y: f64,
    #[prost(double, tag="16")]
    pub fees_usd: f64,
    #[prost(int64, tag="17")]
    pub log_index: i64,
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
    #[prost(double, tag="9")]
    pub amount: f64,
    #[prost(double, tag="10")]
    pub amount_usd: f64,
    #[prost(double, tag="11")]
    pub fees: f64,
    #[prost(double, tag="12")]
    pub fees_usd: f64,
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
    #[prost(double, tag="5")]
    pub amount_x: f64,
    #[prost(double, tag="6")]
    pub amount_y: f64,
    #[prost(string, tag="7")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub origin: ::prost::alloc::vec::Vec<u8>,
    #[prost(double, tag="9")]
    pub collected_avax: f64,
    #[prost(double, tag="10")]
    pub collected_usd: f64,
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
