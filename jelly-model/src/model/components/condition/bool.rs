use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkError};

/// Boer's comparative way
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionBoolCompare {
    /// Be null
    #[serde(rename = "null")]
    Null,
    /// Not null
    #[serde(rename = "not_null")]
    NotNull,
    /// equal
    #[serde(rename = "equal")]
    Equal(InputValue),
    /// Incompatible
    #[serde(rename = "not_equal")]
    NotEqual(InputValue),
    /// is true
    #[serde(rename = "is_true")]
    IsTrue,
    /// is false
    #[serde(rename = "is_false")]
    IsFalse,
}

impl ConditionBoolCompare {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::NotNull => false,
            Self::Equal(_) => false,
            Self::NotEqual(_) => false,
            Self::IsTrue => false,
            Self::IsFalse => false,
        }
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        match self {
            ConditionBoolCompare::Null | ConditionBoolCompare::NotNull => {}
            ConditionBoolCompare::Equal(value) | ConditionBoolCompare::NotEqual(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if !value.is_bool() {
                    return Err(LinkError::InvalidCondition((from, "value is not bool".into()).into()));
                }
            }
            ConditionBoolCompare::IsTrue | ConditionBoolCompare::IsFalse => {}
        }
        Ok(self.clone())
    }
}
