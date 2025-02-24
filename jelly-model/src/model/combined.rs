use serde::{Deserialize, Serialize};

use crate::store::{api::anchor::ApiDataAnchor, code::anchor::CodeDataAnchor, combined::anchor::CombinedAnchor};

use super::{
    common::{identity::ComponentId, types::LinkType},
    components::{identity::IdentityInnerMetadata, interaction::InteractionInnerMetadata},
};

/// Combined metadata needs to be used when referenced
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CombinedMetadata {
    // ! The data that must be provided can be run to run
    /// Record the parameter name
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub params: Option<Vec<ComponentParamRequired>>,
    /// The identity data required for running is not anonymous
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub identities: Option<Vec<ComponentIdentityRequired>>,

    // ! If it is referenced, the parameter prompt is required
    /// Record form component
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub forms: Option<Vec<ComponentFormRequired>>,
    /// Record interaction component
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub interactions: Option<Vec<ComponentInteractionRequired>>,

    // ! The data that can be loaded in advance, the data in Anchor is cached
    /// code
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub code_anchors: Option<Vec<CodeDataAnchor>>,
    /// api
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub apis_anchors: Option<Vec<ApiDataAnchor>>,
    /// combined
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub combined_anchors: Option<Vec<CombinedAnchor>>,

    // ! Output type
    /// Record output type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<LinkType>,
}

impl CombinedMetadata {
    /// Whether the object is empty, meaningless object
    pub fn is_empty(&self) -> bool {
        crate::is_empty_option_vec(&self.params)
            && crate::is_empty_option_vec(&self.identities)
            && crate::is_empty_option_vec(&self.code_anchors)
            && crate::is_empty_option_vec(&self.apis_anchors)
            && crate::is_empty_option_vec(&self.combined_anchors)
            && crate::is_empty_option_vec(&self.forms)
            && crate::is_empty_option_vec(&self.interactions)
            && self.output.is_none()
    }

    /// Whether
    pub fn is_metadata_empty(metadata: &Option<Self>) -> bool {
        metadata.as_ref().is_none_or(|m| m.is_empty())
    }
}

/// param
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentParamRequired {
    /// Id with each component
    pub id: ComponentId,
    /// param name //! Must be in line with variable naming rules
    pub name: String,
    /// default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// User form input
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentFormRequired {
    /// Id with each component
    pub id: ComponentId,

    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Output type
    pub output: LinkType, // User specified type
}

/// identity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentIdentityRequired {
    /// Id with each component
    pub id: ComponentId,

    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Metall data required for this component execution
    pub metadata: IdentityInnerMetadata,
}

/// Interaction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentInteractionRequired {
    /// Id with each component
    pub id: ComponentId,

    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Metall data required for this component execution
    pub metadata: InteractionInnerMetadata,
}
