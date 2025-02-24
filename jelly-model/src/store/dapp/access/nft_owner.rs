use serde::{Deserialize, Serialize};

/// NFT owner
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NFTOwner {
    /// chain
    pub chain: super::chain::Chain,
    /// Specify NFT contract
    pub address: String,
    /// Any NFT or specified NFT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}

/// NFT owner
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VerifiedNFTOwner {
    /// chain
    pub chain: super::chain::Chain,
    /// Specify NFT contract
    pub address: String,
    /// Any NFT or specified NFT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,

    // =============== Verification information ===============
    /// Identity address
    pub identity: String,
    /// Signature information
    pub message: String,
    /// signature
    pub signature: String,
}
