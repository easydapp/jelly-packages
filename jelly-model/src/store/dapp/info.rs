use serde::{Deserialize, Serialize};

/// dapp info
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DappInfo {
    /// icon
    pub icon: String,
    /// name
    pub name: String,
    /// describe
    pub description: String,
    /// Social information
    pub social: String, // web twitter telegram discord
}
