use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArrayLinkValue, CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor, ComponentId, InputValue,
    LinkError, LinkType, LinkValue, ValidateForm,
};

/// choose full form
pub mod form;

use form::ChooseFullForm;

lazy_static! {
    static ref INPUT_VALUE_TYPE: LinkType = LinkType::Array(Box::new(
        LinkType::object_builder()
            .push("option", LinkType::Text) // Display option
            .push("value", LinkType::Text) // The value corresponding to the option
            .build()
    ));

    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text; // Output text
}

/// choose full metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionChooseFullMetadata {
    /// Support
    values: InputValue, // must be { option: string; value: string }[]

    /// Can optional contain FORM
    #[serde(skip_serializing_if = "Option::is_none")]
    form: Option<ChooseFullForm>,

    /// Other auxiliary style data Such as Placeholder, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl InteractionChooseFullMetadata {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Option<Vec<CodeDataAnchor>> {
        let mut anchors = Vec::new();

        if let Some(form) = &self.form {
            anchors.extend(form.get_code_anchors());
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

        if let Some(form) = &self.form {
            codes.extend(form.get_origin_codes(
                &self.get_output_type(),
                from,
                0,
                "Interaction choose_full -> validate",
            ));
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
        fn error(from: ComponentId, message: &str) -> LinkError {
            LinkError::InvalidInteractionComponent((from, message.into()).into())
        }

        // 0 Check whether the reference is matched
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        // 1. check values
        {
            let values = endpoints.check_input_value(&self.values, from)?;
            if *values.as_ref() != *INPUT_VALUE_TYPE {
                return Err(error(from, "unsupported type for choose full component"));
            }
            if let InputValue::Const(LinkValue::Array(ArrayLinkValue { values, .. })) = &self.values {
                if values.is_empty() {
                    return Err(LinkError::InvalidInteractionComponent(
                        (from, "interaction choose full: must has values".into()).into(),
                    ));
                }
            }
        }

        // 2. check form
        let mut form = None;
        if let Some(form_ref) = &self.form {
            form = Some(form_ref.check(&OUTPUT_LINK_TYPE, from, fetch, codes)?);
        }

        Ok(Self {
            values: self.values.clone(),
            form,
            style: self.style.clone(),
        })
    }
}
