use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InnerViewObjectItem, InnerViewObjectMetadata, InputValue, LinkError, LinkType};

/// view object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewObjectMetadata {
    /// Introduced Must be a corresponding array
    value: InputValue,

    /// How to display each element
    inner: Vec<InnerViewObjectItem>,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl ViewObjectMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        InnerViewObjectMetadata::is_supported_type(&self.inner, ty)
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId) -> LinkError {
            LinkError::InvalidViewComponent((from, "unsupported type for object view".into()).into())
        }

        // 1. Not allowed to be empty
        if self.inner.is_empty() {
            return Err(error(from));
        }

        // 2. Check whether the key is effective and the corresponding child constraint is effective
        LinkType::check_keys(self.inner.iter().map(|item| &item.key), from)?;
        for inner in &self.inner {
            inner.inner.check(from)?;
        }

        // 3. Check Value
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
        let output = endpoints.check_input_value(&self.value, from)?;
        if !self.is_supported_type(&output) {
            return Err(error(from));
        }

        Ok(self.clone())
    }
}
