use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    AbiParam, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem, CodeType,
    ComponentId, LinkError, LinkType,
};

/// Consequences
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmCallRet {
    /// Code calculation
    #[serde(rename = "code")]
    Code(CodeContent),
}

impl EvmCallRet {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            EvmCallRet::Code(code) => anchors.extend(code.get_code_anchors()),
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        data: CodeType,
        args: CodeType,
        data_of_args: CodeType,
        output: CodeType,
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        match self {
            EvmCallRet::Code(code) => {
                if let Some(code) = code.get_origin_code() {
                    codes.push(CheckedCodeItem::new(
                        from,
                        1,
                        "Evm call action -> ret".into(),
                        CodeItem {
                            code,
                            args: Some(vec![
                                ArgCodeType::from("data", data),                 // API output
                                ArgCodeType::from("args", args),                 // API parameters
                                ArgCodeType::from("data_of_args", data_of_args), // Parameters of API parameters
                            ]),
                            ret: Some(output),
                        },
                    ));
                }
            }
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        data: CodeType,         // API result
        args: CodeType,         // API parameter
        data_of_args: CodeType, // parameters of API parameters
        output: CodeType,       // Component requires output
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        let ret = match self {
            Self::Code(code) => {
                let code = code.clone().try_into_anchor(
                    Some(vec![
                        ArgCodeType::from("data", data),                 // API output
                        ArgCodeType::from("args", args),                 // API parameters
                        ArgCodeType::from("data_of_args", data_of_args), // Parameters of API parameters
                    ]),
                    Some(output),
                    from,
                    fetch,
                    codes,
                )?;
                Self::Code(code)
            }
        };

        Ok(ret)
    }

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check_ret<F: CheckFunction>(
        ret_ref: &Option<EvmCallRet>,
        rets: &[AbiParam],      // API result
        api_output: CodeType,   // API result
        api_data: CodeType,     // API parameter
        data_of_args: CodeType, // parameters of API parameters
        output: &LinkType,      // Component output results
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Option<EvmCallRet>, LinkError> {
        // 1 Whether the existence of treatment after inspection is matched
        match rets.len() {
            0 => {
                if ret_ref.is_none() && !output.is_array() {
                    return Err(LinkError::InvalidCallEvmActionRet(
                        (from, "output must be array".into()).into(),
                    ));
                }
            }
            1 => {
                if ret_ref.is_none() && rets[0].typescript(from)? != output.typescript() {
                    // The type needs to check whether the matching
                    return Err(LinkError::InvalidCallEvmActionRet(
                        (from, "output type mismatch".into()).into(),
                    ));
                }
            }
            _ => {
                if ret_ref.is_none() {
                    // Can't be empty
                    return Err(LinkError::InvalidCallEvmActionRet(
                        (from, "output type mismatch".into()).into(),
                    ));
                }
            }
        }

        // 2. Treatment after examination
        let mut ret = None;
        if let Some(ret_ref) = &ret_ref {
            ret = Some(ret_ref.check(
                api_output,
                api_data,
                data_of_args,
                CodeType::from_ty(output.typescript()),
                from,
                fetch,
                codes,
            )?);
        }

        Ok(ret)
    }
}
