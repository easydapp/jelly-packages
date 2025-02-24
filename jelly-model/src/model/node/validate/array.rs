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

use super::NodeTemplateValidate;

/// Array template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateArray {
    /// Sub -type constraint
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Box<NodeTemplateValidate>>, // You need to exclude the type of sub -type, which is also ARRAY, at most one layer of nested

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

impl NodeTemplateValidateArray {
    /// get code
    pub fn get_origin_codes(&self, output: &LinkType, mut index: u32, from: ComponentId) -> Vec<CheckedCodeItem> {
        let mut codes = vec![];

        if let Some(subtype) = &self.subtype {
            codes.extend(subtype.get_origin_codes(output, index, from));
        }

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
        if !output.is_array() {
            return Err(LinkError::WrongConstValue(
                (from, "array template must has array output".into()).into(),
            ));
        }

        // Check value
        {
            let value = match value {
                LinkValue::Array(value) => value,
                _ => {
                    return Err(LinkError::WrongConstValue(
                        (from, "array template must has array value".into()).into(),
                    ))
                }
            };

            if let Some(subtype) = &self.subtype {
                for (i, v) in value.values.iter().enumerate() {
                    if let Err(err) = subtype.validate(&value.ty, v, from, fetch) {
                        return Err(LinkError::WrongConstValue(
                            (from, format!("Item: {} error: {err:?}", i + 1)).into(),
                        ));
                    }
                }
            }

            if let Some(min_length) = self.min_length {
                if value.values.len() < min_length as usize {
                    return Err(LinkError::WrongConstValue(
                        (from, format!("required min length: {min_length}")).into(),
                    ));
                }
            }

            if let Some(max_length) = self.max_length {
                if value.values.len() > max_length as usize {
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
