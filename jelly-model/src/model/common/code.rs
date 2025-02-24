use std::collections::HashMap;

use candid::Principal;
use serde::{Deserialize, Serialize};

use crate::{
    model::types::check::CheckFunction,
    store::code::{
        anchor::{CodeDataAnchor, CodeDataParsedId},
        item::{args::ArgCodeType, types::CodeType, CodeItem},
        CodeData,
    },
};

use super::{
    error::{system_error, LinkError, LinkErrorWrongCode},
    identity::ComponentId,
};

#[cfg(feature = "validate")]
use super::{error::LinkErrorValidateCodeFailed, values::LinkValue};

/// Code data
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OriginCodeContent {
    /// code Original code
    pub code: CodeItem,

    /// compiled js code
    pub js: String,
}

/// Code
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CodeContent {
    /// Code itself
    #[serde(rename = "code")]
    Code(OriginCodeContent),

    /// Introduce code code data in the jar, you need to query to get it
    #[serde(rename = "anchor")]
    Anchor(CodeDataAnchor),
}

impl CodeContent {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        if let CodeContent::Anchor(anchor) = &self {
            anchors.push(anchor.clone());
        }

        anchors
    }

    /// Query original code
    pub fn get_origin_code(&self) -> Option<String> {
        if let CodeContent::Code(OriginCodeContent { code, .. }) = &self {
            return Some(code.code.clone());
        }
        None
    }

    /// Processing code
    pub fn try_into_anchor<F: CheckFunction>(
        mut self,
        args: Option<Vec<ArgCodeType>>,
        ret: Option<CodeType>,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<CodeContent, LinkError> {
        if let CodeContent::Code(OriginCodeContent { code: item, js }) = &mut self {
            // key
            item.args = args;
            item.ret = ret;

            let compiled = fetch
                .compile_code(item)
                .map_err(|err| {
                    LinkError::WrongCode(Box::new(LinkErrorWrongCode {
                        from,
                        code: item.clone(),
                        message: err,
                    }))
                })?
                .to_string();

            if item.should_into_anchor() {
                // Calculate ID
                let canister_id = fetch.canister_id().map_err(system_error)?;
                let hash = item.hash(js).map_err(system_error)?;
                let parsed_id = CodeDataParsedId::from(
                    Principal::from_text(canister_id).map_err(|e| system_error(format!("{e}")))?,
                    hash.into(),
                );
                let anchor: CodeDataAnchor = (&parsed_id).into();

                // Record
                {
                    let code_data = CodeData {
                        anchor: anchor.clone(),
                        created: 0.into(),
                        code: item.clone(),
                        js: compiled,
                    };
                    codes.insert(anchor.clone(), code_data);
                }

                return Ok(Self::Anchor(anchor));
            }

            *js = compiled;
        }
        Ok(self)
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate_by_js(code: &CodeItem, js: &str, value: &LinkValue, from: ComponentId) -> Result<(), LinkError> {
        // ! Execution code verification
        jelly_executor::execute_validate_code(
            js,
            &serde_json::to_string(value).map_err(|e| system_error(e.to_string()))?,
        )
        .map_err(|e| format!("{e:?}"))
        .and_then(|checked| {
            let result: String = serde_json::from_str(&checked).map_err(|e| format!("{e:?}"))?;
            if result.is_empty() {
                Ok(()) // ! Only returned to the empty string can be counted
            } else {
                Err(format!("validate failed: {checked}"))
            }
        })
        .map_err(|message| {
            LinkError::ValidateCodeFailed(Box::new(LinkErrorValidateCodeFailed {
                from,
                code: code.to_owned(),
                js: js.to_string(),
                value: value.to_owned(),
                message,
            }))
        })?;

        Ok(())
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate<F: CheckFunction>(&self, value: &LinkValue, from: ComponentId, fetch: &F) -> Result<(), LinkError> {
        let (code, js) = match self {
            CodeContent::Code(OriginCodeContent { code, js }) => (code, js),
            CodeContent::Anchor(anchor) => {
                let code_data = fetch.fetch_code(anchor).map_err(system_error)?;
                (&code_data.code, &code_data.js)
            }
        };

        Self::validate_by_js(code, js, value, from)
    }
}
