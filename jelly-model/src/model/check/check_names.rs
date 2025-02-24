use std::collections::HashSet;

use super::{LinkComponent, LinkError};

/// check param name
#[inline]
pub(super) fn check_param_names(components: &[LinkComponent]) -> Result<(), LinkError> {
    let mut names = HashSet::new();
    for component in components {
        if let Some(name) = component.get_param_name() {
            if names.contains(&name) {
                return Err(LinkError::DuplicateParamName { name: name.to_owned() });
            }
            names.insert(name);
        }
    }
    Ok(())
}

/// check form name
#[inline]
pub(super) fn check_form_names(components: &[LinkComponent]) -> Result<(), LinkError> {
    let mut names = HashSet::new();
    for component in components {
        if let Some(name) = component.get_form_name() {
            if names.contains(&name) {
                return Err(LinkError::DuplicateFormName { name: name.to_owned() });
            }
            names.insert(name);
        }
    }
    Ok(())
}

/// check identity name
#[inline]
pub(super) fn check_identity_names(components: &[LinkComponent]) -> Result<(), LinkError> {
    let mut names = HashSet::new();
    for component in components {
        if let Some(name) = component.get_identity_name() {
            if names.contains(&name) {
                return Err(LinkError::DuplicateIdentityName { name: name.to_owned() });
            }
            names.insert(name);
        }
    }
    Ok(())
}

/// check interaction name
#[inline]
pub(super) fn check_interaction_names(components: &[LinkComponent]) -> Result<(), LinkError> {
    let mut names = HashSet::new();
    for component in components {
        if let Some(name) = component.get_interaction_name() {
            if names.contains(&name) {
                return Err(LinkError::DuplicateInteractionName { name: name.to_owned() });
            }
            names.insert(name);
        }
    }
    Ok(())
}
