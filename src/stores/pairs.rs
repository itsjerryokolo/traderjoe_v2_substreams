use crate::utils::helper::generate_key;

use substreams::pb::substreams::Clock;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreDelete, StoreGetProto, StoreSetProto};
use substreams::{
    pb::substreams::{store_delta::Operation, StoreDelta},
    scalar::BigInt,
    store::{StoreNew, StoreSet},
    Hex,
};

use crate::pb::traderjoe::v2 as traderjoe_v2;

#[substreams::handlers::store]
pub fn store_pairs(i: traderjoe_v2::FactoryEvents, o: StoreSetProto<traderjoe_v2::LbPair>) {
    for pair_created in i.lb_pair_createds {
        let pair = traderjoe_v2::LbPair {
            bin_step: pair_created.bin_step.to_string(),
            token_x: pair_created.token_x.unwrap().address,
            name: pair_created.token_x.unwrap().name,
            token_y: pair_created.token_y.unwrap().address,
            ..Default::default()
        };

        o.set(
            0,
            generate_key("Pair", &pair_created.lb_pair.to_string()),
            &pair,
        );
    }
}

#[substreams::handlers::store]
pub fn store_pair_count(i: traderjoe_v2::FactoryEvents, o: StoreAddBigInt) {
    for pairs in i.lb_pair_createds {
        o.add(
            pairs.evt_index,
            format!("factory:pairCount"),
            &BigInt::one(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_pairs_tvl(
    clock: Clock,
    pairs: StoreGetProto<traderjoe_v2::LbPair>,
    bundle: StoreGetProto<traderjoe_v2::Bundle>,
    events: traderjoe_v2::TemplateEvents,
    store: StoreAddBigInt,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;

    store.delete_prefix(0, &format!("PoolDayData:{prev_day_id}:"));
    store.delete_prefix(0, &format!("PoolHourData:{prev_hour_id}:"));

    for pair_tvl in events.swaps {
        let pool_address = &pair_tvl.pool_address;
        let token0_address = &pair_tvl.token0;
        let token1_address = &pair_tvl.token1;

        let a = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));

        let amount_x_in = 1;
        let amount_x_out = 1;

        let amount_y_in = 1;
        let amount_y_out = 1;

        let reserve_x = amount_x_in - amount_x_out;
        let reserve_y = amount_y_in - amount_y_out;

        let tvl = amount_x_in - amount_x_out;

        store.add_many(
            pair_tvl.event_index,
            &vec![
                format!("pool:{pool_address}"),
                format!("pair:{token0_address}:{token1_address}"),
                format!("pair:{token1_address}:{token0_address}"),
                format!("PoolDayData:{day_id}:{pool_address}"),
                format!("PoolHourData:{hour_id}:{pool_address}"),
            ],
            &BigInt::try_from(tvl).unwrap(),
        )
    }
}
