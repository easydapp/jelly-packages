use std::{borrow::Cow, collections::HashSet};

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::types::TimestampMills;

/// anchor
pub mod anchor;

/// Access right
pub mod access;

/// category
pub mod category;

/// info
pub mod info;

/// dapp
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dapp {
    /// share_id in App.  canister_id and id. base58 encode
    pub id: anchor::DappAnchor,

    /// Creation time
    pub created: TimestampMills,
    /// There is a update time for None
    updated: TimestampMills,
    /// Withdrawing or freezing time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<TimestampMills>, // Once frozen, it is not allowed to run
    /// Reasons for withdrawal or freezing
    pub reason: String, // Frozen and withdrawal requires reasons

    /// Access permission
    pub access: access::DappAccess,

    /// Number of access, each loading complete update
    pub accessed: u64,
    /// Call the number of times, update after each running
    pub called: u64,
    /// The number of collections, the back -end is updated regularly
    pub collected: u64,

    /// Classification
    pub category: category::DappCategory,

    /// Basic information
    pub info: info::DappInfo,

    /// Author information publisher#aaaaa-aa#123
    pub publisher: super::publisher::anchor::PublisherAnchor,

    /// combined anchor combined#aaaaa-aa#abcd
    pub combined: super::combined::anchor::CombinedAnchor,

    /// chains
    #[serde(skip_serializing_if = "crate::is_empty_option_set")]
    pub chains: Option<HashSet<crate::types::CallChain>>,

    /// metadata
    #[serde(skip_serializing_if = "crate::model::CombinedMetadata::is_metadata_empty")]
    pub metadata: Option<crate::model::CombinedMetadata>,
}

impl Storable for Dapp {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = vec![];
        #[allow(clippy::unwrap_used)] // ? SAFETY
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        #[allow(clippy::expect_used)] // ? SAFETY
        ciborium::de::from_reader(&bytes[..]).expect("deserialization must succeed.")
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Dapp {
    /// new
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        id: anchor::DappAnchor, // share_id in App.  canister_id and id. base58 encode

        created: TimestampMills,
        updated: TimestampMills,
        frozen: Option<TimestampMills>,
        reason: String,

        access: access::DappAccess,

        accessed: u64,
        called: u64,
        collected: u64,

        category: category::DappCategory,

        info: info::DappInfo,

        publisher: super::publisher::anchor::PublisherAnchor,
        combined: super::combined::anchor::CombinedAnchor,
        chains: Option<HashSet<crate::types::CallChain>>,
        metadata: Option<crate::model::CombinedMetadata>,
    ) -> Self {
        Self {
            id,
            created,
            updated,
            frozen,
            reason,
            access,
            accessed,
            called,
            collected,
            category,
            info,
            publisher,
            combined,
            chains,
            metadata,
        }
    }
}

/// dapp
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DappView {
    /// share_id in App.  canister_id and id. base58 encode
    pub id: anchor::DappAnchor,

    /// Creation time
    pub created: TimestampMills,
    /// There is a update time for None
    updated: TimestampMills,
    /// Withdrawing or freezing time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<TimestampMills>, // Once frozen, it is not allowed to run
    /// Reasons for withdrawal or freezing
    pub reason: String, // Frozen and withdrawal requires reasons

    /// Access permission
    pub access: access::DappAccessView,

    /// Number of access, each loading complete update
    pub accessed: u64,
    /// Call the number of times, update after each running
    pub called: u64,
    /// The number of collections, the back -end is updated regularly
    pub collected: u64,

    /// Classification
    pub category: category::DappCategory,

    /// Basic information
    pub info: info::DappInfo,

    /// Author information publisher#aaaaa-aa#123
    pub publisher: super::publisher::anchor::PublisherAnchor,

    /// combined anchor combined#aaaaa-aa#abcd
    pub combined: super::combined::anchor::CombinedAnchor,

    /// chains
    #[serde(skip_serializing_if = "crate::is_empty_option_set")]
    pub chains: Option<HashSet<crate::types::CallChain>>,

    /// metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::model::CombinedMetadata>,
}

impl From<Dapp> for DappView {
    fn from(dapp: Dapp) -> Self {
        Self {
            id: dapp.id,
            created: dapp.created,
            updated: dapp.updated,
            frozen: dapp.frozen,
            reason: dapp.reason,
            access: dapp.access.into(),
            accessed: dapp.accessed,
            called: dapp.called,
            collected: dapp.collected,
            category: dapp.category,
            info: dapp.info,
            publisher: dapp.publisher,
            combined: dapp.combined,
            chains: dapp.chains,
            metadata: dapp.metadata,
        }
    }
}
