use serde::{Deserialize, Serialize};

use crate::model::{
    common::{identity::ComponentId, types::LinkType},
    types::check::CheckedCodeItem,
};

use super::validate::NodeTemplateValidate;

/// Node template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeTemplate {
    node_id: String,
    /// Constant type
    pub output: LinkType,
    title: String,
    description: String,
    /// Verification method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<NodeTemplateValidate>,
}

impl NodeTemplate {
    /// Query code
    pub fn get_origin_codes(&self, from: ComponentId) -> Vec<CheckedCodeItem> {
        if let Some(validate) = &self.validate {
            return validate.get_origin_codes(&self.output, 0, from);
        }
        vec![]
    }
}
