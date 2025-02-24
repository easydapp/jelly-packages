use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use crate::model::common::{error::LinkError, identity::ComponentId, types::LinkType, values::LinkValue};
use crate::model::types::check::CheckedCodeItem;

/// Boole
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateBool {
    /// label
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    /// true
    #[serde(skip_serializing_if = "Option::is_none")]
    true_text: Option<String>,
    /// false
    #[serde(skip_serializing_if = "Option::is_none")]
    false_text: Option<String>,
}

impl NodeTemplateValidateBool {
    /// get code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        vec![]
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate(&self, output: &LinkType, value: &LinkValue, from: ComponentId) -> Result<(), LinkError> {
        if !output.is_bool() {
            return Err(LinkError::WrongConstValue(
                (from, "bool template must has bool output".into()).into(),
            ));
        }

        // Check value
        {
            if !matches!(value, LinkValue::Bool(_)) {
                return Err(LinkError::WrongConstValue(
                    (from, "bool template must has bool value".into()).into(),
                ));
            }
        }

        Ok(())
    }
}
