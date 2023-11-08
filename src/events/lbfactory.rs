use crate::abi;
use crate::utils::constants::DEXCANDLES_FACTORY;

use crate::utils::{helper::append_0x, rpc::get_token_data};
use abi::lb_factory as traderjoe_v2_factory_events;
use substreams::Hex;

use crate::pb::traderjoe::v2 as traderjoe_v2;
use substreams_ethereum::pb::eth;

use substreams::{errors::Error, log};

#[substreams::handlers::map]
pub fn map_pairs_created(block: eth::v2::Block) -> Result<traderjoe_v2::Pairs, Error> {
    Ok(traderjoe_v2::Pairs {
        pairs: block
            .events::<traderjoe_v2_factory_events::events::LbPairCreated>(&[&DEXCANDLES_FACTORY])
            .map(|(pair, log)| {
                log::info!("New Pair Created 🚀🚀 ");

                let token_x_data = get_token_data(&pair.token_x);
                let token_y_data = get_token_data(&pair.token_y);

                traderjoe_v2::Pair {
                    address: append_0x(&Hex(pair.lb_pair).to_string()).to_lowercase(),
                    token_x: Some(traderjoe_v2::Token {
                        address: append_0x(&Hex(pair.token_x).to_string()),
                        decimal: token_x_data.2,
                        symbol: token_x_data.1,
                        name: token_x_data.0,
                    }),
                    token_y: Some(traderjoe_v2::Token {
                        address: append_0x(&Hex(pair.token_y).to_string()),
                        decimal: token_y_data.2,
                        symbol: token_y_data.1,
                        name: token_y_data.0,
                    }),
                    bin_step: pair.bin_step.to_string(),
                    block_number: block.number,
                    timestamp: block.timestamp_seconds(),
                    tx_hash: append_0x(&Hex(&log.receipt.transaction.hash).to_string()),
                    log_index: log.index(),
                }
            })
            .collect(),
    })
}
