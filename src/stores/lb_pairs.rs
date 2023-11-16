use crate::utils::constants::DEXCANDLES_FACTORY;
use crate::utils::helper::{append_0x, bigint_to_u64, generate_key};
use crate::utils::rpc::get_token_data;

use ethabi::token;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigDecimal;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreDelete, StoreGetProto, StoreSetProto, StoreSetString,
};

use substreams::{
    pb::substreams::{store_delta::Operation, StoreDelta},
    scalar::BigInt,
    store::{StoreGet, StoreNew, StoreSet},
    Hex,
};

use crate::pb::traderjoe::v2 as traderjoe_v2;

// pub const BIG_DECIMAL_1E6: BigDecimal = BigDecimal::from_str("1_000_000").unwrap();
// pub static BIG_DECIMAL_1E10: BigDecimal = BigDecimal::from_str("10_000_000_000").unwrap();
// pub static BIG_DECIMAL_1E12: BigDecimal = BigDecimal::from_str("1_000_000_000_000").unwrap();
// pub static BIG_DECIMAL_1E18: BigDecimal =
//     BigDecimal::from_str("1_000_000_000_000_000_000").unwrap();
// pub static BIG_DECIMAL_ZERO: BigDecimal = BigDecimal::from_str("0").unwrap();
// pub static BIG_DECIMAL_ONE: BigDecimal = BigDecimal::from_str("1").unwrap();

