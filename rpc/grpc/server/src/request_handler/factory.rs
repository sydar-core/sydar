use std::sync::Arc;

use super::{
    handler::RequestHandler,
    handler_trait::Handler,
    interface::{Interface, sydardMethod, sydardRoutingPolicy},
    method::Method,
};
use crate::{
    connection::{Connection, IncomingRoute},
    connection_handler::ServerContext,
    error::GrpcServerError,
};
use sydar_grpc_core::protowire::{sydard_request::Payload, *};
use sydar_grpc_core::{ops::sydardPayloadOps, protowire::NotifyFinalityConflictResponseMessage};
use sydar_notify::{scope::FinalityConflictResolvedScope, subscriber::SubscriptionManager};
use sydar_rpc_core::{SubmitBlockRejectReason, SubmitBlockReport, SubmitBlockResponse};
use sydar_rpc_macros::build_grpc_server_interface;

pub struct Factory {}

impl Factory {
    pub fn new_handler(
        rpc_op: sydardPayloadOps,
        incoming_route: IncomingRoute,
        server_context: ServerContext,
        interface: &Interface,
        connection: Connection,
    ) -> Box<dyn Handler> {
        Box::new(RequestHandler::new(rpc_op, incoming_route, server_context, interface, connection))
    }

    pub fn new_interface(server_ctx: ServerContext, network_bps: u64) -> Interface {
        // The array as last argument in the macro call below must exactly match the full set of
        // sydardPayloadOps variants.
        let mut interface = build_grpc_server_interface!(
            server_ctx.clone(),
            ServerContext,
            Connection,
            sydardRequest,
            sydardResponse,
            sydardPayloadOps,
            [
                SubmitBlock,
                GetBlockTemplate,
                GetCurrentNetwork,
                GetBlock,
                GetBlocks,
                GetInfo,
                Shutdown,
                GetPeerAddresses,
                GetSink,
                GetMempoolEntry,
                GetMempoolEntries,
                GetConnectedPeerInfo,
                AddPeer,
                SubmitTransaction,
                SubmitTransactionReplacement,
                GetSubnetwork,
                GetVirtualChainFromBlock,
                GetBlockCount,
                GetsydarDagInfo,
                ResolveFinalityConflict,
                GetHeaders,
                GetUtxosByAddresses,
                GetBalanceByAddress,
                GetBalancesByAddresses,
                GetSinkBlueScore,
                Ban,
                Unban,
                EstimateNetworkHashesPerSecond,
                GetMempoolEntriesByAddresses,
                GetCoinSupply,
                Ping,
                GetMetrics,
                GetConnections,
                GetSystemInfo,
                GetServerInfo,
                GetSyncStatus,
                GetDaaScoreTimestampEstimate,
                GetFeeEstimate,
                GetFeeEstimateExperimental,
                GetCurrentBlockColor,
                GetUtxoReturnAddress,
                GetVirtualChainFromBlockV2,
                NotifyBlockAdded,
                NotifyNewBlockTemplate,
                NotifyFinalityConflict,
                NotifyUtxosChanged,
                NotifySinkBlueScoreChanged,
                NotifyPruningPointUtxoSetOverride,
                NotifyVirtualDaaScoreChanged,
                NotifyVirtualChainChanged,
                StopNotifyingUtxosChanged,
                StopNotifyingPruningPointUtxoSetOverride,
                SubmitAccountTransaction,
            ]
        );

        // Manually reimplementing the NotifyFinalityConflictRequest method so subscription
        // gets mirrored to FinalityConflictResolved notifications as well.
        let method: sydardMethod = Method::new(|server_ctx: ServerContext, connection: Connection, request: sydardRequest| {
            Box::pin(async move {
                let mut response: sydardResponse = match request.payload {
                    Some(Payload::NotifyFinalityConflictRequest(ref request)) => {
                        match sydar_rpc_core::NotifyFinalityConflictRequest::try_from(request) {
                            Ok(request) => {
                                let listener_id = connection.get_or_register_listener_id()?;
                                let command = request.command;
                                let result = server_ctx
                                    .notifier
                                    .clone()
                                    .execute_subscribe_command(listener_id, request.into(), command)
                                    .await
                                    .and(
                                        server_ctx
                                            .notifier
                                            .clone()
                                            .execute_subscribe_command(
                                                listener_id,
                                                FinalityConflictResolvedScope::default().into(),
                                                command,
                                            )
                                            .await,
                                    );
                                NotifyFinalityConflictResponseMessage::from(result).into()
                            }
                            Err(err) => NotifyFinalityConflictResponseMessage::from(err).into(),
                        }
                    }
                    _ => {
                        return Err(GrpcServerError::InvalidRequestPayload);
                    }
                };
                response.id = request.id;
                Ok(response)
            })
        });
        interface.replace_method(sydardPayloadOps::NotifyFinalityConflict, method);

        // SubmitAccountTransaction custom handler
        let method: sydardMethod = Method::new(|server_ctx: ServerContext, _connection: Connection, request: sydardRequest| {
            Box::pin(async move {
                let mut response: sydardResponse = match request.payload {
                    Some(Payload::SubmitAccountTransactionRequest(ref req)) => {
                        match sydar_rpc_core::SubmitAccountTransactionRequest::try_from(req) {
                            Ok(rpc_req) => match server_ctx.core_service.submit_account_transaction(rpc_req).await {
                                Ok(res) => SubmitAccountTransactionResponseMessage::from(res).into(),
                                Err(e) => SubmitAccountTransactionResponseMessage::from(e).into(),
                            },
                            Err(e) => SubmitAccountTransactionResponseMessage::from(e).into(),
                        }
                    }
                    _ => return Err(GrpcServerError::InvalidRequestPayload),
                };
                response.id = request.id;
                Ok(response)
            })
        });
        interface.replace_method(sydardPayloadOps::SubmitAccountTransaction, method);

        // SubmitAccountTransactionResponseMessage -> sydardResponse impl

        // Methods with special properties
        let network_bps = network_bps as usize;
        interface.set_method_properties(
            sydardPayloadOps::SubmitBlock,
            network_bps,
            10.max(network_bps * 2),
            sydardRoutingPolicy::DropIfFull(Arc::new(Box::new(|_: &sydardRequest| {
                Ok(Ok(SubmitBlockResponse { report: SubmitBlockReport::Reject(SubmitBlockRejectReason::RouteIsFull) }).into())
            }))),
        );

        interface
    }
}
