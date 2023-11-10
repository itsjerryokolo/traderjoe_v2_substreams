use crate::utils::helper::generate_key;

use substreams::store::StoreSetProto;
use substreams::{
    pb::substreams::{store_delta::Operation, StoreDelta},
    scalar::{BigDecimal, BigInt},
    store::{StoreNew, StoreSet},
    Hex,
};

use crate::pb::traderjoe::v2 as traderjoe_v2;

#[substreams::handlers::store]
pub fn store_pairs(
    i: traderjoe_v2::events::FactoryEvents,
    o: StoreSetProto<traderjoe_v2::entities::Pair>,q
) {
    for e in i.lb_pair_createds {
        o.set(0, generate_key("Pair", &e.lb_pair.to_string()), &e);
    }
}
