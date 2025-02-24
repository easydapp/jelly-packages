use std::{borrow::Cow, collections::HashMap};

use candid::Principal;
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        common::{
            error::{system_error, LinkError},
            identity::ComponentId,
        },
        types::{
            abi::types::{AbiItem, AbiType},
            check::CheckFunction,
        },
    },
    store::api::{
        anchor::{ApiDataAnchor, ApiDataParsedId},
        content::{
            evm::{EvmApi, OriginEvmApi, SingleEvmApi},
            ApiDataContent,
        },
        ApiData,
    },
};

// ============================ evm ============================

/// api
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmCallApi {
    /// Code itself
    #[serde(rename = "api")]
    Api(EvmApi),

    /// Introducing the API API data in the jar, you need to query to get it
    #[serde(rename = "anchor")]
    Anchor(ApiDataAnchor),
}

impl EvmCallApi {
    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        if let EvmCallApi::Anchor(anchor) = &self {
            anchors.push(anchor.clone());
        }

        anchors
    }

    /// Query parameters and output
    pub fn get_data_and_output<F: CheckFunction>(&self, from: ComponentId, fetch: &F) -> Result<AbiItem, LinkError> {
        let api = match self {
            Self::Api(api) => Cow::Borrowed(api),
            Self::Anchor(anchor) => {
                let data = fetch
                    .fetch_api(anchor)
                    .map_err(|error| system_error(format!("fetch api failed: {error}")))?;
                if let ApiDataContent::Evm(api) = &data.content {
                    Cow::Borrowed(api)
                } else {
                    return Err(system_error("fetch evm api failed".into()));
                }
            }
        };
        api.get_data_and_output(from, fetch)
    }

    /// check api
    pub fn try_into_anchor<F: CheckFunction>(
        self,
        fetch: &F,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<Self, LinkError> {
        if let EvmCallApi::Api(api) = &self {
            // ! Take back the original data first
            let api = api
                .clone()
                .restore(fetch)
                .map_err(|error| system_error(format!("fetch origin api failed: {error}")))?;

            if api.should_into_anchor() {
                // get id
                let canister_id = fetch.canister_id().map_err(system_error)?;
                let hash = api.hash().map_err(system_error)?;
                let parsed_id = ApiDataParsedId::from(
                    Principal::from_text(canister_id).map_err(|e| system_error(format!("{e}")))?,
                    hash.into(),
                );
                let anchor: ApiDataAnchor = (&parsed_id).into();

                // Record
                {
                    let api_data = ApiData {
                        anchor: anchor.clone(),
                        created: 0.into(),
                        content: ApiDataContent::Evm(api.clone()),
                    };
                    apis.insert(anchor.clone(), api_data);
                }

                return Ok(Self::Anchor(anchor));
            }
        }
        Ok(self)
    }
}

impl EvmApi {
    /// Query parameters and output
    // ====== Parameter ======
    // 1. 0 parameter -> arg Must be empty -> []
    // 2. 1 parameter ->
    //  The output value of ARG CODE must be the unique value {..} -> any*
    //  arg refer The split parameters complete the construction of the parameters in a reference manner, respectively // ? todo To be developed
    // 3. Multi-parameter ->
    //  The output value of ARG CODE must be tuple {..} -> [any*, any*, ..]
    //  ARG Refer split parameters to complete the construction of the parameter in a reference manner // ? todo To be developed
    // ====== Output type ======
    // 1. 0 parameter ->
    //  If RET is empty, the output type must be*array type*-> []
    //  RET CODE parameter is [] -> output
    // 2. 1 parameter ->
    //  If RET is empty, the only type is the output type，// ! Is it consistent with Link Type?
    //  RET CODE parameter is this type any* -> output
    //  RET REFER split results combine output any* -> output
    // 3. Multi-parameter ->
    //  RET CODE parameter is this type [any*, any*, ..] -> output
    //  RET REFER split results combine output [any*, any*, ..] -> output
    pub fn get_data_and_output<F: CheckFunction>(&self, from: ComponentId, fetch: &F) -> Result<AbiItem, LinkError> {
        let func = match self {
            Self::Single(SingleEvmApi { api }) => {
                let func: AbiItem = serde_json::from_str(api)
                    .map_err(|_| LinkError::InvalidCallEvmActionApi((from, "parse abi api failed".into()).into()))?;

                func
            }
            Self::Origin(OriginEvmApi { abi, name, index }) => {
                let abi = fetch
                    .fetch_origin_api(abi)
                    .map_err(|error| system_error(format!("fetch origin api failed: {error}")))?;
                let functions: Vec<AbiItem> = serde_json::from_str(abi)
                    .map(|functions: Vec<AbiItem>| {
                        functions
                            .into_iter()
                            .filter(|func| {
                                func.name.is_some()
                                    && matches!(func.ty, AbiType::Function)
                                    && func.state_mutability.is_some()
                            })
                            .collect()
                    })
                    .map_err(|e| LinkError::InvalidCallEvmActionApi((from, format!("parse abi failed: {e}")).into()))?;

                let mut func = index.as_ref().and_then(|index| functions.get(*index as usize).cloned());
                if func.is_none() {
                    func = functions
                        .into_iter()
                        .find(|f| f.name.as_ref().is_some_and(|n| n == name));
                }
                func.ok_or_else(|| LinkError::InvalidCallEvmActionApi((from, "name or index not found".into()).into()))?
            }
        };

        if !matches!(func.ty, AbiType::Function) {
            return Err(LinkError::InvalidCallEvmActionApi(
                (from, "abi is not a function".into()).into(),
            ));
        }

        func.state_mutability.as_ref().ok_or_else(|| {
            LinkError::InvalidCallEvmActionApi((from, "function abi must has stateMutability".into()).into())
        })?;

        Ok(func)
    }
}
