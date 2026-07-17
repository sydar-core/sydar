use crate::protowire::{sydardRequest, sydardResponse, sydard_request};

impl From<sydard_request::Payload> for sydardRequest {
    fn from(item: sydard_request::Payload) -> Self {
        sydardRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<sydardRequest> for sydardRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<sydardResponse> for sydardResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod sydard_request_convert {
    use crate::protowire::*;
    use sydar_rpc_core::{RpcError, RpcResult};

    impl_into_sydard_request!(Shutdown);
    impl_into_sydard_request!(SubmitBlock);
    impl_into_sydard_request!(GetBlockTemplate);
    impl_into_sydard_request!(GetBlock);
    impl_into_sydard_request!(GetInfo);

    impl_into_sydard_request!(GetCurrentNetwork);
    impl_into_sydard_request!(GetPeerAddresses);
    impl_into_sydard_request!(GetSink);
    impl_into_sydard_request!(GetMempoolEntry);
    impl_into_sydard_request!(GetMempoolEntries);
    impl_into_sydard_request!(GetConnectedPeerInfo);
    impl_into_sydard_request!(AddPeer);
    impl_into_sydard_request!(SubmitTransaction);
    impl_into_sydard_request!(SubmitTransactionReplacement);
    impl_into_sydard_request!(SubmitAccountTransaction);
    impl_into_sydard_request!(GetSubnetwork);
    impl_into_sydard_request!(GetVirtualChainFromBlock);
    impl_into_sydard_request!(GetBlocks);
    impl_into_sydard_request!(GetBlockCount);
    impl_into_sydard_request!(GetsydarDagInfo);
    impl_into_sydard_request!(ResolveFinalityConflict);
    impl_into_sydard_request!(GetHeaders);
    impl_into_sydard_request!(GetUtxosByAddresses);
    impl_into_sydard_request!(GetBalanceByAddress);
    impl_into_sydard_request!(GetBalancesByAddresses);
    impl_into_sydard_request!(GetSinkBlueScore);
    impl_into_sydard_request!(Ban);
    impl_into_sydard_request!(Unban);
    impl_into_sydard_request!(EstimateNetworkHashesPerSecond);
    impl_into_sydard_request!(GetMempoolEntriesByAddresses);
    impl_into_sydard_request!(GetCoinSupply);
    impl_into_sydard_request!(Ping);
    impl_into_sydard_request!(GetMetrics);
    impl_into_sydard_request!(GetConnections);
    impl_into_sydard_request!(GetSystemInfo);
    impl_into_sydard_request!(GetServerInfo);
    impl_into_sydard_request!(GetSyncStatus);
    impl_into_sydard_request!(GetDaaScoreTimestampEstimate);
    impl_into_sydard_request!(GetFeeEstimate);
    impl_into_sydard_request!(GetFeeEstimateExperimental);
    impl_into_sydard_request!(GetCurrentBlockColor);
    impl_into_sydard_request!(GetUtxoReturnAddress);
    impl_into_sydard_request!(GetVirtualChainFromBlockV2);

    impl_into_sydard_request!(NotifyBlockAdded);
    impl_into_sydard_request!(NotifyNewBlockTemplate);
    impl_into_sydard_request!(NotifyUtxosChanged);
    impl_into_sydard_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_sydard_request!(NotifyFinalityConflict);
    impl_into_sydard_request!(NotifyVirtualDaaScoreChanged);
    impl_into_sydard_request!(NotifyVirtualChainChanged);
    impl_into_sydard_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_sydard_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_sydard_request_ex!(sydar_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_sydard_request;

    macro_rules! impl_into_sydard_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for sydard_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for sydardRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for sydard_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for sydardRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&sydard_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &sydard_request::Payload) -> RpcResult<Self> {
                    if let sydard_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&sydardRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &sydardRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("sydarRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for sydardRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(sydard_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for sydard_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    sydard_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_sydard_request_ex;
}

pub mod sydard_response_convert {
    use crate::protowire::*;
    use sydar_rpc_core::{RpcError, RpcResult};

    impl_into_sydard_response!(Shutdown);
    impl_into_sydard_response!(SubmitBlock);
    impl_into_sydard_response!(GetBlockTemplate);
    impl_into_sydard_response!(GetBlock);
    impl_into_sydard_response!(GetInfo);
    impl_into_sydard_response!(GetCurrentNetwork);

    impl_into_sydard_response!(GetPeerAddresses);
    impl_into_sydard_response!(GetSink);
    impl_into_sydard_response!(GetMempoolEntry);
    impl_into_sydard_response!(GetMempoolEntries);
    impl_into_sydard_response!(GetConnectedPeerInfo);
    impl_into_sydard_response!(AddPeer);
    impl_into_sydard_response!(SubmitTransaction);
    impl_into_sydard_response!(SubmitTransactionReplacement);
    impl_into_sydard_response!(SubmitAccountTransaction);
    impl_into_sydard_response!(GetSubnetwork);
    impl_into_sydard_response!(GetVirtualChainFromBlock);
    impl_into_sydard_response!(GetBlocks);
    impl_into_sydard_response!(GetBlockCount);
    impl_into_sydard_response!(GetsydarDagInfo);
    impl_into_sydard_response!(ResolveFinalityConflict);
    impl_into_sydard_response!(GetHeaders);
    impl_into_sydard_response!(GetUtxosByAddresses);
    impl_into_sydard_response!(GetBalanceByAddress);
    impl_into_sydard_response!(GetBalancesByAddresses);
    impl_into_sydard_response!(GetSinkBlueScore);
    impl_into_sydard_response!(Ban);
    impl_into_sydard_response!(Unban);
    impl_into_sydard_response!(EstimateNetworkHashesPerSecond);
    impl_into_sydard_response!(GetMempoolEntriesByAddresses);
    impl_into_sydard_response!(GetCoinSupply);
    impl_into_sydard_response!(Ping);
    impl_into_sydard_response!(GetMetrics);
    impl_into_sydard_response!(GetConnections);
    impl_into_sydard_response!(GetSystemInfo);
    impl_into_sydard_response!(GetServerInfo);
    impl_into_sydard_response!(GetSyncStatus);
    impl_into_sydard_response!(GetDaaScoreTimestampEstimate);
    impl_into_sydard_response!(GetFeeEstimate);
    impl_into_sydard_response!(GetFeeEstimateExperimental);
    impl_into_sydard_response!(GetCurrentBlockColor);
    impl_into_sydard_response!(GetUtxoReturnAddress);
    impl_into_sydard_response!(GetVirtualChainFromBlockV2);

    impl_into_sydard_notify_response!(NotifyBlockAdded);
    impl_into_sydard_notify_response!(NotifyNewBlockTemplate);
    impl_into_sydard_notify_response!(NotifyUtxosChanged);
    impl_into_sydard_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_sydard_notify_response!(NotifyFinalityConflict);
    impl_into_sydard_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_sydard_notify_response!(NotifyVirtualChainChanged);
    impl_into_sydard_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_sydard_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_sydard_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_sydard_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_sydard_response_ex!(sydar_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_sydard_response_base!(sydar_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_sydard_response;

    macro_rules! impl_into_sydard_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for sydard_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    sydard_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for sydardResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(sydard_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_sydard_response_base;

    macro_rules! impl_into_sydard_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for sydard_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    sydard_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for sydardResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for sydard_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    sydard_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for sydardResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_sydard_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&sydard_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &sydard_response::Payload) -> RpcResult<Self> {
                    if let sydard_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&sydardResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &sydardResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("sydarResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_sydard_response_ex;

    macro_rules! impl_into_sydard_notify_response {
        ($name:tt) => {
            impl_into_sydard_response!($name);

            paste::paste! {
                impl_into_sydard_notify_response_ex!(sydar_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_sydard_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_sydard_notify_response_ex!(sydar_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_sydard_notify_response;

    macro_rules! impl_into_sydard_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_sydard_notify_response_ex;
}
