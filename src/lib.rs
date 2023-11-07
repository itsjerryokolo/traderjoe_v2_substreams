mod abi;
mod pb;
mod utils;

use std::str::FromStr;

use crate::utils::{
    helper::{
        append_0x, decode_x, decode_y, generate_key, get_sorted_amount0, get_sorted_amount1,
        get_sorted_price, get_sorted_token0, get_sorted_token1,
    },
    pricing::get_price_y,
    rpc::get_token_data,
};
use abi::{lb_factory as traderjoe_v2_factory_events, lb_pair as traderjoe_v2_pair_events};
use bigdecimal::ToPrimitive;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use pb::traderjoe::v2 as traderjoe_v2;
use substreams::{
    pb::substreams::{store_delta::Operation, StoreDelta},
    scalar::{BigDecimal, BigInt},
    store::{StoreNew, StoreSet},
    Hex,
};
use substreams_ethereum::{pb::eth, Event};

use substreams::{
    errors::Error,
    log,
    store::{StoreGet, StoreGetProto, StoreSetProto},
};
use utils::{
    constants::{CANDLESTICK_PERIODS, DEXCANDLES_FACTORY},
    helper::{bigint_to_i32, get_amount_traded},
};

#[substreams::handlers::map]
pub fn map_pairs_created(block: eth::v2::Block) -> Result<traderjoe_v2::Pairs, Error> {
    Ok(traderjoe_v2::Pairs {
        pairs: block
            .events::<traderjoe_v2_factory_events::events::LbPairCreated>(&[&DEXCANDLES_FACTORY])
            .map(|(pair, log)| {
                log::info!("New Pair Created 🚀🚀 ");

                let token_x_data = get_token_data(&pair.token_x);
                let token_y_data = get_token_data(&pair.token_y);

                traderjoe_v2::Pair {
                    address: append_0x(&Hex(pair.lb_pair).to_string()).to_lowercase(),
                    token_x: Some(traderjoe_v2::Token {
                        address: append_0x(&Hex(pair.token_x).to_string()),
                        decimal: token_x_data.2,
                        symbol: token_x_data.1,
                        name: token_x_data.0,
                    }),
                    token_y: Some(traderjoe_v2::Token {
                        address: append_0x(&Hex(pair.token_y).to_string()),
                        decimal: token_y_data.2,
                        symbol: token_y_data.1,
                        name: token_y_data.0,
                    }),
                    bin_step: pair.bin_step.to_string(),
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                    log_index: log.index(),
                }
            })
            .collect(),
    })
}

#[substreams::handlers::map]
pub fn map_swaps(block: eth::v2::Block) -> Result<traderjoe_v2::Swaps, Error> {
    let mut swaps: Vec<traderjoe_v2::Swap> = Vec::new();
    for log in block.logs() {
        if let Some(swap_event) = traderjoe_v2_pair_events::events::Swap::match_and_decode(log) {
            log::info!("Swap Event Found");

            swaps.push(traderjoe_v2::Swap {
                pair_address: append_0x(&Hex(&log.address()).to_string()).to_lowercase(),
                amounts_in: swap_event.amount_in.to_string(),
                amounts_out: swap_event.amount_in.to_string(),
                id: swap_event.id.to_string(),
                block_number: block.number,
                timestamp: block.timestamp_seconds(),
                tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                log_index: log.index(),
            })
        }
    }
    Ok(traderjoe_v2::Swaps { swaps })
}

#[substreams::handlers::store]
pub fn store_pairs(i: traderjoe_v2::Pairs, o: StoreSetProto<traderjoe_v2::Pair>) {
    for pair in i.pairs {
        o.set(0, generate_key("Pair", &pair.address), &pair);
    }
}

