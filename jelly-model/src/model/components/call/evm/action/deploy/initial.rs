use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeType, CodeValue, ComponentId, LinkError,
};

/// evm deploy initial code
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EvmDeployInitialCode {
    /// Code parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    data: Option<Vec<CodeValue>>,
    /// Code
    code: CodeContent,
}

impl EvmDeployInitialCode {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.code.get_code_anchors());

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        if let Some(code) = self.code.get_origin_code() {
            // Check the introduction variable
            let data =
                endpoints.check_code_values(self.data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
            let data = CodeType::from_ty(data.typescript());
            let output = CodeType::from_ty("any[]"); // ? I don't know what the parameters of the contract code are

            codes.push(CheckedCodeItem::new(
                from,
                0,
                "Evm deploy action -> initial".into(),
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
    fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // Check the introduction variable
        let data =
            endpoints.check_code_values(self.data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;
        let data = CodeType::from_ty(data.typescript());
        let output = CodeType::from_ty("any[]"); // ? I don't know what the parameters of the contract code are

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
pub enum EvmDeployInitial {
    /// Code calculation
    #[serde(rename = "code")]
    Code(EvmDeployInitialCode),
}

impl EvmDeployInitial {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            EvmDeployInitial::Code(code) => anchors.extend(code.get_code_anchors()),
        }

        anchors
    }

    /// get origin code
    pub fn get_origin_codes(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        match self {
            EvmDeployInitial::Code(code) => codes.extend(code.get_origin_codes(endpoints, from)?),
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<EvmDeployInitial, LinkError> {
        let initial = match self {
            Self::Code(code) => Self::Code(code.check(endpoints, from, fetch, codes)?),
        };

        Ok(initial)
    }
}
