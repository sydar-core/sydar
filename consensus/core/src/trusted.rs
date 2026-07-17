use crate::{BlockHashMap, BlueWorkType, KType, block::Block, header::Header};
use sydar_hashes::Hash;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Represents semi-trusted externally provided sydarConsensus data (by a network peer)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalsydarConsensusData {
    pub blue_score: u64,
    pub blue_work: BlueWorkType,
    pub selected_parent: Hash,
    pub mergeset_blues: Vec<Hash>,
    pub mergeset_reds: Vec<Hash>,
    pub blues_anticone_sizes: BlockHashMap<KType>,
}

/// Represents an externally provided block with associated sydarConsensus data which
/// is only partially validated by the consensus layer. Note there is no actual trust
/// but rather these blocks are indirectly validated through the PoW mined over them
#[derive(Clone)]
pub struct TrustedBlock {
    pub block: Block,
    pub sydar_consensus: ExternalsydarConsensusData,
}

impl TrustedBlock {
    pub fn new(block: Block, sydar_consensus: ExternalsydarConsensusData) -> Self {
        Self { block, sydar_consensus }
    }
}

/// Represents an externally provided header with associated sydarConsensus data which
/// is only partially validated by the consensus layer. Note there is no actual trust
/// but rather these headers are indirectly validated through the PoW mined over them
pub struct TrustedHeader {
    pub header: Arc<Header>,
    pub sydar_consensus: ExternalsydarConsensusData,
}

impl TrustedHeader {
    pub fn new(header: Arc<Header>, sydar_consensus: ExternalsydarConsensusData) -> Self {
        Self { header, sydar_consensus }
    }
}

/// Represents externally provided sydarConsensus data associated with a block Hash
pub struct TrustedsydarConsensusData {
    pub hash: Hash,
    pub sydar_consensus: ExternalsydarConsensusData,
}

impl TrustedsydarConsensusData {
    pub fn new(hash: Hash, sydar_consensus: ExternalsydarConsensusData) -> Self {
        Self { hash, sydar_consensus }
    }
}
