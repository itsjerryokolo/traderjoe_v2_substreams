use crate::utils::constants::DEXCANDLES_FACTORY;

use crate::utils::helper::generate_key;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreSetProto};

use crate::pb::traderjoe::v2::{entities, events};
use substreams_ethereum::pb::eth;

use substreams::{errors::Error, log};

#[substreams::handlers::store]
pub fn store_totals(pair: traderjoe_v2::events::LbPairCreated, o: StoreAddBigInt) {
    for s in pair.s {
        o.add(0, generate_key(name, val), p)
    }
}
