use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::model::common::to_typescript::abi_params_to_typescript;

use super::{
    AbiStateMutability, AllEndpoints, ApiData, ApiDataAnchor, CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor,
    CodeType, ComponentId, EvmCallApi, EvmCallArg, InputValue, LinkError, LinkType,
};

lazy_static! {
    static ref OUTPUT_LINK_TYPE: LinkType = LinkType::Text;
}

/// Transaction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EvmActionTransaction {
    /// If the target address is reference, it must be the text type // It must be in line with a string in 0xaaa format
    contract: InputValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pay_value: Option<InputValue>, // Whether to send tokens wei unit // text type

    #[serde(skip_serializing_if = "Option::is_none")]
    gas_limit: Option<InputValue>, // gas limit // integer type

    #[serde(skip_serializing_if = "Option::is_none")]
    gas_price: Option<InputValue>, // gas price // text type

    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<InputValue>, // Whether to specify nonce // integer type

    /// Specified method
    api: EvmCallApi,

    /// Call parameter
    // No parameter parameters
    // ! Simple parameters can be met with reference methods, and very complicated data structures are required to achieve
    // ? In most cases, users need to write code to meet the parameter data
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<EvmCallArg>,
    // There should be post -processing transactions, because only the transaction ID can be returned
    // /// Treatment after call results
    // // Simple parameters can be converted into support types
    // // ! Users can choose the specified simple data, which requires a very complicated data structure to achieve
    // // ? In most cases, users need to write code to meet the output data
    // #[serde(skip_serializing_if = "Option::is_none")]
    // ret: Option<EvmCallRet>,
}

impl EvmActionTransaction {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        // arg
        if let Some(arg) = &self.arg {
            anchors.extend(arg.get_code_anchors());
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
        _output: &LinkType,
        from: ComponentId,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        let func = self.api.get_data_and_output(from, fetch)?;

        let api_data = abi_params_to_typescript(&func.inputs.unwrap_or_default(), from)?;
        // let api_output = abi_params_to_typescript(&func.outputs.unwrap_or_default(), from)?;

        let mut data_of_args = CodeType::undefined();

        // arg
        if let Some(arg) = &self.arg {
            // DATA is the data calculation in ARG
            let output = api_data.clone();
            codes.extend(arg.get_origin_codes(endpoints, output, from, |data| data_of_args = data)?);
        }

        // Transaction without post -processing
        // // ret
        // if let Some(ret) = &self.ret {
        //     let data = api_output;
        //     let output = CodeType::from_ty(output.typescript());
        //     codes.extend(ret.get_origin_codes(data, api_data, data_of_args, output, from)?);
        // }

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

        // 2. check pay_value
        let mut pay_value = None;
        if let Some(pay_value_ref) = &self.pay_value {
            pay_value = Some(pay_value_ref.check_pay_value(endpoints, from)?);
        }

        // 3. check gas_limit
        let mut gas_limit = None;
        if let Some(gas_limit_ref) = &self.gas_limit {
            gas_limit = Some(gas_limit_ref.check_gas_limit(endpoints, from)?);
        }

        // 4. check gas_price
        let mut gas_price = None;
        if let Some(gas_price_ref) = &self.gas_price {
            gas_price = Some(gas_price_ref.check_gas_price(endpoints, from)?);
        }

        // 5. check nonce
        let mut nonce = None;
        if let Some(nonce_ref) = &self.nonce {
            nonce = Some(nonce_ref.check_nonce(endpoints, from)?);
        }

        let func = self.api.get_data_and_output(from, fetch)?;
        let inputs = func.inputs.unwrap_or_default();
        // let outputs = func.outputs.unwrap_or_default();
        let state_mutability = func.state_mutability.ok_or_else(|| {
            LinkError::InvalidCallEvmActionApi((from, "function abi must has stateMutability".into()).into())
        })?;

        let api_data = abi_params_to_typescript(&inputs, from)?;
        // let api_output = abi_params_to_typescript(&outputs, from)?;

        let mut data_of_args = CodeType::undefined();

        // 6. check api
        let api = self.api.clone().try_into_anchor(fetch, apis)?;

        // 7. Modify the interface
        match state_mutability {
            AbiStateMutability::Pure | AbiStateMutability::View => {
                return Err(LinkError::InvalidCallEvmActionApi(
                    (from, "api of transaction action can not be pure or view".into()).into(),
                ));
            }
            AbiStateMutability::Nonpayable => {
                if pay_value.is_some() {
                    return Err(LinkError::InvalidCallEvmActionPayValue(
                        (from, "nonpayable function should not pay".into()).into(),
                    ));
                }
            }
            AbiStateMutability::Payable => {}
        }

        // 8. check arg
        let arg = EvmCallArg::check_arg(&self.arg, endpoints, &inputs, &api_data, from, fetch, codes, |data| {
            data_of_args = data
        })?;

        // // 9. check ret
        // let ret = EvmCallRet::check_ret(
        //     &self.ret,
        //     &outputs,
        //     api_output,
        //     api_data,
        //     data_of_args,
        //     output,
        //     from,
        //     fetch,
        //     codes,
        // )?;

        // 10. check output
        if *output != *OUTPUT_LINK_TYPE {
            return Err(LinkError::InvalidCallEvmActionOutput(
                (from, "output must be text for transaction".into()).into(),
            ));
        }

        Ok(Self {
            contract,
            pay_value,
            gas_limit,
            gas_price,
            nonce,
            api,
            arg,
            // ret,
        })
    }
}
