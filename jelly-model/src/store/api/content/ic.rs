use serde::{Deserialize, Serialize};

use crate::{
    common::hash::hash_sha256,
    model::types::check::CheckFunction,
    types::{ContentHash, MAX_DATA_LENGTH},
};

/// Single interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SingleInternetComputerApi {
    /// api Method name and restraint
    pub api: String,
}

/// Original interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OriginInternetComputerApi {
    /// origin candid
    pub candid: String,
    /// method
    pub method: String,
}

impl OriginInternetComputerApi {
    /// Retrieve the original data
    pub fn restore<F: CheckFunction>(self, fetch: &F) -> Result<Self, String> {
        let candid = fetch.fetch_origin_api(&self.candid)?;
        if candid == self.candid {
            return Ok(self);
        }
        Ok(Self {
            candid: candid.to_string(),
            method: self.method,
        })
    }
}

/// IC content
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum InternetComputerApi {
    /// Simply
    #[serde(rename = "single")]
    Single(SingleInternetComputerApi),

    /// origin
    #[serde(rename = "origin")]
    Origin(OriginInternetComputerApi),
}

impl InternetComputerApi {
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
            Self::Single(SingleInternetComputerApi { api }) => MAX_DATA_LENGTH < api.len(),
            Self::Origin(OriginInternetComputerApi { candid, .. }) => MAX_DATA_LENGTH < candid.len(),
        }
    }

    /// hash
    pub fn hash(&self) -> Result<ContentHash, String> {
        let hash = hash_sha256(&serde_json::to_string(self).map_err(|_| format!("serde ic api failed: {:?}", self))?);
        Ok(hash)
    }
}
