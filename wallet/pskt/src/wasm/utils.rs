use sydar_consensus_core::constants::*;
use sydar_consensus_core::network::NetworkType;
use separator::{Separatable, separated_float, separated_int, separated_uint_with_output};

#[inline]
pub fn kana_to_sydar(kana: u64) -> f64 {
    kana as f64 / KANA_PER_sydar as f64
}

#[inline]
pub fn sydar_to_kana(sydar: f64) -> u64 {
    (sydar * KANA_PER_sydar as f64) as u64
}

#[inline]
pub fn kana_to_sydar_string(kana: u64) -> String {
    kana_to_sydar(kana).separated_string()
}

#[inline]
pub fn kana_to_sydar_string_with_trailing_zeroes(kana: u64) -> String {
    separated_float!(format!("{:.8}", kana_to_sydar(kana)))
}

pub fn sydar_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "CSM",
        NetworkType::Testnet => "TKAS",
        NetworkType::Simnet => "SKAS",
        NetworkType::Devnet => "DKAS",
    }
}

#[inline]
pub fn kana_to_sydar_string_with_suffix(kana: u64, network_type: &NetworkType) -> String {
    let kas = kana_to_sydar_string(kana);
    let suffix = sydar_suffix(network_type);
    format!("{kas} {suffix}")
}
