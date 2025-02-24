use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, CodeValue, ComponentId, Endpoint, LinkError, LinkType};

/// Output component
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentOutput {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    #[serde(skip_serializing_if = "is_output_metadata_empty")]
    metadata: Option<OutputMetadata>,

    /// Types introduced by the output type to the output type
    pub output: LinkType, // User specified type
}

/// output metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct OutputMetadata {
    /// Code execution parameter
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    data: Option<Vec<CodeValue>>,
}

/// Determine whether it is necessary to serialize
fn is_output_metadata_empty(metadata: &Option<OutputMetadata>) -> bool {
    metadata
        .as_ref()
        .is_none_or(|m| m.data.as_ref().is_none_or(|d| d.is_empty()))
}

impl ComponentOutput {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>) -> Result<Self, LinkError> {
        self.output.check(self.id)?; // ? Check whether the output type is correct

        // Check the introduction variable
        let output = endpoints
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_default()
            .check_code_values(
                self.metadata
                    .as_ref()
                    .and_then(|d| d.data.as_ref())
                    .map(Cow::Borrowed)
                    .unwrap_or_default()
                    .iter(),
                self.id,
            )?;
        if self.output != output {
            return Err(LinkError::MismatchedOutput { from: self.id });
        }

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata: self.metadata.clone(),
            output: self.output.clone(),
        })
    }
}
