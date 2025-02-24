use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::types::CallChain;

use super::{
    AbiItem, AbiParam, AbiStateMutability, AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction,
    CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem, CodeType, CodeValue, ComponentCallTrigger,
    ComponentId, ComponentIdentity, ComponentTriggered, Endpoint, EvmCallApi, EvmChain, IcCallApi, IcFunctionArgsType,
    IdentityInnerMetadata, IdentityMetadata, InputValue, LinkComponent, LinkError, LinkType, NamedValue,
    TimestampMills, ToTypescript,
};

/// http call
pub mod http;

/// ic call
pub mod ic;

/// evm call
pub mod evm;

use http::CallHttpMetadata;

use ic::CallIcMetadata;

use evm::CallEvmMetadata;

/// call
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentCall {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    pub metadata: CallMetadata,

    /// Output type
    pub output: LinkType, // User specified type
}

/// call metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CallMetadata {
    /// http
    #[serde(rename = "http")]
    Http(CallHttpMetadata),
    /// ic
    #[serde(rename = "ic")]
    Ic(CallIcMetadata),
    /// evm
    #[serde(rename = "evm")]
    Evm(CallEvmMetadata),
}

impl ComponentCall {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        match &self.metadata {
            CallMetadata::Http(metadata) => metadata.get_code_anchors(),
            CallMetadata::Ic(metadata) => metadata.get_code_anchors(),
            CallMetadata::Evm(metadata) => metadata.get_code_anchors(),
        }
    }

    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        match &self.metadata {
            CallMetadata::Http(_) => vec![],
            CallMetadata::Ic(metadata) => metadata.get_apis_anchors(),
            CallMetadata::Evm(metadata) => metadata.get_apis_anchors(),
        }
    }

    /// get origin code
    pub fn get_origin_codes<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        match &self.metadata {
            CallMetadata::Http(metadata) => metadata.get_origin_codes(endpoints, &self.output, self.id),
            CallMetadata::Ic(metadata) => metadata.get_origin_codes(endpoints, &self.output, self.id, fetch),
            CallMetadata::Evm(metadata) => metadata.get_origin_codes(endpoints, &self.output, self.id, fetch),
        }
    }

    /// Check whether the component is effective
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<Self, LinkError> {
        self.output.check(self.id)?; // ? Check whether the output type is correct

        // 0 Check whether the reference is matched and check the introduction variable
        if !matches!(
            (endpoints.as_ref(), self.inlets.as_ref()),
            (Some(_), Some(_)) | (None, None)
        ) {
            return Err(LinkError::MismatchedInlets { from: self.id });
        }
        let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        // 1 Check metadata
        let metadata = match &self.metadata {
            CallMetadata::Http(metadata) => {
                CallMetadata::Http(metadata.check(&endpoints, &self.output, self.id, fetch, triggers, codes)?)
            }
            CallMetadata::Ic(metadata) => {
                CallMetadata::Ic(metadata.check(&endpoints, &self.output, self.id, fetch, triggers, codes, apis)?)
            }
            CallMetadata::Evm(metadata) => {
                CallMetadata::Evm(metadata.check(&endpoints, &self.output, self.id, fetch, triggers, codes, apis)?)
            }
        };

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata,
            output: self.output.clone(),
        })
    }

    /// get call chain
    pub fn get_call_chain(&self) -> CallChain {
        match &self.metadata {
            CallMetadata::Http(_) => CallChain::Http,
            CallMetadata::Ic(_) => CallChain::InternetComputer,
            CallMetadata::Evm(evm) => evm.get_call_chain(),
        }
    }
}

impl ComponentId {
    /// Unified check the identity referenced
    #[inline]
    fn check_identity<F, M>(
        &self,
        endpoints: &AllEndpoints<'_>,
        check: F,        // Check the function
        check_err: &str, // Check the error message
        other: M,        // Continue to check
        from: ComponentId,
    ) -> Result<Self, LinkError>
    where
        F: Fn(&IdentityInnerMetadata) -> bool,
        M: Fn(&IdentityInnerMetadata, ComponentId) -> Result<(), LinkError>,
    {
        // Find the corresponding identity
        let component = endpoints
            .find_endpoint(&Endpoint { id: *self, index: None })
            .ok_or(LinkError::UnknownComponentOrNotRefer {
                from: Some(from),
                id: *self,
            })?
            .component;

        if let LinkComponent::Identity(ComponentIdentity {
            metadata: IdentityMetadata { metadata, .. },
            ..
        }) = component
        {
            // check
            if !check(metadata) {
                return Err(LinkError::InvalidCallIdentity((from, check_err.into()).into()));
            }
            // Continue to check
            other(metadata, from)?;
        } else {
            // Not identity component
            return Err(LinkError::InvalidCallIdentity(
                (from, "reference component is not identity".into()).into(),
            ));
        }

        Ok(*self)
    }
}
