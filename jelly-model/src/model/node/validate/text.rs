use serde::{Deserialize, Serialize};

#[cfg(feature = "validate")]
use crate::model::{
    common::{code::CodeContent, error::LinkError, values::LinkValue},
    types::check::CheckFunction,
};
use crate::model::{
    common::{identity::ComponentId, types::LinkType, validate::ValidateForm},
    types::check::CheckedCodeItem,
};

/// Text template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateText {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    /// suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    /// min length
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u32>,
    /// max length
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u32>,
    /// code like ValidateForm
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl NodeTemplateValidateText {
    /// get code
    pub fn get_origin_codes(&self, output: &LinkType, mut index: u32, from: ComponentId) -> Vec<CheckedCodeItem> {
        let mut codes = vec![];

        if let Some(code) = &self.code {
            let code = ValidateForm::get_validate_code_item(code.to_owned(), output);
            index += 1;
            codes.push(CheckedCodeItem::new(from, index, "Const Validate Code".into(), code));
        }

        codes
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate<F: CheckFunction>(
        &self,
        output: &LinkType,
        value: &LinkValue,
        from: ComponentId,
        fetch: &F,
    ) -> Result<(), LinkError> {
        if !output.is_text() {
            return Err(LinkError::WrongConstValue(
                (from, "text template must has text output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Text(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "text template must has text value".into()).into(),
                    ))
                }
            };

            if let Some(min_length) = self.min_length {
                if value.len() < min_length as usize {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required min length: {min_length}")).into(),
                    ));
                }
            }

            if let Some(max_length) = self.max_length {
                if value.len() > max_length as usize {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required max length: {max_length}")).into(),
                    ));
                }
            }
        }

        // Check the code
        if let Some(code) = &self.code {
            let code = ValidateForm::get_validate_code_item(code.to_owned(), output);
            let js = fetch
                .compile_code(&code)
                .map_err(|error| LinkError::WrongConstValue((from, error).into()).into())?;
            CodeContent::validate_by_js(&code, js, value, from)?;
        }

        Ok(())
    }
}
