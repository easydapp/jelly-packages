use serde::{Deserialize, Serialize};

use crate::types::MAX_DATA_LENGTH;

/// Code parameter constraint type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct CodeType {
    /// TS constraint type
    pub ty: String,
    /// If there is a separate type constraint
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub types: Option<Vec<String>>,
}

impl CodeType {
    /// new
    pub fn from_ty(ty: impl Into<String>) -> Self {
        Self {
            ty: ty.into(),
            types: None,
        }
    }
    /// new
    pub fn from_types(ty: impl Into<String>, types: Option<Vec<String>>) -> Self {
        Self { ty: ty.into(), types }
    }
    /// new
    pub fn from_name(ty: impl Into<String>, name: &Option<String>) -> Self {
        match name.as_ref() {
            Some(name) => Self::from_types(name.clone(), Some(vec![format!("type {name} = {};", ty.into())])),
            None => Self::from_ty(ty),
        }
    }
    /// new
    pub fn from_types_and_name(ty: impl Into<String>, types: Option<Vec<String>>, name: &Option<String>) -> Self {
        match name.as_ref() {
            Some(name) => {
                let mut types = types.unwrap_or_default();
                types.push(format!("type {name} = {};", ty.into()));
                Self::from_types(name.clone(), Some(types))
            }
            None => Self::from_types(ty, types),
        }
    }

    /// new undefined
    pub fn undefined() -> Self {
        Self::from_ty("undefined")
    }
    /// new any
    pub fn any() -> Self {
        Self::from_ty("any")
    }

    /// Judging whether it is equal
    pub fn is_same_typescript(&self, typescript: &str) -> bool {
        if self.types.is_some() {
            return false;
        }
        self.ty == typescript
    }

    /// Should
    pub fn should_into_anchor(&self) -> bool {
        MAX_DATA_LENGTH < self.ty.len() || self.types.as_ref().is_some_and(|types| MAX_DATA_LENGTH < types.len())
    }
}
