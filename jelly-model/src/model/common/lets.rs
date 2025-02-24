use std::borrow::Cow;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::common::check::is_valid_variant_name;

use super::super::LinkComponent;
use super::error::LinkError;
use super::identity::ComponentId;
use super::refer::{CodeValue, InputValue, KeyRefer, NamedValue, ReferValue};
use super::types::{LinkType, ObjectSubitem};

/// Link point
/// Each component is input for data input, and only one import point will be
/// Call component will have an additional IDentity import point
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Endpoint {
    /// Component id
    pub id: ComponentId,
    /// The link point is default 0
    #[serde(skip_serializing_if = "is_endpoint_index_skip")]
    pub index: Option<u32>,
}

fn is_endpoint_index_skip(index: &Option<u32>) -> bool {
    index.as_ref().is_none_or(|index| *index == 0)
}

/// Front path dependencies
#[derive(Debug, Clone)]
pub struct AllEndpoint<'a> {
    /// component id
    pub id: ComponentId,
    /// Link point
    pub index: u32,
    /// Corresponding component
    pub component: &'a LinkComponent,
    /// The introduction point of the corresponding node
    pub inlets: Option<AllEndpoints<'a>>,
}

/// All front path dependencies
#[derive(Debug, Clone, Default)]
pub struct AllEndpoints<'a> {
    /// All introduction point
    pub endpoints: Vec<AllEndpoint<'a>>,
}

impl AllEndpoint<'_> {
    /// Find whether to include a specified node
    pub fn find(&self, id: ComponentId) -> bool {
        if self.id == id {
            return true;
        }
        if let Some(inlets) = &self.inlets {
            if inlets.find(id) {
                return true;
            }
        }
        false
    }

    fn find_all_inlet_interrupt_by_form(&self) -> HashSet<ComponentId> {
        let mut set = HashSet::with_capacity(1);
        match &self.component {
            LinkComponent::Form(_) => {} // Form is interrupted, because even if there is, even if there is, it will be interrupted by Form, which is useless for this level.
            LinkComponent::Interaction(_) => {
                set.insert(self.id); // Itself is click
            }
            _ => {
                set.insert(self.id);

                // Other components need to trace up again
                if let Some(inlets) = &self.inlets {
                    set.extend(inlets.find_all_inlet_interrupt_by_form());
                }
            }
        }

        set
    }
}

impl<'a> AllEndpoints<'a> {
    fn find(&self, id: ComponentId) -> bool {
        for endpoint in self.endpoints.iter() {
            if endpoint.find(id) {
                return true;
            }
        }
        false
    }

    /// Statistics all introduction points
    pub fn find_all_inlet_interrupt_by_form(&self) -> HashSet<ComponentId> {
        self.endpoints
            .iter()
            .flat_map(|endpoint| endpoint.find_all_inlet_interrupt_by_form())
            .collect()
    }

