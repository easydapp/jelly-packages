use serde::{Deserialize, Serialize};

use crate::types::CallChain;

/// EVM compatible chain
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmChain {
    /// Ethereum
    #[serde(rename = "ethereum")]
    Ethereum, // chain id -> 1
    /// Ethereum test
    #[serde(rename = "ethereum-test-sepolia")]
    EthereumTestnetSepolia, // chain id -> 11155111

    /// BSC
    #[serde(rename = "bsc")]
    BinanceSmartChain, // chain id -> 56
    /// BSC test
    #[serde(rename = "bsc-test")]
    BinanceSmartChainTestnet, // chain id -> 97

    /// HashKey
    #[serde(rename = "hsk")]
    HashKeyChain, // chain id -> 177
    /// HashKey test
    #[serde(rename = "hsk-test")]
    HashKeyChainTestnet, // chain id -> 133

    /// Polygon
    #[serde(rename = "polygon")]
    Polygon, // chain id -> 137
    /// Polygon test
    #[serde(rename = "polygon-test-amoy")]
    PolygonTestnetAmoy, // chain id -> 80002
}

impl EvmChain {
    /// get call chain
    pub fn get_call_chain(&self) -> CallChain {
        match &self {
            EvmChain::Ethereum => CallChain::Ethereum,
            EvmChain::EthereumTestnetSepolia => CallChain::EthereumTestnetSepolia,
            EvmChain::BinanceSmartChain => CallChain::BinanceSmartChain,
            EvmChain::BinanceSmartChainTestnet => CallChain::BinanceSmartChainTestnet,
            EvmChain::HashKeyChain => CallChain::HashKeyChain,
            EvmChain::HashKeyChainTestnet => CallChain::HashKeyChainTestnet,
            EvmChain::Polygon => CallChain::Polygon,
            EvmChain::PolygonTestnetAmoy => CallChain::PolygonTestnetAmoy,
        }
    }
}
