use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkError};

/// Number comparison
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionNumberCompare {
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
    /// Greater than
    #[serde(rename = "greater")]
    Greater(InputValue),
    /// Greater than equal
    #[serde(rename = "greater_equal")]
    GreaterEqual(InputValue),
    /// Less than
    #[serde(rename = "less")]
    Less(InputValue),
    /// Less than equal
    #[serde(rename = "less_equal")]
    LessEqual(InputValue),
}

impl ConditionNumberCompare {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::NotNull => false,
            Self::Equal(_) => false,
            Self::NotEqual(_) => false,
            Self::Greater(_) => false,
            Self::GreaterEqual(_) => false,
            Self::Less(_) => false,
            Self::LessEqual(_) => false,
        }
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, is_integer: bool, from: ComponentId) -> Result<Self, LinkError> {
        match self {
            ConditionNumberCompare::Null | ConditionNumberCompare::NotNull => {}
            ConditionNumberCompare::Equal(value)
            | ConditionNumberCompare::NotEqual(value)
            | ConditionNumberCompare::Greater(value)
            | ConditionNumberCompare::GreaterEqual(value)
            | ConditionNumberCompare::Less(value)
            | ConditionNumberCompare::LessEqual(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if is_integer {
                    if !value.is_integer() {
                        return Err(LinkError::InvalidCondition(
                            (from, "value is not integer".into()).into(),
                        ));
                    }
                } else if !value.is_number() {
                    return Err(LinkError::InvalidCondition((from, "value is not number".into()).into()));
                }
            }
        }
        Ok(self.clone())
    }
}
