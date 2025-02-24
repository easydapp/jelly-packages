use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use crate::model::common::error::system_error;

use super::{ComponentColor, ComponentId, LinkComponent, LinkError};

/// After checking the node, the reference error is referenced
/// ! If the output of more than two points of the conditional component is introduced, it cannot be satisfied at the same time. Therefore
/// ! The key point is that there is no variable that can be set, and the effect of if expression can only be a thorough branch. Once there is a assembly, the variables generated on each branch should never be quoted
#[inline]
pub(super) fn check_afflux<'a>(
    components: &HashMap<ComponentId, &'a LinkComponent>,
) -> Result<HashMap<ComponentId, ComponentColor<'a>>, LinkError> {
    // See the conflict of inspection, disconnect the empty conditions of the air conditioning method to prevent the introduction of empty data
    let mut colors: HashMap<ComponentId, ComponentColor<'a>> = HashMap::with_capacity(components.len());

    let mut changed;
    loop {
        changed = false; // Set it first to false

        'outer: for (from, component) in components {
            if colors.contains_key(from) {
                continue; // Has been processed
            }

            let inlets = component.get_inlets().map(Cow::Borrowed).unwrap_or_default();

            let mut endpoints = Vec::with_capacity(inlets.len());
            for inlet in inlets.iter() {
                let info = match colors.get(&inlet.id) {
                    Some(info) => info,
                    None => continue 'outer,
                };
                let index = inlet.index.unwrap_or_default();
                endpoints.push((info, index));
            }

            let nullable = component.get_nullable_endpoints().is_some();

            fn push_color(
                color: &mut HashMap<ComponentId, HashSet<u32>>,
                id: ComponentId,
                index: HashSet<u32>,
                _from: ComponentId,
            ) -> Result<(), LinkError> {
                match color.get_mut(&id) {
                    Some(exists) => {
                        if *exists != index {
                            // return Err(LinkError::AffluxComponentId { from, afflux: id });
                            exists.extend(index);
                        }
                    }
                    None => {
                        color.insert(id, index);
                    }
                }
                Ok(())
            }

            fn push_color_with_nullable(
                color: &mut HashMap<ComponentId, HashSet<u32>>,
                id: ComponentId,
                index: HashSet<u32>,
            ) -> Option<ComponentId> {
                match color.get_mut(&id) {
                    Some(exits) => {
                        if *exits != index {
                            exits.extend(index);
                            return Some(id);
                        }
                        None
                    }
                    None => {
                        color.insert(id, index);
                        None
                    }
                }
            }

            let info = if nullable {
                // Can empty nodes, record conflict nodes
                let mut conflict = HashSet::new();
                let mut color = HashMap::new();
                for (info, index) in endpoints {
                    for (id, index) in info.color.iter() {
                        if let Some(con) = push_color_with_nullable(&mut color, *id, index.clone()) {
                            conflict.insert(con);
                        }
                    }
                    let mut set = HashSet::with_capacity(1);
                    set.insert(index);
                    if let Some(con) = push_color_with_nullable(&mut color, info.id, set) {
                        conflict.insert(con);
                    }
                }
                ComponentColor {
                    id: *from,
                    component,
                    color,
                    conflict,
                }
            } else {
                // Ordinary nodes, encountering conflicts is a mistake
                let mut color = HashMap::new();
                for (info, index) in endpoints {
                    for (id, index) in info.color.iter() {
                        push_color(&mut color, *id, index.clone(), *from)?;
                    }
                    let mut set = HashSet::with_capacity(1);
                    set.insert(index);
                    push_color(&mut color, info.id, set, *from)?;
                }
                ComponentColor {
                    id: *from,
                    component,
                    color,
                    conflict: HashSet::new(),
                }
            };
            colors.insert(*from, info);
            changed = true;
        }

        if !changed {
            break;
        }
    }

    for id in components.keys() {
        if !colors.contains_key(id) {
            return Err(system_error("every component should has info".into()));
        }
    }

    Ok(colors)
}
