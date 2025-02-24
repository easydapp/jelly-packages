#[cfg(feature = "validate")]
use candid::Principal;
use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use crate::model::common::{error::LinkError, identity::ComponentId, types::LinkType, values::LinkValue};
use crate::model::types::check::CheckedCodeItem;

/// Principal template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidatePrincipal {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
}

impl NodeTemplateValidatePrincipal {
    /// get code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        vec![]
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate(&self, output: &LinkType, value: &LinkValue, from: ComponentId) -> Result<(), LinkError> {
        use std::str::FromStr;

        if !output.is_text() {
            return Err(LinkError::WrongConstValue(
                (from, "principal template must has text output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Text(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "principal template must has text value".into()).into(),
                    ))
                }
            };

            if Principal::from_str(&value).is_err() {
                return Err(LinkError::WrongConstValue(
                    (from, format!("required principal: {value}")).into(),
                ));
            }
        }

        Ok(())
    }
}
