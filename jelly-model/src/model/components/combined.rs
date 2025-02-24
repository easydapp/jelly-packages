use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::model::common::types::LinkType;

use super::{AllEndpoints, CheckFunction, CombinedAnchor, CombinedMetadata, ComponentId, Endpoint, LinkError};

/// The combined component is the packing unit that is packaged well
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentCombined {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    inlets: Option<Vec<Endpoint>>,

    /// metadata data required for this component execution
    metadata: ComponentCombinedMetadata,
    // Output type
    // pub output: LinkType, // Combined output type
}

/// combined metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentCombinedMetadata {
    /// Which one is referenced
    anchor: CombinedAnchor,
    /// Corresponding metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<CombinedMetadata>,
    // /// Whether to hide the display component
    // #[serde(skip_serializing_if = "Option::is_none")]
    // hidden: Option<bool>,

    // /// Parameter specification
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // params: Option<Vec<CombinedParamItem>>,
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // identities: Option<Vec<CombinedIdentityItem>>,
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // forms: Option<Vec<CombinedFormItem>>,
    // #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    // interactions: Option<Vec<CombinedInteractionItem>>,
}

// /// Parameter component
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct CombinedParamItem {
//     /// Internal need
//     inner: ComponentId,
//     /// Quote external
//     value: InputValue,
// }

// /// Identity component
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct CombinedIdentityItem {
//     /// Internal need
//     inner: ComponentId,
//     /// Quote external
//     identity: ComponentId,
// }

// /// Form component
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct CombinedFormItem {
//     /// Internal need
//     inner: ComponentId,
//     /// Quote external
//     value: InputValue,
// }

// /// Interactive component
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
// pub struct CombinedInteractionItem {
//     /// Internal need
//     inner: ComponentId,
//     /// Quote external ones may not need to be quoted
//     #[serde(skip_serializing_if = "Option::is_none")]
//     value: Option<InputValue>,
// }

impl ComponentCombined {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// get combined anchor
    pub fn get_combined_anchors(&self) -> Vec<CombinedAnchor> {
        vec![self.metadata.anchor.clone()]
    }

    /// Find the output of the specified point
    pub fn get_output_type(&self) -> Option<Cow<'_, LinkType>> {
        self.metadata
            .metadata
            .as_ref()
            .and_then(|m| m.output.as_ref().map(Cow::Borrowed))
    }

    /// check
    pub fn check<F: CheckFunction>(
        &self,
        _endpoints: &Option<AllEndpoints<'_>>,
        _fetch: &F,
    ) -> Result<Self, LinkError> {
        todo!()
        //         // 0 Check whether the reference is matched
        //         let endpoints = endpoints.as_ref().map(Cow::Borrowed).unwrap_or_default();

        //         // Check condition
        //         let Combined = fetch
        //             .fetch_Combined(self.metadata.anchor.clone())
        //             .await
        //             .map_err(|error| LinkError::SystemError {
        //                 message: format!("fetch Combined failed: {error}"),
        //             })?;
        //         if Combined.metadata != self.metadata.metadata {
        //             return Err(LinkError::MismatchedCombinedMetadata {
        //                 from: self.id,
        //                 anchor: self.metadata.anchor.clone(),
        //             });
        //         }

        //         // check param
        //         fn param_error(from: ComponentId, anchor: CombinedAnchor) -> LinkError {
        //             LinkError::InvalidCombinedRefer {
        //                 from,
        //                 anchor,
        //                 message: "mismatched params".into(),
        //             }
        //         }
        //         match (
        //             self.metadata.metadata.as_ref().and_then(|m| m.params.as_ref()),
        //             self.metadata.params.as_ref(),
        //         ) {
        //             (Some(need), Some(params)) => {
        //                 for n in need {
        //                     let found = params.iter().find(|p| p.inner == n.id);
        //                     match found {
        //                         Some(found) => {
        //                             let output = endpoints.check_input_value(&found.value, self.id)?;
        //                             if !output.is_text() {
        //                                 return Err(param_error(self.id, self.metadata.anchor.clone()));
        //                             }
        //                         }
        //                         None => {
        //                             if n.default.is_none() {
        //                                 // If there is no default, you must quote
        //                                 return Err(param_error(self.id, self.metadata.anchor.clone()));
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //             (None, None) => {}
        //             _ => return Err(param_error(self.id, self.metadata.anchor.clone())),
        //         }

        //         // check identity
        //         fn identity_error(from: ComponentId, anchor: CombinedAnchor) -> LinkError {
        //             LinkError::InvalidCombinedRefer {
        //                 from,
        //                 anchor,
        //                 message: "mismatched identity".into(),
        //             }
        //         }
        //         match (
        //             self.metadata.metadata.as_ref().and_then(|m| m.identities.as_ref()),
        //             self.metadata.identities.as_ref(),
        //         ) {
        //             (Some(need), Some(identities)) => {
        //                 if need.len() != identities.len() {
        //                     return Err(identity_error(self.id, self.metadata.anchor.clone()));
        //                 }
        //                 for n in need {
        //                     let found = identities
        //                         .iter()
        //                         .find(|p| p.inner == n.id)
        //                         .ok_or_else(|| identity_error(self.id, self.metadata.anchor.clone()))?;
        //                     let component = endpoints
        //                         .find_endpoint(&Endpoint {
        //                             id: found.identity,
        //                             index: None,
        //                         })
        //                         .ok_or_else(|| LinkError::UnknownComponentOrNotRefer {
        //                             from: Some(self.id),
        //                             id: found.identity,
        //                         })?
        //                         .component;
        //                     if !component.is_match_identity(n) {
        //                         return Err(identity_error(self.id, self.metadata.anchor.clone()));
        //                     }
        //                 }
        //             }
        //             (None, None) => {}
        //             _ => return Err(identity_error(self.id, self.metadata.anchor.clone())),
        //         }

        //         // check form
        //         fn form_error(from: ComponentId, anchor: CombinedAnchor) -> LinkError {
        //             LinkError::InvalidCombinedRefer {
        //                 from,
        //                 anchor,
        //                 message: "mismatched forms".into(),
        //             }
        //         }
        //         match (
        //             self.metadata.metadata.as_ref().and_then(|m| m.forms.as_ref()),
        //             self.metadata.forms.as_ref(),
        //         ) {
        //             (Some(need), Some(forms)) => {
        //                 if need.len() != forms.len() {
        //                     return Err(form_error(self.id, self.metadata.anchor.clone()));
        //                 }
        //                 for n in need {
        //                     let found = forms
        //                         .iter()
        //                         .find(|p| p.inner == n.id)
        //                         .ok_or_else(|| form_error(self.id, self.metadata.anchor.clone()))?;
        //                     let output = endpoints.check_input_value(&found.value, self.id)?;
        //                     if output.as_ref() != &n.output {
        //                         return Err(form_error(self.id, self.metadata.anchor.clone()));
        //                     }
        //                 }
        //             }
        //             (None, None) => {}
        //             _ => return Err(form_error(self.id, self.metadata.anchor.clone())),
        //         }

        //         // check interaction
        //         fn interaction_error(from: ComponentId, anchor: CombinedAnchor) -> LinkError {
        //             LinkError::InvalidCombinedRefer {
        //                 from,
        //                 anchor,
        //                 message: "mismatched interactions".into(),
        //             }
        //         }
        //         match (
        //             self.metadata.metadata.as_ref().and_then(|m| m.interactions.as_ref()),
        //             self.metadata.interactions.as_ref(),
        //         ) {
        //             (Some(need), Some(interactions)) => {
        //                 if need.len() != interactions.len() {
        //                     return Err(interaction_error(self.id, self.metadata.anchor.clone()));
        //                 }
        //                 for n in need {
        //                     let found = interactions
        //                         .iter()
        //                         .find(|p| p.inner == n.id)
        //                         .ok_or_else(|| interaction_error(self.id, self.metadata.anchor.clone()))?;
        //                     if let Some(value) = found.value.as_ref() {
        //                         let output = endpoints.check_input_value(value, self.id)?;
        //                         if output.as_ref() != &n.output {
        //                             return Err(interaction_error(self.id, self.metadata.anchor.clone()));
        //                         }
        //                     }
        //                 }
        //             }
        //             (None, None) => {}
        //             _ => return Err(interaction_error(self.id, self.metadata.anchor.clone())),
        //         }

        //         Ok(Self {
        //             id: self.id,
        //             inlets: self.inlets.clone(),
        //             metadata: self.metadata.clone(),
        //         })
        //     }

        //     /// get apis anchor
        //     pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        //         if let Some(metadata) = self.metadata.metadata.as_ref() {
        //             if let Some(anchors) = metadata.apis_anchors.as_ref() {
        //                 return anchors.clone();
        //             }
        //         }
        //         vec![]
        //     }
        //     /// get Combined anchor
        //     pub fn get_Combined_anchors(&self) -> Vec<CombinedAnchor> {
        //         let mut list = vec![self.metadata.anchor.clone()];
        //         if let Some(metadata) = self.metadata.metadata.as_ref() {
        //             if let Some(anchors) = metadata.Combined_anchors.as_ref() {
        //                 list.extend(anchors.iter().cloned());
        //             }
        //         }
        //         list
    }
}
