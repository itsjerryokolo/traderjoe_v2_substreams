mod abi;
mod events;
mod pb;
mod stores;
mod utils;

use std::str::FromStr;

use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use substreams::{
    pb::substreams::store_delta::Operation,
    scalar::{BigDecimal, BigInt},
};

pub use events::*;
pub use stores::*;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn graph_out(// tokens: Deltas<DeltaProto<traderjoe_v2::Token>>,
    // candles: Deltas<DeltaProto<traderjoe_v2::Candle>>,
) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for delta in tokens.deltas {
        let token_address = delta.key.as_str().split(":").last().unwrap();

        match delta.operation {
            Operation::Create => {
                tables
                    .create_row("Token", token_address)
                    .set("symbol", &delta.new_value.symbol)
                    .set("name", delta.new_value.name)
                    .set(
                        "decimals",
                        BigInt::from_str(&delta.new_value.decimal).unwrap(),
                    );
            }
            Operation::Update => {}
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
    }

    for delta in pairs.deltas {
        let pair_address = delta.key.as_str().split(":").last().unwrap();

        match delta.operation {
            Operation::Create => {
                tables
                    .create_row("Pair", pair_address)
                    .set("tokenX", &delta.new_value.token_x.unwrap().address)
                    .set("tokenY", delta.new_value.token_y.unwrap().address)
                    .set(
                        "binStep",
                        BigInt::from_str(&delta.new_value.bin_step).unwrap(),
                    );
            }
            Operation::Update => {}
            Operation::Delete => {}
            x => panic!("unsupported operation {:?}", x),
        };
    }

    for delta in candles.deltas {
        let candle_id = delta.key.as_str().split(":").last().unwrap();

        match delta.operation {
            Operation::Create => {
                tables
                    .create_row("Candle", candle_id)
                    .set("time", BigInt::from_str(&delta.new_value.time).unwrap())
                    .set("period", BigInt::from_str(&delta.new_value.period).unwrap())
                    .set(
                        "lastBlock",
                        BigInt::from_str(&delta.new_value.last_block).unwrap(),
                    )
                    .set("token0", delta.new_value.token0)
                    .set("token1", delta.new_value.token1)
                    .set(
                        "token0TotalAmount",
                        BigInt::from_str(&delta.new_value.token0_amount_traded).unwrap(),
                    )
                    .set(
                        "token1TotalAmount",
                        BigInt::from_str(&delta.new_value.token1_amount_traded).unwrap(),
                    )
                    .set("high", BigDecimal::from_str(&delta.new_value.high).unwrap())
                    .set("open", BigDecimal::from_str(&delta.new_value.open).unwrap())
                    .set(
                        "close",
                        BigDecimal::from_str(&delta.new_value.close).unwrap(),
                    )
                    .set("low", BigDecimal::from_str(&delta.new_value.low).unwrap());
            }
            Operation::Update => {
                let high = if BigDecimal::from_str(&delta.new_value.open).unwrap()
                    > BigDecimal::from_str(&delta.new_value.high).unwrap()
                {
                    BigDecimal::from_str(&delta.new_value.open).unwrap()
                } else {
                    BigDecimal::from_str(&delta.old_value.high).unwrap()
                };
                let low = if BigDecimal::from_str(&delta.new_value.open).unwrap()
                    < BigDecimal::from_str(&delta.new_value.low).unwrap()
                {
                    BigDecimal::from_str(&delta.new_value.open).unwrap()
                } else {
                    BigDecimal::from_str(&delta.new_value.low).unwrap()
                };
                let old_token0_total_amount =
                    BigInt::from_str(&delta.old_value.token0_amount_traded).unwrap();
                let old_token1_total_amount =
                    BigInt::from_str(&delta.old_value.token1_amount_traded).unwrap();

                let new_token0_total_amount =
                    BigInt::from_str(&delta.new_value.token0_amount_traded).unwrap();
                let new_token1_total_amount =
                    BigInt::from_str(&delta.new_value.token1_amount_traded).unwrap();

                let token0_total_amount = old_token0_total_amount + new_token0_total_amount;
                let token1_total_amount = old_token1_total_amount + new_token1_total_amount;

                tables
                    .update_row("Candle", candle_id)
                    .set(
                        "close",
                        BigDecimal::from_str(&delta.new_value.close).unwrap(),
                    )
                    .set(
                        "lastBlock",
                        BigInt::from_str(&delta.new_value.last_block).unwrap(),
                    )
                    .set("token0TotalAmount", token0_total_amount)
                    .set("token1TotalAmount", token1_total_amount)
                    .set("high", high)
                    .set("low", low);
            }
            Operation::Delete => todo!(),
            x => panic!("unsupported operation {:?}", x),
        };
    }

    let entity_changes = tables.to_entity_changes();
    Ok(entity_changes)
}
