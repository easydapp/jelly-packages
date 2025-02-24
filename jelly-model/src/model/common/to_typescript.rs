use ic_canister_kit::types::{
    WrappedCandidType, WrappedCandidTypeFunction, WrappedCandidTypeName, WrappedCandidTypeRecord,
    WrappedCandidTypeRecursion, WrappedCandidTypeReference, WrappedCandidTypeService, WrappedCandidTypeSubtype,
    WrappedCandidTypeTuple, WrappedCandidTypeVariant,
};

use crate::{model::types::abi::types::AbiParam, store::code::item::types::CodeType};

use super::{error::LinkError, identity::ComponentId, types::LinkType};

const MAX_LENGTH: usize = 50;

/// Analysis into TypeScript constraints
pub trait ToTypescript {
    /// ts constraint
    fn to_typescript(&self, from: ComponentId) -> Result<CodeType, LinkError>;
}

// =========== Combined function ===========

/// combination object
fn combine_typescript_object(key_and_types: Vec<(String, String)>) -> String {
    if key_and_types.is_empty() {
        return "{}".into();
    }
    let single = format!(
        "{{ {} }}",
        key_and_types
            .iter()
            .map(|sub| format!("{}: {}", sub.0, sub.1))
            .collect::<Vec<String>>()
            .join("; ")
    );
    if single.len() <= MAX_LENGTH && key_and_types.iter().all(|sub| !sub.1.contains('\n')) {
        return single;
    }
    format!(
        "{{\n{}\n}}",
        key_and_types
            .iter()
            .map(|sub| format!(
                "  {}: {};", // ! tabSize=2
                sub.0,
                sub.1
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("\n  ") // ! tabSize=2
            ))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

/// combination tuple
fn combine_typescript_tuple(types: Vec<(String,)>) -> String {
    if types.is_empty() {
        return "[]".into();
    }
    let single = format!("[{}]", types.iter().map(|s| s.0.clone()).collect::<Vec<_>>().join(", "));
    if single.len() <= MAX_LENGTH && types.iter().all(|sub| !sub.0.contains('\n')) {
        return single;
    }
    format!(
        "[\n{}\n]",
        types
            .iter()
            .map(|sub| format!(
                "  {},", // ! tabSize=2
                sub.0
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("\n  ")  // ! tabSize=2
            ))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

/// combination option
fn combine_typescript_option(ty: String) -> String {
    let single = format!("([] | [{ty}])");
    if single.len() <= MAX_LENGTH && !ty.contains("\n") {
        return single;
    }
    format!(
        "(\n  | []\n  | [\n      {}\n    ]\n)", // ! tabSize=2 ？
        ty.split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n      ")  // ! tabSize=2 ？
    )
}

/// combination variant
fn combine_typescript_variant(key_and_types: Vec<(String, String)>) -> String {
    if key_and_types.is_empty() {
        return "{}".into();
    }
    let ty2 = key_and_types
        .iter()
        .map(|s| combine_typescript_object(vec![s.clone()]))
        .collect::<Vec<_>>();
    let single = format!("({})", ty2.join(" | "));
    if single.len() <= MAX_LENGTH && ty2.iter().all(|ty| !ty.contains('\n')) {
        return single;
    }
    format!(
        "(\n{}\n)",
        ty2.iter()
            .map(|ty| format!(
                "  | {}", // ! tabSize=2
                ty.split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("\n    ")  // ! tabSize=2
            ))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

// =========== link type ===========

impl LinkType {
    /// Convert typescript
    pub fn typescript(&self) -> String {
        match self {
            LinkType::Text => "string".to_string(),
            LinkType::Bool => "boolean".to_string(),
            LinkType::Integer => "number".to_string(),
            LinkType::Number => "number".to_string(),
            LinkType::Array(ty) => format!("{}[]", ty.typescript()),
            LinkType::Object(items) => {
                let key_and_types = items
                    .iter()
                    .map(|sub| (sub.key.clone(), sub.ty.typescript()))
                    .collect::<Vec<_>>();
                combine_typescript_object(key_and_types)
            }
        }
    }
}

impl ToTypescript for LinkType {
    fn to_typescript(&self, _from: ComponentId) -> Result<CodeType, LinkError> {
        Ok(CodeType::from_ty(self.typescript()))
    }
}

// =========== candid type ===========

impl ToTypescript for WrappedCandidType {
    /// Convert typescript
    fn to_typescript(&self, from: ComponentId) -> Result<CodeType, LinkError> {
        let ty = match self {
            WrappedCandidType::Bool(WrappedCandidTypeName { name }) => CodeType::from_name("boolean", name),
            WrappedCandidType::Nat(WrappedCandidTypeName { name }) => CodeType::from_name("bigint", name), // * The code supports Bigint constraints and Bigint structure
            WrappedCandidType::Int(WrappedCandidTypeName { name }) => CodeType::from_name("bigint", name), // * The code supports Bigint constraints and Bigint structure
            WrappedCandidType::Nat8(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Nat16(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Nat32(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Nat64(WrappedCandidTypeName { name }) => CodeType::from_name("bigint", name), // * The code supports Bigint constraints and Bigint structure
            WrappedCandidType::Int8(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Int16(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Int32(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Int64(WrappedCandidTypeName { name }) => CodeType::from_name("bigint", name), // * The code supports Bigint constraints and Bigint structure
            WrappedCandidType::Float32(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Float64(WrappedCandidTypeName { name }) => CodeType::from_name("number", name),
            WrappedCandidType::Null(WrappedCandidTypeName { name }) => CodeType::from_name("null", name),
            WrappedCandidType::Text(WrappedCandidTypeName { name }) => CodeType::from_name("string", name),
            WrappedCandidType::Principal(WrappedCandidTypeName { name }) => CodeType::from_name("Principal", name), // * The code supports the use of Principal // ! During the front -end inspection, you need to check whether the object is a Principal object
            WrappedCandidType::Vec(WrappedCandidTypeSubtype { subtype, name }) => {
                if let WrappedCandidType::Nat8(_) = subtype.as_ref() {
                    return Ok(CodeType::from_name("(Uint8Array | number[])", name));
                }
                let sub = subtype.to_typescript(from)?;
                CodeType::from_types_and_name(format!("{}[]", sub.ty), sub.types, name)
            }
            WrappedCandidType::Opt(WrappedCandidTypeSubtype { subtype, name }) => {
                let sub = subtype.to_typescript(from)?;
                CodeType::from_types_and_name(combine_typescript_option(sub.ty), sub.types, name)
            }
            WrappedCandidType::Record(WrappedCandidTypeRecord { subitems, name }) => {
                let mut ty = Vec::with_capacity(subitems.len());
                let mut types = Vec::new();

                for (key, sub) in subitems {
                    let sub = sub.to_typescript(from)?;
                    ty.push((key.clone(), sub.ty));
                    if let Some(_types) = sub.types {
                        types.extend(_types);
                    }
                }

                CodeType::from_types_and_name(
                    combine_typescript_object(ty),
                    if types.is_empty() { None } else { Some(types) },
                    name,
                )
            }
            WrappedCandidType::Variant(WrappedCandidTypeVariant { subitems, name }) => {
                let mut ty = Vec::with_capacity(subitems.len());
                let mut types = Vec::new();

                for (key, sub) in subitems {
                    match sub.as_ref() {
                        Some(sub) => {
                            let sub = sub.to_typescript(from)?;
                            ty.push((key.clone(), sub.ty));
                            if let Some(_types) = sub.types {
                                types.extend(_types);
                            }
                        }
                        None => ty.push((key.clone(), "null".into())),
                    }
                }

                CodeType::from_types_and_name(
                    combine_typescript_variant(ty),
                    if types.is_empty() { None } else { Some(types) },
                    name,
                )
            }
            WrappedCandidType::Tuple(WrappedCandidTypeTuple { subitems, name }) => {
                let mut ty = Vec::with_capacity(subitems.len());
                let mut types = Vec::new();

                for sub in subitems {
                    let sub = sub.to_typescript(from)?;
                    ty.push((sub.ty,));
                    if let Some(_types) = sub.types {
                        types.extend(_types);
                    }
                }

                CodeType::from_types_and_name(
                    combine_typescript_tuple(ty),
                    if types.is_empty() { None } else { Some(types) },
                    name,
                )
            }
            WrappedCandidType::Unknown(WrappedCandidTypeName { name }) => {
                CodeType::from_name("unknown", name)
                // return Err(LinkError::CompileCallIcCandidTypeUnsupported {
                //     from,
                //     ty: "unknown".into(),
                // })
            } // ! unknown
            WrappedCandidType::Empty(WrappedCandidTypeName { name }) => {
                CodeType::from_name("any", name)
                // return Err(LinkError::CompileCallIcCandidTypeUnsupported { from, ty: "empty".into() })
            } // ! unknown
            WrappedCandidType::Reserved(WrappedCandidTypeName { name }) => {
                CodeType::from_name("any", name)
                // return Err(LinkError::CompileCallIcCandidTypeUnsupported {
                //     from,
                //     ty: "reserved".into(),
                // })
            } // ! unknown
            WrappedCandidType::Func(WrappedCandidTypeFunction { name, .. }) => {
                CodeType::from_name("any", name)
                // return Err(LinkError::CompileCallIcCandidTypeUnsupported {
                //     from,
                //     ty: "func".into(),
                // })
            } // ! unknown
            WrappedCandidType::Service(WrappedCandidTypeService { .. }) => {
                return Err(LinkError::CompileCallIcCandidTypeUnsupported {
                    from,
                    ty: "service".into(),
                })
            } // ! unknown
            WrappedCandidType::Rec(WrappedCandidTypeRecursion { ty, name, .. }) => {
                let CodeType { ty, types } = ty.to_typescript(from)?;

                let name = name.clone().ok_or_else(|| LinkError::CompileCallIcCandid {
                    from,
                    candid: self.to_text(),
                    message: "recursion type must has name".into(),
                })?;

                CodeType::from_types_and_name(ty, types, &Some(name))
            }
            WrappedCandidType::Reference(WrappedCandidTypeReference { name, .. }) => {
                let name = name.clone().ok_or_else(|| LinkError::CompileCallIcCandid {
                    from,
                    candid: self.to_text(),
                    message: "recursion type must has name".into(),
                })?;

                CodeType::from_ty(name) // ! The name referenced
            }
        };
        Ok(ty)
    }
}

/// Analyze
pub fn candid_types_to_typescript(items: &[WrappedCandidType], from: ComponentId) -> Result<CodeType, LinkError> {
    // Empty parameter
    if items.is_empty() {
        return Ok(CodeType::from_ty("[]"));
    }

    // Only one parameter
    if 1 == items.len() {
        return items[0].to_typescript(from);
    }

    // Multiple parameters
    let mut ty = Vec::with_capacity(items.len());
    let mut types = Vec::new();
    for item in items {
        let code_type = item.to_typescript(from)?;
        ty.push((code_type.ty,));
        if let Some(_types) = code_type.types {
            types.extend(_types)
        }
    }
    Ok(CodeType::from_types(
        combine_typescript_tuple(ty),
        if types.is_empty() { None } else { Some(types) },
    ))
}

// =========== abi type ===========

mod evm_abi {
    use crate::model::{
        common::{error::LinkError, identity::ComponentId},
        types::abi::types::AbiParam,
    };

    use super::combine_typescript_tuple;

    #[inline]
    fn evm_api_error(from: ComponentId, ty: &str) -> LinkError {
        LinkError::InvalidCallEvmActionApi((from, format!("unsupported type {}", ty)).into())
    }

    #[derive(Debug)]
    pub(super) struct AbiTy<'a, 'b> {
        ty: &'a str,
        components: &'a Option<Vec<AbiParam>>,
        codes: &'b [char],
        cursor: usize,
    }
    impl<'a, 'b> AbiTy<'a, 'b> {
        pub(super) fn from(ty: &'a str, components: &'a Option<Vec<AbiParam>>, codes: &'b [char]) -> Self {
            Self {
                ty,
                components,
                codes,
                cursor: 0,
            }
        } // Whether the later characters are valid
        fn has(&self, n: usize) -> bool {
            self.cursor + n < self.codes.len()
        } // Remaining string
        fn remain(&self, cursor: Option<usize>) -> String {
            let cursor = cursor.unwrap_or(self.cursor);
            let mut remain = String::new();
            let mut offset = 0;
            while self.has(offset) {
                remain.push(self.codes[cursor + offset]);
                offset += 1;
            }
            remain
        }
        // Whether the next field is a custom character sequence
        fn is_next(&self, types: &[char]) -> bool {
            if !self.has(types.len() - 1) {
                return false;
            }
            for (i, c) in types.iter().enumerate() {
                if *c == self.codes[self.cursor + i] {
                    continue;
                }
                return false;
            }
            true
        }
        // Skip invalid characters
        fn trim_start(&mut self, chars: &[char]) {
            while self.has(0) {
                let current = self.codes[self.cursor];
                if current == ' ' || current == '\t' {
                    self.cursor += 1
                } else if chars.contains(&current) {
                    self.cursor += 1;
                } else {
                    break;
                }
            }
        }
        // Convert the entire type
        pub(super) fn parse_typescript(&mut self, from: ComponentId) -> Result<String, LinkError> {
            let ty = self
                .read_typescript(from)?
                .ok_or_else(|| evm_api_error(from, self.ty))?;
            // println!("parse_typescript: {}", ty);
            if self.cursor != self.codes.len() {
                return Err(evm_api_error(from, self.ty));
            }
            Ok(ty)
        }
        // Try to take a type
        fn read_typescript(&mut self, from: ComponentId) -> Result<Option<String>, LinkError> {
            // println!("{:?}", self);
            self.trim_start(&[]);
            let ty = if self.codes.len() <= self.cursor {
                return Ok(None);
            } else if matches!(self.codes.last(), Some(']')) {
                let mut index = self.codes.len() - 1;
                loop {
                    if self.codes[index] == '[' {
                        break;
                    }
                    if index == 0 {
                        break;
                    }
                    index -= 1;
                }
                if index == 0 || index == self.codes.len() - 1 {
                    return Err(evm_api_error(from, self.ty));
                }
                let mut abi = AbiTy::from(self.ty, self.components, &self.codes[0..index]);
                // println!("{:?}", abi);
                let ty = abi.parse_typescript(from)?;
                self.cursor = self.codes.len();
                &format!("{ty}[]",) // Array // ? The length of the change is not processed
            } else if self.is_next(&['m', 'a', 'p', 'p', 'i', 'n', 'g']) {
                self.cursor += 7;
                self.trim_start(&[]);
                if !self.is_next(&['(']) {
                    return Err(evm_api_error(from, self.ty));
                }
                self.cursor += 1;
                let key = self
                    .read_typescript(from)?
                    .ok_or_else(|| evm_api_error(from, self.ty))?;
                self.trim_start(&[]);
                if !self.is_next(&['=', '>']) {
                    return Err(evm_api_error(from, self.ty));
                }
                self.cursor += 2;
                let value = self
                    .read_typescript(from)?
                    .ok_or_else(|| evm_api_error(from, self.ty))?;
                self.trim_start(&[]);
                if !self.is_next(&[')']) {
                    return Err(evm_api_error(from, self.ty));
                }
                self.cursor += 1;
                &format!("Record<{key}, {value}>")
            } else {
                let remain = self.remain(None);
                // println!("remain: {:?}", remain);
                match remain.as_str() {
                    // Boolean
                    "bool" => {
                        self.cursor += 4;
                        "boolean"
                    }
                    // Integral
                    "int" => {
                        self.cursor += 3;
                        "bigint" // * Support Bigint
                    } // int256 Alias
                    "uint" => {
                        self.cursor += 4;
                        "bigint" // * Support Bigint
                    } // uint256 Alias
                    // * Other unified judgments
                    "fixed" => {
                        self.cursor += 5;
                        "string"
                    } // fixed128x18
                    // Fixed floating point type: ufixedMxN and fixedMxN M Must in 8-256 and 8 of the integer，N must be between 0-80
                    "ufixed" => {
                        self.cursor += 6;
                        "string"
                    } // ufixed128x18
                    // * Other unified judgments
                    // address type uint160 Integer bytes20
                    "address" => {
                        self.cursor += 7;
                        "string"
                    } // ? When calculating, you should need to be converted
                    "address payable" => {
                        self.cursor += 15;
                        "string"
                    } // ? When calculating, you should need to be converted
                    // Fixed -length byte array bytes1, bytes2, bytes3, ..., bytes32
                    // * Other unified judgments
                    // Changing byte array
                    // Strough string word quantity and type
                    // Array
                    "bytes" => {
                        self.cursor += 5;
                        "(string | Uint8Array)" // * Support UINT8ARAY
                    }
                    "string" => {
                        self.cursor += 6;
                        "string"
                    }
                    // Structure
                    // Mapping
                    // mapping(address => uint256)
                    // mapping(address => mapping(address => uint256))
                    // Functional type
                    "function" => {
                        self.cursor += 8;
                        "string"
                    } // bytes24 20 is the contract address 4 is the method hash
                    "tuple" => {
                        self.cursor += 5;
                        match self.components.as_ref() {
                            Some(components) => {
                                let mut subtypes = Vec::with_capacity(components.len());
                                for param in components {
                                    let ty = param.typescript(from)?;
                                    subtypes.push((ty,));
                                }
                                &combine_typescript_tuple(subtypes)
                            }
                            None => "[]",
                        }
                    }
                    _ => {
                        fn is_valid_bits(bits: u16) -> bool {
                            0 < bits && bits <= 256 && bits % 8 == 0
                        }
                        fn is_valid_n(n: u8) -> bool {
                            0 < n && n <= 80
                        }
                        fn is_valid_bytes_length(length: u8) -> bool {
                            0 < length && length <= 32
                        }
                        // Integer
                        fn parse_bits(bits: &str, from: ComponentId, ty: &str) -> Result<&'static str, LinkError> {
                            let bits = bits.parse::<u16>().map_err(|_| evm_api_error(from, ty))?;
                            if !is_valid_bits(bits) {
                                return Err(evm_api_error(from, ty));
                            }
                            Ok("bigint") // * Support Bigint
                        }
                        // Fixed -point number
                        fn parse_fixed(left: &str, from: ComponentId, ty: &str) -> Result<&'static str, LinkError> {
                            let mut left = left.split("x");
                            let m = left
                                .next()
                                .ok_or_else(|| evm_api_error(from, ty))?
                                .parse::<u16>()
                                .map_err(|_| evm_api_error(from, ty))?;
                            if !is_valid_bits(m) {
                                return Err(evm_api_error(from, ty));
                            }
                            let n = left
                                .next()
                                .ok_or_else(|| evm_api_error(from, ty))?
                                .parse::<u8>()
                                .map_err(|_| evm_api_error(from, ty))?;
                            if !is_valid_n(n) {
                                return Err(evm_api_error(from, ty));
                            }
                            if m < n as u16 {
                                return Err(evm_api_error(from, ty));
                            }
                            Ok("string")
                        }
                        // Fixed -length byte array
                        fn parse_bytes(length: &str, from: ComponentId, ty: &str) -> Result<&'static str, LinkError> {
                            let length = length.parse::<u8>().map_err(|_| evm_api_error(from, ty))?;
                            if !is_valid_bytes_length(length) {
                                return Err(evm_api_error(from, ty));
                            }
                            Ok("(string | Uint8Array)") // * Support UINT8ARAY
                        }
                        if let Some(bits) = remain.strip_prefix("int") {
                            self.cursor += 3 + bits.len();
                            parse_bits(bits, from, self.ty)?
                        } else if let Some(bits) = remain.strip_prefix("uint") {
                            self.cursor += 4 + bits.len();
                            parse_bits(bits, from, self.ty)?
                        } else if let Some(left) = remain.strip_prefix("ufixed") {
                            self.cursor += 6 + left.len();
                            parse_fixed(left, from, self.ty)?
                        } else if let Some(left) = remain.strip_prefix("fixed") {
                            self.cursor += 5 + left.len();
                            parse_fixed(left, from, self.ty)?
                        } else if let Some(length) = remain.strip_prefix("bytes") {
                            self.cursor += 5 + length.len();
                            parse_bytes(length, from, self.ty)?
                        } else {
                            return Err(evm_api_error(from, self.ty));
                        }
                    }
                }
            };
            Ok(Some(ty.into()))
        }
    }
}

impl AbiParam {
    /// ts
    pub fn typescript(&self, from: ComponentId) -> Result<String, LinkError> {
        let codes = self.ty.trim().chars().collect::<Vec<_>>();
        let mut ty = evm_abi::AbiTy::from(&self.ty, &self.components, &codes);
        ty.parse_typescript(from)
    }
}

impl ToTypescript for AbiParam {
    fn to_typescript(&self, from: ComponentId) -> Result<CodeType, LinkError> {
        let ty = self.typescript(from)?;
        Ok(CodeType::from_ty(ty))
    }
}

/// Analyze
pub fn abi_params_to_typescript(items: &[AbiParam], from: ComponentId) -> Result<CodeType, LinkError> {
    // Empty parameter
    if items.is_empty() {
        return Ok(CodeType::from_ty("[]"));
    }

    // Only one parameter
    if 1 == items.len() {
        return items[0].to_typescript(from);
    }

    // Multiple parameters
    let mut ty = Vec::with_capacity(items.len());
    let mut types = Vec::new();
    for item in items {
        let code_type = item.to_typescript(from)?;
        ty.push((code_type.ty,));
        if let Some(_types) = code_type.types {
            types.extend(_types)
        }
    }
    Ok(CodeType::from_types(
        combine_typescript_tuple(ty),
        if types.is_empty() { None } else { Some(types) },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let param = AbiParam {
            name: "owner".into(),
            ty: "address".into(),
            internal_type: Some("address".into()),
            components: None,
            indexed: None,
        };
        let r = param.typescript(1.into());
        println!("{:?}", r);

        let param = AbiParam {
            name: "owner".into(),
            ty: "uint[]".into(),
            internal_type: Some("uint[]".into()),
            components: None,
            indexed: None,
        };
        let r = param.typescript(1.into());
        println!("{:?}", r);

        let param = AbiParam {
            name: "owner".into(),
            ty: "uint256[]".into(),
            internal_type: Some("uint256[]".into()),
            components: None,
            indexed: None,
        };
        let r = param.typescript(1.into());
        println!("{:?}", r);
    }
}
