use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{LinkError, LinkType};

lazy_static! {
    static ref IMAGE_SUPPORTED_LINK_TYPES: Vec<LinkType> = vec![
        LinkType::Text,  // Text HTTPS Starting
        LinkType::Array(Box::new(LinkType::Integer)), // Binary picture
    ]; // Support type
}

/// inner view image
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewImageMetadata {
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewImageMetadata {
    /// Get support type
    fn supported_types() -> &'static [LinkType] {
        &IMAGE_SUPPORTED_LINK_TYPES
    }

    /// Get support type
    pub fn is_supported_type(ty: &LinkType) -> bool {
        Self::supported_types().contains(ty)
    }

    /// check
    pub fn check(&self) -> Result<(), LinkError> {
        Ok(())
    }

    /// Check the constant value
    pub fn check_value(value: &str) -> bool {
        value.starts_with("https://") || value.starts_with("data:image/")
    }
}
