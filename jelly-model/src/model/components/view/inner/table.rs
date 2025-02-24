use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{LinkError, LinkType};

lazy_static! {
    static ref TABLE_SUPPORTED_LINK_TYPES: Vec<LinkType> = vec![
        LinkType::object_builder()
            .push("headers", LinkType::Array(Box::new(LinkType::Text))) // Display header
            .push("rows", LinkType::Array(Box::new(LinkType::Array(Box::new(LinkType::Text))))) // Per line
            .build(),
    ]; // Support type
}

/// inner view table
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewTableMetadata {
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewTableMetadata {
    /// Get support type
    pub fn supported_types() -> &'static [LinkType] {
        &TABLE_SUPPORTED_LINK_TYPES
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
