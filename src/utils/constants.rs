use hex_literal::hex;
use lazy_static::lazy_static;

use std::str::FromStr;
use substreams::scalar::{BigDecimal, BigInt};

// Contracts
pub const DEXCANDLES_FACTORY: [u8; 20] = hex!("1886D09C9Ade0c5DB822D85D21678Db67B6c2982");
pub const DEX_LENS: [u8; 20] = hex!("f450749aeA1c5feF27Ae0237C56FecC43f6bE244");
pub const WAVAX_ADDRESS: [u8; 20] = hex!("82aF49447D8a07e3bd95BD0d56f35241523fBab1");

pub const CANDLESTICK_PERIODS: [i32; 6] = [
    5 * 60,           // 5m
    15 * 60,          // 15m
    60 * 60,          // 1h
    4 * 60 * 60,      // 4h
    24 * 60 * 60,     // 1d
    7 * 24 * 60 * 60, // 1w
];

lazy_static! {
    pub static ref JOE_DEX_LENS_USD_DECIMALS: BigDecimal =
        BigDecimal::from_str("1000000000000000000").unwrap();
    pub static ref BIG_DECIMAL_1E6: BigDecimal = BigDecimal::from_str("1000000").unwrap();
    pub static ref BIG_DECIMAL_1E10: BigDecimal = BigDecimal::from_str("10000000000").unwrap();
    pub static ref BIG_DECIMAL_1E12: BigDecimal = BigDecimal::from_str("1000000000000").unwrap();
    pub static ref BIG_DECIMAL_1E18: BigDecimal =
        BigDecimal::from_str("1000000000000000000").unwrap();
    pub static ref BIG_DECIMAL_ZERO: BigDecimal = BigDecimal::from_str("0").unwrap();
    pub static ref BIG_DECIMAL_ONE: BigDecimal = BigDecimal::from_str("1").unwrap();
    pub static ref BIG_DECIMAL_HUNDRED: BigDecimal = BigDecimal::from_str("100").unwrap();
    pub static ref BIG_ZERO: BigInt = BigInt::zero();
    pub static ref BIG_INT_ONE: BigInt = BigInt::from_str("1").unwrap();
}
