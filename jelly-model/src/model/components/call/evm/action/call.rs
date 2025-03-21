use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::common::to_typescript::abi_params_to_typescript;

use super::{
    AbiParam, AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData,
    CodeDataAnchor, CodeItem, CodeType, CodeValue, ComponentId, EvmCallApi, InputValue, LinkError, LinkType,
};

/// evm call arg
pub mod arg;

/// evm call ret
pub mod ret;

pub use arg::EvmCallArg;

use ret::EvmCallRet;

/// Call contract
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EvmActionCall {
    /// If the target address is reference, it must be the text type // It must be in line with a string in 0xaaa format
    contract: InputValue,

    /// Specified method
    api: EvmCallApi,

    /// Call parameter
    // No parameter parameters
    // ! Simple parameters can be met with reference methods, and very complicated data structures are required to achieve
    // ? In most cases, users need to write code to meet the parameter data
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<EvmCallArg>,

    /// Treatment after call results
    // Simple parameters can be converted into support types
    // ! Users can choose the specified simple data, which requires a very complicated data structure to achieve
    // ? In most cases, users need to write code to meet the output data
    #[serde(skip_serializing_if = "Option::is_none")]
    ret: Option<EvmCallRet>,
}

impl EvmActionCall {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        // arg
        if let Some(arg) = &self.arg {
            anchors.extend(arg.get_code_anchors());
        }

        // ret
        if let Some(ret) = &self.ret {
            anchors.extend(ret.get_code_anchors());
        }

        anchors
    }

    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.api.get_apis_anchors());

        anchors
    }

    /// get origin code
    pub fn get_origin_codes<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        let func = self.api.get_data_and_output(from, true, fetch)?;

        let api_data = abi_params_to_typescript(&func.inputs.unwrap_or_default(), from)?;
        let api_output = abi_params_to_typescript(&func.outputs.unwrap_or_default(), from)?;

        let mut data_of_args = CodeType::undefined();

        // arg
        if let Some(arg) = &self.arg {
            // DATA is the data calculation in ARG
            let output = api_data.clone();
            codes.extend(arg.get_origin_codes(endpoints, output, from, |data| data_of_args = data)?);
        }

        // ret
        if let Some(ret) = &self.ret {
            let data = api_output;
            let output = CodeType::from_ty(output.typescript());
            codes.extend(ret.get_origin_codes(data, api_data, data_of_args, output, from)?);
        }

        Ok(codes)
    }

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<Self, LinkError> {
        // 1. check contract
        let contract = self.contract.check_contract(endpoints, from)?;

        let func = self.api.get_data_and_output(from, true, fetch)?;
        let inputs = func.inputs.unwrap_or_default();
        let outputs = func.outputs.unwrap_or_default();
        let _state_mutability = func.state_mutability.ok_or_else(|| {
            LinkError::InvalidCallEvmActionApi((from, "function abi must has stateMutability".into()).into())
        })?;

        let api_data = abi_params_to_typescript(&inputs, from)?;
        let api_output = abi_params_to_typescript(&outputs, from)?;

        let mut data_of_args = CodeType::undefined();

        // 2. check api
        let api = self.api.clone().try_into_anchor(fetch, apis)?;

        // 3. Interface that does not modify status
        // ! static call could be simulate..
        // use super::AbiStateMutability;
        // if !matches!(_state_mutability, AbiStateMutability::Pure | AbiStateMutability::View) {
        //     return Err(LinkError::InvalidCallEvmActionApi(
        //         (from, "call api must be pure or view".into()).into(),
        //     ));
        // }

        // 4. check arg
        let arg = EvmCallArg::check_arg(&self.arg, endpoints, &inputs, &api_data, from, fetch, codes, |data| {
            data_of_args = data
        })?;

        // 5. check ret
        let ret = EvmCallRet::check_ret(
            &self.ret,
            &outputs,
            api_output,
            api_data,
            data_of_args,
            output,
            from,
            fetch,
            codes,
        )?;

        Ok(Self {
            contract,
            api,
            arg,
            ret,
        })
    }
}
