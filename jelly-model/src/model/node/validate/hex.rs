use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use crate::model::common::{error::LinkError, identity::ComponentId, types::LinkType, values::LinkValue};
use crate::model::types::check::CheckedCodeItem;

/// hex template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateHex {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,

    /// min length
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u32>,
    /// max length
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u32>,
}

impl NodeTemplateValidateHex {
    /// get code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        vec![]
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate(&self, output: &LinkType, value: &LinkValue, from: ComponentId) -> Result<(), LinkError> {
        use crate::common::check::is_valid_hex_text;

        if !output.is_text() {
            return Err(LinkError::WrongConstValue(
                (from, "hex template must has text output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Text(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "hex template must has text value".into()).into(),
                    ))
                }
            };

            if !is_valid_hex_text(value) {
                return Err(LinkError::WrongConstValue(
                    (from, format!("required hex data: {value}")).into(),
                ));
            }

            let length = value.len() / 2;

            if let Some(min_length) = self.min_length {
                if length < min_length as usize {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required min length: {min_length}")).into(),
                    ));
                }
            }

            if let Some(max_length) = self.max_length {
                if length > max_length as usize {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required max length: {max_length}")).into(),
                    ));
                }
            }
        }

        Ok(())
    }
}
