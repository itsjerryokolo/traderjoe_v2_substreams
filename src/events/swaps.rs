use crate::abi;
use crate::events;
use crate::utils;

use std::str::FromStr;

use crate::utils::{helper::append_0x, pricing::get_price_y, rpc::get_token_data};
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
