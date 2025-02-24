use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{AllEndpoints, ComponentId, Endpoint, InputValue, LinkError, LinkType, ObjectSubitem, ReferValue};

/// text
pub mod text;

/// bool
pub mod bool;

/// integer and number
pub mod number;

/// array
pub mod array;

/// object
pub mod object;

use array::ConditionArrayCompare;
use bool::ConditionBoolCompare;
use number::ConditionNumberCompare;
use object::ConditionObjectCompare;
use text::ConditionTextCompare;

/// condition
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentCondition {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    pub metadata: ConditionMetadata,
    // Output type // ? No need here
    // pub output: LinkType, // ! The output of each branch is BOOL type
}

/// condition metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ConditionMetadata {
    /// Condition judgment, at least one
    pub conditions: Vec<Condition>,
}

/// condition
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Condition {
    /// No demand
    #[serde(rename = "none")]
    None,
    /// Must be completely satisfied
    #[serde(rename = "required")]
    Required(ConditionItem),
    /// Must be completely dissatisfied
    #[serde(rename = "deny")]
    Deny(ConditionItem),
    /// Be satisfied
    #[serde(rename = "and")]
    And(Vec<Condition>),
    /// Arbitrarily
    #[serde(rename = "or")]
    Or(Vec<Condition>),
    /// Must be not satisfied
    #[serde(rename = "not")]
    Not(Vec<Condition>),
}

/// condition
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ConditionItem {
    /// Left value
    pub value: ReferValue,
    /// Comparison and right value
    pub matches: ConditionMatches,
}

/// Compare
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ConditionMatches {
    /// text
    #[serde(rename = "text")]
    Text(ConditionTextCompare),
    /// Boolean
    #[serde(rename = "bool")]
    Bool(ConditionBoolCompare),
    /// Integer
    #[serde(rename = "integer")]
    Integer(ConditionNumberCompare),
    /// Floating point number
    #[serde(rename = "number")]
    Number(ConditionNumberCompare),
    /// Array
    #[serde(rename = "array")]
    Array(ConditionArrayCompare),
    /// Object
    #[serde(rename = "object")]
    Object(ConditionObjectCompare),
}

impl ComponentCondition {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// Calculate the output point of the component
    /// Number of conditions + 1 ELSE
    pub fn count_outputs(&self) -> u32 {
        1 + self.metadata.conditions.len() as u32
    }

    /// Query can be empty introduced point
    pub fn get_nullable_endpoints(&self) -> Option<Vec<Endpoint>> {
        let mut endpoints = Vec::new();

        for item in self.metadata.conditions.iter() {
            endpoints.extend(item.get_nullable_endpoints());
        }

        (!endpoints.is_empty()).then(|| endpoints.into_iter().collect::<HashSet<_>>().into_iter().collect())
    }

    /// check
    pub fn check(&self, endpoints: &Option<AllEndpoints<'_>>) -> Result<Self, LinkError> {
        // 0 Check whether the reference is matched
        let endpoints = endpoints
            .as_ref()
            .ok_or(LinkError::MismatchedInlets { from: self.id })?; // There must be reference

        // 1. Check condition
        let mut conditions = Vec::with_capacity(self.metadata.conditions.len());
        for item in self.metadata.conditions.iter() {
            conditions.push(item.check(endpoints, self.id)?);
        }

        // 2. The number of conditions cannot be empty
        if conditions.is_empty() {
            return Err(LinkError::InvalidCondition(
                (self.id, "condition must greater then 1".into()).into(),
            ));
        }

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata: ConditionMetadata { conditions },
        })
    }
}

