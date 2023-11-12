mod abi;
mod events;
mod pb;
mod stores;
mod utils;

use std::str::FromStr;

use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

use substreams::{
    pb::substreams::store_delta::Operation,
    scalar::{BigDecimal, BigInt},
};

pub use events::*;
pub use stores::*;
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn graph_out() -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    // // Tokens:
    // db::tokens_created_token_entity_changes(&mut tables, &pools_created, tokens_store);
    // db::swap_volume_token_entity_change(&mut tables, &swaps_volume_deltas);
    // db::tx_count_token_entity_change(&mut tables, &tx_count_deltas);
    // db::total_value_locked_by_token_token_entity_change(&mut tables, &token_tvl_deltas);
    // db::total_value_locked_usd_token_entity_change(&mut tables, &derived_tvl_deltas);

    let entity_changes = tables.to_entity_changes();
    Ok(entity_changes)
}
