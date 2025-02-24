use candid::Principal;
use lazy_static::lazy_static;
use regex::Regex;

use crate::types::ContentHash;

lazy_static! {
    static ref REGEX_HASH_ANCHOR: Regex = get_hash_anchor_regex();
    static ref REGEX_U64_ANCHOR: Regex = get_u64_anchor_regex();
    static ref REGEX_VARIANT_NAME: Regex = get_variant_name_regex();
    static ref REGEX_EVM_ADDRESS: Regex = get_evm_address_regex();
    static ref REGEX_HEX_TEXT: Regex = get_hex_text_regex();
}

#[inline]
fn get_hash_anchor_regex() -> Regex {
    #[allow(clippy::unwrap_used)] // ? checked
    Regex::new(r"^([a-z0-9]{5}-){4}[a-z0-9]{3}#[a-f0-9]{64}$").unwrap()
}

#[inline]
fn get_u64_anchor_regex() -> Regex {
    #[allow(clippy::unwrap_used)] // ? checked
    Regex::new(r"^([a-z0-9]{5}-){4}[a-z0-9]{3}#[1-9][0-9]*$").unwrap()
}

#[inline]
fn get_variant_name_regex() -> Regex {
    #[allow(clippy::unwrap_used)] // ? checked
    Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]{0,63}$").unwrap()
}

#[inline]
fn get_evm_address_regex() -> Regex {
    #[allow(clippy::unwrap_used)] // ? checked
    Regex::new(r"^0[x|X][0-9a-fA-F]{40}$").unwrap()
}

#[inline]
fn get_hex_text_regex() -> Regex {
    #[allow(clippy::unwrap_used)] // ? checked
    Regex::new(r"^0[x|X]([0-9a-fA-F][0-9a-fA-F])+$").unwrap()
}

/// check hash anchor
#[inline]
pub fn check_hash_anchor(anchor: &str, prefix: &str) -> Result<(Principal, ContentHash), String> {
    #[inline]
    fn error() -> String {
        "anchor is invalid".to_string()
    }

    let prefix = format!("{prefix}#");
    let anchor = anchor
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("anchor must started with '{prefix}#': {anchor}"))?;

    if !REGEX_HASH_ANCHOR.is_match(anchor) {
        return Err(error());
    }

    let mut anchor = anchor.split('#');
    let canister_id = anchor.next().ok_or_else(error)?;
    let hash = anchor.next().ok_or_else(error)?;

    let canister_id = Principal::from_text(canister_id).map_err(|_| error())?;
    let hash = {
        let mut data = [0; 32];
        data.copy_from_slice(&hex::decode(hash).map_err(|_| error())?);
        data
    };

    Ok((canister_id, hash))
}

/// check number anchor
#[inline]
pub fn check_u64_anchor(anchor: &str, prefix: &str) -> Result<(Principal, u64), String> {
    #[inline]
    fn error() -> String {
        "anchor is invalid".to_string()
    }

    let prefix = format!("{prefix}#");
    let anchor = anchor
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("anchor must started with '{prefix}#': {anchor}"))?;

    if !REGEX_U64_ANCHOR.is_match(anchor) {
        return Err(error());
    }

    let mut anchor = anchor.split('#');
    let canister_id = anchor.next().ok_or_else(error)?;
    let id = anchor.next().ok_or_else(error)?;

    let canister_id = Principal::from_text(canister_id).map_err(|_| error())?;
    let id = id.parse().map_err(|_| error())?;

    Ok((canister_id, id))
}

/// check variant name
#[inline]
pub fn is_valid_variant_name(name: &str) -> bool {
    REGEX_VARIANT_NAME.is_match(name)
        && !matches!(
            name,
            "break"
                | "case"
                | "catch"
                | "class"
                | "const"
                | "continue"
                | "debugger"
                | "default"
                | "delete"
                | "do"
                | "else"
                | "enum"
                | "export"
                | "extends"
                | "false"
                | "finally"
                | "for"
                | "function"
                | "if"
                | "import"
                | "in"
                | "instanceof"
                | "new"
                | "null"
                | "return"
                | "super"
                | "switch"
                | "this"
                | "throw"
                | "true"
                | "try"
                | "typeof"
                | "var"
                | "void"
                | "while"
                | "with"
                | "yield"
                | "let"
                | "static"
                | "implements"
                | "interface"
                | "package"
                | "private"
                | "protected"
                | "public"
        )
}

/// check evm address
#[inline]
pub fn is_valid_evm_address(address: &str) -> bool {
    REGEX_EVM_ADDRESS.is_match(address)
}

/// check hex text
#[inline]
pub fn is_valid_hex_text(hex: &str) -> bool {
    REGEX_HEX_TEXT.is_match(hex)
}
