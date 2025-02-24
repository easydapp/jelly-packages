use serde::{Deserialize, Serialize};

// ABI type

/// Interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AbiType {
    /// function
    #[serde(rename = "function")]
    Function,
    /// Constructor
    #[serde(rename = "constructor")]
    Constructor,
    /// "Receive Ether Coin" function?
    #[serde(rename = "receive")]
    Receive,
    /// "callback" function?
    #[serde(rename = "fallback")]
    Fallback,
    /// event
    #[serde(rename = "event")]
    Event,
    /// mistake
    #[serde(rename = "error")]
    Error,
}

/// Parameter
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AbiParam {
    /// Parameter name
    pub name: String,
    /// type
    #[serde(rename = "type")]
    pub ty: String, // ? Pay attention to the effectiveness of the inspection, not all the string are types
    /// Internal type
    #[serde(rename = "internalType")]
    pub internal_type: Option<String>, // ? Sometimes you need additional judgment
    /// Tuple's specific subclass
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<AbiParam>>, // used for tuple types
    /// Event only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
}

/// stateMutability
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AbiStateMutability {
    /// Do not read the blockchain status, such as the calculation function
    #[serde(rename = "pure")]
    Pure,
    /// Do not modify the blockchain status, such as query interface
    #[serde(rename = "view")]
    View,
    /// Not accepting Ether
    #[serde(rename = "nonpayable")]
    Nonpayable,
    /// Accept Ether
    #[serde(rename = "payable")]
    Payable,
}

/// Single interface
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AbiItem {
    /// Functional type
    #[serde(rename = "type")]
    pub ty: AbiType,
    /// Function name
    /// constructor、receive、fallback No name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// enter
    /// receive and fallback No input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<AbiParam>>,
    /// Output
    /// constructor、receive、fallback No output
    /// event No output
    /// error No output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AbiParam>>,
    /// Degeneration
    /// event No
    /// error No
    #[serde(rename = "stateMutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<AbiStateMutability>,
    /// Whether to
    /// event only
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<bool>,
}
