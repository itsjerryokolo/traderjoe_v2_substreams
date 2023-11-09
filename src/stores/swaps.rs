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
