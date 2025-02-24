use std::borrow::Cow;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArrayLinkValue, CheckedCodeItem, CodeDataAnchor, ComponentId, InputValue, LinkError, LinkType,
    LinkValue,
};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Integer; // The index of the output selection
}

/// choose metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionChooseTipMetadata {
    /// Support
    values: InputValue, // The introduction must be a string array

    /// Prompt value
    #[serde(skip_serializing_if = "Option::is_none")]
    tips: Option<InputValue>, // A prompt corresponding to each option

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InteractionChooseTipMetadata {
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
        fn error(from: ComponentId, message: &str) -> LinkError {
            LinkError::InvalidInteractionComponent((from, message.into()).into())
        }

        // 0 Check whether the reference is matched
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        // 1. check values
        {
            let values = endpoints.check_input_value(&self.values, from)?;
            if !values.is_array_text() {
                return Err(error(from, "unsupported type for choose tip component"));
            }

            if let InputValue::Const(LinkValue::Array(ArrayLinkValue { values, .. })) = &self.values {
                if values.is_empty() {
                    return Err(LinkError::InvalidInteractionComponent(
                        (from, "interaction choose tips: must has values".into()).into(),
                    ));
                }
            }
        }

        // 2. check tips
        {
            if let Some(tips) = &self.tips {
                let tips = endpoints.check_input_value(tips, from)?;
                if !tips.is_array_text() {
                    return Err(error(from, "unsupported tips type for choose tip component"));
                }
            }
        }

        Ok(self.clone())
    }
}
