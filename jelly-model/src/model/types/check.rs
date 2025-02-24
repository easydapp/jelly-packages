use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    model::{CombinedMetadata, LinkComponent, common::identity::ComponentId},
    store::{
        api::{ApiData, anchor::ApiDataAnchor},
        code::{CodeData, anchor::CodeDataAnchor, item::CodeItem},
        combined::{Combined, anchor::CombinedAnchor},
    },
};

/// Find all the cache key
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckedAnchors {
    /// Code
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub code_anchors: Option<Vec<CodeDataAnchor>>,
    /// api
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub api_anchors: Option<Vec<ApiDataAnchor>>,
    /// combined
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    pub combined_anchors: Option<Vec<CombinedAnchor>>,
}

/// Retrieve the function
pub trait CheckFunction {
    /// Get the latest Canister_id
    fn canister_id(&self) -> Result<&str, String>;

    /// get code
    fn fetch_code(&self, code_anchor: &CodeDataAnchor) -> Result<&CodeData, String>;

    /// get api
    fn fetch_api(&self, api_anchor: &ApiDataAnchor) -> Result<&ApiData, String>;

    /// get combined, If there is an introduction component, it still needs to be checked
    fn fetch_combined(&self, combined_anchor: &CombinedAnchor) -> Result<&Combined, String>;

    /// get origin api, Some of the submitted data are replaced by the HASH value, so each time you use the API, you need to try to find it from the cache
    fn fetch_origin_api<'a, 'b: 'a>(&'a self, key: &'b str) -> Result<&'a str, String>;

    /// compile code
    fn compile_code(&self, item: &CodeItem) -> Result<&str, String>;
}

/// origin apis
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CombinedOriginApis {
    /// record hash -> origin // There is no contract name through the file, so you can only record haSh
    pub hash_origins: HashMap<String, String>,
    /// record key -> hash // The API that query through the contract, first point to Hash, and then query the specific content
    /// key:
    ///   api#ic#aaaa-aa Positioning canister
    ///   api#ethereum#0xabcd Positioning contract
    pub key_hashes: HashMap<String, String>,
}

impl CombinedOriginApis {
    /// Determine whether it is key
    pub fn is_key(key: &str) -> bool {
        if key.starts_with("api#") {
            return true;
        }
        hex::decode(key).is_ok_and(|v| v.len() == 32)
    }
    /// get origin
    pub fn get_origin(&self, key: &str) -> Option<&str> {
        if let Some(origin) = self.hash_origins.get(key) {
            return Some(origin);
        }
        let key = self.key_hashes.get(key)?;
        self.hash_origins.get(key).map(|s| s.as_str())
    }
}

/// Retrieve the function
#[derive(Debug, Serialize, Deserialize)]
pub struct ApisCheckFunction {
    /// The latest jar
    pub canister_id: String,

    /// Codes in advance
    pub codes: HashMap<CodeDataAnchor, CodeData>,
    /// APIS that query in advance
    pub apis: HashMap<ApiDataAnchor, ApiData>,
    /// Combined in advance
    pub combines: HashMap<CombinedAnchor, Combined>,

    /// cached api
    pub origin_apis: CombinedOriginApis,

    /// compiled code
    pub compiled: Vec<(CodeItem, String)>,
}

impl CheckFunction for ApisCheckFunction {
    fn canister_id(&self) -> Result<&str, String> {
        Ok(&self.canister_id)
    }

    fn fetch_code(&self, code_anchor: &CodeDataAnchor) -> Result<&CodeData, String> {
        self.codes
            .get(code_anchor)
            .ok_or_else(|| format!("can not fetch code by {}", code_anchor.as_ref()))
    }

    fn fetch_api(&self, api_anchor: &ApiDataAnchor) -> Result<&ApiData, String> {
        self.apis
            .get(api_anchor)
            .ok_or_else(|| format!("can not fetch api by {}", api_anchor.as_ref()))
    }

    fn fetch_combined(&self, combined_anchor: &CombinedAnchor) -> Result<&Combined, String> {
        self.combines
            .get(combined_anchor)
            .ok_or_else(|| format!("can not fetch combined by {}", combined_anchor.as_ref()))
    }

    fn fetch_origin_api<'a, 'b: 'a>(&'a self, key: &'b str) -> Result<&'a str, String> {
        let mut origin = self.origin_apis.get_origin(key);
        if origin.is_none() && !CombinedOriginApis::is_key(key) {
            origin = Some(key);
        }
        origin.ok_or_else(|| format!("can not find origin api: {key:?}"))
    }

    fn compile_code(&self, item: &CodeItem) -> Result<&str, String> {
        self.compiled
            .iter()
            .find(|(code, _)| code == item)
            .map(|(_, s)| s.as_str())
            .ok_or_else(|| format!("can not find parsed code: {item:?}"))
    }
}

/// Find all code content
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckedCodeItem {
    /// from
    pub from: ComponentId,

    /// index
    pub index: u32,

    /// mark
    pub mark: String,

    /// code item
    pub code: CodeItem,
}

impl CheckedCodeItem {
    /// new
    pub fn new(from: ComponentId, index: u32, mark: String, code: CodeItem) -> Self {
        Self {
            from,
            index,
            mark,
            code,
        }
    }
}

/// After passing the inspection, the generated combination
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckedCombined {
    /// stored code
    pub codes: HashMap<CodeDataAnchor, CodeData>,

    /// stored api
    pub apis: HashMap<ApiDataAnchor, ApiData>,

    /// new components
    pub components: Vec<LinkComponent>,

    /// After the censorship is approved, new components will be generated for hash
    pub combined_anchor: CombinedAnchor,

    /// chains
    #[serde(skip_serializing_if = "crate::is_empty_option_set")]
    pub chains: Option<HashSet<crate::types::CallChain>>,

    /// metadata
    #[serde(skip_serializing_if = "CombinedMetadata::is_metadata_empty")]
    pub metadata: Option<CombinedMetadata>,
}
