use crate::abi;
use crate::utils::constants::DEXCANDLES_FACTORY;

use crate::utils::helper::generate_key;
use crate::utils::{helper::append_0x, rpc::get_token_data};
use abi::lb_factory as traderjoe_v2_factory_events;
use substreams::pb::substreams::module::input::Store;
use substreams::store::StoreSetProto;
use substreams::Hex;

use crate::pb::traderjoe::v2 as traderjoe_v2;
use substreams_ethereum::pb::eth;

use substreams::{errors::Error, log};

#[substreams::handlers::store]
pub fn store_pairs(i: traderjoe_v2::Pairs, o: StoreSetProto<traderjoe_v2::Pair>) {
    for pair in i.pairs {
        o.set(0, generate_key("Pair", &pair.address), &pair);
    }
}
