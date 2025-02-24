use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeType, CodeValue, ComponentId, LinkError, NamedValue,
};

/// http body plain
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HttpBodyPlain {
    /// The dependencies are combined into an object as the parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub data: Option<Vec<NamedValue>>,
}

impl HttpBodyPlain {
    /// Check whether the component is effectiveether the component is effective
    #[inline]
    fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        if let Some(data) = &self.data {
            endpoints.check_named_values(data.iter(), from, None)?;
        }
        Ok(self.clone())
    }
}

/// http body code
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HttpBodyCode {
    /// Code parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    data: Option<Vec<CodeValue>>,
    /// Code
    pub code: CodeContent,
}

impl HttpBodyCode {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.code.get_code_anchors());

        anchors
    }
}

/// Request
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum HttpBody {
    /// Constructed object
    #[serde(rename = "plain")]
    Plain(HttpBodyPlain),
    /// Code calculation
    #[serde(rename = "code")]
    Code(HttpBodyCode),
}

impl HttpBody {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        if let HttpBody::Code(code) = self {
            anchors.extend(code.get_code_anchors());
        }

        anchors
    }

    /// Calculation parameters and output types
    #[inline]
    fn get_data_and_output(
        endpoints: &AllEndpoints<'_>,
        data: &Option<Vec<CodeValue>>,
        from: ComponentId,
    ) -> Result<(CodeType, Option<CodeType>), LinkError> {
        // Check the introduction variable
        let data = endpoints.check_code_values(data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;

        // Calculate parameter type and output type
        let data = CodeType::from_ty(data.typescript());
        let output = None; // HTTP interface request data required, no need to check

        Ok((data, output))
    }

    /// get origin code
    pub fn get_origin_codes<H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
        index: u32,
        mark: &str,
        mut handle: H,
    ) -> Result<Vec<CheckedCodeItem>, LinkError>
    where
        H: FnMut(CodeType),
    {
        let mut codes = Vec::new();

        if let HttpBody::Code(body_code) = self {
            if let Some(code) = body_code.code.get_origin_code() {
                // Calculate parameter type and output type
                let (data, output) = Self::get_data_and_output(endpoints, &body_code.data, from)?;

                handle(data.clone());

                codes.push(CheckedCodeItem::new(
                    from,
                    index,
                    mark.into(),
                    CodeItem {
                        code,
                        args: Some(vec![ArgCodeType::from("data", data)]),
                        ret: output,
                    },
                ));
            }
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[inline]
    pub fn check<F: CheckFunction, H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        mut handle: H,
    ) -> Result<Self, LinkError>
    where
        H: FnMut(CodeType),
    {
        let body = match self {
            Self::Plain(plain) => Self::Plain(plain.check(endpoints, from)?),
            Self::Code(body_code) => {
                // Calculate parameter type and output type
                let (data, output) = Self::get_data_and_output(endpoints, &body_code.data, from)?;

                handle(data.clone());

                let code = body_code.code.clone().try_into_anchor(
                    Some(vec![ArgCodeType::from("data", data)]),
                    output,
                    from,
                    fetch,
                    codes,
                )?;

                Self::Code(HttpBodyCode {
                    data: body_code.data.clone(),
                    code,
                })
            }
        };

        Ok(body)
    }
}
