use serde::{Deserialize, Serialize};

use crate::common::check::is_valid_variant_name;

use super::{ComponentId, ComponentParamRequired, LinkError};

/// Request parameters
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentParam {
    /// Id with each component
    pub id: ComponentId,

    // /// Dependencies // ? No need here
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // inlets: Option<Vec<Endpoint>>,
    /// metadata required for this component execution
    pub metadata: ParamMetadata,
    // Output type // ? No need here
    // output: LinkType, // ! Only output text type
}

/// param metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ParamMetadata {
    /// param name //! Must be in line with variable naming rules
    pub name: String,
    /// default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

impl ComponentParam {
    /// Get the parameter name
    pub fn get_param_name(&self) -> &String {
        &self.metadata.name
    }

    /// check
    pub fn check(&self) -> Result<Self, LinkError> {
        // 1. Check the validity of the name
        if !is_valid_variant_name(&self.metadata.name) {
            return Err(LinkError::InvalidVariantKey {
                from: self.id,
                key: self.metadata.name.clone(),
            });
        }

        Ok(self.clone())
    }

    /// required
    pub fn get_required(&self) -> ComponentParamRequired {
        ComponentParamRequired {
            id: self.id,
            name: self.metadata.name.clone(),
            default: self.metadata.default.clone(),
        }
    }
}
