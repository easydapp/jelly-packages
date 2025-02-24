use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::types::CallChain;

use super::{
    AllEndpoints, ComponentId, ComponentIdentityRequired, ComponentTriggered, Endpoint, EvmChain, EvmWallet, IcWallet,
    InputValue, LinkError, LinkType,
};

use evm::IdentityEvmMetadata;
use http::IdentityHttpMetadata;
use ic::IdentityIcMetadata;

/// http identity
pub mod http;

/// ic identity
pub mod ic;

/// evm identity
pub mod evm;

/// identity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentIdentity {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    inlets: Option<Vec<Endpoint>>,

    /// metadata data required for this component execution
    pub metadata: IdentityMetadata,
    // The specific output type of each identity of the output type is different
    // pub output: LinkType,
}

/// identity metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct IdentityMetadata {
    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// metadata
    pub metadata: IdentityInnerMetadata,
}

/// identity metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IdentityInnerMetadata {
    /// http
    #[serde(rename = "http")]
    Http(IdentityHttpMetadata),

    /// ic
    #[serde(rename = "ic")]
    Ic(IdentityIcMetadata),

    /// evm
    #[serde(rename = "evm")]
    Evm(IdentityEvmMetadata),
}

impl ComponentIdentity {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(metadata) => metadata.get_output_type(),
            IdentityInnerMetadata::Ic(metadata) => metadata.get_output_type(),
            IdentityInnerMetadata::Evm(metadata) => metadata.get_output_type(),
        }
    }

    /// Get the parameter name
    pub fn get_identity_name(&self) -> Option<&String> {
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(_) => {}
            IdentityInnerMetadata::Ic(_) => return self.metadata.name.as_ref(),
            IdentityInnerMetadata::Evm(_) => return self.metadata.name.as_ref(),
        }
        None
    }

    /// check
    pub fn check(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
    ) -> Result<Self, LinkError> {
        // 1. Check the introduction variable
        if !matches!(
            (endpoints.as_ref(), self.inlets.as_ref()),
            (Some(_), Some(_)) | (None, None)
        ) {
            return Err(LinkError::MismatchedInlets { from: self.id });
        }

        // 2. check name
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(_) => {
                if self.metadata.name.is_some() {
                    // ! HTTP is not allowed to have name
                    return Err(LinkError::NeedlessCallHttpName { from: self.id });
                }
            }
            IdentityInnerMetadata::Ic(_) => {}
            IdentityInnerMetadata::Evm(_) => {}
        }

        // 3. check metadata
        let metadata = match &self.metadata.metadata {
            IdentityInnerMetadata::Http(metadata) => IdentityInnerMetadata::Http(metadata.check(self.id, triggers)?),
            IdentityInnerMetadata::Ic(metadata) => {
                IdentityInnerMetadata::Ic(metadata.check(endpoints, self.id, triggers)?)
            }
            IdentityInnerMetadata::Evm(metadata) => {
                IdentityInnerMetadata::Evm(metadata.check(endpoints, self.id, triggers)?)
            }
        };

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata: IdentityMetadata {
                name: self.metadata.name.clone(),
                metadata,
            },
        })
    }

    /// Whether to
    pub fn is_anonymous(&self) -> bool {
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(_) => true,
            IdentityInnerMetadata::Ic(metadata) => metadata.is_anonymous(),
            IdentityInnerMetadata::Evm(metadata) => metadata.is_anonymous(),
        }
    }

    /// identity
    pub fn get_required(&self) -> Option<ComponentIdentityRequired> {
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(_) => {}
            IdentityInnerMetadata::Ic(metadata) => {
                if !metadata.is_anonymous() {
                    return Some(ComponentIdentityRequired {
                        id: self.id,
                        name: self.metadata.name.clone(),
                        metadata: IdentityInnerMetadata::Ic(metadata.clone()),
                    });
                }
            }
            IdentityInnerMetadata::Evm(metadata) => {
                if !metadata.is_anonymous() {
                    return Some(ComponentIdentityRequired {
                        id: self.id,
                        name: self.metadata.name.clone(),
                        metadata: IdentityInnerMetadata::Evm(metadata.clone()),
                    });
                }
            }
        }
        None
    }

    /// get call chain
    pub fn get_call_chain(&self) -> CallChain {
        match &self.metadata.metadata {
            IdentityInnerMetadata::Http(_) => CallChain::Http,
            IdentityInnerMetadata::Ic(_) => CallChain::InternetComputer,
            IdentityInnerMetadata::Evm(evm) => evm.get_call_chain(),
        }
    }

    // /// Whether to match your identity
    // pub fn is_match_identity(&self, metadata: &ComponentIdentityRequired) -> bool {
    //     match &self.metadata.metadata {
    //         IdentityInnerMetadata::Http(_) => matches!(metadata.metadata, IdentityInnerMetadata::Http(_)),
    //         IdentityInnerMetadata::Ic(_) => matches!(metadata.metadata, IdentityInnerMetadata::Ic(_)),
    //         IdentityInnerMetadata::Evm(_) => matches!(metadata.metadata, IdentityInnerMetadata::Evm(_)),
    //     }
    // }
}
