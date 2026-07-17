use crate::pb::sydard_message::Payload as sydardMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum sydardMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
    BlockBody,
    RequestBlockBodies,
}

impl From<&sydardMessagePayload> for sydardMessagePayloadType {
    fn from(payload: &sydardMessagePayload) -> Self {
        match payload {
            sydardMessagePayload::Addresses(_) => sydardMessagePayloadType::Addresses,
            sydardMessagePayload::Block(_) => sydardMessagePayloadType::Block,
            sydardMessagePayload::Transaction(_) => sydardMessagePayloadType::Transaction,
            sydardMessagePayload::BlockLocator(_) => sydardMessagePayloadType::BlockLocator,
            sydardMessagePayload::RequestAddresses(_) => sydardMessagePayloadType::RequestAddresses,
            sydardMessagePayload::RequestRelayBlocks(_) => sydardMessagePayloadType::RequestRelayBlocks,
            sydardMessagePayload::RequestTransactions(_) => sydardMessagePayloadType::RequestTransactions,
            sydardMessagePayload::IbdBlock(_) => sydardMessagePayloadType::IbdBlock,
            sydardMessagePayload::InvRelayBlock(_) => sydardMessagePayloadType::InvRelayBlock,
            sydardMessagePayload::InvTransactions(_) => sydardMessagePayloadType::InvTransactions,
            sydardMessagePayload::Ping(_) => sydardMessagePayloadType::Ping,
            sydardMessagePayload::Pong(_) => sydardMessagePayloadType::Pong,
            sydardMessagePayload::Verack(_) => sydardMessagePayloadType::Verack,
            sydardMessagePayload::Version(_) => sydardMessagePayloadType::Version,
            sydardMessagePayload::TransactionNotFound(_) => sydardMessagePayloadType::TransactionNotFound,
            sydardMessagePayload::Reject(_) => sydardMessagePayloadType::Reject,
            sydardMessagePayload::PruningPointUtxoSetChunk(_) => sydardMessagePayloadType::PruningPointUtxoSetChunk,
            sydardMessagePayload::RequestIbdBlocks(_) => sydardMessagePayloadType::RequestIbdBlocks,
            sydardMessagePayload::UnexpectedPruningPoint(_) => sydardMessagePayloadType::UnexpectedPruningPoint,
            sydardMessagePayload::IbdBlockLocator(_) => sydardMessagePayloadType::IbdBlockLocator,
            sydardMessagePayload::IbdBlockLocatorHighestHash(_) => sydardMessagePayloadType::IbdBlockLocatorHighestHash,
            sydardMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                sydardMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            sydardMessagePayload::DonePruningPointUtxoSetChunks(_) => sydardMessagePayloadType::DonePruningPointUtxoSetChunks,
            sydardMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                sydardMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            sydardMessagePayload::BlockWithTrustedData(_) => sydardMessagePayloadType::BlockWithTrustedData,
            sydardMessagePayload::DoneBlocksWithTrustedData(_) => sydardMessagePayloadType::DoneBlocksWithTrustedData,
            sydardMessagePayload::RequestPruningPointAndItsAnticone(_) => {
                sydardMessagePayloadType::RequestPruningPointAndItsAnticone
            }
            sydardMessagePayload::BlockHeaders(_) => sydardMessagePayloadType::BlockHeaders,
            sydardMessagePayload::RequestNextHeaders(_) => sydardMessagePayloadType::RequestNextHeaders,
            sydardMessagePayload::DoneHeaders(_) => sydardMessagePayloadType::DoneHeaders,
            sydardMessagePayload::RequestPruningPointUtxoSet(_) => sydardMessagePayloadType::RequestPruningPointUtxoSet,
            sydardMessagePayload::RequestHeaders(_) => sydardMessagePayloadType::RequestHeaders,
            sydardMessagePayload::RequestBlockLocator(_) => sydardMessagePayloadType::RequestBlockLocator,
            sydardMessagePayload::PruningPoints(_) => sydardMessagePayloadType::PruningPoints,
            sydardMessagePayload::RequestPruningPointProof(_) => sydardMessagePayloadType::RequestPruningPointProof,
            sydardMessagePayload::PruningPointProof(_) => sydardMessagePayloadType::PruningPointProof,
            sydardMessagePayload::Ready(_) => sydardMessagePayloadType::Ready,
            sydardMessagePayload::BlockWithTrustedDataV4(_) => sydardMessagePayloadType::BlockWithTrustedDataV4,
            sydardMessagePayload::TrustedData(_) => sydardMessagePayloadType::TrustedData,
            sydardMessagePayload::RequestIbdChainBlockLocator(_) => sydardMessagePayloadType::RequestIbdChainBlockLocator,
            sydardMessagePayload::IbdChainBlockLocator(_) => sydardMessagePayloadType::IbdChainBlockLocator,
            sydardMessagePayload::RequestAntipast(_) => sydardMessagePayloadType::RequestAntipast,
            sydardMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                sydardMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
            sydardMessagePayload::BlockBody(_) => sydardMessagePayloadType::BlockBody,
            sydardMessagePayload::RequestBlockBodies(_) => sydardMessagePayloadType::RequestBlockBodies,
        }
    }
}
