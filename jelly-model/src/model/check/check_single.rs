use std::collections::HashMap;

use super::{
    ApiData, ApiDataAnchor, CheckFunction, CodeData, CodeDataAnchor, ComponentColor, ComponentId, ComponentTriggered,
    LinkComponent, LinkError,
};

/// Check a single component
#[allow(clippy::too_many_arguments)]
#[inline]
pub(super) fn check_single_component<F: CheckFunction>(
    component: &LinkComponent,                         // Current component
    components: &HashMap<ComponentId, &LinkComponent>, // Maybe other component information
    colors: &HashMap<ComponentId, ComponentColor<'_>>, // Dyeing information of components
    fetch: &F,
    triggers: &mut HashMap<ComponentId, ComponentTriggered>, // Record component trigger information
    codes: &mut HashMap<CodeDataAnchor, CodeData>,           // Records that need to be stored separately
    apis: &mut HashMap<ApiDataAnchor, ApiData>,              // Records need to be stored separately
    checked: &mut HashMap<ComponentId, LinkComponent>,       // Recorded components that have been checked
) -> Result<(), LinkError> {
    let id = component.id();
    if checked.contains_key(&id) {
        return Ok(()); // Has been checked
    }

    // 1. Calculate the historical dependence path
    let all_endpoints = component.get_all_endpoints(components, colors, true);

    // 2. Check yourself
    let component = component.check(&all_endpoints, fetch, triggers, codes, apis)?;

    checked.insert(id, component);

    Ok(())
}
