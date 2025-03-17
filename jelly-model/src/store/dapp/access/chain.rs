use serde::{Deserialize, Serialize};

/// chain
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Chain {
    /// bitcoin
    #[serde(rename = "bitcoin")]
    Bitcoin,

    /// ic
    #[serde(rename = "ic")]
    InternetComputer, // ic principal

    /// ethereum
    #[serde(rename = "ethereum")]
    Ethereum,
    /// ethereum test
    #[serde(rename = "ethereum-test-sepolia")]
    EthereumTestnetSepolia,

    /// bsc
    #[serde(rename = "bsc")]
    BinanceSmartChain,
    /// bsc test
    #[serde(rename = "bsc-test")]
    BinanceSmartChainTestnet,

    /// hsk
    #[serde(rename = "hsk")]
    HashKeyChain,
    /// hsk test
    #[serde(rename = "hsk-test")]
    HashKeyChainTestnet,

    /// polygon
    #[serde(rename = "polygon")]
    Polygon,
    /// polygon test
    #[serde(rename = "polygon-test-amoy")]
    PolygonTestnetAmoy,
}
