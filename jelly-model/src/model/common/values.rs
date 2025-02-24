use serde::{Deserialize, Serialize};

use super::{
    error::LinkError,
    identity::ComponentId,
    types::{LinkType, ObjectSubitem},
};

/// Array value
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ArrayLinkValue {
    /// If the array is empty, it will lose the subtype. Here to record subtype
    pub ty: LinkType,
    /// array value
    pub values: Vec<LinkValue>,
}

/// Support value
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LinkValue {
    // ========== Simple type ==========
    /// text, Corresponding to JS string
    #[serde(rename = "text")]
    Text(String),
    /// boolean, Corresponding to JS boolean
    #[serde(rename = "bool")]
    Bool(bool),
    /// integer, Corresponding to JS number
    /// Since the number of JS is only Number type, it is F64 type
    /// //! The supported integer range is -2^53 + 1 ~ 2^53 – 1
    /// //! Pay attention to check whether the range of this type is in line with the range Number.MIN_SAFE_INTEGER(-9007199254740991) <= x <= Number.MAX_SAFE_INTEGER(9007199254740991)
    /// Some data operations exceeding this range will fail
    #[serde(rename = "integer")]
    Integer(i64),
    /// float, Corresponding to JS number
    #[serde(rename = "number")]
    Number(f64),
    // ========== Composite ==========
    /// array, corresponding to JS Array
    #[serde(rename = "array")]
    Array(ArrayLinkValue),
    /// object, Corresponding to JS object
    #[serde(rename = "object")]
    Object(Vec<ObjectSubitemValue>),
}

impl Eq for LinkValue {}

/// sub value of object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ObjectSubitemValue {
    /// key //! Must be in line with variable naming rules
    pub key: String,
    /// sub value
    pub value: LinkValue,
}

impl LinkValue {
    /// Get value type
    #[inline]
    pub fn link_type(&self) -> LinkType {
        match self {
            LinkValue::Text(_) => LinkType::Text,
            LinkValue::Bool(_) => LinkType::Bool,
            LinkValue::Integer(_) => LinkType::Integer,
            LinkValue::Number(_) => LinkType::Number,
            LinkValue::Array(ArrayLinkValue { ty, .. }) => LinkType::Array(Box::new(ty.clone())),
            LinkValue::Object(values) => LinkType::Object(
                values
                    .iter()
                    .map(|value| ObjectSubitem {
                        key: value.key.clone(),
                        ty: value.value.link_type(),
                    })
                    .collect(),
            ),
        }
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        match self {
            LinkValue::Text(_) => {}
            LinkValue::Bool(_) => {}
            LinkValue::Integer(_) => {}
            LinkValue::Number(_) => {}
            LinkValue::Array(ArrayLinkValue { ty, values }) => {
                ty.check(from)?;
                for value in values {
                    if !ty.is_match(value) {
                        return Err(LinkError::MismatchedLinkValueType {
                            from,
                            value: self.clone(),
                        });
                    }
                }
            }
            LinkValue::Object(values) => {
                LinkType::check_keys(values.iter().map(|v| &v.key), from)?;
                for value in values {
                    value.value.check(from)?;
                }
            }
        }
        Ok(())
    }
}
