use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::common::identity::ComponentId;

use super::{CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor, LinkError, LinkType, ValidateForm};

/// The data required by FORM
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ChooseFullForm {
    /// default value The default value of the input box, because there is a certain button, the default here is only auxiliary filling, waiting for the confirmation button to trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,

    /// Confirm the text displayed by the button
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<String>,

    /// Verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<ValidateForm>,
}

impl ChooseFullForm {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        if let Some(validate) = &self.validate {
            anchors.extend(validate.get_code_anchors());
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        output: &LinkType,
        from: &ComponentId,
        index: u32, // The code serial number of this component, and some components have multiple positions of code
        mark: &str, // mark
    ) -> Vec<CheckedCodeItem> {
        let mut codes = Vec::new();

        if let Some(validate) = &self.validate {
            codes.extend(validate.get_origin_codes(output, from, index, mark));
        }

        codes
    }

    /// check
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // 1. Check the confirmation button
        if let Some(confirm) = &self.confirm {
            if confirm.trim().is_empty() {
                return Err(LinkError::InvalidConfirmText { from });
            }
        }

        // 2. validate
        let mut validate = None;
        if let Some(validate_ref) = &self.validate {
            validate = Some(validate_ref.check(output, from, fetch, codes)?);
        }

        Ok(Self {
            default: self.default.clone(),
            confirm: self.confirm.clone(),
            validate,
        })
    }
}
