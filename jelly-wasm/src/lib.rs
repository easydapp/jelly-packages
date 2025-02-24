#![doc = include_str!("../README.md")]

use jelly_model::model::types::check::CheckedCombined;
use wasm_bindgen::prelude::*;

/// test
#[cfg(test)]
mod test;

// ===================== Transport object =====================

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum JellyResult {
    #[serde(rename = "ok")]
    Ok(String),
    #[serde(rename = "err")]
    Err(String),
}

impl From<Result<String, String>> for JellyResult {
    fn from(value: Result<String, String>) -> Self {
        match value {
            Ok(value) => JellyResult::Ok(value),
            Err(err) => JellyResult::Err(err),
        }
    }
}

impl From<JellyResult> for String {
    fn from(value: JellyResult) -> Self {
        serde_json::to_string(&value).unwrap_or_default()
    }
}

// ===================== Execute code =====================

/// execute code
///
/// # Arguments
///
/// * `code` - Code
/// * `args` - The string of parameter name and parameter. For example "[]" or "[[\"data\",\"{}\"]]"
#[wasm_bindgen]
pub fn execute_code(code: &str, args: &str) -> String {
    let result = jelly_executor::execute_code(code, args).map_err(|err| format!("{:?}", err));
    let result: JellyResult = result.into();
    result.into()
}

/// execute validate code
///
/// # Arguments
///
/// * `code` - Code
/// * `value` - Verified value. For example "{\"text\":\"text\""}"
#[wasm_bindgen]
pub fn execute_validate_code(code: &str, value: &str) -> String {
    let result = jelly_executor::execute_validate_code(code, value).map_err(|err| format!("{:?}", err));
    let result: JellyResult = result.into();
    result.into()
}

// ===================== parse candid =====================

/// parse candid
///
/// # Arguments
///
/// * `candid` - candid
#[wasm_bindgen]
pub fn parse_service_candid(candid: &str) -> String {
    fn inner(candid: &str) -> Result<String, String> {
        let service = ic_canister_kit::candid::parse_service_candid(candid).map_err(|err| format!("{:?}", err))?;
        serde_json::to_string(&service).map_err(|e| format!("stringify service failed: {}", e))
    }

    let result = inner(candid);
    let result: JellyResult = result.into();
    result.into()
}

/// parse func candid
///
/// # Arguments
///
/// * `func` - func candid
#[wasm_bindgen]
pub fn parse_func_candid(func: &str) -> String {
    fn inner(func: &str) -> Result<String, String> {
        let service = ic_canister_kit::candid::parse_service_candid(
            &r##"service : {
    #func#
}"##
            .replace("#func#", func),
        )
        .map_err(|err| format!("{:?}", err))?;
        serde_json::to_string(&service).map_err(|e| format!("stringify service failed: {}", e))
    }

    let result = inner(func);
    let result: JellyResult = result.into();
    result.into()
}

// ===================== check combined =====================

/// find all anchors
///
/// # Arguments
///
/// * `components` - components
#[wasm_bindgen]
pub fn find_all_anchors(components: &str) -> String {
    use jelly_model::model::LinkComponent;

    fn inner(components: &str) -> Result<String, String> {
        let components: Vec<LinkComponent> =
            serde_json::from_str(components).map_err(|e| format!("parse components failed: {}", e))?;
        let anchors = match jelly_model::model::check::find_all_anchors(&components) {
            Ok(anchors) => anchors,
            Err(err) => {
                return Err(serde_json::to_string(&err).map_err(|e| format!("stringify link error failed: {}", e))?)
            }
        };
        serde_json::to_string(&anchors).map_err(|e| format!("stringify anchors failed: {}", e))
    }

    let result = inner(components);
    let result: JellyResult = result.into();
    result.into()
}

