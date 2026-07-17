//!
//!  Key-related type aliases used by the wallet framework.
//!

use std::sync::Arc;

pub type ExtendedPublicKeyDilithium = sydar_bip32::ExtendedPublicKey<sydar_bip32::DilithiumPkHash>;

pub type ExtendedPublicKeys = Arc<Vec<ExtendedPublicKeyDilithium>>;
