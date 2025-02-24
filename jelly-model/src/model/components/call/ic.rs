use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction, CheckedCodeItem, CodeContent, CodeData,
    CodeDataAnchor, CodeItem, CodeValue, ComponentCallTrigger, ComponentId, ComponentTriggered, IcCallApi,
    IcFunctionArgsType, IdentityInnerMetadata, InputValue, LinkError, LinkType, TimestampMills, ToTypescript,
};

/// ic action
pub mod action;

use action::IcAction;

/// ic metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallIcMetadata {
    /// Trigger condition
    pub trigger: ComponentCallTrigger,

    /// The required identity empty indicates the use of anonymous identity
    /// It must be a component of IdentityIcMetadata type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ComponentId>,

    /// IC call behavior
    pub action: IcAction,
}

impl CallIcMetadata {
    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.action.get_code_anchors());

        anchors
    }

    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        anchors.extend(self.action.get_apis_anchors());

        anchors
    }

    /// get origin code
    pub fn get_origin_codes<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        output: &LinkType,
        from: ComponentId,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let mut codes = Vec::new();

        // 0 Check whether the reference is matched
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        codes.extend(self.action.get_origin_codes(&endpoints, output, from, fetch)?);

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
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<CallIcMetadata, LinkError> {
        // 1 check trigger
        let trigger = self.trigger.check(endpoints, from)?;

        // 2. Check identity
        let mut identity = None;
        if let Some(identity_ref) = self.identity {
            identity = Some(identity_ref.check_identity(
                endpoints,
                |metadata| matches!(metadata, IdentityInnerMetadata::Ic(_)),
                "identity component is not ic",
                |_, _| Ok(()),
                from,
            )?);
        }

        // 3. check action
        let action = self.action.check(
            endpoints, &trigger, identity, output, from, fetch, triggers, codes, apis,
        )?;

        Ok(Self {
            trigger,
            identity,
            action,
        })
    }
}
