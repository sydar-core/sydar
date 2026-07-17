use super::error::Result;
use core::fmt::Debug;
use sydar_grpc_core::{
    ops::sydardPayloadOps,
    protowire::{sydardRequest, sydardResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: sydardPayloadOps, request: &sydardRequest) -> sydardResponseReceiver;
    fn handle_response(&self, response: sydardResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type sydardResponseSender = oneshot::Sender<Result<sydardResponse>>;
pub(crate) type sydardResponseReceiver = oneshot::Receiver<Result<sydardResponse>>;
