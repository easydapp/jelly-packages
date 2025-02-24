use candid::Principal;

use crate::{
    common::check::check_hash_anchor,
    types::{HashIdentity, StringIdentity},
};

use super::ApiData;

/// Prefix
const PREFIX: &str = "api";

/// share id
pub type ApiDataAnchor = StringIdentity<ApiData>;

/// hash id
pub type ApiDataHash = HashIdentity<ApiData>;

/// parsed id
pub struct ApiDataParsedId {
    /// canister
    pub canister_id: Principal,
    /// hash
    pub hash: ApiDataHash,
}

impl TryFrom<&str> for ApiDataParsedId {
    type Error = String;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (canister_id, hash) = check_hash_anchor(value, PREFIX)?;
        Ok(Self {
            canister_id,
            hash: hash.into(),
        })
    }
}

impl From<&ApiDataParsedId> for ApiDataAnchor {
    fn from(value: &ApiDataParsedId) -> Self {
        Self::from(format!(
            "{PREFIX}#{}#{}",
            value.canister_id.to_text(),
            hex::encode(value.hash.as_ref())
        ))
    }
}

impl ApiDataParsedId {
    /// new
    pub fn from(canister_id: Principal, hash: ApiDataHash) -> Self {
        Self { canister_id, hash }
    }

    /// check
    pub fn check_canister_id(&self, self_canister_id: &Principal) -> Result<(), String> {
        if self.canister_id != *self_canister_id {
            return Err("canister id is mismatched".into());
        }
        Ok(())
    }
}
