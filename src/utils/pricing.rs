use bigdecimal::ToPrimitive;
use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};

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
