use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem, CodeType,
    ComponentId, LinkError,
};

/// Consequences
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IcCallRet {
    /// Code calculation
    #[serde(rename = "code")]
    Code(CodeContent),
}

impl IcCallRet {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            IcCallRet::Code(code) => anchors.extend(code.get_code_anchors()),
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        data: CodeType,         // API result
        args: CodeType,         // API parameter
        data_of_args: CodeType, // parameters of API parameters
        output: CodeType,       // Component requires output
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        match self {
            IcCallRet::Code(code) => {
                if let Some(code) = code.get_origin_code() {
                    codes.push(CheckedCodeItem::new(
                        from,
                        1,
                        "Ic call action -> ret".into(),
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
}
