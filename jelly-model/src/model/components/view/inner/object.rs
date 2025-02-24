use serde::{Deserialize, Serialize};

use super::{ComponentId, InnerViewMetadata, LinkError, LinkType};

/// item
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewObjectItem {
    /// key
    pub key: String,
    /// How to display each element
    pub inner: InnerViewMetadata,
}

/// inner view object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewObjectMetadata {
    /// How to display each element
    pub inner: Vec<InnerViewObjectItem>,
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewObjectMetadata {
    /// Get support type
    pub fn is_supported_type(inner: &[InnerViewObjectItem], ty: &LinkType) -> bool {
        if let LinkType::Object(subitems) = ty {
            if inner.len() != subitems.len() {
                return false;
            }
            for (inner, subitem) in inner.iter().zip(subitems) {
                if inner.key != subitem.key {
                    return false;
                }
                if !inner.inner.is_supported_type(&subitem.ty) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        LinkType::check_keys(self.inner.iter().map(|item| &item.key), from)?;

        for inner in &self.inner {
            inner.inner.check(from)?;
        }

        Ok(())
    }
}
