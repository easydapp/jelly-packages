use array::NodeTemplateValidateArray;
use boolean::NodeTemplateValidateBool;
use evm_address::NodeTemplateValidateEvmAddress;
use hex::NodeTemplateValidateHex;
use integer::NodeTemplateValidateInteger;
use number::NodeTemplateValidateNumber;
use object::NodeTemplateValidateObject;
use serde::{Deserialize, Serialize};

use image::NodeTemplateValidateImage;
use principal::NodeTemplateValidatePrincipal;
use text::NodeTemplateValidateText;

#[cfg(feature = "validate")]
use super::super::{
    common::{error::LinkError, values::LinkValue},
    types::check::CheckFunction,
};
use super::super::{
    common::{identity::ComponentId, types::LinkType},
    types::check::CheckedCodeItem,
};

/// text
pub mod text;

/// picture
pub mod image;

/// address
pub mod evm_address;

/// principal
pub mod principal;

/// hex
pub mod hex;

/// boolean
pub mod boolean;
/// Integer
pub mod integer;
/// Floating point number
pub mod number;

/// array
pub mod array;
/// object
pub mod object;

/// Template verification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeTemplateValidate {
    /// String constraint
    #[serde(rename = "text")]
    Text(NodeTemplateValidateText),
    /// Picture constraint
    #[serde(rename = "image")]
    Image(NodeTemplateValidateImage),
    /// Principal constraint
    #[serde(rename = "principal")]
    Principal(NodeTemplateValidatePrincipal),
    /// String constraint
    #[serde(rename = "evm_address")]
    EvmAddress(NodeTemplateValidateEvmAddress),
    /// hex constraint
    #[serde(rename = "hex")]
    Hex(NodeTemplateValidateHex),
    /// Boer restraint
    #[serde(rename = "bool")]
    Bool(NodeTemplateValidateBool),
    /// Integer constraint
    #[serde(rename = "integer")]
    Integer(NodeTemplateValidateInteger),
    /// Digital constraint
    #[serde(rename = "number")]
    Number(NodeTemplateValidateNumber),
    /// Array
    #[serde(rename = "array")]
    Array(NodeTemplateValidateArray),
    /// Object
    #[serde(rename = "object")]
    Object(NodeTemplateValidateObject),
}

impl NodeTemplateValidate {
    /// Query code
    pub fn get_origin_codes(&self, output: &LinkType, index: u32, from: ComponentId) -> Vec<CheckedCodeItem> {
        let mut codes = vec![];
        match self {
            Self::Text(text) => codes.extend(text.get_origin_codes(output, index, from)),
            Self::Image(image) => codes.extend(image.get_origin_codes()),
            Self::Principal(principal) => codes.extend(principal.get_origin_codes()),
            Self::EvmAddress(evm_address) => codes.extend(evm_address.get_origin_codes()),
            Self::Hex(hex) => codes.extend(hex.get_origin_codes()),
            Self::Bool(boolean) => codes.extend(boolean.get_origin_codes()),
            Self::Integer(integer) => codes.extend(integer.get_origin_codes(output, index, from)),
            Self::Number(number) => codes.extend(number.get_origin_codes(output, index, from)),
            Self::Array(array) => codes.extend(array.get_origin_codes(output, index, from)),
            Self::Object(object) => codes.extend(object.get_origin_codes(output, index, from)),
        }
        codes
    }

    /// Verification code
    #[cfg(feature = "validate")]
    pub fn validate<F: CheckFunction>(
        &self,
        output: &LinkType,
        value: &LinkValue,
        from: ComponentId,
        fetch: &F,
    ) -> Result<(), LinkError> {
        match self {
            Self::Text(text) => text.validate(output, value, from, fetch),
            Self::Image(image) => image.validate(output, value, from),
            Self::Principal(principal) => principal.validate(output, value, from),
            Self::EvmAddress(evm_address) => evm_address.validate(output, value, from),
            Self::Hex(hex) => hex.validate(output, value, from),
            Self::Bool(bool) => bool.validate(output, value, from),
            Self::Integer(integer) => integer.validate(output, value, from, fetch),
            Self::Number(number) => number.validate(output, value, from, fetch),
            Self::Array(array) => array.validate(output, value, from, fetch),
            Self::Object(object) => object.validate(output, value, from, fetch),
        }
    }
}
