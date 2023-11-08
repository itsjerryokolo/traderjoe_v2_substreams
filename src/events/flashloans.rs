use crate::abi;
use crate::events;
use crate::utils;

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

use crate::pb::traderjoe::v2 as traderjoe_v2;
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
