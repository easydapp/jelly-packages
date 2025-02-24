/// Allow the use of custom web pages
/// Need to provide data supply data: Option<Vec<CodeValue>>
/// Output specified data

/// choose metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionFrameMetadata {
    /// Web link
    url: InputValue,

    /// Provide data
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    data: Option<Vec<CodeValue>>,

    /// Other auxiliary style data
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}
