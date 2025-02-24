use std::{borrow::Cow, collections::HashSet};

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::types::TimestampMills;

/// anchor
pub mod anchor;

/// link
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Combined {
    /// canister and components hash. combined#aaaaa-aa#abcd
    pub anchor: anchor::CombinedAnchor,

    created: TimestampMills,

    /// Call the number of times, update after each running
    pub called: u64,

    version: String,

    /// Specific core content
    pub components: Vec<crate::model::LinkComponent>,

    /// chains
    #[serde(skip_serializing_if = "crate::is_empty_option_set")]
    pub chains: Option<HashSet<crate::types::CallChain>>,

    /// Metadata
    #[serde(skip_serializing_if = "crate::model::CombinedMetadata::is_metadata_empty")]
    pub metadata: Option<crate::model::CombinedMetadata>, // Record interaction component
}

impl Storable for Combined {
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

impl Combined {
    /// new
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        anchor: anchor::CombinedAnchor,

        created: TimestampMills,

        called: u64,

        version: String,

        components: Vec<crate::model::LinkComponent>,

        chains: Option<HashSet<crate::types::CallChain>>,

        metadata: Option<crate::model::CombinedMetadata>,
    ) -> Self {
        Self {
            anchor,
            created,
            called,
            version,
            components,
            chains,
            metadata,
        }
    }
}
