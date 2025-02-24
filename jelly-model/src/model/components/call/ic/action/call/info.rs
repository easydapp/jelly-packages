use serde::{Deserialize, Serialize};

use super::TimestampMills;

/// Basic information
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CanisterInfo {
    /// module hash
    module_hash: String,
    /// Update time
    updated: TimestampMills,
}
