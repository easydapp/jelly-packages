use candid::Principal;

use crate::{
    common::check::check_hash_anchor,
    types::{HashIdentity, StringIdentity},
};

use super::Combined;

/// Prefix
const PREFIX: &str = "combined";

/// share id
pub type CombinedAnchor = StringIdentity<Combined>;

/// hash id
pub type CombinedHash = HashIdentity<Combined>;

/// parsed id
pub struct CombinedParsedId {
    /// canister
    pub canister_id: Principal,
    /// hash
    pub hash: CombinedHash,
}

impl TryFrom<&str> for CombinedParsedId {
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

impl From<&CombinedParsedId> for CombinedAnchor {
    fn from(value: &CombinedParsedId) -> Self {
        Self::from(format!(
            "{PREFIX}#{}#{}",
            value.canister_id.to_text(),
            hex::encode(value.hash.as_ref())
        ))
    }
}

impl CombinedParsedId {
    /// new
    pub fn from(canister_id: Principal, hash: CombinedHash) -> Self {
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
