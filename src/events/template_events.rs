use crate::abi;
use crate::utils;

use crate::pb::traderjoe::v2::events as traderjoe_v2;

use abi::lb_factory as traderjoe_v2_pair_events;
use substreams::Hex;
use substreams_ethereum::{pb::eth, Event};

#[substreams::handlers::map]
fn map_template_events(
    blk: eth::v2::Block,
) -> Result<traderjoe_v2::TemplateEvents, substreams::errors::Error> {
    Ok(traderjoe_v2::TemplateEvents {
        fee_recipient_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::FeeRecipientSet::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::FeeRecipientSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_recipient: event.new_recipient,
                            old_recipient: event.old_recipient,
                        });
                    }

                    None
                })
            })
            .collect(),
        flash_loan_fee_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::FlashLoanFeeSet::match_and_decode(log)
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
        lb_pair_ignored_state_changeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::LbPairIgnoredStateChanged::match_and_decode(log)
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
        lb_pair_implementation_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::LbPairImplementationSet::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::LbPairImplementationSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            lb_pair_implementation: event.lb_pair_implementation,
                            old_lb_pair_implementation: event.old_lb_pair_implementation,
                        });
                    }

                    None
                })
            })
            .collect(),

        pending_owner_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::PendingOwnerSet::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::PendingOwnerSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pending_owner: event.pending_owner,
                        });
                    }

                    None
                })
            })
            .collect(),
        preset_open_state_changeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::PresetOpenStateChanged::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::PresetOpenStateChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bin_step: event.bin_step.to_string(),
                            is_open: event.is_open,
                        });
                    }

                    None
                })
            })
            .collect(),
        preset_removeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::PresetRemoved::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::PresetRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bin_step: event.bin_step.to_string(),
                        });
                    }

                    None
                })
            })
            .collect(),
        preset_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) = traderjoe_v2_pair_events::events::PresetSet::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::PresetSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            base_factor: event.base_factor.to_string(),
                            bin_step: event.bin_step.to_string(),
                            decay_period: event.decay_period.to_string(),
                            filter_period: event.filter_period.to_string(),
                            max_volatility_accumulator: event
                                .max_volatility_accumulator
                                .to_string(),
                            protocol_share: event.protocol_share.to_string(),
                            reduction_factor: event.reduction_factor.to_string(),
                            variable_fee_control: event.variable_fee_control.to_string(),
                        });
                    }

                    None
                })
            })
            .collect(),
        quote_asset_addeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::QuoteAssetAdded::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::QuoteAssetAdded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            quote_asset: event.quote_asset,
                        });
                    }

                    None
                })
            })
            .collect(),
        quote_asset_removeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::QuoteAssetRemoved::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::QuoteAssetRemoved {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            quote_asset: event.quote_asset,
                        });
                    }

                    None
                })
            })
            .collect(),
    })
}
