use serde::{Deserialize, Serialize};

use super::{ComponentId, InnerViewMetadata, LinkError, LinkType};

/// inner view array
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewArrayMetadata {
    /// How to display each element
    pub inner: Box<InnerViewMetadata>,
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewArrayMetadata {
    /// Get support type
    pub fn is_supported_type(inner: &InnerViewMetadata, ty: &LinkType) -> bool {
        if let LinkType::Array(subtype) = ty {
            return inner.is_supported_type(subtype);
        }
        false
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        self.inner.check(from)?;
        Ok(())
    }
}
