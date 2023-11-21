use bigdecimal::ToPrimitive;
use std::str::FromStr;

use crate::abi;
use substreams::scalar::{BigDecimal, BigInt};

use super::constants::BIG_DECIMAL_1E10;
use super::constants::DEX_LENS;
use super::constants::JOE_DEX_LENS_USD_DECIMALS;
use super::constants::WAVAX_ADDRESS;
use super::helper::str_to_bigdecimal;

pub fn get_price_y(
    bin_step: i32,
    bin_id: i32,
    token_x_decimals: i32,
    token_y_decimals: i32,
) -> BigDecimal {
    let basis_point_max: BigDecimal = BigDecimal::from_str("10000").unwrap();
    let bin_step: BigDecimal = BigDecimal::from(bin_step);
    let real_shift: i32 = 8388608;
    let big_decimal_one: BigDecimal = BigDecimal::from_str("1").unwrap();

    let bp_val = big_decimal_one + bin_step / basis_point_max;

    let loop_count = bin_id - real_shift;
    let is_positive = loop_count > 0;

    let mut result = BigDecimal::one();

    for _ in 0..loop_count.abs() {
        if is_positive {
            result = result * bp_val.clone()
        } else {
            result = result / bp_val.clone();
        }
    }

    let token_y_decimals = BigDecimal::new(BigInt::one(), token_y_decimals.to_i64().unwrap());
    let token_x_decimals = BigDecimal::new(BigInt::one(), token_x_decimals.to_i64().unwrap());

    result * token_x_decimals / token_y_decimals
}

pub fn get_token_price_in_avax(token_address: &Vec<u8>) -> BigDecimal {
    let price = abi::dexlens::functions::GetTokenPriceNative {
        u_token: token_address.to_vec(),
    };
    let token_price = price.call(DEX_LENS.to_vec()).unwrap_or(BigInt::zero());

    let price_in_avax = str_to_bigdecimal(&token_price.to_string()) / &*BIG_DECIMAL_1E10;
    price_in_avax
}

pub fn get_avax_price_in_usd() -> BigDecimal {
    let price = abi::dexlens::functions::GetTokenPriceUsd {
        u_token: WAVAX_ADDRESS.to_vec(),
    };
    let token_price = price.call(DEX_LENS.to_vec()).unwrap_or(BigInt::zero());

    let price_usd = str_to_bigdecimal(&token_price.to_string()) / &*JOE_DEX_LENS_USD_DECIMALS;
    price_usd
}

pub fn get_tracked_liquidity_usd(
    token_x_amount: BigDecimal,
    token_x: &Vec<u8>,
    token_y_amount: BigDecimal,
    token_y: &Vec<u8>,
    avax_price_usd: BigDecimal,
) -> BigDecimal {
    let price_x_usd = get_token_price_in_avax(token_x) * avax_price_usd;
    let price_y_usd = get_token_price_in_avax(token_y) * avax_price_usd;

    token_x_amount * price_x_usd + token_y_amount * price_y_usd
}

pub fn get_tracked_volume_usd(
    token_x_amount: BigDecimal,
    token_x: &Vec<u8>,
    token_y_amount: BigDecimal,
    token_y: &Vec<u8>,
    avax_price_usd: BigDecimal,
) -> BigDecimal {
    let price_x_usd = get_token_price_in_avax(token_x) * avax_price_usd;
    let price_y_usd = get_token_price_in_avax(token_y) * avax_price_usd;

    (token_x_amount * price_x_usd + token_y_amount * price_y_usd)
        / BigDecimal::from_str("2").unwrap()
}
