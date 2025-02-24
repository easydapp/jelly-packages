use serde::{Deserialize, Serialize};

use super::{ComponentId, LinkError, LinkType, LinkValue};

/// constant
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentConst {
    /// Id with each component
    pub id: ComponentId,

    // /// Dependencies // ? No need here
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // inlets: Option<Vec<Endpoint>>,
    /// metadata data required for this component execution
    pub metadata: ConstMetadata,

    /// Output type
    pub output: LinkType, // User specified type
}

/// const metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ConstMetadata {
    /// const value
    pub value: LinkValue,
}

impl ComponentConst {
    /// check
    pub fn check(&self) -> Result<Self, LinkError> {
        self.output.check(self.id)?; // ? Check whether the output type is correct

        // 1. Check whether the silent recognition is matched with the output type
        if !self.output.is_match(&self.metadata.value) {
            return Err(LinkError::MismatchedConstValue {
                from: self.id,
                output: self.output.clone(),
                value: self.metadata.value.clone(),
            });
        }

        Ok(self.clone())
    }
}
