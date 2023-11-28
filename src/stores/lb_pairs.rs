use crate::utils::constants::DEXCANDLES_FACTORY;
use crate::utils::helper::{append_0x, bigint_to_u64, generate_key};
use crate::utils::pricing::get_token_price_in_avax;
use crate::utils::rpc::get_token_data;

use ethabi::token;
use std::str::FromStr;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigDecimal;
use substreams::store::{
    StoreAdd, StoreAddBigInt, StoreDelete, StoreGetProto, StoreSetBigDecimal, StoreSetProto,
    StoreSetString,
};

use substreams::{
    pb::substreams::{store_delta::Operation, StoreDelta},
    scalar::BigInt,
    store::{StoreGet, StoreNew, StoreSet},
    Hex,
};

use crate::pb::traderjoe::v2 as traderjoe_v2;

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
        let pool_address = &Hex(&event.lb_pair).to_string();
        let token0_addr = event.token_x.unwrap().address.to_string();
        let token1_addr = event.token_y.unwrap().address.to_string();

        // const id = binId || lbPair.activeId;
        // const tokenX = loadToken(Address.fromString(lbPair.tokenX));
        // const tokenY = loadToken(Address.fromString(lbPair.tokenY));

        // tokenX.derivedAVAX = getTokenPriceInAVAX(tokenX);
        // tokenY.derivedAVAX = getTokenPriceInAVAX(tokenY);

        // const bundle = loadBundle();
        // const tokenXPriceUSD = tokenX.derivedAVAX.times(bundle.avaxPriceUSD);
        // const tokenYPriceUSD = tokenY.derivedAVAX.times(bundle.avaxPriceUSD);
        // lbPair.tokenXPriceUSD = tokenXPriceUSD;
        // lbPair.tokenYPriceUSD = tokenYPriceUSD;

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
                format!("TraderJoeDayData:{day_id}:{pool_address}"),
                format!("TraderJoeHourData:{hour_id}:{pool_address}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for event in factory_events.lb_pair_ignored_state_changeds {
        let pool_address = Hex(event.lb_pair).to_string();

        output.add_many(
            0,
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

    for _ in factory_events.flash_loan_fee_sets {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in factory_events.fee_parameters_sets {
        output.add_many(
            0,
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

    for _ in template_events.swaps {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.composition_fees {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }
    for _ in template_events.deposited_to_bins {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.transfer_batches {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }
    for _ in template_events.flash_loans {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.fees_collected {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.transfer_singles {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.protocol_fees_collected {
        output.add_many(
            0,
            &vec![
                format!("factory:{factory_addr}"),
                format!("SJoeDayData:{day_id}"),
            ],
            &BigInt::from(1 as i32),
        );
    }

    for _ in template_events.withdrawn_from_bins {
        output.add_many(
            0,
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
        o.set(
            0,
            generate_key("Pair", &Hex(&pair.lb_pair).to_string()),
            &pair,
        );
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
pub fn store_prices(
    pairs: StoreGetProto<traderjoe_v2::LbPair>,
    bundles: StoreGetProto<traderjoe_v2::Bundle>,
    events: traderjoe_v2::TemplateEvents,
    store: StoreSetBigDecimal,
) {
    for pair_tvl in events.swaps {
        let pair = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));
        let pool_address = &pair.id;
        let token_x_address = pair.token_x.unwrap().address;
        let token_y_address = pair.token_y.unwrap().address;

        let bundle = bundles.must_get_last(generate_key("Bundle", "1"));

        let token_x_derived_avax = get_token_price_in_avax(&token_x_address.clone().into_bytes());
        let token_y_derived_avax = get_token_price_in_avax(&token_y_address.clone().into_bytes());

        let token_x_price_usd =
            token_x_derived_avax.clone() * BigDecimal::from_str(&bundle.avax_price_usd).unwrap();
        let token_y_price_usd =
            token_x_derived_avax.clone() * BigDecimal::from_str(&bundle.avax_price_usd).unwrap();

        store.set(
            pair.log_ordinal,
            format!("Pair:{pool_address}"),
            &token_x_price_usd,
        );
        store.set(
            pair.log_ordinal,
            format!("Pair:{pool_address}"),
            &token_y_price_usd,
        );
        store.set(
            pair.log_ordinal,
            format!("Token:{token_x_address}"),
            &token_x_derived_avax,
        );

        store.set(
            pair.log_ordinal,
            format!("Token:{token_y_address}"),
            &token_y_derived_avax,
        );
    }
}

#[substreams::handlers::store]
pub fn store_volumes(
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

    store.delete_prefix(0, &format!("TokenDayData:{prev_day_id}"));
    store.delete_prefix(0, &format!("TokenHourData:{prev_hour_id}"));

    for pair_tvl in events.swaps {
        let pair = pairs.must_get_last(generate_key("Pair", &pair_tvl.id));
        let bundle = bundles.must_get_last(generate_key("Bundle", "1"));

        let pool_address = &pair.id;
        let avax_price_usd = bundle.avax_price_usd;

        let volume = &pair.token_x;
        let volume_avax = &pair.token_x;
        let volume_usd = &pair.token_y;
        let volume_token_x = &pair.token_x;
        let volume_token_y = &pair.token_x;
        let untracked_volume_usd = &pair.token_y;

        store.set_many(
            pair.log_ordinal,
            &vec![
                format!("TokenDayData:{day_id}:{pool_address}"),
                format!("TokenHourData:{hour_id}:{pool_address}"),
            ],
            &pool_address.to_string(),
        )
    }
}
