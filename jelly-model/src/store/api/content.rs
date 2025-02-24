use serde::{Deserialize, Serialize};

/// ic
pub mod ic;

/// evm
pub mod evm;

/// content
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ApiDataContent {
    /// ic
    #[serde(rename = "ic")]
    InternetComputer(ic::InternetComputerApi),

    /// evm
    #[serde(rename = "evm")]
    Evm(evm::EvmApi),
}
