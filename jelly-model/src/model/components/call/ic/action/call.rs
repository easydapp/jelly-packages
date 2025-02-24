use std::collections::HashMap;

use ic_canister_kit::types::WrappedCandidType;
use serde::{Deserialize, Serialize};

use crate::{
    model::common::{
        api::ic::candid::types_args_type, call_trigger::ComponentCallTrigger, to_typescript::candid_types_to_typescript,
    },
    store::code::item::types::CodeType,
};

use super::{
    AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData,
    CodeDataAnchor, CodeItem, CodeValue, ComponentId, ComponentTriggered, IcCallApi, IcFunctionArgsType, InputValue,
    LinkError, LinkType, TimestampMills, ToTypescript,
};

/// ic call info
pub mod info;

/// ic call arg
pub mod arg;

/// ic call ret
pub mod ret;

use info::CanisterInfo;

use arg::IcCallArg;

use ret::IcCallRet;

/// Call contract
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IcActionCall {
    /// If the target address is reference, it must be the text type // It must be in line with the character string in Principal format
    pub canister_id: InputValue,

    /// If it is a constant, you need to record the Module Hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<CanisterInfo>,

    /// Specified method
    pub api: IcCallApi,

    /// Call parameter
    // No parameter parameters
    // ! Simple parameters can be met with reference methods, and very complicated data structures are required to achieve
    // ? In most cases, users need to write code to meet the parameter data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<IcCallArg>,

    /// Treatment after call results
    // Simple parameters can be converted into support types
    // ! Users can choose the specified simple data, which requires a very complicated data structure to achieve
    // ? In most cases, users need to write code to meet the output data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret: Option<IcCallRet>,
}

impl IcActionCall {
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

        let func = self.api.get_data_and_output(from, fetch)?;

        let api_data = candid_types_to_typescript(&func.args, from)?;
        let api_output = candid_types_to_typescript(&func.rets, from)?;

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

    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn check_arg<F: CheckFunction, H>(
        &self,
        endpoints: &AllEndpoints<'_>,
        args_type: IcFunctionArgsType,
        api_data: &CodeType, // API parameter type, here is the output result of the code
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        handle: H,
    ) -> Result<Option<IcCallArg>, LinkError>
    where
        H: FnMut(CodeType),
    {
        // 1 Check whether the presence of the parameter matches
        match args_type {
            IcFunctionArgsType::None => {
                if self.arg.is_some() {
                    // 0 parameters do not require ARG
                    return Err(LinkError::InvalidCallIcApiArg(
                        (from, "arg must be none for empty args".into()).into(),
                    ));
                }
            }
            IcFunctionArgsType::Single { opt } => {
                if !opt && self.arg.is_none() {
                    // 1 Parameter non -OPT requires ARG
                    return Err(LinkError::InvalidCallIcApiArg((from, "arg is missing".into()).into()));
                }
            }
            IcFunctionArgsType::Multi { opt } => {
                if !opt && self.arg.is_none() {
                    // Multi -parameter non -OPT requires ARG
                    return Err(LinkError::InvalidCallIcApiArg((from, "arg is missing".into()).into()));
                }
            }
        }

        // 2. Check the parameters
        let mut arg = None;
        if let Some(arg_ref) = &self.arg {
            arg = Some(arg_ref.check(endpoints, api_data.to_owned(), from, fetch, codes, handle)?);
        }

        Ok(arg)
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn check_ret<F: CheckFunction>(
        &self,
        rets: &[WrappedCandidType], // API result
        api_output: CodeType,       // API result
        api_data: CodeType,         // API parameter
        data_of_args: CodeType,     // parameters of API parameters
        output: &LinkType,          // Component output results
        from: ComponentId,
        fetch: &F,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Option<IcCallRet>, LinkError> {
        // 1 Whether the existence of treatment after inspection is matched
        match rets.len() {
            0 => {
                if self.ret.is_none() && !output.is_array() {
                    return Err(LinkError::InvalidCallIcApiRet(
                        (from, "output must be array".into()).into(),
                    ));
                }
            }
            1 => {
                if self.ret.is_none() && !rets[0].to_typescript(from)?.is_same_typescript(&output.typescript()) {
                    // The type needs to check whether the matching
                    return Err(LinkError::InvalidCallIcApiRet(
                        (from, "output type mismatch".into()).into(),
                    ));
                }
            }
            _ => {
                if self.ret.is_none() {
                    // Can't be empty
                    return Err(LinkError::InvalidCallIcApiRet(
                        (from, "output type mismatch".into()).into(),
                    ));
                }
            }
        }

        // 2. Treatment after examination
        let mut ret = None;
        if let Some(ret_ref) = &self.ret {
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

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &AllEndpoints<'_>,
        trigger: &ComponentCallTrigger,
        identity: Option<ComponentId>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<Self, LinkError> {
        // 1. check canister_id
        let canister_id = self.canister_id.check_canister_id(endpoints, from)?;

        // 2. check info
        let info = self.info.clone();

        let func = self.api.get_data_and_output(from, fetch)?;

        let api_data = candid_types_to_typescript(&func.args, from)?;
        let api_output = candid_types_to_typescript(&func.rets, from)?;

        let mut data_of_args = CodeType::undefined();

        // 3. check api
        let api = self.api.clone().try_into_anchor(fetch, apis)?;

        // 4. Recording trigger
        triggers.insert(
            from,
            ComponentTriggered::from_call(
                from,
                identity,
                matches!(trigger, ComponentCallTrigger::Click(_)),
                func.annotation.is_none(),
            ),
        );

        // 5. check arg
        let args_type = types_args_type(&func.args);
        let arg = self.check_arg(endpoints, args_type, &api_data, from, fetch, codes, |data| {
            data_of_args = data
        })?;

        // 6. check ret
        let ret = self.check_ret(
            &func.rets,
            api_output,
            api_data,
            data_of_args,
            output,
            from,
            fetch,
            codes,
        )?;

        Ok(Self {
            canister_id,
            info,
            api,
            arg,
            ret,
        })
    }
}
