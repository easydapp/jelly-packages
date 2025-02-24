use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InnerViewTextMetadata, InputValue, LinkError, LinkType};

/// view text
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewTextMetadata {
    /// Introduced
    pub value: InputValue,

    /// Can have a hyperlink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<InputValue>,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl ViewTextMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        InnerViewTextMetadata::is_supported_type(ty)
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId) -> LinkError {
            LinkError::InvalidViewComponent((from, "unsupported type for text view".into()).into())
        }

        // 1. check value
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
        let output = endpoints.check_input_value(&self.value, from)?;
        if !self.is_supported_type(&output) {
            return Err(error(from));
        }

        // 2. Check the hyperlink
        if let Some(href) = &self.href {
            href.check_text_input_value(
                &endpoints,
                |href| href.starts_with("https://"),
                "href must start with https",
                "href must be text",
                "href must be text",
                LinkError::InvalidViewComponent,
                from,
            )?;
        }

        Ok(self.clone())
    }
}
