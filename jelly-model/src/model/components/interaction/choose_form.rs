use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::model::common::identity::ComponentId;

use super::{
    AllEndpoints, CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor, LinkError, LinkType, NamedValue,
    ValidateForm,
};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text; // Output text
}

/// choose form metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionChooseFormMetadata {
    /// Support
    values: Vec<NamedValue>,

    /// default value The default value of the input box, because there is a certain button, the default here is only auxiliary filling, waiting for the confirmation button to trigger
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<String>,

    /// Confirm the text displayed by the button
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<String>,

    /// Verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<ValidateForm>,

    /// Other auxiliary style data for example placeholder etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InteractionChooseFormMetadata {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Option<Vec<CodeDataAnchor>> {
        let mut anchors = Vec::new();

        if let Some(validate) = &self.validate {
            anchors.extend(validate.get_code_anchors());
        }

        Some(anchors)
    }

    /// get output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        Cow::Borrowed(&OUTPUT_LINK_TYPE)
    }

    /// get origin code
    pub fn get_origin_codes(&self, from: &ComponentId) -> Option<Vec<CheckedCodeItem>> {
        let mut codes = Vec::new();

        if let Some(validate) = &self.validate {
            codes.extend(validate.get_origin_codes(
                &self.get_output_type(),
                from,
                0,
                "Interaction choose_form -> validate",
            ))
        }

        Some(codes)
    }

    /// check
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // 0 Check whether the reference is matched
        endpoints
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_default()
            .check_named_values(self.values.iter(), from, Some(LinkType::Text))?;

        // 1. check value

        // 2. Check the confirmation button
        if let Some(confirm) = &self.confirm {
            if confirm.trim().is_empty() {
                return Err(LinkError::InvalidConfirmText { from });
            }
        }

        // 3. Check the verification
        let mut validate = None;
        if let Some(validate_ref) = &self.validate {
            validate = Some(validate_ref.check(&OUTPUT_LINK_TYPE, from, fetch, codes)?);
        }

        Ok(Self {
            values: self.values.clone(),
            default: self.default.clone(),
            confirm: self.confirm.clone(),
            validate,
            style: self.style.clone(),
        })
    }
}
