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

    let entity_changes = tables.to_entity_changes();
    Ok(entity_changes)
}