#[substreams::handlers::store]
pub fn store_total_tx_counts(
    clock: Clock,
    factory_events: traderjoe_v2::FactoryEvents,
    template_events: traderjoe_v2::TemplateEvents,
    output: StoreAddBigInt,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id = timestamp_seconds / 86400;
    let hour_id = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;
    let factory_addr = Hex(DEXCANDLES_FACTORY).to_string();

    output.delete_prefix(0, &format!("SJoeDayData:{prev_day_id}:"));
    output.delete_prefix(0, &format!("LbPairDayData:{prev_day_id}:"));
    output.delete_prefix(0, &format!("LbPairHourData:{prev_hour_id}:"));
    output.delete_prefix(0, &format!("TokenDayData:{prev_day_id}:"));
    output.delete_prefix(0, &format!("TokenHourData:{prev_hour_id}:"));

    for event in factory_events.lb_pairs {
        let pool_address = &event.lb_pair;
        let token0_addr = event.token_x.unwrap().address.to_string();
        let token1_addr = event.token_y.unwrap().address.to_string();

        output.add_many(
            event.evt_index,
            &vec![
                format!("pool:{pool_address}"),
                format!("token:{token0_addr}"),
                format!("token:{token1_addr}"),
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
                format!("LbPairDayData:{day_id}:{pool_address}"),
                format!("LbPairHourData:{hour_id}:{pool_address}"),
                format!("TokenDayData:{day_id}:{token0_addr}"),
                format!("TokenDayData:{day_id}:{token1_addr}"),
                format!("TokenHourData:{hour_id}:{token0_addr}"),
                format!("TokenHourData:{hour_id}:{token1_addr}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in factory_events.lb_pair_ignored_state_changeds {
        let pool_address = Hex(event.lb_pair).to_string();

        output.add_many(
            event.evt_index,
            &vec![
                format!("pool:{pool_address}"),
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
                format!("LbPairDayData:{day_id}:{pool_address}"),
                format!("LbPairHourData:{hour_id}:{pool_address}"),
                format!("TraderJoeDayData:{day_id}:{pool_address}"),
                format!("TraderJoeHourData:{hour_id}:{pool_address}"),
                format!("TokenDayData:{day_id}:{pool_address}"),
                format!("TokenJoeHourData:{hour_id}:{pool_address}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in factory_events.flash_loan_fee_sets {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in factory_events.fee_parameters_sets {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    //##################################//
    //          TEMPLATES              //
    //################################//

    for event in template_events.swaps {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.composition_fees {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }
    for event in template_events.deposited_to_bins {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.transfer_batches {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }
    for event in template_events.flash_loans {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.fees_collected {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.transfer_singles {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.protocol_fees_collected {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in template_events.withdrawn_from_bins {
        output.add_many(
            event.evt_index,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }
}

#[substreams::handlers::store]
pub fn store_tokens(
    i: StoreGetProto<traderjoe_v2::LbPair>,
    i2: traderjoe_v2::FactoryEvents,
    o: StoreSetProto<traderjoe_v2::Token>,
) {
    for pair_created in i2.lb_pairs {
        let token_x = pair_created.token_x.unwrap();
        let token_y = pair_created.token_y.unwrap();

        o.set(
            0,
            generate_key("Token", &token_x.address).to_string(),
            &token_x,
        );
        o.set(
            0,
            generate_key("Token", &token_y.address).to_string(),
            &token_y,
        );
    }
}

#[substreams::handlers::store]
pub fn store_pairs(i: traderjoe_v2::FactoryEvents, o: StoreSetProto<traderjoe_v2::LbPair>) {
    for pair in i.lb_pairs {
        o.set(0, generate_key("Pair", &pair.lb_pair.to_string()), &pair);
    }
}

#[substreams::handlers::store]
pub fn store_pair_count(i: traderjoe_v2::FactoryEvents, o: StoreAddBigInt) {
    for pairs in i.lb_pairs {
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

    store.delete_prefix(0, &format!("LbPairDayData:{prev_day_id}:"));
    store.delete_prefix(0, &format!("LbPairHourData:{prev_hour_id}:"));

    for pair_tvl in events.swaps {
        let pair = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));

        let pool_address = &pair.id;

        let token0_address = pair.token_x.unwrap().address;
        let token1_address = pair.token_y.unwrap().address;

        let amount_x_in = 1;
        let amount_x_out = 1;

        let amount_y_in = 1;
        let amount_y_out = 1;

        let reserve_x = amount_x_in - amount_x_out;
        let reserve_y = amount_y_in - amount_y_out;

        let tvl = amount_x_in - amount_x_out;

        store.add_many(
            pair.log_ordinal,
            &vec![
                format!("Pair:{pool_address}"),
                format!("Pair:{token0_address}:{token1_address}"),
                format!("Pair:{token1_address}:{token0_address}"),
                format!("LbPairDayData:{day_id}:{pool_address}"),
                format!("LbPairHourData:{hour_id}:{pool_address}"),
                format!("TokenDayData:{day_id}:{pool_address}"),
                format!("TokenHourData:{hour_id}:{pool_address}"),
            ],
            &BigInt::try_from(tvl).unwrap(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_token_prices(
    clock: Clock,
    pairs: StoreGetProto<traderjoe_v2::LbPair>,
    bundles: StoreGetProto<traderjoe_v2::Bundle>,
    events: traderjoe_v2::TemplateEvents,
    store: StoreSetString,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;

    store.delete_prefix(0, &format!("LbPairDayData:{prev_day_id}:"));
    store.delete_prefix(0, &format!("LbPairHourData:{prev_hour_id}:"));

    for pair_tvl in events.swaps {
        let pair = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));
        let bundle = bundles.must_get_last(generate_key("Bundle", "1"));

        let pool_address = &pair.id;
        let avax_price_usd = bundle.avax_price_usd;

        let token0_address = pair.token_x.unwrap().address;
        let token1_address = pair.token_y.unwrap().address;

        let amount_x_in = avax_price_usd;
        let amount_x_out = 1;

        let amount_y_in = 1;
        let amount_y_out = 1;

        let reserve_x = 3 - 1;
        let reserve_y = amount_y_in - amount_y_out;

        let price = 4 - 3;

        store.set_many(
            pair.log_ordinal,
            &vec![
                format!("pool:{pool_address}"),
                format!("pair:{token0_address}:{token1_address}"),
                format!("pair:{token1_address}:{token0_address}"),
                format!("LbPairDayData:{day_id}:{pool_address}"),
                format!("LbPairHourData:{hour_id}:{pool_address}"),
            ],
            &price.to_string(),
        )
    }
}

#[substreams::handlers::store]
pub fn store_ticks(
    clock: Clock,
    pairs: StoreGetProto<traderjoe_v2::LbPair>,
    bundles: StoreGetProto<traderjoe_v2::Bundle>,
    events: traderjoe_v2::TemplateEvents,
    store: StoreSetString,
) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;

    store.delete_prefix(0, &format!("TokenDayData:{prev_day_id}:"));
    store.delete_prefix(0, &format!("TokenHourData:{prev_hour_id}:"));

    for pair_tvl in events.swaps {
        let pair = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));
        let bundle = bundles.must_get_last(generate_key("Bundle", "1"));

        let pool_address = &pair.id;
        let avax_price_usd = bundle.avax_price_usd;

        let token0_address = &pair.token_x;
        let token1_address = &pair.token_y;

        let open = avax_price_usd;
        let high = 1;

        let low = 1;
        let close = 1;

        store.set_many(
            pair.log_ordinal,
            &vec![
                format!("TokenDayData:{day_id}:{pool_address}"),
                format!("TokenHourData:{hour_id}:{pool_address}"),
            ],
            &low.to_string(),
        )
    }
}
