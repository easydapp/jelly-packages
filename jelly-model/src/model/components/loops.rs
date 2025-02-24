use serde::{Deserialize, Serialize};

use crate::model::common::{lets::Endpoint, refer::ReferValue};

use super::ComponentId;

/// Output component
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentLoop {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    metadata: LoopMetadata,
    // Output type
    // pub output: LinkType, // Loop output type
}

/// loop metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LoopMetadata {
    /// Cycle
    way: LoopWay,
    // /// Cyclic content
    // /// OUTPUT must be the only one, otherwise it will not be sure that the output type
    // components: Vec<LoopComponent>,
}

/// loop way
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LoopWay {
    /// Iteration data
    #[serde(rename = "iteration")]
    Iteration(ReferValue), // This reference must be an array
    /// unlimited
    #[serde(rename = "infinite")]
    Infinite,
}
