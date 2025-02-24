use serde::{Deserialize, Serialize};

/// Chain identity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChainIdentity {
    /// chain
    pub chain: super::chain::Chain,
    /// Identity address
    pub identity: String,
}

/// Chain authentication
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VerifiedChainIdentity {
    /// chain
    pub chain: super::chain::Chain,
    /// Identity address
    pub identity: String,

    // =============== Verification information ===============
    /// Signature information
    pub message: String,
    /// signature
    pub signature: String,
}
