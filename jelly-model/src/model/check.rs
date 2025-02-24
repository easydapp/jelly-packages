use std::collections::{HashMap, HashSet};

use super::super::store::api::anchor::ApiDataAnchor;
use super::super::store::api::ApiData;
use super::super::store::code::anchor::CodeDataAnchor;
use super::super::store::code::CodeData;
use super::super::store::combined::anchor::{CombinedAnchor, CombinedParsedId};
use super::common::color::ComponentColor;
use super::common::error::{system_error, LinkError};
use super::common::identity::ComponentId;
use super::common::trigger::ComponentTriggered;
use super::common::types::LinkType;
use super::node::TrimmedNode;
use super::types::check::{CheckFunction, CheckedAnchors, CheckedCodeItem, CheckedCombined};
use super::CombinedMetadata;
use super::LinkComponent;

mod check_id;

mod check_circular;

mod check_empty;

mod check_names;

mod check_output;

mod check_inlets;

mod check_afflux;

mod check_single;

mod checked;

/// test
#[cfg(test)]
mod test;

/// find all anchors
pub fn find_all_anchors(components: &[LinkComponent]) -> Result<CheckedAnchors, LinkError> {
    // 1. Check whether the component ID is repeated
    let all_components = check_id::check_component_id(components)?;

    // 2. Is there a cycle reference
    check_circular::check_circular_reference(components, &all_components)?;

    // 3. Statistical code
    let mut code_anchors = Vec::new();
    for component in components {
        code_anchors.extend(component.get_code_anchors());
    }
    let code_anchors = code_anchors
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // 4. Statistical API
    let mut api_anchors = Vec::new();
    for component in components {
        api_anchors.extend(component.get_apis_anchors());
    }
    let api_anchors = api_anchors
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // 5. Statistics Combined
    let mut combined_anchors = Vec::new();
    for component in components {
        combined_anchors.extend(component.get_combined_anchors());
    }
    let combined_anchors = combined_anchors
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    Ok(CheckedAnchors {
        code_anchors: if code_anchors.is_empty() {
            None
        } else {
            Some(code_anchors)
        },
        api_anchors: if api_anchors.is_empty() {
            None
        } else {
            Some(api_anchors)
        },
        combined_anchors: if combined_anchors.is_empty() {
            None
        } else {
            Some(combined_anchors)
        },
    })
}

/// find all origin code
/// The codes in check function, cloud be empty
pub fn find_origin_codes<F: CheckFunction>(
    components: &[LinkComponent],
    fetch: &F,
) -> Result<Vec<CheckedCodeItem>, LinkError> {
    // 1. Check whether the component ID is repeated
    let all_components = check_id::check_component_id(components)?;

    // 2. Is there a cycle reference
    check_circular::check_circular_reference(components, &all_components)?;

    // 3. Find all the original code
    let mut codes = Vec::new();
    let colors = HashMap::new();
    for component in components {
        let all_endpoints = component.get_all_endpoints(&all_components, &colors, true);
        codes.extend(component.get_origin_codes(&all_endpoints, fetch)?)
    }

    Ok(codes)
}

/// find all template origin code
/// The codes in check function, cloud be empty
pub fn find_template_origin_codes(nodes: &[TrimmedNode]) -> Result<Vec<CheckedCodeItem>, LinkError> {
    let codes = nodes
        .iter()
        .filter_map(|n| {
            let id = n.data.data.component.as_ref()?.id;
            let template = n.data.data.template.as_ref()?;
            Some((id, template))
        })
        .flat_map(|(id, template)| template.get_origin_codes(id))
        .collect();
    Ok(codes)
}

/// final check
/// check function must has all data
pub fn check<F: CheckFunction>(components: &[LinkComponent], fetch: &F) -> Result<CheckedCombined, LinkError> {
    // 0. Check whether it is empty
    check_empty::check_empty(components)?;

    // 1. Check whether the component ID is repeated
    let all_components = check_id::check_component_id(components)?;

    // 2. Is there a cycle reference
    check_circular::check_circular_reference(components, &all_components)?;

    // 3. Whether the basic logic between the inspection components is self -consistent

    // 3.1 params cannot be repeated
    check_names::check_param_names(components)?;
    // 3.2 form cannot be repeated
    check_names::check_form_names(components)?;
    // 3.3 identity cannot be repeated
    check_names::check_identity_names(components)?;
    // 3.4 interaction cannot be repeated
    check_names::check_interaction_names(components)?;

    // 4. The output type can only have only one at most
    let output = check_output::check_output_number(components)?; // Record output type

    // 5. Check whether the Inlets of each component is wrong
    {
        let mut checked = HashSet::new();
        for component in components {
            check_inlets::check_single_component_inlets(component, &all_components, &mut checked)?;
        }
    }

    // 6. The cither error that may exist after the inspection component is collected
    let colors = check_afflux::check_afflux(&all_components)?;

    // 7. Whether each component consistent
    let mut checked = HashMap::new();
    let mut codes = HashMap::new(); // Records that need to be stored separately
    let mut apis = HashMap::new(); // Records need to be stored separately
    {
        let mut triggers = HashMap::new(); // Record component's own trigger information
        for component in components {
            check_single::check_single_component(
                component,
                &all_components,
                &colors,
                fetch,
                &mut triggers,
                &mut codes,
                &mut apis,
                &mut checked,
            )?;
        }
        for trigger in triggers.values() {
            trigger.check(&triggers, &all_components, &colors)?;
        }
    }

    // 6. Calculate the return result
    let canister_id = fetch.canister_id().map_err(system_error)?;
    let result = checked::parse_checked(canister_id, components, codes, apis, checked, output)?;

    Ok(result)
}

/// check template
#[cfg(feature = "validate")]
pub fn check_templates<F: CheckFunction>(
    nodes: &[TrimmedNode],
    checked: &CheckedCombined,
    fetch: &F,
) -> Result<u32, LinkError> {
    let templates = nodes
        .iter()
        .filter_map(|n| {
            let id = n.data.data.component.as_ref()?.id;
            let template = n.data.data.template.as_ref()?;
            Some((id, template))
        })
        .collect::<Vec<_>>();
    // The constraints of the template must also be checked
    for (id, template) in templates.iter() {
        let component = checked
            .components
            .iter()
            .find(|c| c.id() == *id)
            .ok_or_else(|| LinkError::InvalidComponentId { id: *id })?;
        let value = match component {
            LinkComponent::Const(constant) => &constant.metadata.value,
            _ => {
                return Err(LinkError::WrongConstValue(
                    (*id, "component is not const".to_string()).into(),
                ))
            }
        };
        if !template.output.is_match(value) {
            return Err(LinkError::WrongConstValue(
                (
                    id.to_owned(),
                    format!("template output not match: {:?}", template.output),
                )
                    .into(),
            ));
        }
        if let Some(validate) = &template.validate {
            validate.validate(&template.output, value, component.id(), fetch)?;
        }
    }
    Ok(templates.len() as u32)
}
