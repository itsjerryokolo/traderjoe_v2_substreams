use crate::utils::constants::DEXCANDLES_FACTORY;

use crate::utils::helper::generate_key;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreSetProto};

use crate::pb::traderjoe::v2::{entities, events};
use substreams_ethereum::pb::eth;

use substreams::{errors::Error, log};

#[substreams::handlers::store]
pub fn store_pool_liquidities(clock: Clock, events: Events, store: StoreSetBigInt) {
    let timestamp_seconds = clock.timestamp.unwrap().seconds;
    let day_id: i64 = timestamp_seconds / 86400;
    let hour_id: i64 = timestamp_seconds / 3600;
    let prev_day_id = day_id - 1;
    let prev_hour_id = hour_id - 1;

    store.delete_prefix(0, &format!("PoolDayData:{prev_day_id}:"));
    store.delete_prefix(0, &format!("PoolHourData:{prev_hour_id}:"));

    for pool_liquidity in events.pool_liquidities {
        let pool_address = &pool_liquidity.pool_address;
        let token0_address = &pool_liquidity.token0;
        let token1_address = &pool_liquidity.token1;
        store.set_many(
            pool_liquidity.log_ordinal,
            &vec![
                format!("pool:{pool_address}"),
                format!("pair:{token0_address}:{token1_address}"),
                format!("pair:{token1_address}:{token0_address}"),
                format!("PoolDayData:{day_id}:{pool_address}"),
                format!("PoolHourData:{hour_id}:{pool_address}"),
            ],
            &BigInt::try_from(pool_liquidity.liquidity).unwrap(),
        )
    }
}
