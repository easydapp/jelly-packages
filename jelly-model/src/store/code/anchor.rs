use candid::Principal;

use crate::{
    common::check::check_hash_anchor,
    types::{HashIdentity, StringIdentity},
};

use super::CodeData;

/// Prefix
const PREFIX: &str = "code";

/// share id
pub type CodeDataAnchor = StringIdentity<CodeData>;

/// hash id
pub type CodeDataHash = HashIdentity<CodeData>;

/// parsed id
pub struct CodeDataParsedId {
    /// canister
    pub canister_id: Principal,
    /// hash
    pub hash: CodeDataHash,
}

impl TryFrom<&str> for CodeDataParsedId {
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

impl From<&CodeDataParsedId> for CodeDataAnchor {
    fn from(value: &CodeDataParsedId) -> Self {
        Self::from(format!(
            "{PREFIX}#{}#{}",
            value.canister_id.to_text(),
            hex::encode(value.hash.as_ref())
        ))
    }
}

impl CodeDataParsedId {
    /// new
    pub fn from(canister_id: Principal, hash: CodeDataHash) -> Self {
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
