use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::model::components::view::inner::InnerViewTextMetadata;

use super::{AllEndpoints, CodeValue, ComponentId, InnerViewHtmlMetadata, InnerViewImageMetadata, LinkError, LinkType};

/// view html
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ViewHtmlMetadata {
    /// The value that needs to be inserted
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    image: Option<Vec<CodeValue>>,

    /// The value that needs to be inserted
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    text: Option<Vec<CodeValue>>,

    /// Template code
    template: String,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl ViewHtmlMetadata {
    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        fn error(from: ComponentId, message: &str) -> LinkError {
            LinkError::InvalidViewComponent((from, message.into()).into())
        }

        // Check the introduction of data
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        // 1. Key to replace the picture out
        let image = {
            let image =
                endpoints.check_code_values(self.image.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
            if let LinkType::Object(items) = image {
                for item in &items {
                    if !InnerViewImageMetadata::is_supported_type(&item.ty) {
                        return Err(error(from, "unsupported type for html(image) view"));
                    }
                }
                items.into_iter().map(|t| t.key).collect::<Vec<_>>()
            } else {
                return Err(error(from, "unsupported type for html(image) view"));
            }
        };

        // 2. Key to replace the text
        let text = {
            let text =
                endpoints.check_code_values(self.text.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
            if let LinkType::Object(items) = text {
                for item in &items {
                    if !InnerViewTextMetadata::is_supported_type(&item.ty) {
                        return Err(error(from, "unsupported type for html(text) view"));
                    }
                }
                items.into_iter().map(|t| t.key).collect::<Vec<_>>()
            } else {
                return Err(error(from, "unsupported type for html(text) view"));
            }
        };

        // 3. Check variables and templates
        InnerViewHtmlMetadata::is_template_support_keys(from, &self.template, &image, &text)?;

        Ok(self.clone())
    }
}
