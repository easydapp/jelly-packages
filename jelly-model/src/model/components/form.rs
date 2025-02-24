use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::model::common::refer::InputValue;

use super::{
    AllEndpoints, CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor, ComponentFormRequired, ComponentId,
    Endpoint, LinkError, LinkType, LinkValue, ValidateForm,
};

/// User form input
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentForm {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    #[serde(skip_serializing_if = "is_form_metadata_empty")]
    pub metadata: Option<FormMetadata>,

    /// Output type
    pub output: LinkType, // User specified type
}

/// form metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FormMetadata {
    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<LinkValue>,

    /// show suffix if need, must be text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<InputValue>,

    /// Verification code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<ValidateForm>,

    /// Other auxiliary styles such as Placeholder, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Determine whether it is necessary to serialize
fn is_form_metadata_empty(metadata: &Option<FormMetadata>) -> bool {
    !metadata.as_ref().is_some_and(|m| {
        m.name.is_some() || m.default.is_some() || m.suffix.is_some() || m.validate.is_some() || m.style.is_some()
    })
}

impl ComponentForm {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        if let Some(validate) = &self.metadata.as_ref().and_then(|m| m.validate.as_ref()) {
            anchors.extend(validate.get_code_anchors());
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        let mut codes = Vec::new();

        if let Some(validate) = &self.metadata.as_ref().and_then(|m| m.validate.as_ref()) {
            codes.extend(validate.get_origin_codes(&self.output, &self.id, 0, "Form -> validate"))
        }

        codes
    }

    /// Get the parameter name
    pub fn get_form_name(&self) -> Option<&String> {
        self.metadata.as_ref().and_then(|m| m.name.as_ref())
    }

    /// check
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        self.output.check(self.id)?; // ? Check whether the output type is correct

        // 1. Check the introduction variable
        if !matches!(
            (endpoints.as_ref(), self.inlets.as_ref()),
            (Some(_), Some(_)) | (None, None)
        ) {
            return Err(LinkError::MismatchedInlets { from: self.id });
        }

        let mut metadata = self.metadata.clone();

        // 2. Check metadata
        if let Some(metadata_ref) = &mut metadata {
            // 2.1 Check whether the silent value matching
            if let Some(default) = metadata_ref.default.as_ref() {
                if !self.output.is_match(default) {
                    return Err(LinkError::MismatchedFormDefaultValue {
                        from: self.id,
                        output: self.output.clone(),
                        value: default.clone(),
                    });
                }
            }

            // 2.2 Check suffix
            if let Some(suffix) = metadata_ref.suffix.as_ref() {
                let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();
                let suffix = endpoints.check_input_value(suffix, self.id)?;
                if !suffix.is_text() {
                    return Err(LinkError::MismatchedFormSuffixValue {
                        from: self.id,
                        value: suffix.as_ref().to_owned(),
                    });
                }
            }

            // 2.3 If there is a verification code
            if let Some(validate) = metadata_ref.validate.as_ref() {
                let validate = validate.check(&self.output, self.id, fetch, codes)?;

                // 2.2.2 Execute the verification code verification default value
                #[cfg(feature = "validate")]
                if let Some(default) = metadata_ref.default.as_ref() {
                    validate.validate(default, self.id, fetch)?;
                }

                metadata_ref.validate = Some(validate);
            }
        }

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata,
            output: self.output.clone(),
        })
    }

    /// Get the necessary information
    pub fn get_required(&self) -> ComponentFormRequired {
        ComponentFormRequired {
            id: self.id,
            name: self.metadata.as_ref().and_then(|m| m.name.clone()),
            output: self.output.clone(),
        }
    }
}
