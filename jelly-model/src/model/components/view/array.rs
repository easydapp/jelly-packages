use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InnerViewArrayMetadata, InnerViewMetadata, InputValue, LinkError, LinkType};

/// view array
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewArrayMetadata {
    /// Introduced Must be a corresponding array
    value: InputValue,

    /// How to display each element
    inner: InnerViewMetadata,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl ViewArrayMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        InnerViewArrayMetadata::is_supported_type(&self.inner, ty)
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId) -> LinkError {
            LinkError::InvalidViewComponent((from, "unsupported type for array view".into()).into())
        }

        // 1. Check the sub -type constraint
        self.inner.check(from)?;

        // 2. check value
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
        let output = endpoints.check_input_value(&self.value, from)?;
        if !self.is_supported_type(&output) {
            return Err(error(from));
        }

        Ok(self.clone())
    }
}
