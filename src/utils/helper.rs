use std::str::FromStr;

use num_traits::Signed;
use substreams::log;

use num_traits::ToPrimitive;

use num_bigint::BigInt;

pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}

pub fn compare_tokens(token_x: &str, token_y: &str) -> bool {
    let value: bool = match token_x.cmp(&token_y) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Equal => {
            log::info!("TokenX : {}", &token_x.to_string());
            log::info!("TokenY : {}", &token_y.to_string());
            panic!("Shouldn't be equal")
        }
        std::cmp::Ordering::Greater => false,
    };
    value
}

pub fn get_sorted_token0(token_x: &str, token_y: &str) -> String {
    let value: bool = compare_tokens(token_x, token_y);
    if value == true {
        log::info!("Value : {}", &value.to_string());

        return token_x.to_string();
    } else {
        token_y.to_string()
    }
}

pub fn get_sorted_token1(token_x: &str, token_y: &str) -> String {
    let value: bool = compare_tokens(token_x, token_y);
    if value == true {
        return token_y.to_string();
    } else {
        token_x.to_string()
    }
}

pub fn get_sorted_amount0(
    token_x: &str,
    token_y: &str,
    amount_x_traded: &str,
    amount_y_traded: &str,
) -> String {
    let value: bool = compare_tokens(token_x, token_y);
    if value == true {
        return amount_x_traded.to_string();
    } else {
        amount_y_traded.to_string()
    }
}

pub fn get_sorted_amount1(
    token_x: &str,
    token_y: &str,
    amount_x_traded: &str,
    amount_y_traded: &str,
) -> String {
    let value: bool = compare_tokens(token_x, token_y);
    if value == true {
        return amount_y_traded.to_string();
    } else {
        amount_x_traded.to_string()
    }
}

pub fn bigint_to_i32(str: &str) -> i32 {
    BigInt::from_str(str).unwrap().to_i32().unwrap()
}

pub fn get_sorted_price(token_x: &str, token_y: &str, value_0: &str, value_1: &str) -> String {
    let value: bool = compare_tokens(token_x, token_y);
    if value == true {
        return value_0.to_string();
    } else {
        value_1.to_string()
    }
}

pub fn generate_key(name: &str, val: &str) -> String {
    return format!("{}:{}", name, val);
}

pub fn decode_x(packed_amounts: Vec<u8>) -> BigInt {
    let x: BigInt = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &packed_amounts);
    log::info!("x : {:?}", &x);

    let x_bit_and = &x & (BigInt::from(1) << 128) - 1;
    log::info!("x_bit_and : {:?}", &x_bit_and);

    x_bit_and
}

pub fn decode_y(packed_amounts: Vec<u8>) -> BigInt {
    let y = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &packed_amounts);
    log::info!("y : {:?}", &y);

    let y_rhs = BigInt::from(y >> 128);
    log::info!("y_rhs : {:?}", &y_rhs);

    y_rhs
}

pub fn get_amount_traded(amount_in: BigInt, amount_out: BigInt, token_decimals: i32) -> BigInt {
    let exponent = BigInt::from(18) - &token_decimals;

    if exponent >= BigInt::from(0) {
        let multiplier = BigInt::from(10).pow(exponent.to_u32().unwrap());

        return (&amount_in - &amount_out).abs() * multiplier;
    } else {
        let divider = BigInt::from(10).pow((-exponent).to_u32().unwrap());

        return (&amount_in - &amount_out).abs() / divider;
    }
}
