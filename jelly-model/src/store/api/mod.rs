use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

use crate::types::TimestampMills;

/// anchor
pub mod anchor;

/// content
pub mod content;

/// api
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiData {
    /// canister and content hash. api#aaaaa-aa#abcd
    pub anchor: anchor::ApiDataAnchor,

    /// Creation time
    pub created: TimestampMills,

    /// content
    pub content: content::ApiDataContent,
}

impl Storable for ApiData {
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
