use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData,
    CodeDataAnchor, CodeItem, CodeValue, ComponentCallTrigger, ComponentId, ComponentTriggered, IcCallApi,
    IcFunctionArgsType, InputValue, LinkError, LinkType, TimestampMills, ToTypescript,
};

/// ic action call
pub mod call;

use call::IcActionCall;

/// IC call behavior
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IcAction {
    /// Call contract
    #[serde(rename = "call")]
    Call(IcActionCall),
}

impl IcAction {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            IcAction::Call(call) => anchors.extend(call.get_code_anchors()),
        }

        anchors
    }

    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        match self {
            IcAction::Call(call) => anchors.extend(call.get_apis_anchors()),
        }

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

        match self {
            IcAction::Call(call) => codes.extend(call.get_origin_codes(endpoints, output, from, fetch)?),
        }

        Ok(codes)
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
        let action = match self {
            Self::Call(call) => {
                Self::Call(call.check(endpoints, trigger, identity, output, from, fetch, triggers, codes, apis)?)
            }
        };

        Ok(action)
    }
}

impl InputValue {
    #[inline]
    fn check_canister_id(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        self.check_text_input_value(
            endpoints,
            |canister_id| candid::Principal::from_text(canister_id).is_ok(),
            "wrong canister id",
            "wrong canister id value",
            "wrong canister id type",
            LinkError::InvalidCallIcCanisterId,
            from,
        )
    }
}
