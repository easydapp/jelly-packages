use std::{borrow::Cow, collections::HashMap};

use ::candid::Principal;
use ic_canister_kit::types::WrappedCandidTypeFunction;
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        common::{
            error::{system_error, LinkError},
            identity::ComponentId,
        },
        types::check::CheckFunction,
    },
    store::api::{
        anchor::{ApiDataAnchor, ApiDataParsedId},
        content::{
            ic::{InternetComputerApi, OriginInternetComputerApi, SingleInternetComputerApi},
            ApiDataContent,
        },
        ApiData,
    },
};

pub(crate) mod candid;

// ============================ ic ============================

/// api
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum IcCallApi {
    /// Code itself
    #[serde(rename = "api")]
    Api(InternetComputerApi),

    /// Introducing the API API data in the jar, you need to query to get it
    #[serde(rename = "anchor")]
    Anchor(ApiDataAnchor),
}

impl IcCallApi {
    /// get apis anchors
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        let mut anchors = Vec::new();

        if let IcCallApi::Anchor(anchor) = &self {
            anchors.push(anchor.clone());
        }

        anchors
    }

    /// Query parameters and output
    pub fn get_data_and_output<F: CheckFunction>(
        &self,
        from: ComponentId,
        fetch: &F,
    ) -> Result<WrappedCandidTypeFunction, LinkError> {
        let api = match self {
            Self::Api(api) => Cow::Borrowed(api),
            Self::Anchor(anchor) => {
                let data = fetch
                    .fetch_api(anchor)
                    .map_err(|error| system_error(format!("fetch api failed: {error}")))?;
                if let ApiDataContent::InternetComputer(api) = &data.content {
                    Cow::Borrowed(api)
                } else {
                    return Err(system_error("fetch ic api failed".into()));
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
        if let IcCallApi::Api(api) = &self {
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
                        content: ApiDataContent::InternetComputer(api.clone()),
                    };
                    apis.insert(anchor.clone(), api_data);
                }

                return Ok(Self::Anchor(anchor));
            }
        }
        Ok(self)
    }
}

impl InternetComputerApi {
    /// Query parameters and output
    // ====== Parameter ======
    // 1. 0 parameter -> arg Must be empty -> []
    // 2. 1 parameter ->
    //  If it is OPT type, ARG can be empty
    //  The output value of ARG CODE must be the unique value {..} -> any*
    //  ARG Refer split parameters to complete the construction of the parameter in a reference manner // ? todo To be developed
    // 3. Multi-parameter ->
    //  If it is all OPT type, ARG can be empty
    //  The output value of ARG CODE must be tuple {..} -> [any*, any*, ..]
    //  ARG Refer split parameters to complete the construction of the parameter in a reference manner // ? todo To be developed
    // ====== Output type ======
    // 1. 0 parameter ->
    //  If RET is empty, the output type must be*array type* -> []
    //  RET CODE parameter is [] -> output
    // 2. 1 parameter ->
    //  If RET is empty, the only type is the output type，// ! Is it consistent with Link Type?
    //  RET CODE parameter is this type any* -> output
    //  RET REFER split results combine output any* -> output
    // 3. Multi-parameter ->
    //  RET CODE parameter is this type [any*, any*, ..] -> output
    //  RET REFER split results combine output [any*, any*, ..] -> output
    pub fn get_data_and_output<F: CheckFunction>(
        &self,
        from: ComponentId,
        fetch: &F,
    ) -> Result<WrappedCandidTypeFunction, LinkError> {
        match self {
            Self::Single(SingleInternetComputerApi { api }) => {
                candid::parse_candid_for_data_and_output(&format!("service: {{ {api} }}"), None, from)
            }
            Self::Origin(OriginInternetComputerApi { candid, method }) => {
                let candid = fetch
                    .fetch_origin_api(candid)
                    .map_err(|error| system_error(format!("fetch origin api failed: {error}")))?;
                candid::parse_candid_for_data_and_output(candid, Some(method), from)
            }
        }
    }
}
