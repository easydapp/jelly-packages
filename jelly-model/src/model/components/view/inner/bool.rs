use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{LinkError, LinkType};

lazy_static! {
    static ref BOOL_SUPPORTED_LINK_TYPES: Vec<LinkType> = vec![
        LinkType::Bool,  // Boolean
    ]; // Support type
}

/// inner view bool
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewBoolMetadata {
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewBoolMetadata {
    /// Get support type
    fn supported_types() -> &'static [LinkType] {
        &BOOL_SUPPORTED_LINK_TYPES
    }

    /// Get support type
    pub fn is_supported_type(ty: &LinkType) -> bool {
        Self::supported_types().contains(ty)
    }

    /// check
    pub fn check(&self) -> Result<(), LinkError> {
        Ok(())
    }
}
