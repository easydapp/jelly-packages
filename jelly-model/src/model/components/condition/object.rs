use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, InputValue, LinkType, ObjectSubitem, LinkError};

/// Comparison of objects
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionObjectCompare {
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
    /// Contains key
    #[serde(rename = "contains_key")]
    ContainsKey(InputValue),
    /// Do not contain key
    #[serde(rename = "not_contains_key")]
    NotContainsKey(InputValue),
    /// Contains value
    #[serde(rename = "contains_value")]
    ContainsValue(InputValue),
    /// Does not include Value
    #[serde(rename = "not_contains_value")]
    NotContainsValue(InputValue),
}

impl ConditionObjectCompare {
    pub(super) fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::NotNull => false,
            Self::Equal(_) => false,
            Self::NotEqual(_) => false,
            Self::ContainsKey(_) => false,
            Self::NotContainsKey(_) => false,
            Self::ContainsValue(_) => false,
            Self::NotContainsValue(_) => false,
        }
    }

    /// check
    pub fn check(
        &self,
        endpoints: &AllEndpoints<'_>,
        items: &Vec<ObjectSubitem>,
        from: ComponentId,
    ) -> Result<Self, LinkError> {
        match self {
            ConditionObjectCompare::Null | ConditionObjectCompare::NotNull => {}
            ConditionObjectCompare::Equal(value) | ConditionObjectCompare::NotEqual(value) => {
                let value = endpoints.check_input_value(value, from)?;
                let matched = if let LinkType::Object(subitems) = value.as_ref() {
                    subitems == items
                } else {
                    false
                };
                if !matched {
                    return Err(LinkError::InvalidCondition(
                        (from, "value is not match".into()).into(),
                    ));
                }
            }
            ConditionObjectCompare::ContainsKey(value) | ConditionObjectCompare::NotContainsKey(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if !value.is_text() {
                    return Err(LinkError::InvalidCondition(
                        (from, "value is not text".into()).into(),
                    ));
                }
            }
            ConditionObjectCompare::ContainsValue(value) | ConditionObjectCompare::NotContainsValue(value) => {
                let value = endpoints.check_input_value(value, from)?;
                if !items.iter().any(|item| &item.ty == value.as_ref()) {
                    return Err(LinkError::InvalidCondition(
                        (from, "value is not match".into()).into(),
                    ));
                }
            }
        }

        Ok(self.clone())
    }
}
