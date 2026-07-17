use sydar_consensus_core::trusted::{TrustedHeader, TrustedsydarConsensusData};

use crate::convert::header::HeaderFormat;
use crate::pb as protowire;

// ----------------------------------------------------------------------------
// consensus_core to protowire
// ----------------------------------------------------------------------------

impl From<(HeaderFormat, &TrustedHeader)> for protowire::DaaBlockV4 {
    fn from(value: (HeaderFormat, &TrustedHeader)) -> Self {
        let (header_format, item) = value;
        Self { header: Some((header_format, &*item.header).into()), sydar_consensus_data: Some((&item.sydar_consensus).into()) }
    }
}

impl From<&TrustedsydarConsensusData> for protowire::BlocksydarConsensusDataHashPair {
    fn from(item: &TrustedsydarConsensusData) -> Self {
        Self { hash: Some(item.hash.into()), sydar_consensus_data: Some((&item.sydar_consensus).into()) }
    }
}
