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

// -------------------
//  Map Factory Entities
// -------------------
pub fn factory_created_factory_entity_change(tables: &mut Tables) {
    let id = append_0x(Hex(DEXCANDLES_FACTORY).to_string());

    let bigint0 = BigInt::zero();
    let bigdecimal0 = BigDecimal::zero();
    tables
        .create_row("Factory", id)
        .set("poolCount", &bigint0)
        .set("txCount", &bigint0)
        .set("totalVolumeUSD", &bigdecimal0)
        .set("totalVolumeETH", &bigdecimal0)
        .set("totalFeesUSD", &bigdecimal0)
        .set("totalFeesETH", &bigdecimal0)
        .set("untrackedVolumeUSD", &bigdecimal0)
        .set("totalValueLockedUSD", &bigdecimal0)
        .set("totalValueLockedETH", &bigdecimal0)
        .set("totalValueLockedUSDUntracked", &bigdecimal0)
        .set("totalValueLockedETHUntracked", &bigdecimal0)
        .set(
            "owner",
            &format!("0x{}", Hex(utils::ZERO_ADDRESS).to_string()),
        );
}

pub fn tx_count_factory_entity_change(tables: &mut Tables, tx_count_deltas: &Deltas<DeltaBigInt>) {
    for delta in tx_count_deltas.iter().key_first_segment_eq("factory") {
        tables
            .update_row("Factory", append_0x(Hex(DEXCANDLES_FACTORY).to_string()))
            .set("txCount", &delta.new_value);
    }
}

pub fn tvl_factory_entity_change(
    tables: &mut Tables,
    derived_factory_tvl_deltas: &Deltas<DeltaBigDecimal>,
) {
    for delta in derived_factory_tvl_deltas
        .iter()
        .key_first_segment_eq("factory")
        .key_last_segment_in([
            "totalValueLockedUSD",
            "totalValueLockedUSDUntracked",
            "totalValueLockedETH",
            "totalValueLockedETHUntracked",
        ])
    {
        tables
            .update_row("Factory", append_0x(Hex(DEXCANDLES_FACTORY).to_string()))
            .set(key::last_segment(&delta.key), &delta.new_value);
    }
}
