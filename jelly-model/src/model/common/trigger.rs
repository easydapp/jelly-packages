use std::{cell::RefCell, collections::HashMap};

use crate::model::LinkComponent;

use super::{color::ComponentColor, error::LinkError, identity::ComponentId};

struct TriggeredComponentIdentity {
    /// Whether to trigger
    click: bool,
}

struct TriggeredComponentCall {
    /// Referenced identity
    identity: Option<ComponentId>,
    /// Is your own trigger?
    click: bool,
    /// Whether it is update
    update: bool,
}

/// Component information with trigger conditions
enum TriggeredComponent {
    /// identity
    Identity(TriggeredComponentIdentity),
    /// Call the trigger of the component
    Call(TriggeredComponentCall),
    /// Interactive component comes with Trigger
    Interaction,
}

/// Trigger inspection
pub struct ComponentTriggered {
    /// id
    pub id: ComponentId,
    /// Identity
    triggered: TriggeredComponent,
    /// See whether it is called
    clickable: RefCell<Option<bool>>,
}

impl ComponentTriggered {
    /// Build
    pub fn from_identity(id: ComponentId, click: bool) -> Self {
        Self {
            id,
            triggered: TriggeredComponent::Identity(TriggeredComponentIdentity { click }),
            clickable: RefCell::new(Some(click)),
        }
    }
    /// Build
    pub fn from_call(id: ComponentId, identity: Option<ComponentId>, click: bool, update: bool) -> Self {
        Self {
            id,
            triggered: TriggeredComponent::Call(TriggeredComponentCall {
                identity,
                click,
                update,
            }),
            clickable: RefCell::new(None),
        }
    }
    /// Build
    pub fn from_interaction(id: ComponentId) -> Self {
        Self {
            id,
            triggered: TriggeredComponent::Interaction,
            clickable: RefCell::new(Some(true)),
        }
    }

    /// Check whether the necessary trigger method
    pub fn check(
        &self,
        triggers: &HashMap<ComponentId, ComponentTriggered>,
        all_components: &HashMap<ComponentId, &LinkComponent>,
        colors: &HashMap<ComponentId, ComponentColor<'_>>,
    ) -> Result<(), LinkError> {
        if self.clickable.borrow().is_some() {
            return Ok(()); // I have checked
        }

        let (identity, update) = match &self.triggered {
            TriggeredComponent::Identity(TriggeredComponentIdentity { click }) => {
                *self.clickable.borrow_mut() = Some(*click);
                return Ok(()); // No need to check the identity
            }
            TriggeredComponent::Call(TriggeredComponentCall {
                identity,
                click,
                update,
            }) => {
                if *click {
                    *self.clickable.borrow_mut() = Some(true);
                    return Ok(()); // It is a click to trigger, it is already triggered
                }
                (identity, update)
            }
            TriggeredComponent::Interaction {} => {
                *self.clickable.borrow_mut() = Some(true);
                return Ok(()); // There is no need to check the interactive component
            }
        };

        // Check the specific details of Call below

        // Check whether the identity of the link is called
        if let Some(identity) = identity {
            if let Some(trigger) = triggers.get(identity) {
                trigger.check(triggers, all_components, colors)?;
                if *trigger.clickable.borrow() == Some(true) {
                    *self.clickable.borrow_mut() = Some(true);
                    return Ok(()); // The identity of the link is triggered
                }
            }
        }

        // Check the historical path if there is any one on the historical path
        let component = all_components
            .get(&self.id)
            .ok_or(LinkError::InvalidComponentId { id: self.id })?;
        if let Some(all_endpoints) = component.get_all_endpoints(all_components, colors, true) {
            for inlet in all_endpoints.find_all_inlet_interrupt_by_form() {
                if let Some(trigger) = triggers.get(&inlet) {
                    trigger.check(triggers, all_components, colors)?;
                    if *trigger.clickable.borrow() == Some(true) {
                        *self.clickable.borrow_mut() = Some(true);
                        return Ok(());
                    }
                }
            }
        }

        let is_anonymous = match identity {
            Some(identity) => {
                let identity = all_components
                    .get(identity)
                    .ok_or(LinkError::InvalidComponentId { id: *identity })?;
                if let LinkComponent::Identity(identity) = identity {
                    identity.is_anonymous()
                } else {
                    false
                }
            }
            None => true,
        };
        if is_anonymous {
            *self.clickable.borrow_mut() = Some(false);
            return Ok(()); // The identity of the link is anonymous, just call it directly, without hidden safety hazards
        }

        // It is modified and not click to check
        if !update {
            // Set the source settings on the crick
            *self.clickable.borrow_mut() = Some(false);
            return Ok(()); // It's not modified, then it doesn't matter if Trigger is
        }

        Err(LinkError::InvalidCallTrigger(
            (self.id, "must be click for updating call".into()).into(),
        ))
    }
}
