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

/// Floating point number template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateNumber {
    /// placeholder
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    /// suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,

    /// min value
    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<f64>,
    /// max value
    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<f64>,
    /// decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    decimals: Option<u8>,
    /// code like ValidateForm
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl NodeTemplateValidateNumber {
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
        if !output.is_number() {
            return Err(LinkError::WrongConstValue(
                (from, "number template must has number output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Number(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "number template must has number value".into()).into(),
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

            if let Some(decimals) = self.decimals {
                let s = value.to_string();
                let mut s = s.split(".");
                s.next();
                let i = s.next().map(|s| s.len());
                if let Some(i) = i {
                    if decimals < i as u8 {
                        return Err(LinkError::WrongConstValue(
                            (from, format!("required max decimals: {decimals}")).into(),
                        ));
                    }
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
