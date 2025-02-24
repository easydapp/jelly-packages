use serde::{Deserialize, Serialize};

/// Execute code error
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ExecuteCodeError {
    /// Execute code parameter error
    InvalidArgs(String),
    /// Direct results output is wrong
    InvalidOutput(String),
    /// The execution result is undefined
    Undefined,
    /// The execution results output type is wrong
    WrongOutput(String),
    /// Execute error
    ExecuteError(String),
}
