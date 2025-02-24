use std::collections::{HashMap, HashSet};

use super::{ComponentId, LinkComponent, LinkError};

/// Check whether the cycle reference and the introduction point are valid
#[inline]
pub(super) fn check_circular_reference(
    components: &[LinkComponent],
    all_components: &HashMap<ComponentId, &LinkComponent>,
) -> Result<(), LinkError> {
    let mut visited = Vec::new(); // Record components in the inspection
    let mut checked = HashSet::new(); // Whether the record has been checked

    fn check_inlets(
        component: &LinkComponent,
        components: &HashMap<ComponentId, &LinkComponent>,
        visited: &mut Vec<ComponentId>,
        checked: &mut HashSet<ComponentId>,
    ) -> Result<(), LinkError> {
        let id = component.id(); // get id

        if visited.contains(&id) {
            return Err(LinkError::CircularReference { id }); // ! The component has appeared
        }

        if checked.contains(&id) {
            return Ok(()); // Has been checked
        }

        visited.push(id);
        // println!("visited: {:?}", visited);
        if let Some(inlets) = component.get_inlets() {
            for inlet in inlets {
                // Travel every dependencies
                let c = components.get(&inlet.id).ok_or(LinkError::UnknownComponentOrNotRefer {
                    from: None,
                    id: inlet.id,
                })?;
                let max_outputs = c.count_outputs(); // How many introduction points
                if max_outputs <= inlet.index.unwrap_or_default() {
                    return Err(LinkError::InvalidEndpoint {
                        from: id,
                        inlet: *inlet,
                    });
                }
                check_inlets(c, components, visited, checked)?; // Recursively check the introduction point of this component
            }
        }
        visited.pop();

        checked.insert(id);
        Ok(())
    }

    for component in components {
        check_inlets(component, all_components, &mut visited, &mut checked)?;
    }

    Ok(())
}
