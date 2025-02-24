use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InnerViewImageMetadata, InputValue, LinkError, LinkType, LinkValue};

/// view image
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewImageMetadata {
    /// Introduced
    value: InputValue,

    /// Can have a hyperlink
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<InputValue>,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl ViewImageMetadata {
    /// Get support type
    pub fn is_supported_type(&self, ty: &LinkType) -> bool {
        InnerViewImageMetadata::is_supported_type(ty)
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId, message: &str) -> LinkError {
            LinkError::InvalidViewComponent((from, message.into()).into())
        }

        // 1. check value
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
        let output = endpoints.check_input_value(&self.value, from)?;
        if !self.is_supported_type(&output) {
            return Err(error(from, "unsupported type for image view"));
        }

        // 2. If it is a constant, you need to check the content
        if let InputValue::Const(LinkValue::Text(constant)) = &self.value {
            if !InnerViewImageMetadata::check_value(constant) {
                return Err(error(from, "invalid image url"));
            }
        }

        // 3. Check the hyperlink
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
