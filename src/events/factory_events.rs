use std::str::FromStr;

use crate::abi;
use crate::utils;

use crate::pb::traderjoe::v2 as traderjoe_v2;
use crate::utils::constants::BIG_DECIMAL_1E10;
use crate::utils::helper::append_0x;
use crate::utils::helper::bigint_to_u64;
use crate::utils::rpc::get_bin_step;
use crate::utils::rpc::get_token_data;

use abi::lb_factory as traderjoe_v2_factory_events;
use substreams::log;
use substreams::Hex;
use substreams::scalar::BigDecimal;
use substreams_ethereum::{pb::eth, Event};

use utils::constants::DEXCANDLES_FACTORY;

#[substreams::handlers::map]
fn map_factory_events(
    blk: eth::v2::Block,
) -> Result<traderjoe_v2::FactoryEvents, substreams::errors::Error> {
    log::info!("Factory Handler - 1");
    Ok(traderjoe_v2::FactoryEvents {

        fee_parameters_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == DEXCANDLES_FACTORY)
                    .filter_map(|log| {
                        if let Some(event) =
                            traderjoe_v2_factory_events::events::FeeParametersSet::match_and_decode(log)
                        {

                            let protocol_share_pct_value =  BigDecimal::from_str(event.protocol_share.to_string().as_str()).unwrap() / BigDecimal::from_str("1000").unwrap();
                            return Some(traderjoe_v2::FeeParametersSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.ordinal,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                filter_period: event.filter_period.to_string(),
                                decay_period: event.decay_period.to_string(),
                                base_factor: event.base_factor.to_string(),
                                reduction_factor: event.reduction_factor.to_string(),
                                variable_fee_control: event.variable_fee_control.to_string(),
                                protocol_share: event.protocol_share.to_string(),
                                protocol_share_pct:protocol_share_pct_value.to_string(),
                                bin_step: event.bin_step.to_string(),
                                max_volatility_accumulator: event
                                    .max_volatility_accumulated
                                    .to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
        flash_loan_fee_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == DEXCANDLES_FACTORY)
                    .filter_map(|log| {
                        if let Some(event) =
                            traderjoe_v2_factory_events::events::FlashLoanFeeSet::match_and_decode(log)
                        {
                            return Some(traderjoe_v2::FlashLoanFeeSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.ordinal,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_flash_loan_fee: event.new_flash_loan_fee.to_string(),
                                old_flash_loan_fee: event.old_flash_loan_fee.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
        lb_pairs: blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == DEXCANDLES_FACTORY)
                    .filter_map(|log| {
                        if let Some(event) =
                            traderjoe_v2_factory_events::events::LbPairCreated::match_and_decode(log)
                        {


                let token_x_data = get_token_data(&event.token_x);
                let token_y_data = get_token_data(&event.token_y);
                let bin_step = get_bin_step();
                // let bin_step = get_bin_step();

                let lb_name = format!("{}-{}-{}", token_x_data.1, token_y_data.1, &bin_step);

                let base_factor = BigDecimal::from_str(&bin_step).unwrap();

                // base fee in 1e18 precision: baseFactor * binStep * 1e10
                let base_fee = BigDecimal::from_str(&bin_step).unwrap() * base_factor * &*BIG_DECIMAL_1E10;

    
                            return Some(traderjoe_v2::LbPair {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.ordinal,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bin_step: event.bin_step.to_string(),
                                lb_pair: Hex(event.lb_pair).to_string(),
                                base_fee_pct: base_fee.to_string(),
                                pid: event.pid.to_string(),
                                factory: append_0x(&Hex(DEXCANDLES_FACTORY).to_string()),
                                name: lb_name,
                                token_x: Some(traderjoe_v2::Token {
                                    address: append_0x(&Hex(event.token_x).to_string()),
                                    decimals: bigint_to_u64(&token_x_data.2),
                                    symbol: token_x_data.1,
                                    factory_address: append_0x(&Hex(DEXCANDLES_FACTORY).to_string()),
                                    name: token_x_data.0,
                                    total_supply: token_x_data.3,
                                    ..Default::default()

                                }),
                                token_y: Some(traderjoe_v2::Token {
                                    address: append_0x(&Hex(event.token_y).to_string()),
                                    decimals: bigint_to_u64(&token_y_data.2),
                                    symbol: token_y_data.1,
                                    factory_address: append_0x(&Hex(DEXCANDLES_FACTORY).to_string()),
                                    name: token_y_data.0,
                                    total_supply: token_y_data.3,
                                    ..Default::default()

                                }),
                                ..Default::default()

                            });
                        }

                        None
                    })
            })
            .collect(),
        lb_pair_ignored_state_changeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == DEXCANDLES_FACTORY)
                    .filter_map(|log| {
                        if let Some(event) =
                            traderjoe_v2_factory_events::events::LbPairIgnoredStateChanged::match_and_decode(
                                log,
                            )
                        {
                            return Some(traderjoe_v2::LbPairIgnoredStateChanged {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.ordinal,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                ignored: event.ignored,
                                lb_pair: event.lb_pair,
                            });
                        }

                        None
                    })
            })
            .collect(),
    })
}
