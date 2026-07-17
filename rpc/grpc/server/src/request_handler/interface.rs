use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use sydar_grpc_core::{
    ops::sydardPayloadOps,
    protowire::{sydardRequest, sydardResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type sydardMethod = Method<ServerContext, Connection, sydardRequest, sydardResponse>;
pub type DynsydardMethod = Arc<dyn MethodTrait<ServerContext, Connection, sydardRequest, sydardResponse>>;
pub type sydardDropFn = DropFn<sydardRequest, sydardResponse>;
pub type sydardRoutingPolicy = RoutingPolicy<sydardRequest, sydardResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`sydardPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<sydardPayloadOps, DynsydardMethod>,
    method_not_implemented: DynsydardMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, sydard_request: sydardRequest| {
            Box::pin(async move {
                match sydard_request.payload {
                    Some(ref request) => Ok(sydardResponse {
                        id: sydard_request.id,
                        payload: Some(
                            sydardPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into()),
                        ),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: sydardPayloadOps, method: sydardMethod) {
        let method: DynsydardMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: sydardPayloadOps, method: sydardMethod) {
        let method: DynsydardMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: sydardPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: sydardRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, sydardRequest, sydardResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, sydardRequest, sydardResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &sydardPayloadOps,
        connection: Connection,
        request: sydardRequest,
    ) -> GrpcServerResult<sydardResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &sydardPayloadOps) -> DynsydardMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
