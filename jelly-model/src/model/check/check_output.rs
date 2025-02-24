use super::{LinkComponent, LinkError, LinkType};

/// check output type
#[inline]
pub(super) fn check_output_number(components: &[LinkComponent]) -> Result<Option<LinkType>, LinkError> {
    let outputs = components
        .iter()
        .filter(|c| matches!(c, LinkComponent::Output(_)))
        .collect::<Vec<_>>();
    if 1 < outputs.len() {
        return Err(LinkError::MultipleOutput);
    }
    Ok(outputs.into_iter().next().and_then(|c| {
        if let LinkComponent::Output(output) = c {
            Some(output.output.clone())
        } else {
            None
        }
    }))
}
