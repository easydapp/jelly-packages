use std::borrow::Cow;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{AllEndpoints, CheckedCodeItem, CodeDataAnchor, ComponentId, LinkError, LinkType, NamedValue};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text; // Output text
}

/// choose metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionChooseMetadata {
    /// Support
    values: Vec<NamedValue>,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InteractionChooseMetadata {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Option<Vec<CodeDataAnchor>> {
        None
    }

    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        Cow::Borrowed(&OUTPUT_LINK_TYPE)
    }

    /// get origin code
    pub fn get_origin_codes(&self) -> Option<Vec<CheckedCodeItem>> {
        None
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>, from: ComponentId) -> Result<Self, LinkError> {
        // 0 Check whether the reference is matched
        endpoints
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_default()
            .check_named_values(self.values.iter(), from, Some(LinkType::Text))?;

        // 1. check value
        match self.values.len() {
            0 => {
                return Err(LinkError::InvalidInteractionComponent(
                    (from, "interaction choose: must has values".into()).into(),
                ));
            }
            1 => {
                return Err(LinkError::InvalidInteractionComponent(
                    (from, "interaction choose: must has more then one value".into()).into(),
                ));
            }
            _ => {}
        }

        Ok(self.clone())
    }
}
