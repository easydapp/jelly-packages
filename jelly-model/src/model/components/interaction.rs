use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::{
    AllEndpoints, ArrayLinkValue, CheckFunction, CheckedCodeItem, CodeData, CodeDataAnchor, ComponentId,
    ComponentInteractionRequired, ComponentTriggered, Endpoint, InputValue, LinkError, LinkType, LinkValue, NamedValue,
    ValidateForm,
};

/// interaction choose
pub mod choose;

/// interaction choose_form
pub mod choose_form;

/// interaction choose_tip
pub mod choose_tip;

/// interaction choose_full
pub mod choose_full;

use choose::InteractionChooseMetadata;

use choose_form::InteractionChooseFormMetadata;

use choose_tip::InteractionChooseTipMetadata;

use choose_full::InteractionChooseFullMetadata;

/// Interaction
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComponentInteraction {
    /// Id with each component
    pub id: ComponentId,

    /// Dependencies
    #[serde(skip_serializing_if = "crate::is_empty_option_vec")]
    inlets: Option<Vec<Endpoint>>,

    /// metadata required for this component execution
    metadata: InteractionMetadata,
    // The output type of the output type is different
    // pub output: LinkType, // Specified type
}

/// interaction metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct InteractionMetadata {
    /// Variable name // ! Follow the variable name rules and unique
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// metadata
    metadata: InteractionInnerMetadata,
}

/// interaction metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum InteractionInnerMetadata {
    /// choose
    #[serde(rename = "choose")]
    Choose(InteractionChooseMetadata),
    /// choose_form
    #[serde(rename = "choose_form")]
    ChooseForm(InteractionChooseFormMetadata),
    /// choose_tip
    #[serde(rename = "choose_tip")]
    ChooseTip(InteractionChooseTipMetadata),
    /// choose_full
    #[serde(rename = "choose_full")]
    ChooseFull(InteractionChooseFullMetadata),
}

impl InteractionInnerMetadata {
    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        match &self {
            Self::Choose(choose) => choose.get_output_type(),
            Self::ChooseForm(choose_form) => choose_form.get_output_type(),
            Self::ChooseTip(choose_tip) => choose_tip.get_output_type(),
            Self::ChooseFull(choose_full) => choose_full.get_output_type(),
        }
    }
}

impl ComponentInteraction {
    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        self.inlets.as_ref()
    }

    /// get code anchors
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        let mut anchors = Vec::new();

        if let Some(items) = match &self.metadata.metadata {
            InteractionInnerMetadata::Choose(choose) => choose.get_code_anchors(),
            InteractionInnerMetadata::ChooseForm(choose_form) => choose_form.get_code_anchors(),
            InteractionInnerMetadata::ChooseTip(choose_tip) => choose_tip.get_code_anchors(),
            InteractionInnerMetadata::ChooseFull(choose_full) => choose_full.get_code_anchors(),
        } {
            anchors.extend(items);
        }

        anchors
    }

    /// Query output type
    pub fn get_output_type(&self) -> Cow<'static, LinkType> {
        self.metadata.metadata.get_output_type()
    }

    /// get origin code
    pub fn get_origin_codes(&self) -> Vec<CheckedCodeItem> {
        let mut codes = Vec::new();

        if let Some(items) = match &self.metadata.metadata {
            InteractionInnerMetadata::Choose(choose) => choose.get_origin_codes(),
            InteractionInnerMetadata::ChooseForm(choose_form) => choose_form.get_origin_codes(&self.id),
            InteractionInnerMetadata::ChooseTip(choose_tip) => choose_tip.get_origin_codes(),
            InteractionInnerMetadata::ChooseFull(choose_full) => choose_full.get_origin_codes(&self.id),
        } {
            codes.extend(items);
        }

        codes
    }

    /// Get the parameter name
    pub fn get_interaction_name(&self) -> Option<&String> {
        self.metadata.name.as_ref()
    }

    /// Check whether the component is effective
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
    ) -> Result<Self, LinkError> {
        // self.output.check(self.id)?; // ? Check whether the output type is correct

        // 1 Check metadata
        let metadata = match &self.metadata.metadata {
            InteractionInnerMetadata::Choose(choose) => {
                InteractionInnerMetadata::Choose(choose.check(endpoints, self.id)?)
            }
            InteractionInnerMetadata::ChooseForm(choose_form) => {
                InteractionInnerMetadata::ChooseForm(choose_form.check(endpoints, self.id, fetch, codes)?)
            }
            InteractionInnerMetadata::ChooseTip(choose_tip) => {
                InteractionInnerMetadata::ChooseTip(choose_tip.check(endpoints, self.id)?)
            }
            InteractionInnerMetadata::ChooseFull(choose_full) => {
                InteractionInnerMetadata::ChooseFull(choose_full.check(endpoints, self.id, fetch, codes)?)
            }
        };

        // 2. Recording trigger
        triggers.insert(self.id, ComponentTriggered::from_interaction(self.id));

        Ok(Self {
            id: self.id,
            inlets: self.inlets.clone(),
            metadata: InteractionMetadata {
                name: self.metadata.name.clone(),
                metadata,
            },
        })
    }

    /// Get the necessary information
    pub fn get_required(&self) -> ComponentInteractionRequired {
        ComponentInteractionRequired {
            id: self.id,
            name: self.metadata.name.clone(),
            metadata: self.metadata.metadata.clone(),
        }
    }
}
