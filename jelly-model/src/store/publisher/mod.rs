use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

/// anchor
pub mod anchor;

/// User metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Publisher {
    /// canister and id. publisher#aaaaa-aa#1234
    pub anchor: anchor::PublisherAnchor,

    // Basic data
    /// avatar
    pub avatar: String,
    /// name
    pub name: String,
    /// Brief introduction
    pub bio: String,
    /// Social information
    pub social: String,
}

impl Storable for Publisher {
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

impl Publisher {}
