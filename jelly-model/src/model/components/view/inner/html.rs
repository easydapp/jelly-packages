use serde::{Deserialize, Serialize};

use super::{ComponentId, InnerViewImageMetadata, InnerViewTextMetadata, LinkError, LinkType};

/// inner view html
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InnerViewHtmlMetadata {
    /// Template code
    pub template: String,
    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InnerViewHtmlMetadata {
    /// Get support type
    pub fn is_supported_type(template: &str, ty: &LinkType) -> bool {
        if let LinkType::Object(subitems) = ty {
            // Replace the picture
            let image = if let Some(image) = subitems
                .iter()
                .find(|subitem| subitem.key == "image")
                .map(|subitem| &subitem.ty)
            {
                // Every field in the image value must support pictures
                if let LinkType::Object(subitems) = image {
                    for item in subitems {
                        if !InnerViewImageMetadata::is_supported_type(&item.ty) {
                            return false;
                        }
                    }
                    subitems.iter().map(|t| t.key.clone()).collect()
                } else {
                    return false;
                }
            } else {
                vec![]
            };

            // Replace text
            let text = if let Some(text) = subitems
                .iter()
                .find(|subitem| subitem.key == "text")
                .map(|subitem| &subitem.ty)
            {
                // Each Field in the text value must support text
                if let LinkType::Object(subitems) = text {
                    for item in subitems {
                        if !InnerViewTextMetadata::is_supported_type(&item.ty) {
                            return false;
                        }
                    }
                    subitems.iter().map(|t| t.key.clone()).collect()
                } else {
                    return false;
                }
            } else {
                vec![]
            };

            return Self::is_template_support_keys(0.into(), template, &image, &text).is_ok();
        }

        false
    }

    /// Get support type
    pub fn is_template_support_keys(
        from: ComponentId,
        template: &str,
        image: &[String], // Pictures that need to be replaced
        text: &[String],  // Text that needs to be replaced
    ) -> Result<(), LinkError> {
        fn error(from: ComponentId, message: &str) -> LinkError {
            LinkError::InvalidViewComponent((from, message.into()).into())
        }

        LinkType::check_keys(image.iter(), from)?;
        LinkType::check_keys(text.iter(), from)?;

        // Key cannot cross
        for name in image {
            if text.contains(name) {
                return Err(error(from, &format!("duplicate value: {name}")));
            }
        }
        for name in text {
            if image.contains(name) {
                return Err(error(from, &format!("duplicate value: {name}")));
            }
        }

        // Check whether the template contains the corresponding key
        for name in image {
            // The picture must be introduced in the way src="${xxx}"
            if !template.contains(&format!("src=\"${{{name}}}\"")) {
                return Err(error(from, &format!("missing image value: {name}")));
            }
        }
        for name in text {
            // The text must be introduced by $ {xxx}
            if !template.contains(&format!("${{{name}}}")) {
                return Err(error(from, &format!("missing text value: {name}")));
            }
        }

        // Cross -site script attack (XSS)
        if template.contains("script") {
            return Err(error(from, "word script is not support."));
        }
        // ! char > need to be replaced to &gt
        // ! char < need to be replaced to &lt

        Ok(())
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        Self::is_template_support_keys(from, &self.template, &[], &[])?;
        Ok(())
    }
}
