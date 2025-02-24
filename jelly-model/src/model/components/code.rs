use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem,
    CodeType, CodeValue, ComponentId, Endpoint, LinkError, LinkType,
};

/// Code conversion
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentCode {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    pub metadata: CodeMetadata,

    /// Output type
    pub output: LinkType, // User specified type
}

/// code metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CodeMetadata {
    /// Code execution parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub data: Option<Vec<CodeValue>>,

    /// Code content
    pub code: CodeContent,
}

impl ComponentCode {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.metadata.code.get_code_anchors());

        anchors
    }

    /// Calculation parameters and output types
    #[inline]
    fn get_data_and_output(
        endpoints: &Option<AllEndpoints<'_>>,
        data: &Option<Vec<CodeValue>>,
        output: &LinkType,
        from: ComponentId,
    ) -> Result<(CodeType, CodeType), LinkError> {
        // Check the introduction variable
        let data = endpoints
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_default()
            .check_code_values(data.as_ref().map(Cow::Borrowed).unwrap_or_default().iter(), from)?;

        // Calculate parameter type and output type
        let data = CodeType::from_ty(data.typescript());
        let output = CodeType::from_ty(output.typescript()); // HTTP interface request data required, no need to check

        Ok((data, output))
    }

    /// get origin code
    pub fn get_origin_codes(&self, endpoints: &Option<AllEndpoints<'_>>) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        if let Some(code) = self.metadata.code.get_origin_code() {
            // Calculate parameter type and output type
            let (data, output) = Self::get_data_and_output(endpoints, &self.metadata.data, &self.output, self.id)?;

            codes.push(CheckedCodeItem::new(
                self.id,
                0,
                "Code".into(),
                CodeItem {
                    code,
                    args: Some(vec![ArgCodeType::from("data", data)]),
                    ret: Some(output),
                },
            ));
        }

        Ok(codes)
    }

    /// check
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        self.output.check(self.id)?; // ? Check whether the output type is correct

        // 1. Calculate parameter type and output type
        let (data, output) = Self::get_data_and_output(endpoints, &self.metadata.data, &self.output, self.id)?;

        // 2. Check whether the code needs to be turned into a reference
        let code = self.metadata.code.clone().try_into_anchor(
            Some(vec![ArgCodeType::from("data", data)]),
            Some(output),
            self.id,
            fetch,
            codes,
        )?;

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata: CodeMetadata {
                data: self.metadata.data.clone(),
                code,
            },
            output: self.output.clone(),
        })
    }
}
