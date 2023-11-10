use crate::abi;
use crate::utils;

use crate::pb::traderjoe::v2::events as traderjoe_v2;

use abi::lb_factory as traderjoe_v2_factory_events;
use substreams::Hex;
use substreams_ethereum::{pb::eth, Event};

use utils::constants::DEXCANDLES_FACTORY;

#[substreams::handlers::map]
fn map_factory_events(
    blk: eth::v2::Block,
) -> Result<traderjoe_v2::FactoryEvents, substreams::errors::Error> {
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
                            return Some(traderjoe_v2::FeeParametersSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                filter_period: event.filter_period.to_string(),
                                decay_period: event.decay_period.to_string(),
                                base_factor: event.base_factor.to_string(),
                                reduction_factor: event.reduction_factor.to_string(),
                                variable_fee_control: event.variable_fee_control.to_string(),
                                protocol_share: event.protocol_share.to_string(),
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
                                evt_index: log.block_index,
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
        lb_pair_createds: blk
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
                            return Some(traderjoe_v2::LbPairCreated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bin_step: event.bin_step.to_string(),
                                lb_pair: Hex(event.lb_pair).to_string(),
                                pid: event.pid.to_string(),
                                token_x: event.token_x,
                                token_y: event.token_y,
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
                                evt_index: log.block_index,
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
