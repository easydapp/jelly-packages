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

/// Object template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplateValidateObject {
    /// code like ValidateForm
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl NodeTemplateValidateObject {
    /// Query code
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
        if !output.is_object() {
            return Err(LinkError::WrongConstValue(
                (from, "object template must has object output".into()).into(),
            ));
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
