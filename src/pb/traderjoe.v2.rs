// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub token_x: ::core::option::Option<Token>,
    #[prost(message, optional, tag="3")]
    pub token_y: ::core::option::Option<Token>,
    #[prost(string, tag="4")]
    pub bin_step: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(uint64, tag="12")]
    pub timestamp: u64,
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
pub struct Swap {
    #[prost(string, tag="1")]
    pub pair_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amounts_in: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amounts_out: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub block_number: u64,
    #[prost(uint64, tag="12")]
    pub timestamp: u64,
    #[prost(uint32, tag="13")]
    pub log_index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swaps {
    #[prost(message, repeated, tag="1")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    #[prost(string, tag="1")]
    pub time: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub period: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub last_block: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub open: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub close: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub token0_amount_traded: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub token1_amount_traded: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candles {
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<Candle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub decimal: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
// @@protoc_insertion_point(module)
