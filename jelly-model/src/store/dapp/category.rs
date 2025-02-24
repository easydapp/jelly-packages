use serde::{Deserialize, Serialize};

/// DAPP classification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DappCategory {
    /// token
    #[serde(rename = "Token")]
    Token,
    /// nft
    #[serde(rename = "NFT")]
    NFT,
    /// defi
    #[serde(rename = "DeFi")]
    DeFi,
    /// game
    #[serde(rename = "Game")]
    Game,
    /// tool
    #[serde(rename = "Tools")]
    Tools,
    /// other
    #[serde(rename = "Others")]
    Others,
}
