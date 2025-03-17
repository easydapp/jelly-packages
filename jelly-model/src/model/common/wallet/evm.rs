use serde::{Deserialize, Serialize};

use crate::model::types::evm::EvmChain;

use super::WalletEmptySettings;

/// EVM supported wallet
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum EvmWallet {
    /// any wallet
    #[serde(rename = "any")]
    Any(WalletEmptySettings),
    /// metamask
    #[serde(rename = "metamask")]
    MetaMask(WalletEmptySettings),
    /// rainbow
    #[serde(rename = "rainbow")]
    Rainbow(WalletEmptySettings),
}

impl EvmWallet {
    /// Text name
    pub fn text(&self) -> &str {
        match self {
            EvmWallet::Any(_) => "any",
            EvmWallet::MetaMask(_) => "metamask",
            EvmWallet::Rainbow(_) => "rainbow",
        }
    }

    /// Get the EVM chain supported by the wallet
    pub fn get_supported_chain(&self) -> &'static [EvmChain] {
        use EvmChain::*;
        match self {
            EvmWallet::Any(_) => &[
                Ethereum,
                EthereumTestnetSepolia,
                BinanceSmartChain,
                BinanceSmartChainTestnet,
                HashKeyChain,
                HashKeyChainTestnet,
                Polygon,
                PolygonTestnetAmoy,
            ],
            EvmWallet::MetaMask(_) => &[
                Ethereum,
                EthereumTestnetSepolia,
                BinanceSmartChain,
                BinanceSmartChainTestnet,
                HashKeyChain,
                HashKeyChainTestnet,
                Polygon,
                PolygonTestnetAmoy,
            ],
            EvmWallet::Rainbow(_) => &[
                Ethereum,
                EthereumTestnetSepolia,
                BinanceSmartChain,
                BinanceSmartChainTestnet,
                HashKeyChain,
                HashKeyChainTestnet,
                Polygon,
                PolygonTestnetAmoy,
            ],
        }
    }
}
