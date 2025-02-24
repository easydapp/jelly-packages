use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkError, LinkType};

/// Comparison of array
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionArrayCompare {
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
    /// Include
    #[serde(rename = "contains")]
    Contains(InputValue), // Sub -type
    /// Not include
    #[serde(rename = "not_contains")]
    NotContains(InputValue),
    /// Length Equal
    #[serde(rename = "length_equal")]
    LengthEqual(InputValue),
    /// Length is not equal
    #[serde(rename = "length_not_equal")]
    LengthNotEqual(InputValue),
    /// Greater than
    #[serde(rename = "length_greater")]
    LengthGreater(InputValue),
    /// Length is greater than equal to
    #[serde(rename = "length_greater_equal")]
    LengthGreaterEqual(InputValue),
    /// Less than
    #[serde(rename = "length_less")]
    LengthLess(InputValue),
    /// Less than equal
    #[serde(rename = "length_less_equal")]
    LengthLessEqual(InputValue),
}

impl ConditionArrayCompare {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::NotNull => false,
            Self::Equal(_) => false,
            Self::NotEqual(_) => false,
            Self::Contains(_) => false,
            Self::NotContains(_) => false,
            Self::LengthEqual(_) => false,
            Self::LengthNotEqual(_) => false,
            Self::LengthGreater(_) => false,
            Self::LengthGreaterEqual(_) => false,
            Self::LengthLess(_) => false,
            Self::LengthLessEqual(_) => false,
        }
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, sub: &LinkType, from: ComponentId) -> Result<Self, LinkError> {
        match self {
            ConditionArrayCompare::Null | ConditionArrayCompare::NotNull => {}
            ConditionArrayCompare::Equal(value) | ConditionArrayCompare::NotEqual(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if value.as_ref() != &LinkType::Array(Box::new(sub.to_owned())) {
                    return Err(LinkError::InvalidCondition((from, "value is not match".into()).into()));
                }
            }
            ConditionArrayCompare::Contains(value) | ConditionArrayCompare::NotContains(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if value.as_ref() != sub {
                    return Err(LinkError::InvalidCondition((from, "value is not match".into()).into()));
                }
            }
            ConditionArrayCompare::LengthEqual(value)
            | ConditionArrayCompare::LengthNotEqual(value)
            | ConditionArrayCompare::LengthGreater(value)
            | ConditionArrayCompare::LengthGreaterEqual(value)
            | ConditionArrayCompare::LengthLess(value)
            | ConditionArrayCompare::LengthLessEqual(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if !value.is_integer() {
                    return Err(LinkError::InvalidCondition(
                        (from, "value is not integer".into()).into(),
                    ));
                }
            }
        }
        Ok(self.clone())
    }
}
