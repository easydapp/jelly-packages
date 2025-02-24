use std::collections::HashMap;

use super::{ComponentId, LinkComponent, LinkError};

/// Check whether the component ID is effective and repeated
#[inline]
pub(super) fn check_component_id(
    components: &[LinkComponent],
) -> Result<HashMap<ComponentId, &LinkComponent>, LinkError> {
    let mut all_components: HashMap<ComponentId, &LinkComponent> = HashMap::with_capacity(components.len());

    for component in components {
        let id = component.id();
        if id.is_zero() {
            return Err(LinkError::InvalidComponentId { id });
        }
        if all_components.contains_key(&id) {
            return Err(LinkError::DuplicateComponentId { id });
        }
        all_components.insert(id, component);
    }

    Ok(all_components)
}
