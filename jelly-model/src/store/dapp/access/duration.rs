use serde::{Deserialize, Serialize};

use crate::types::TimestampMills;

/// Visit time
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessDuration {
    /// Start time, including
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TimestampMills>,
    /// End time, not included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<TimestampMills>,
}

impl AccessDuration {
    /// Judgment authority
    pub fn access(&self, now: TimestampMills) -> bool {
        if let Some(start) = self.start {
            if now < start {
                return false;
            }
        }
        if let Some(end) = self.end {
            if end <= now {
                return false;
            }
        }
        true
    }

    /// Whether
    pub fn is_same(&self, verified: &VerifiedAccessDuration) -> bool {
        if self.start != verified.start {
            return false;
        }
        if self.end != verified.end {
            return false;
        }
        true
    }
}

/// View time verification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiedAccessDuration {
    /// Start time, including
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TimestampMills>,
    /// End time, not included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<TimestampMills>,
}
