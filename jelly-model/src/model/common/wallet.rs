use serde::{Deserialize, Serialize};

/// ic wallet
pub mod ic;

/// evm wallet
pub mod evm;

/// Empty settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct WalletEmptySettings {}
