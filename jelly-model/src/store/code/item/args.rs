use serde::{Deserialize, Serialize};

use crate::types::MAX_DATA_LENGTH;

use super::types::CodeType;

/// Parameters support multiple
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ArgCodeType {
    /// Parameter alias
    name: String,
    /// Parameter constraint
    ty: CodeType,
}

impl ArgCodeType {
    /// new
    pub fn from(name: &str, ty: CodeType) -> Self {
        Self { name: name.into(), ty }
    }

    /// Should
    pub fn should_into_anchor(&self) -> bool {
        MAX_DATA_LENGTH < self.name.len() || self.ty.should_into_anchor()
    }
}
