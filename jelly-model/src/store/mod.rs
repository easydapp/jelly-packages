use api::anchor::{ApiDataAnchor, ApiDataHash};
use code::anchor::{CodeDataAnchor, CodeDataHash};
use combined::anchor::{CombinedAnchor, CombinedHash};
use dapp::anchor::{DappAnchor, DappId};
use publisher::anchor::{PublisherAnchor, PublisherId};

/// publisher
pub mod publisher;

/// code
pub mod code;

/// api
pub mod api;

/// combined
pub mod combined;

/// dapp
pub mod dapp;

// ====================== debug id ======================

macro_rules! debug_id {
    ($id_struct:ty, $name:expr) => {
        impl std::fmt::Debug for $id_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:?})", $name, self.as_ref())
            }
        }
    };
}

debug_id!(PublisherId, "PublisherId");
debug_id!(PublisherAnchor, "PublisherAnchor");

debug_id!(CodeDataHash, "CodeDataHash");
debug_id!(CodeDataAnchor, "CodeDataAnchor");

debug_id!(ApiDataHash, "ApiDataHash");
debug_id!(ApiDataAnchor, "ApiDataAnchor");

debug_id!(CombinedHash, "CombinedHash");
debug_id!(CombinedAnchor, "CombinedAnchor");

debug_id!(DappId, "DappId");
debug_id!(DappAnchor, "DappAnchor");
