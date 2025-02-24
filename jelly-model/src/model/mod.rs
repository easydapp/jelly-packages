use serde::{Deserialize, Serialize};

use node::TrimmedNode;
use types::check::CombinedOriginApis;

/// types
pub mod types;

/// common
pub mod common;

/// component
pub mod components;

/// combined
pub mod combined;

/// node
pub mod node;

/// check
pub mod check;

/// link component
pub use components::LinkComponent;

/// link Combined metadata
pub use combined::CombinedMetadata;

/// Edit specific content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombinedJson {
    /// Version
    pub version: String,
    /// content
    pub components: Vec<LinkComponent>,
    /// origin api
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_apis: Option<CombinedOriginApis>,
    /// node
    pub nodes: Vec<TrimmedNode>,
}
