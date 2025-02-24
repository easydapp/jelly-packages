use super::{LinkComponent, LinkError};

/// Check whether the component ID is effective and repeated
#[inline]
pub(super) fn check_empty(components: &[LinkComponent]) -> Result<(), LinkError> {
    if components.is_empty() {
        return Err(LinkError::EmptyComponents {
            message: "no components".into(),
        });
    }
    Ok(())
}
