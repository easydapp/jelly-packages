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

/// Integer template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateInteger {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    /// suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    /// min value
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<i64>,
    /// max value
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<i64>,
    /// code like ValidateForm
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl NodeTemplateValidateInteger {
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
        if !output.is_integer() {
            return Err(LinkError::WrongConstValue(
                (from, "integer template must has integer output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Integer(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "integer template must has integer value".into()).into(),
                    ))
                }
            };

            if let Some(min_value) = self.min_value {
                if *value < min_value {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required min value: {min_value}")).into(),
                    ));
                }
            }

            if let Some(max_value) = self.max_value {
                if *value > max_value {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required max value: {max_value}")).into(),
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
