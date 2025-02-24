use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::common::check::is_valid_variant_name;

use super::{
    error::LinkError,
    identity::ComponentId,
    values::{ArrayLinkValue, LinkValue},
};

/// Support type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LinkType {
    // ========== Simple type ==========
    /// text, Corresponding to JS string
    #[serde(rename = "text")]
    Text,
    /// boolean, Corresponding to JS boolean
    #[serde(rename = "bool")]
    Bool,
    /// integer, Corresponding to JS number
    /// Since the number of JS is only Number type, it is F64 type
    /// //! The supported integer range is -2^53 + 1 ~ 2^53 – 1
    /// //! Pay attention to check whether the range of this type is in line with the range Number.MIN_SAFE_INTEGER(-9007199254740991) <= x <= Number.MAX_SAFE_INTEGER(9007199254740991)
    /// Some data operations exceeding this range will fail
    #[serde(rename = "integer")]
    Integer,
    /// float, Corresponding to JS number
    #[serde(rename = "number")]
    Number,
    // ========== Composite ==========
    /// array, corresponding to JS Array
    #[serde(rename = "array")]
    Array(Box<LinkType>),
    /// object, Corresponding to JS object
    #[serde(rename = "object")]
    Object(Vec<ObjectSubitem>),
}

/// Subtype of object
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ObjectSubitem {
    /// key //! Must be in line with variable naming rules
    pub key: String,
    /// subtype
    pub ty: LinkType,
}

/// Object constructor
#[derive(Debug, Default)]
pub(crate) struct ObjectLinkTypeBuilder {
    subitems: Vec<ObjectSubitem>,
}

impl ObjectLinkTypeBuilder {
    pub(crate) fn push(mut self, key: impl Into<String>, ty: LinkType) -> Self {
        self.subitems.push(ObjectSubitem { key: key.into(), ty });
        self
    }
    pub(crate) fn build(self) -> LinkType {
        LinkType::Object(self.subitems)
    }
}

impl LinkType {
    /// Whether text
    pub fn is_text(&self) -> bool {
        matches!(self, LinkType::Text)
    }
    /// Whether
    pub fn is_bool(&self) -> bool {
        matches!(self, LinkType::Bool)
    }
    /// Whether an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, LinkType::Integer)
    }
    /// Whether to float point number
    pub fn is_number(&self) -> bool {
        matches!(self, LinkType::Number)
    }
    /// Whether an array
    pub fn is_array(&self) -> bool {
        matches!(self, LinkType::Array(_))
    }
    /// Whether
    pub fn is_object(&self) -> bool {
        matches!(self, LinkType::Object(_))
    }
    /// Whether the string array
    pub fn is_array_text(&self) -> bool {
        if let LinkType::Array(ty) = self {
            return ty.is_text();
        }
        false
    }
    /// Whether to BLOB
    pub fn is_blob(&self) -> bool {
        if let LinkType::Array(ty) = self {
            return ty.is_integer();
        }
        false
    }
    /// Whether to an empty object
    pub fn is_empty_object(&self) -> bool {
        if let LinkType::Object(items) = self {
            return items.is_empty();
        }
        false
    }

    /// New object
    pub fn new_object() -> LinkType {
        LinkType::Object(vec![])
    }

    /// Object constructor
    pub(crate) fn object_builder() -> ObjectLinkTypeBuilder {
        ObjectLinkTypeBuilder::default()
    }

    /// Whether the verification value matches the type
    pub fn is_match(&self, value: &LinkValue) -> bool {
        match (self, value) {
            (LinkType::Text, LinkValue::Text(_)) => true,
            (LinkType::Bool, LinkValue::Bool(_)) => true,
            (LinkType::Integer, LinkValue::Integer(_)) => true,
            (LinkType::Number, LinkValue::Number(_)) => true,
            (LinkType::Array(ty), LinkValue::Array(ArrayLinkValue { ty: value_ty, values })) => {
                if ty.as_ref() != value_ty {
                    return false;
                }
                for value in values {
                    if !ty.is_match(value) {
                        return false;
                    }
                }
                true
            }
            (LinkType::Object(subitems), LinkValue::Object(values)) => {
                if subitems.len() != values.len() {
                    return false;
                }
                for (subitem, value) in subitems.iter().zip(values.iter()) {
                    if subitem.key != value.key || !subitem.ty.is_match(&value.value) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// check
    pub fn check_keys<'a, I>(keys: I, from: ComponentId) -> Result<(), LinkError>
    where
        I: Iterator<Item = &'a String>,
    {
        let mut exist = HashSet::new(); // Records have appeared

        for key in keys {
            if exist.contains(key) {
                return Err(LinkError::DuplicateObjectKey { from, key: key.clone() });
            }

            if !is_valid_variant_name(key) {
                return Err(LinkError::InvalidObjectKey { from, key: key.clone() });
            }

            exist.insert(key);
        }

        Ok(())
    }

    /// check
    pub fn check(&self, from: ComponentId) -> Result<(), LinkError> {
        match self {
            LinkType::Text => {}
            LinkType::Bool => {}
            LinkType::Integer => {}
            LinkType::Number => {}
            LinkType::Array(ty) => ty.check(from)?,
            LinkType::Object(subitems) => {
                Self::check_keys(subitems.iter().map(|item| &item.key), from)?;
                for item in subitems {
                    item.ty.check(from)?;
                }
            }
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         println!("{}", serde_json::to_string(&LinkType::Text).unwrap());
//         println!(
//             "{}",
//             serde_json::to_string(&LinkType::Array(Box::new(LinkType::Text))).unwrap()
//         );
//         println!(
//             "{}",
//             serde_json::to_string(&LinkType::Object(vec![ObjectSubitem {
//                 key: "a".into(),
//                 ty: LinkType::Text
//             }]))
//             .unwrap()
//         );
//         println!("{}", LinkType::Text.typescript());
//         println!("{}", LinkType::Bool.typescript());
//         println!("{}", LinkType::Integer.typescript());
//         println!("{}", LinkType::Number.typescript());
//         println!("{}", LinkType::Array(Box::new(LinkType::Integer)).typescript());
//         println!(
//             "{}",
//             LinkType::Array(Box::new(LinkType::Object(vec![
//                 ObjectSubitem {
//                     key: "a".into(),
//                     ty: LinkType::Text
//                 },
//                 ObjectSubitem {
//                     key: "b".into(),
//                     ty: LinkType::Integer
//                 }
//             ])))
//             .typescript()
//         );
//     }
// }
