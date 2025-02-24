use serde::{Deserialize, Serialize};

use crate::{
    common::hash::hash_sha256,
    model::types::check::CheckFunction,
    types::{ContentHash, MAX_DATA_LENGTH},
};

/// Single interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SingleEvmApi {
    /// The only one interface
    pub api: String, // A single Abi item json
}

/// Original interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OriginEvmApi {
    /// origin abi
    pub abi: String, // Vec<AbiItem> json
    /// method
    pub name: String,
    /// find by index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
}

impl OriginEvmApi {
    /// Retrieve the original data
    pub fn restore<F: CheckFunction>(self, fetch: &F) -> Result<Self, String> {
        let abi = fetch.fetch_origin_api(&self.abi)?;
        if abi == self.abi {
            return Ok(self);
        }
        Ok(Self {
            abi: abi.to_string(),
            name: self.name,
            index: self.index,
        })
    }
}

/// EVM content
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmApi {
    /// Simply
    #[serde(rename = "single")]
    Single(SingleEvmApi),
    /// Origin
    #[serde(rename = "origin")]
    Origin(OriginEvmApi),
}

impl EvmApi {
    /// Retrieve the original data
    pub fn restore<F: CheckFunction>(self, fetch: &F) -> Result<Self, String> {
        Ok(match self {
            Self::Single(single) => Self::Single(single),
            Self::Origin(origin) => Self::Origin(origin.restore(fetch)?),
        })
    }

    /// Should it become anchor
    pub fn should_into_anchor(&self) -> bool {
        match &self {
            Self::Single(SingleEvmApi { api }) => MAX_DATA_LENGTH < api.len(),
            Self::Origin(OriginEvmApi { abi, .. }) => MAX_DATA_LENGTH < abi.len(),
        }
    }

    /// hash
    pub fn hash(&self) -> Result<ContentHash, String> {
        let hash = hash_sha256(&serde_json::to_string(self).map_err(|_| format!("serde evm api failed: {:?}", self))?);
        Ok(hash)
    }
}
