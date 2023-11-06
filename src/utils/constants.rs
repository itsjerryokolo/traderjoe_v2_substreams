use hex_literal::hex;

// Contracts
pub const DEXCANDLES_FACTORY: [u8; 20] = hex!("DC8d77b69155c7E68A95a4fb0f06a71FF90B943a");

pub const CANDLESTICK_PERIODS: [i32; 6] = [
    5 * 60,           // 5m
    15 * 60,          // 15m
    60 * 60,          // 1h
    4 * 60 * 60,      // 4h
    24 * 60 * 60,     // 1d
    7 * 24 * 60 * 60, // 1w
];
