use serde::{Deserialize, Serialize};
use template::NodeTemplate;

use super::common::identity::ComponentId;

/// Node template
pub mod template;

/// Template verification
pub mod validate;

/// get id
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimmedLinkComponent {
    /// Corresponding component ID
    pub id: ComponentId,
}

/// The value of constant template data is available
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimmedNodeDataTemplate {
    /// Analyze
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<TrimmedLinkComponent>,
    /// Template data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<NodeTemplate>,
}
/// Constitution template data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimmedNodeData {
    node_id: String,
    /// Simple template data
    pub data: TrimmedNodeDataTemplate,
}
/// Simple node
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimmedNode {
    /// Simple node data
    pub data: TrimmedNodeData,
}
