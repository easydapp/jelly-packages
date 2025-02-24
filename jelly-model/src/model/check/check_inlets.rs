use std::collections::{HashMap, HashSet};

use super::{ComponentId, LinkComponent, LinkError};

/// Check whether the reference of a single component is wrong
#[inline]
pub(super) fn check_single_component_inlets(
    component: &LinkComponent,
    components: &HashMap<ComponentId, &LinkComponent>,
    checked: &mut HashSet<ComponentId>,
) -> Result<(), LinkError> {
    let id = component.id();
    if checked.contains(&id) {
        return Ok(()); // Has been checked
    }

    // 1. Is the traversal inspection dependencies correct
    if let Some(inlets) = component.get_inlets() {
        for inlet in inlets {
            // Travel every dependencies
            let c = components.get(&inlet.id).ok_or(LinkError::UnknownComponentOrNotRefer {
                from: Some(id),
                id: inlet.id,
            })?;
            check_single_component_inlets(c, components, checked)?;
        }

        // There is no need to check whether it is collected here, and check the dyeing later
        // let mut endpoints = HashSet::with_capacity(inlets.len());
        // for inlet in inlets {
        //     let inlet = inlet.id;
        //     if endpoints.contains(&inlet) {
        //         return Err(LinkError::AffluxComponentId {
        //             from: id,
        //             afflux: inlet,
        //         });
        //     }
        //     endpoints.insert(inlet);
        // }
    }

    checked.insert(id);

    Ok(())
}
