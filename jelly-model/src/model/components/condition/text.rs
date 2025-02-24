use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkError};

/// Comparison of string
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionTextCompare {
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
    Contains(InputValue),
    /// Incompatible
    #[serde(rename = "not_contains")]
    NotContains(InputValue),
    /// starts with
    #[serde(rename = "starts_with")]
    StartsWith(InputValue),
    /// not starts with
    #[serde(rename = "not_starts_with")]
    NotStartsWith(InputValue),
    /// ends with
    #[serde(rename = "ends_with")]
    EndsWith(InputValue),
    /// not ends with
    #[serde(rename = "not_ends_with")]
    NotEndsWith(InputValue),
    /// Length equal
    #[serde(rename = "length_equal")]
    LengthEqual(InputValue),
    /// Length is not equal to
    #[serde(rename = "length_not_equal")]
    LengthNotEqual(InputValue),
    /// length Greater than
    #[serde(rename = "length_greater")]
    LengthGreater(InputValue),
    /// Length is greater than equal to
    #[serde(rename = "length_greater_equal")]
    LengthGreaterEqual(InputValue),
    /// Greater than
    #[serde(rename = "length_less")]
    LengthLess(InputValue),
    /// Length is greater than equal to
    #[serde(rename = "length_less_equal")]
    LengthLessEqual(InputValue),
    /// Regular
    #[serde(rename = "regex")]
    Regex(InputValue),
    /// not Regular
    #[serde(rename = "not_regex")]
    NotRegex(InputValue),
}

impl ConditionTextCompare {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::NotNull => false,
            Self::Equal(_) => false,
            Self::NotEqual(_) => false,
            Self::Contains(_) => false,
            Self::NotContains(_) => false,
            Self::StartsWith(_) => false,
            Self::NotStartsWith(_) => false,
            Self::EndsWith(_) => false,
            Self::NotEndsWith(_) => false,
            Self::LengthEqual(_) => false,
            Self::LengthNotEqual(_) => false,
            Self::LengthGreater(_) => false,
            Self::LengthGreaterEqual(_) => false,
            Self::LengthLess(_) => false,
            Self::LengthLessEqual(_) => false,
            Self::Regex(_) => false,
            Self::NotRegex(_) => false,
        }
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        match self {
            ConditionTextCompare::Null | ConditionTextCompare::NotNull => {}
            ConditionTextCompare::Equal(value)
            | ConditionTextCompare::NotEqual(value)
            | ConditionTextCompare::Contains(value)
            | ConditionTextCompare::NotContains(value)
            | ConditionTextCompare::StartsWith(value)
            | ConditionTextCompare::NotStartsWith(value)
            | ConditionTextCompare::EndsWith(value)
            | ConditionTextCompare::NotEndsWith(value)
            | ConditionTextCompare::Regex(value)
            | ConditionTextCompare::NotRegex(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if !value.is_text() {
                    return Err(LinkError::InvalidCondition((from, "value is not text".into()).into()));
                }
            }
            ConditionTextCompare::LengthEqual(value)
            | ConditionTextCompare::LengthNotEqual(value)
            | ConditionTextCompare::LengthGreater(value)
            | ConditionTextCompare::LengthGreaterEqual(value)
            | ConditionTextCompare::LengthLess(value)
            | ConditionTextCompare::LengthLessEqual(value) => {
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