/// find all origin codes
///
/// # Arguments
///
/// * `components` - components
/// * `fetch` - check function
#[wasm_bindgen]
pub fn find_origin_codes(components: &str, fetch: &str) -> String {
    use jelly_model::model::types::check::ApisCheckFunction;
    use jelly_model::model::LinkComponent;

    fn inner(components: &str, fetch: &str) -> Result<String, String> {
        let components: Vec<LinkComponent> =
            serde_json::from_str(components).map_err(|e| format!("parse components failed: {}", e))?;
        let fetch: ApisCheckFunction =
            serde_json::from_str(fetch).map_err(|e| format!("parse ApisCheckFunction failed: {}", e))?;
        let code_items = match jelly_model::model::check::find_origin_codes(&components, &fetch) {
            Ok(code_items) => code_items,
            Err(err) => {
                return Err(serde_json::to_string(&err).map_err(|e| format!("stringify link error failed: {}", e))?)
            }
        };
        serde_json::to_string(&code_items).map_err(|e| format!("stringify code_items failed: {}", e))
    }

    let result = inner(components, fetch);
    let result: JellyResult = result.into();
    result.into()
}

/// find all template origin codes
///
/// # Arguments
///
/// * `nodes` - nodes
#[wasm_bindgen]
pub fn find_template_origin_codes(nodes: &str) -> String {
    use jelly_model::model::node::TrimmedNode;

    fn inner(nodes: &str) -> Result<String, String> {
        let nodes: Vec<TrimmedNode> = serde_json::from_str(nodes).map_err(|e| format!("parse nodes failed: {}", e))?;
        let code_items = match jelly_model::model::check::find_template_origin_codes(&nodes) {
            Ok(code_items) => code_items,
            Err(err) => {
                return Err(serde_json::to_string(&err).map_err(|e| format!("stringify link error failed: {}", e))?)
            }
        };
        serde_json::to_string(&code_items).map_err(|e| format!("stringify code_items failed: {}", e))
    }

    let result = inner(nodes);
    let result: JellyResult = result.into();
    result.into()
}

/// Final checking
///
/// # Arguments
///
/// * `components` - components
/// * `fetch` - check function
#[wasm_bindgen]
pub fn check(components: &str, fetch: &str) -> String {
    use jelly_model::model::types::check::ApisCheckFunction;
    use jelly_model::model::LinkComponent;

    fn inner(components: &str, fetch: &str) -> Result<String, String> {
        let components: Vec<LinkComponent> =
            serde_json::from_str(components).map_err(|e| format!("parse components failed: {}", e))?;
        let fetch: ApisCheckFunction =
            serde_json::from_str(fetch).map_err(|e| format!("parse ApisCheckFunction failed: {}", e))?;
        let checked = match jelly_model::model::check::check(&components, &fetch) {
            Ok(checked) => checked,
            Err(err) => {
                return Err(serde_json::to_string(&err).map_err(|e| format!("stringify link error failed: {}", e))?)
            }
        };
        serde_json::to_string(&checked).map_err(|e| format!("stringify checked failed: {}", e))
    }

    let result = inner(components, fetch);
    let result: JellyResult = result.into();
    result.into()
}

/// Final checking
///
/// # Arguments
///
/// * `nodes` - nodes
/// * `checked` - checked
/// * `fetch` - check function
#[wasm_bindgen]
pub fn check_template(nodes: &str, checked: &str, fetch: &str) -> String {
    use jelly_model::model::node::TrimmedNode;
    use jelly_model::model::types::check::ApisCheckFunction;

    fn inner(nodes: &str, checked: &str, fetch: &str) -> Result<String, String> {
        let nodes: Vec<TrimmedNode> = serde_json::from_str(nodes).map_err(|e| format!("parse nodes failed: {}", e))?;
        let checked: CheckedCombined =
            serde_json::from_str(checked).map_err(|e| format!("parse checked failed: {}", e))?;
        let fetch: ApisCheckFunction =
            serde_json::from_str(fetch).map_err(|e| format!("parse ApisCheckFunction failed: {}", e))?;
        let template = match jelly_model::model::check::check_templates(&nodes, &checked, &fetch) {
            Ok(template) => template,
            Err(err) => {
                return Err(serde_json::to_string(&err).map_err(|e| format!("stringify link error failed: {}", e))?)
            }
        };
        serde_json::to_string(&template).map_err(|e| format!("stringify template failed: {}", e))
    }

    let result = inner(nodes, checked, fetch);
    let result: JellyResult = result.into();
    result.into()
}
