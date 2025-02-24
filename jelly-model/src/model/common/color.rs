use std::collections::{HashMap, HashSet};

use super::super::LinkComponent;
use super::identity::ComponentId;

/// Chromosome
#[derive(Debug)]
pub struct ComponentColor<'a> {
    /// id
    pub id: ComponentId,
    /// Component
    pub component: &'a LinkComponent,
    /// Data quoted in this component
    /// The color required for this component, unless the conditional components can be empty, the component id does not allow conflicts
    pub color: HashMap<ComponentId, HashSet<u32>>,
    /// Record conflict component
    pub conflict: HashSet<ComponentId>,
}
