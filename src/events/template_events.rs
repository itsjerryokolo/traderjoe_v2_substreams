use crate::abi;

use crate::pb::traderjoe::v2::events as traderjoe_v2;

use abi::lb_pair as traderjoe_v2_pair_events;
use substreams::Hex;
use substreams_ethereum::{pb::eth, Event};

#[substreams::handlers::map]
fn map_template_events(
    blk: eth::v2::Block,
) -> Result<traderjoe_v2::TemplateEvents, substreams::errors::Error> {
    Ok(traderjoe_v2::TemplateEvents {
        swaps: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::Swap::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::Swap {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            swap_for_y: event.swap_for_y,
                            id: event.id.to_string(),
                            sender: event.sender,
                            recipient: event.recipient,
                            amount_in: event.amount_in.to_u64(),
                            amount_out: event.amount_out.to_u64(),
                            fees: event.fees.to_u64(),
                            volatility_accumulated: event.volatility_accumulated.to_u64(),
                        });
                    }

                    None
                })
            })
            .collect(),
        composition_fees: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::CompositionFee::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::CompositionFee {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            recipient: event.recipient,
                            id: event.id.to_u64(),
                            fees_x: event.fees_x.to_u64(),
                            fees_y: event.fees_y.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),

        deposited_to_bins: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::DepositedToBin::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::DepositedToBin {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            recipient: event.recipient,
                            id: event.id.to_u64(),
                            amount_x: event.amount_x.to_u64(),
                            amount_y: event.amount_y.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),

        fees_collected: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::FeesCollected::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::FeesCollected {
                            sender: event.sender,
                            recipient: event.recipient,
                            amount_x: event.amount_x.to_u64(),
                            amount_y: event.amount_y.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),

        flash_loans: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::FlashLoan::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::FlashLoan {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            receiver: event.receiver,
                            token: event.token,
                            amount: event.amount.to_u64(),
                            fee: event.fee.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),
        protocol_fees_collected: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::ProtocolFeesCollected::match_and_decode(
                            log,
                        )
                    {
                        return Some(traderjoe_v2::ProtocolFeesCollected {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            recipient: event.recipient,
                            amount_x: event.amount_x.to_u64(),
                            amount_y: event.amount_y.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),
        transfer_batches: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::TransferBatch::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::TransferBatch {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            from: event.from,
                            to: event.to,
                            ids: event.ids.iter().map(|id| id.to_string()).collect(),
                            amounts: event.amounts.iter().map(|id| id.to_string()).collect(),
                        });
                    }
                    None
                })
            })
            .collect(),
        transfer_singles: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::TransferSingle::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::TransferSingle {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            from: event.from,
                            to: event.to,
                            id: event.id.to_u64(),
                            amount: event.amount.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),

        withdrawn_from_bins: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter().filter_map(|log| {
                    if let Some(event) =
                        traderjoe_v2_pair_events::events::WithdrawnFromBin::match_and_decode(log)
                    {
                        return Some(traderjoe_v2::WithdrawnFromBin {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            sender: event.sender,
                            recipient: event.recipient,
                            id: event.id.to_u64(),
                            amount_x: event.amount_x.to_u64(),
                            amount_y: event.amount_y.to_u64(),
                        });
                    }
                    None
                })
            })
            .collect(),
    })
}
