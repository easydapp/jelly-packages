use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InnerViewBoolMetadata, InputValue, LinkError, LinkType};

/// view text
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewBoolMetadata {
    /// Introduced
    value: InputValue,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl ViewBoolMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        InnerViewBoolMetadata::is_supported_type(ty)
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId) -> LinkError {
            LinkError::InvalidViewComponent((from, "unsupported type for bool view".into()).into())
        }

        // 1. check value
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
        let output = endpoints.check_input_value(&self.value, from)?;
        if !self.is_supported_type(&output) {
            return Err(error(from));
        }

        Ok(self.clone())
    }
}
