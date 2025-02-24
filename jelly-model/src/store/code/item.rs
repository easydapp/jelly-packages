use serde::{Deserialize, Serialize};

use crate::{
    common::hash::hash_sha256,
    types::{ContentHash, MAX_DATA_LENGTH},
};

/// Code constraint type
pub mod types;

/// Parameter multiple types
pub mod args;

/// Code with constraints
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct CodeItem {
    /// A constrained code
    pub code: String,
    /// Parameter type Parameters support multiple, so it needs to be named
    /// The post -processing code of http call may require multiple parameters
    /// IC calls have a front parameter, and the post -processing code should be processed
    /// EVM calls have the front parameters, and the code should be processed to the back processing
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub args: Option<Vec<args::ArgCodeType>>,
    /// The result of the result must be unique. The reason is that if you want to output Tuple, why not use Object?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret: Option<types::CodeType>,
}

impl CodeItem {
    /// Should
    pub fn should_into_anchor(&self) -> bool {
        MAX_DATA_LENGTH < self.code.len()
            || self
                .args
                .as_ref()
                .is_some_and(|args| args.iter().any(|arg| arg.should_into_anchor()))
            || self.ret.as_ref().is_some_and(|ret| ret.should_into_anchor())
    }

    /// hash
    /// The compiled results are also added to HASH, which is commonly used because it is prevented from upgrading the compiler. The results are different
    pub fn hash(&self, compiled: &str) -> Result<ContentHash, String> {
        let key = serde_json::to_string(self).map_err(|_| format!("serde code item failed: {:?}", self))?;
        Ok(hash_sha256(&format!("{key}{compiled}")))
    }
}