    /// Find the introduction point
    pub fn find_endpoint(&self, endpoint: &Endpoint) -> Option<&AllEndpoint> {
        for e in &self.endpoints {
            if e.id == endpoint.id && e.index == endpoint.index.unwrap_or_default() {
                return Some(e);
            }
            if let Some(inlets) = &e.inlets {
                if let Some(found) = inlets.find_endpoint(endpoint) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find the type introduced corresponding type
    pub fn find_output_type(
        &'a self,
        endpoint: &Endpoint,
        refer: &Option<KeyRefer>,
        from: ComponentId,
    ) -> Result<Cow<'a, LinkType>, LinkError> {
        // Find the corresponding reference node
        let all_endpoint = self
            .find_endpoint(endpoint)
            .ok_or(LinkError::UnknownComponentOrNotRefer {
                from: Some(from),
                id: endpoint.id,
            })?;

        // Find the corresponding output type
        let output = all_endpoint
            .component
            .get_output_type(endpoint.index.unwrap_or_default(), &from)?;

        // Check recursive key
        if let Some(refer) = refer {
            // Make sure that Key can find the corresponding type
            let ty = refer.get_output(&output, &from, endpoint, refer)?;
            return Ok(Cow::Owned(ty.to_owned()));
        }

        Ok(output)
    }

    /// Check a single introduction
    pub fn check_refer_value(
        &'a self,
        refer_value: &ReferValue,
        from: ComponentId,
    ) -> Result<Cow<'a, LinkType>, LinkError> {
        // Check whether the reference is wrong
        let ty = self.find_output_type(&refer_value.endpoint, &refer_value.refer, from)?;
        Ok(ty)
    }

    /// Check a single introduction
    pub fn check_input_value(&'a self, input: &InputValue, from: ComponentId) -> Result<Cow<'a, LinkType>, LinkError> {
        match input {
            InputValue::Const(constant) => {
                constant.check(from)?; // Check whether there are mistakes
                Ok(Cow::Owned(constant.link_type()))
            }
            InputValue::Refer(refer) => {
                let ty = self.find_output_type(&refer.endpoint, &refer.refer, from)?; // Check whether the reference is wrong
                Ok(ty)
            }
        }
    }

    /// Check whether the reference is effective and not cared for the type of reference
    pub fn check_code_values<'b, I>(&self, data: I, from: ComponentId) -> Result<LinkType, LinkError>
    where
        I: Iterator<Item = &'b CodeValue>,
    {
        let mut visited = HashSet::new(); // Name is not allowed to repeat

        let mut subitems = Vec::new(); // RRecord each sub-item each sub-item

        for code_value in data {
            // Is it already existed?
            if visited.contains(&code_value.key) {
                return Err(LinkError::DuplicateVariantKey {
                    from,
                    key: code_value.key.clone(),
                });
            }

            // Check whether the variable name meets the rules
            if !is_valid_variant_name(&code_value.key) {
                return Err(LinkError::InvalidVariantKey {
                    from,
                    key: code_value.key.clone(),
                });
            }

            visited.insert(&code_value.key);

            // Check the reference type
            let ty = self.check_input_value(&code_value.value, from)?;

            subitems.push(ObjectSubitem {
                key: code_value.key.clone(),
                ty: ty.into_owned(),
            });
        }

        Ok(LinkType::Object(subitems))
    }

    /// Check whether the reference is effective and not cared for the type of reference
    pub fn check_named_values<'b, I>(
        &self,
        names: I,
        from: ComponentId,
        subtype: Option<LinkType>, // Whether to specify the type
    ) -> Result<LinkType, LinkError>
    where
        I: Iterator<Item = &'b NamedValue>,
    {
        let mut visited = HashSet::new(); // Name is not allowed to repeat

        let mut subitems = Vec::new(); // Record each sub -item

        for named_value in names {
            let mut name = named_value.name.clone();
            if name.is_empty() {
                return Err(LinkError::InvalidName {
                    from,
                    name: name.clone(),
                });
            }
            if 64 < name.len() {
                return Err(LinkError::InvalidName {
                    from,
                    name: name.clone(),
                });
            }
            if !is_valid_variant_name(&name) {
                name = format!("\"{name}\"");
            }

            // Is it already existed?
            if visited.contains(&name) {
                return Err(LinkError::DuplicateName {
                    from,
                    name: named_value.name.clone(),
                });
            }

            visited.insert(name.clone());

            // Check the reference type
            let ty = self.check_input_value(&named_value.value, from)?;

            // ? If the request must be specified type
            if let Some(subtype) = &subtype {
                if subtype != ty.as_ref() {
                    return Err(LinkError::InvalidNamedValueType(
                        (
                            from,
                            format!("wrong value type of {}, must be {:?}", named_value.name, subtype),
                        )
                            .into(),
                    ));
                }
            }

            subitems.push(ObjectSubitem {
                key: name.clone(),
                ty: ty.into_owned(),
            });
        }

        Ok(LinkType::Object(subitems))
    }
}