#[substreams::handlers::store]
pub fn store_candles(
    pair: StoreGetProto<traderjoe_v2::Pair>,
    swap: traderjoe_v2::Swaps,
    o: StoreSetProto<traderjoe_v2::Candle>,
) {
    for s in swap.swaps {
        log::info!("Candle Found - 1");

        let pair_address = s.pair_address;

        log::info!(&pair_address);

        let pairs = pair.get_last(generate_key("Pair", &pair_address));

        log::info!("Candle Found - 2");

        match pairs {
            Some(p) => {
                log::info!("Candle Found - 3");

                let token_x = p.token_x.unwrap();
                let token_y = p.token_y.unwrap();

                let token_x_address = token_x.address;
                let token_y_address = token_y.address;

                log::info!("TokenX Address : {}", &token_x_address.to_string());
                log::info!("TokenY Address: {}", &token_y_address.to_string());

                let token_x_decimals = token_x.decimal;
                let token_y_decimals = token_y.decimal;

                let token0 = get_sorted_token0(&token_x_address, &token_y_address);
                let token1 = get_sorted_token1(&token_x_address, &token_y_address);

                let price_y = get_price_y(
                    bigint_to_i32(&p.bin_step),
                    bigint_to_i32(&s.id),
                    bigint_to_i32(&token_x_decimals),
                    bigint_to_i32(&token_y_decimals),
                );
                let price_x = BigDecimal::one() / &price_y;

                log::info!("PriceX : {}", &price_x);
                log::info!("PriceY : {}", &price_y);
                log::info!("isSortedToken0 : {}", &token0);
                log::info!("isSortedToken1 : {}", &token1);

                let price = get_sorted_price(
                    &token_x_address,
                    &token_y_address,
                    &price_x.to_string(),
                    &price_y.to_string(),
                );

                log::info!(&price.to_string());

                for candle_period in CANDLESTICK_PERIODS {
                    let timestamp = s.timestamp;
                    let time_id = timestamp / candle_period.to_u64().unwrap();
                    let time_params = &timestamp - (timestamp % candle_period.to_u64().unwrap());

                    let time_bytes = time_id.to_ne_bytes();
                    let period_bytes = candle_period.to_ne_bytes();

                    let candle_id = format!(
                        "{}{}{}{}",
                        Hex(&time_bytes[0..4]).to_string(),
                        Hex(&period_bytes[0..4]).to_string(),
                        &token0.split("0x").last().unwrap(),
                        &token1.split("0x").last().unwrap(),
                    );

                    let amount_in = s.amounts_in.clone();
                    let amount_out = s.amounts_out.clone();

                    log::info!("AmountIn : {:?}", &amount_in);
                    log::info!("AmountOut : {:?}", &amount_out);

                    let amount_in_x = decode_x(amount_in.clone());
                    let amount_in_y = decode_y(amount_in.clone());
                    let amount_out_x = decode_x(amount_out.clone());
                    let amount_out_y = decode_y(amount_out.clone());

                    log::info!("amount_in_x : {}", &amount_in_x.to_string());
                    log::info!("amount_in_y : {}", &amount_in_y.to_string());
                    log::info!("amount_out_x : {}", &amount_out_x.to_string());
                    log::info!("amount_out_y : {}", &amount_out_y.to_string());

                    let amount_x_traded = get_amount_traded(
                        amount_in_x,
                        amount_out_x,
                        BigInt::from_str(&token_x_decimals).unwrap().to_i32(),
                    );
                    let amount_y_traded = get_amount_traded(
                        amount_in_y,
                        amount_out_y,
                        bigint_to_i32(&token_y_decimals),
                    );
                    log::info!("amount_x_traded : {}", &amount_x_traded.to_string());
                    log::info!("amount_y_traded : {}", &amount_y_traded.to_string());

                    let amount_0_traded = get_sorted_amount0(
                        &token_x_address,
                        &token_y_address,
                        &amount_x_traded.to_string(),
                        &amount_y_traded.to_string(),
                    );
                    let amount_1_traded = get_sorted_amount1(
                        &token_x_address,
                        &token_y_address,
                        &amount_x_traded.to_string(),
                        &amount_y_traded.to_string(),
                    );

                    log::info!("amount_0_traded : {}", &amount_0_traded.to_string());
                    log::info!("amount_1_traded : {}", &amount_1_traded.to_string());

                    o.set(
                        0,
                        generate_key("Candle", &append_0x(&candle_id)),
                        &traderjoe_v2::Candle {
                            time: time_params.to_string(),
                            period: candle_period.to_string(),
                            last_block: s.timestamp.to_string(),
                            token0: token0.to_string(),
                            token1: token1.to_string(),
                            token0_amount_traded: amount_0_traded,
                            token1_amount_traded: amount_1_traded,
                            high: price.clone(),
                            open: price.clone(),
                            close: price.clone(),
                            low: price.clone(),
                        },
                    );
                }
            }
            None => log::info!("None Variant"),
        }
    }
}

#[substreams::handlers::store]
pub fn store_tokens(i: traderjoe_v2::Pairs, o: StoreSetProto<traderjoe_v2::Token>) {
    for pair in i.pairs {
        let token_x = &pair.token_x.unwrap();
        let token_y = &pair.token_y.unwrap();
        o.set(0, generate_key("Token", &token_x.address), &token_x);
        o.set(0, generate_key("Token", &token_y.address), &token_y);
    }
}

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
