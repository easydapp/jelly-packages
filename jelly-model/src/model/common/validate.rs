use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    model::types::check::{CheckFunction, CheckedCodeItem},
    store::code::{
        anchor::CodeDataAnchor,
        item::{args::ArgCodeType, types::CodeType, CodeItem},
        CodeData,
    },
};

use super::{
    code::{CodeContent, OriginCodeContent},
    error::LinkError,
    identity::ComponentId,
    types::LinkType,
};

#[cfg(feature = "validate")]
use super::values::LinkValue;

/// Verification code
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ValidateForm {
    /// Verification code input is the input type of FORM, the output is a string, and the empty string indicates the passing
    /// It is not allowed to introduce other running data, because it is too complicated
    #[serde(rename = "code")]
    Code(CodeContent),
}

impl ValidateForm {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            ValidateForm::Code(code) => anchors.extend(code.get_code_anchors()),
        }

        anchors
    }

    /// Unified verification code output type
    #[inline]
    pub fn get_validate_code_output() -> CodeType {
        CodeType::from_types(
            "ErrorMessage",
            Some(vec!["type ErrorMessage = string; // pass if return ''".into()]),
        )
    }

    /// structure Validate code item
    #[inline]
    pub fn get_output_and_code(output: &LinkType) -> (Vec<ArgCodeType>, CodeType) {
        (
            vec![ArgCodeType::from("data", CodeType::from_ty(output.typescript()))],
            Self::get_validate_code_output(),
        )
    }

    /// get otigin code
    pub fn get_origin_codes(
        &self,
        output: &LinkType,
        from: &ComponentId,
        index: u32, // The code serial number of this component, and some components have multiple positions of code
        mark: &str, // mark
    ) -> Vec<CheckedCodeItem> {
        let mut codes = Vec::new();

        if let Self::Code(CodeContent::Code(OriginCodeContent { code, .. })) = self {
            let (data, output) = Self::get_output_and_code(output);
            codes.push(CheckedCodeItem::new(
                *from,
                index,
                mark.into(),
                CodeItem {
                    code: code.code.clone(),
                    args: Some(data),
                    ret: Some(output),
                },
            ));
        }

        codes
    }

    /// check
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        Ok(match self {
            Self::Code(code) => {
                // Calculate parameter type and output type
                let (data, output) = Self::get_output_and_code(output);

                let code = code
                    .clone()
                    .try_into_anchor(Some(data), Some(output), from, fetch, codes)?;

                Self::Code(code)
            }
        })
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate<F: CheckFunction>(&self, value: &LinkValue, from: ComponentId, fetch: &F) -> Result<(), LinkError> {
        match self {
            ValidateForm::Code(code) => code.validate(value, from, fetch),
        }
    }

    /// get code item
    pub fn get_validate_code_item(code: String, output: &LinkType) -> CodeItem {
        let (data, output) = Self::get_output_and_code(output);
        CodeItem {
            code,
            args: Some(data),
            ret: Some(output),
        }
    }
}
