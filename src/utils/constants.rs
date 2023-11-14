use hex_literal::hex;
use substreams::scalar::{BigDecimal, BigInt};

// Contracts
pub const DEXCANDLES_FACTORY: [u8; 20] = hex!("DC8d77b69155c7E68A95a4fb0f06a71FF90B943a");
pub const DEX_LENS: [u8; 20] = hex!("034389902aD4772c03102a3414EC71901ef2a5ad");

pub const CANDLESTICK_PERIODS: [i32; 6] = [
    5 * 60,           // 5m
    15 * 60,          // 15m
    60 * 60,          // 1h
    4 * 60 * 60,      // 4h
    24 * 60 * 60,     // 1d
    7 * 24 * 60 * 60, // 1w
];

pub const BIG_DECIMAL_1E6: BigDecimal = BigDecimal::from(1_000_000);
pub const BIG_DECIMAL_1E10: BigDecimal = BigDecimal::from(10_000_000_000);
pub const BIG_DECIMAL_1E12: BigDecimal = BigDecimal::from(1_000_000_000_000);
pub const BIG_DECIMAL_1E18: BigDecimal = BigDecimal::from(1_000_000_000_000_000_000);
pub const BIG_DECIMAL_ZERO: BigDecimal = BigDecimal::from(0);
pub const BIG_DECIMAL_ONE: BigDecimal = BigDecimal::from(1);
