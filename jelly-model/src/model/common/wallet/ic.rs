use serde::{Deserialize, Serialize};

use super::WalletEmptySettings;

/// IC supported wallet
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum IcWallet {
    /// any wallet
    #[serde(rename = "any")]
    Any(WalletEmptySettings),
    /// ii
    #[serde(rename = "ii")]
    InternetIdentity(WalletEmptySettings),
    /// plug
    #[serde(rename = "plug")]
    Plug(WalletEmptySettings),
    /// me
    #[serde(rename = "me")]
    Me(WalletEmptySettings),
    /// bitfinity
    #[serde(rename = "bitfinity")]
    Bitfinity(WalletEmptySettings),
    /// nfid
    #[serde(rename = "nfid")]
    Nfid(WalletEmptySettings),
    /// stoic
    #[serde(rename = "stoic")]
    Stoic(WalletEmptySettings),
}

impl IcWallet {
    /// Text name
    pub fn text(&self) -> &str {
        match self {
            IcWallet::Any(_) => "any",
            IcWallet::InternetIdentity(_) => "ii",
            IcWallet::Plug(_) => "plug",
            IcWallet::Me(_) => "me",
            IcWallet::Bitfinity(_) => "bitfinity",
            IcWallet::Nfid(_) => "nfid",
            IcWallet::Stoic(_) => "stoic",
        }
    }
}

/// Empty settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct IcWalletInternetIdentitySettings {
    #[serde(rename = "derivationOrigin", skip_serializing_if = "Option::is_none")]
    derivation_origin: Option<String>,
}
