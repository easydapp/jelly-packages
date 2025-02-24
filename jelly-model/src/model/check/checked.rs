use std::collections::{HashMap, HashSet};

use candid::Principal;

use crate::{common::hash::hash_sha256, model::common::error::system_error};

use super::{
    ApiData, ApiDataAnchor, CheckedCombined, CodeData, CodeDataAnchor, CombinedAnchor, CombinedMetadata,
    CombinedParsedId, ComponentId, LinkComponent, LinkError, LinkType,
};

/// Combination result
#[inline]
pub(super) fn parse_checked(
    canister_id: &str,
    components: &[LinkComponent],
    codes: HashMap<CodeDataAnchor, CodeData>,
    apis: HashMap<ApiDataAnchor, ApiData>,
    mut checked: HashMap<ComponentId, LinkComponent>,
    output: Option<LinkType>,
) -> Result<CheckedCombined, LinkError> {
    // 0. Take out the component to be submitted
    let mut parsed = Vec::with_capacity(components.len());
    for component in components {
        let id = component.id();
        parsed.push(
            checked
                .remove(&id)
                .ok_or_else(|| system_error(format!("can not find checked component: {:?}", id)))?,
        );
    }

    // 1. Calculate ID
    let json = serde_json::to_string(&parsed).map_err(|e| system_error(format!("serde error: {}", e)))?;
    let hash = hash_sha256(&json); // Calculate Hash
    let parsed_id = CombinedParsedId::from(
        Principal::from_text(canister_id).map_err(|e| system_error(format!("{e}")))?,
        hash.into(),
    );
    let anchor: CombinedAnchor = (&parsed_id).into();

    // 2. Take out the data required by various components
    let params = parsed
        .iter()
        .filter_map(|component| component.get_param_required())
        .collect::<Vec<_>>();

    let forms = parsed
        .iter()
        .filter_map(|component| component.get_form_required())
        .collect::<Vec<_>>();

    let identities = parsed
        .iter()
        .filter_map(|component| component.get_identity_required())
        .collect::<Vec<_>>();

    let interactions = parsed
        .iter()
        .filter_map(|component| component.get_interaction_required())
        .collect::<Vec<_>>();

    // 3. Remove the other kept KEY
    let code_anchors = parsed
        .iter()
        .flat_map(|component| component.get_code_anchors())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let apis_anchors = parsed
        .iter()
        .flat_map(|component| component.get_apis_anchors())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let combined_anchors = parsed
        .iter()
        .flat_map(|component| component.get_combined_anchors())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let chains = parsed.iter().filter_map(|c| c.get_call_chain()).collect::<HashSet<_>>();

    let metadata = (!params.is_empty()
        || !forms.is_empty()
        || !identities.is_empty()
        || !interactions.is_empty()
        || !code_anchors.is_empty()
        || !apis_anchors.is_empty()
        || !combined_anchors.is_empty()
        || output.is_some())
    .then(|| CombinedMetadata {
        params: (!params.is_empty()).then_some(params),
        identities: (!identities.is_empty()).then_some(identities),

        forms: (!forms.is_empty()).then_some(forms),
        interactions: (!interactions.is_empty()).then_some(interactions),

        code_anchors: (!code_anchors.is_empty()).then_some(code_anchors),
        apis_anchors: (!apis_anchors.is_empty()).then_some(apis_anchors),
        combined_anchors: (!combined_anchors.is_empty()).then_some(combined_anchors),

        output,
    });

    let result = CheckedCombined {
        codes,
        apis,
        components: parsed,
        combined_anchor: anchor,
        chains: (!chains.is_empty()).then_some(chains),
        metadata,
    };

    Ok(result)
}
