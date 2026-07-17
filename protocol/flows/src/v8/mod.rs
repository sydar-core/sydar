use crate::v7::{
    address::{ReceiveAddressesFlow, SendAddressesFlow},
    blockrelay::{flow::HandleRelayInvsFlow, handle_requests::HandleRelayBlockRequests},
    ping::{ReceivePingsFlow, SendPingsFlow},
    request_antipast::HandleAntipastRequests,
    request_block_locator::RequestBlockLocatorFlow,
    request_headers::RequestHeadersFlow,
    request_ibd_blocks::HandleIbdBlockRequests,
    request_ibd_chain_block_locator::RequestIbdChainBlockLocatorFlow,
    request_pp_proof::RequestPruningPointProofFlow,
    request_pruning_point_and_anticone::PruningPointAndItsAnticoneRequestsFlow,
    request_pruning_point_utxo_set::RequestPruningPointUtxoSetFlow,
    txrelay::flow::{RelayTransactionsFlow, RequestTransactionsFlow},
};
pub(crate) mod request_block_bodies;
use crate::{flow_context::FlowContext, flow_trait::Flow};

use crate::ibd::IbdFlow;
use request_block_bodies::HandleBlockBodyRequests;
use sydar_p2p_lib::{Router, sydardMessagePayloadType, SharedIncomingRoute, convert::header::HeaderFormat};
use sydar_utils::channel;
use std::sync::Arc;

pub fn register(ctx: FlowContext, router: Arc<Router>, protocol_version: u32) -> Vec<Box<dyn Flow>> {
    // IBD flow <-> invs flow communication uses a job channel in order to always
    // maintain at most a single pending job which can be updated
    let (ibd_sender, relay_receiver) = channel::job();
    let body_only_ibd_permitted = true;
    let header_format = HeaderFormat::from(protocol_version);
    let mut flows: Vec<Box<dyn Flow>> = vec![
        Box::new(IbdFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                sydardMessagePayloadType::BlockHeaders,
                sydardMessagePayloadType::DoneHeaders,
                sydardMessagePayloadType::IbdBlockLocatorHighestHash,
                sydardMessagePayloadType::IbdBlockLocatorHighestHashNotFound,
                sydardMessagePayloadType::BlockWithTrustedDataV4,
                sydardMessagePayloadType::DoneBlocksWithTrustedData,
                sydardMessagePayloadType::IbdChainBlockLocator,
                sydardMessagePayloadType::IbdBlock,
                sydardMessagePayloadType::BlockBody,
                sydardMessagePayloadType::TrustedData,
                sydardMessagePayloadType::PruningPoints,
                sydardMessagePayloadType::PruningPointProof,
                sydardMessagePayloadType::UnexpectedPruningPoint,
                sydardMessagePayloadType::PruningPointUtxoSetChunk,
                sydardMessagePayloadType::DonePruningPointUtxoSetChunks,
            ]),
            relay_receiver,
            body_only_ibd_permitted,
            header_format,
        )),
        Box::new(HandleRelayBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestRelayBlocks]),
            header_format,
        )),
        Box::new(ReceivePingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![sydardMessagePayloadType::Ping]))),
        Box::new(SendPingsFlow::new(ctx.clone(), router.clone(), router.subscribe(vec![sydardMessagePayloadType::Pong]))),
        Box::new(RequestHeadersFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestHeaders, sydardMessagePayloadType::RequestNextHeaders]),
            header_format,
        )),
        Box::new(RequestPruningPointProofFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestPruningPointProof]),
            header_format,
        )),
        Box::new(RequestIbdChainBlockLocatorFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestIbdChainBlockLocator]),
        )),
        Box::new(PruningPointAndItsAnticoneRequestsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                sydardMessagePayloadType::RequestPruningPointAndItsAnticone,
                sydardMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks,
            ]),
            header_format,
        )),
        Box::new(RequestPruningPointUtxoSetFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![
                sydardMessagePayloadType::RequestPruningPointUtxoSet,
                sydardMessagePayloadType::RequestNextPruningPointUtxoSetChunk,
            ]),
        )),
        Box::new(HandleIbdBlockRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestIbdBlocks]),
            header_format,
        )),
        Box::new(HandleBlockBodyRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestBlockBodies]),
        )),
        Box::new(HandleAntipastRequests::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestAntipast]),
            header_format,
        )),
        Box::new(RelayTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe_with_capacity(
                vec![sydardMessagePayloadType::InvTransactions],
                RelayTransactionsFlow::invs_channel_size(),
            ),
            router.subscribe_with_capacity(
                vec![sydardMessagePayloadType::Transaction, sydardMessagePayloadType::TransactionNotFound],
                RelayTransactionsFlow::txs_channel_size(),
            ),
        )),
        Box::new(RequestTransactionsFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestTransactions]),
        )),
        Box::new(ReceiveAddressesFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::Addresses]),
        )),
        Box::new(SendAddressesFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestAddresses]),
        )),
        Box::new(RequestBlockLocatorFlow::new(
            ctx.clone(),
            router.clone(),
            router.subscribe(vec![sydardMessagePayloadType::RequestBlockLocator]),
        )),
    ];

    let invs_route = router.subscribe_with_capacity(vec![sydardMessagePayloadType::InvRelayBlock], ctx.block_invs_channel_size());
    let shared_invs_route = SharedIncomingRoute::new(invs_route);

    let num_relay_flows = (ctx.config.bps() as usize / 2).max(1);
    flows.extend((0..num_relay_flows).map(|_| {
        Box::new(HandleRelayInvsFlow::new(
            ctx.clone(),
            router.clone(),
            shared_invs_route.clone(),
            router.subscribe(vec![]),
            ibd_sender.clone(),
            header_format,
        )) as Box<dyn Flow>
    }));

    // The reject message is handled as a special case by the router
    // sydardMessagePayloadType::Reject,

    // We do not register the below two messages since they are deprecated also in go-sydar
    // sydardMessagePayloadType::BlockWithTrustedData,
    // sydardMessagePayloadType::IbdBlockLocator,

    flows
}
