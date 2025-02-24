use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::types::CallChain;

use super::{
    AbiItem, AbiParam, AbiStateMutability, AllEndpoints, ApiData, ApiDataAnchor, ArgCodeType, CheckFunction,
    CheckedCodeItem, CodeContent, CodeData, CodeDataAnchor, CodeItem, CodeType, CodeValue, ComponentCallTrigger,
    ComponentId, ComponentTriggered, EvmCallApi, EvmChain, IdentityInnerMetadata, InputValue, LinkError, LinkType,
};

/// evm action
pub mod action;

use action::EvmAction;

/// evm metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CallEvmMetadata {
    /// Trigger condition
    trigger: ComponentCallTrigger,

    /// The required identity empty indicates the use of anonymous identity
    /// It must be the IdentityEvmMetadata type node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ComponentId>,

    /// EVM compatible chain
    chain: EvmChain,

    /// EVM call behavior
    action: EvmAction,
}

impl CallEvmMetadata {
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
    ) -> Result<CallEvmMetadata, LinkError> {
        // 1 check trigger
        let trigger = self.trigger.check(endpoints, from)?;

        // 2. Check identity
        let mut identity = None;
        if let Some(identity_ref) = self.identity {
            identity = Some(identity_ref.check_identity(
                endpoints,
                |metadata| matches!(metadata, IdentityInnerMetadata::Evm(_)),
                "identity component is not evm",
                |evm, from| {
                    if let IdentityInnerMetadata::Evm(evm) = evm {
                        // 3. Check whether the chain matches
                        if evm.chain != self.chain {
                            return Err(LinkError::InvalidCallIdentity((from, "chain mismatch".into()).into()));
                        }
                    }
                    Ok(())
                },
                from,
            )?);
        }

        // 3. Recording trigger method
        triggers.insert(
            from,
            ComponentTriggered::from_call(
                from,
                identity,
                matches!(trigger, ComponentCallTrigger::Click { .. }),
                matches!(
                    self.action,
                    EvmAction::Sign(_) | EvmAction::Transaction(_) | EvmAction::Deploy(_) | EvmAction::Transfer(_)
                ),
            ),
        );

        // 4. check chain
        let chain = self.chain.clone();

        // 5. check action
        let action = self.action.check(endpoints, output, from, fetch, codes, apis)?;

        Ok(Self {
            trigger,
            identity,
            chain,
            action,
        })
    }

    /// get call chain
    pub fn get_call_chain(&self) -> CallChain {
        self.chain.get_call_chain()
    }
}
