use serde::{Deserialize, Serialize};

/// chains
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CallChain {
    /// http
    #[serde(rename = "http")]
    Http,
    /// ic
    #[serde(rename = "ic")]
    InternetComputer,

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
