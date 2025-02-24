use candid::Principal;

use crate::{
    common::check::check_u64_anchor,
    types::{StringIdentity, U64Identity},
};

use super::Publisher;

/// Prefix
const PREFIX: &str = "publisher";

/// share id
pub type PublisherAnchor = StringIdentity<Publisher>;

/// hash id
pub type PublisherId = U64Identity<Publisher>;

/// parsed id
#[derive(Debug)]
pub struct PublisherParsedId {
    /// canister
    pub canister_id: Principal,
    /// id
    pub id: PublisherId,
}

impl TryFrom<&str> for PublisherParsedId {
    type Error = String;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (canister_id, id) = check_u64_anchor(value, PREFIX)?;
        Ok(Self {
            canister_id,
            id: id.into(),
        })
    }
}

impl From<&PublisherParsedId> for PublisherAnchor {
    fn from(value: &PublisherParsedId) -> Self {
        Self::from(format!(
            "{PREFIX}#{}#{}",
            value.canister_id.to_text(),
            value.id.as_ref()
        ))
    }
}

impl PublisherParsedId {
    /// new
    pub fn from(canister_id: Principal, id: PublisherId) -> Self {
        Self { canister_id, id }
    }

    /// check
    pub fn check_canister_id(&self, self_canister_id: &Principal) -> Result<(), String> {
        if self.canister_id != *self_canister_id {
            return Err("canister id is mismatched".into());
        }
        Ok(())
    }
}
