use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use crate::model::common::{error::LinkError, identity::ComponentId, types::LinkType, values::LinkValue};
use crate::model::types::check::CheckedCodeItem;

/// Address template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateEvmAddress {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
}

impl NodeTemplateValidateEvmAddress {
    /// get code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        vec![]
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate(&self, output: &LinkType, value: &LinkValue, from: ComponentId) -> Result<(), LinkError> {
        use crate::common::check::is_valid_evm_address;

        if !output.is_text() {
            return Err(LinkError::WrongConstValue(
                (from, "evm address template must has text output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Text(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "evm address template must has text value".into()).into(),
                    ))
                }
            };

            if !is_valid_evm_address(value) {
                return Err(LinkError::WrongConstValue(
                    (from, format!("required evm address: {value}")).into(),
                ));
            }
        }

        Ok(())
    }
}
