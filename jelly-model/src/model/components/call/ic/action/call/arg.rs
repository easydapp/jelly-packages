use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::store::code::item::types::CodeType;

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeValue, ComponentId, LinkError,
};

/// ic call arg code
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IcCallArgCode {
    /// Code parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    data: Option<Vec<CodeValue>>,
    /// Code
    code: CodeContent,
}

impl IcCallArgCode {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.code.get_code_anchors());

        anchors
    }

    /// get origin code
    pub fn get_origin_codes<H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: CodeType,
        from: ComponentId,
        mut handle: H,
    ) -> Result<Vec<CheckedCodeItem>, LinkError>
    where
        H: FnMut(CodeType),
    {
        let mut codes = Vec::new();

        if let Some(code) = self.code.get_origin_code() {
            // Check the introduction variable
            let data =
                endpoints.check_code_values(self.data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
            let data = CodeType::from_ty(data.typescript());

            handle(data.clone());

            codes.push(CheckedCodeItem::new(
                from,
                0,
                "Ic call action -> arg".into(),
                CodeItem {
                    code,
                    args: Some(vec![ArgCodeType::from("data", data)]),
                    ret: Some(output),
                },
            ));
        }

        Ok(codes)
    }

    #[inline]
    fn check<F: CheckFunction, H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: CodeType, // API parameter type
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        mut handle: H,
    ) -> Result<Self, LinkError>
    where
        H: FnMut(CodeType),
    {
        // Check the introduction variable
        let data =
            endpoints.check_code_values(self.data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
        let data = CodeType::from_ty(data.typescript());

        handle(data.clone());

        let code = self.code.clone().try_into_anchor(
            Some(vec![ArgCodeType::from("data", data)]),
            Some(output),
            from,
            fetch,
            codes,
        )?;

        Ok(Self {
            data: self.data.clone(),
            code,
        })
    }
}

/// Request parameters
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IcCallArg {
    /// Code calculation
    #[serde(rename = "code")]
    Code(IcCallArgCode),
}

impl IcCallArg {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            IcCallArg::Code(code) => anchors.extend(code.get_code_anchors()),
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes<H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: CodeType,
        from: ComponentId,
        handle: H,
    ) -> Result<Vec<CheckedCodeItem>, LinkError>
    where
        H: FnMut(CodeType),
    {
        let mut codes = Vec::new();

        match self {
            IcCallArg::Code(code) => codes.extend(code.get_origin_codes(endpoints, output, from, handle)?),
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[inline]
    pub fn check<F: CheckFunction, H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: CodeType, // API parameter type
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        handle: H,
    ) -> Result<Self, LinkError>
    where
        H: FnMut(CodeType),
    {
        let arg = match self {
            Self::Code(code) => Self::Code(code.check(endpoints, output, from, fetch, codes, handle)?),
        };

        Ok(arg)
    }
}