impl Condition {
    /// Query can be empty introduced point
    pub fn get_nullable_endpoints(&self) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();
        match self {
            Condition::None => {}
            Condition::Required(item) => endpoints.extend(item.get_nullable_endpoints()),
            Condition::Deny(item) => endpoints.extend(item.get_nullable_endpoints()),
            Condition::And(items) => {
                for item in items {
                    endpoints.extend(item.get_nullable_endpoints());
                }
            }
            Condition::Or(items) => {
                for item in items {
                    endpoints.extend(item.get_nullable_endpoints());
                }
            }
            Condition::Not(items) => {
                for item in items {
                    endpoints.extend(item.get_nullable_endpoints());
                }
            }
        }
        endpoints
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        match self {
            Condition::None => Ok(Self::None),
            Condition::Required(item) => Ok(Self::Required(item.check(endpoints, from)?)),
            Condition::Deny(item) => Ok(Self::Deny(item.check(endpoints, from)?)),
            Condition::And(items) => {
                if items.len() < 2 {
                    return Err(LinkError::InvalidCondition(
                        (from, "items of condition AND must greater then 2".into()).into(),
                    ));
                }
                let mut inner = Vec::with_capacity(items.len());
                for item in items {
                    inner.push(item.check(endpoints, from)?);
                }
                Ok(Self::And(inner))
            }
            Condition::Or(items) => {
                if items.is_empty() {
                    return Err(LinkError::InvalidCondition(
                        (from, "items of condition OR must greater then 2".into()).into(),
                    ));
                }
                let mut inner = Vec::with_capacity(items.len());
                for item in items {
                    inner.push(item.check(endpoints, from)?);
                }
                Ok(Self::Or(inner))
            }
            Condition::Not(items) => {
                if items.is_empty() {
                    return Err(LinkError::InvalidCondition(
                        (from, "items of condition NOT must greater then 2".into()).into(),
                    ));
                }
                let mut inner = Vec::with_capacity(items.len());
                for item in items {
                    inner.push(item.check(endpoints, from)?);
                }
                Ok(Self::Not(inner))
            }
        }
    }
}

impl ConditionItem {
    /// Query can be empty introduced point
    pub fn get_nullable_endpoints(&self) -> Vec<Endpoint> {
        if self.matches.is_nullable() {
            vec![self.value.endpoint]
        } else {
            Vec::new()
        }
    }

    /// check
    pub fn check(&self, endpoints: &AllEndpoints<'_>, from: ComponentId) -> Result<Self, LinkError> {
        let value = endpoints.check_refer_value(&self.value, from)?;

        let matches = match (value.as_ref(), &self.matches) {
            (LinkType::Text, ConditionMatches::Text(compare)) => {
                ConditionMatches::Text(compare.check(endpoints, from)?)
            }
            (LinkType::Bool, ConditionMatches::Bool(compare)) => {
                ConditionMatches::Bool(compare.check(endpoints, from)?)
            }
            (LinkType::Integer, ConditionMatches::Integer(compare)) => {
                ConditionMatches::Integer(compare.check(endpoints, true, from)?)
            }
            (LinkType::Number, ConditionMatches::Number(compare)) => {
                ConditionMatches::Number(compare.check(endpoints, false, from)?)
            }
            (LinkType::Array(sub), ConditionMatches::Array(compare)) => {
                ConditionMatches::Array(compare.check(endpoints, sub, from)?)
            }
            (LinkType::Object(items), ConditionMatches::Object(compare)) => {
                ConditionMatches::Object(compare.check(endpoints, items, from)?)
            }
            _ => {
                return Err(LinkError::InvalidCondition(
                    (from, "refer type is not match".into()).into(),
                ))
            }
        };

        Ok(Self {
            value: self.value.clone(),
            matches,
        })
    }
}

impl ConditionMatches {
    fn is_nullable(&self) -> bool {
        match self {
            ConditionMatches::Text(compare) => compare.is_nullable(),
            ConditionMatches::Bool(compare) => compare.is_nullable(),
            ConditionMatches::Integer(compare) => compare.is_nullable(),
            ConditionMatches::Number(compare) => compare.is_nullable(),
            ConditionMatches::Array(compare) => compare.is_nullable(),
            ConditionMatches::Object(compare) => compare.is_nullable(),
        }
    }
}
