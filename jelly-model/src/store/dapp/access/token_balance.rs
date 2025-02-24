use serde::{Deserialize, Serialize};

/// Token balance
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TokenBalance {
    /// chain
    pub chain: super::chain::Chain,
    /// Main currency or designated token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>, // Specify tokens, or main currency
    /// Minimum quota
    pub balance: u64, // Minimum ownership
}

/// Tokens balance verification
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct VerifiedTokenBalance {
    /// chain
    pub chain: super::chain::Chain,
    /// Main currency or designated token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>, // Specify tokens, or main currency
    /// Minimum quota
    pub balance: u64, // Minimum ownership

    // =============== Verification information ===============
    /// Identity address
    pub identity: String,
    /// Signature information
    pub message: String,
    /// signature
    pub signature: String,
}
