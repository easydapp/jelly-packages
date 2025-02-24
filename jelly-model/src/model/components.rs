use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::CallChain;

use super::super::store::api::ApiData;
use super::super::store::api::anchor::ApiDataAnchor;
use super::super::store::code::CodeData;
use super::super::store::code::anchor::CodeDataAnchor;
use super::super::store::code::item::CodeItem;
use super::super::store::code::item::args::ArgCodeType;
use super::super::store::code::item::types::CodeType;
use super::super::store::combined::anchor::CombinedAnchor;
use super::super::types::TimestampMills;
use super::combined::{
    CombinedMetadata, ComponentFormRequired, ComponentIdentityRequired, ComponentInteractionRequired,
    ComponentParamRequired,
};
use super::common::api::evm::EvmCallApi;
use super::common::api::ic::IcCallApi;
use super::common::api::ic::candid::IcFunctionArgsType;
use super::common::call_trigger::ComponentCallTrigger;
use super::common::code::CodeContent;
use super::common::color::ComponentColor;
use super::common::error::LinkError;
use super::common::identity::ComponentId;
use super::common::lets::{AllEndpoint, AllEndpoints, Endpoint};
use super::common::refer::{CodeValue, InputValue, NamedValue, ReferValue};
use super::common::to_typescript::ToTypescript;
use super::common::trigger::ComponentTriggered;
use super::common::types::{LinkType, ObjectSubitem};
use super::common::validate::ValidateForm;
use super::common::values::{ArrayLinkValue, LinkValue};
use super::common::wallet::evm::EvmWallet;
use super::common::wallet::ic::IcWallet;
use super::types::abi::types::{AbiItem, AbiParam, AbiStateMutability};
use super::types::check::{CheckFunction, CheckedCodeItem};
use super::types::evm::EvmChain;

/// parameter
pub mod param;

/// constant
pub mod constant;

/// form
pub mod form;

/// code
pub mod code;

/// identity
pub mod identity;

/// call
pub mod call;

/// interaction
pub mod interaction;

/// view
pub mod view;

/// condition
pub mod condition;

/// loop // ! Not yet
/// Four components
/// 1. LoopStart: At the beginning of the cycle, the specified loop method, the traversal object is still infinitely loop
/// 2. LoopEnd: Output each execution object
/// 3. LoopContinue: Ignore the object of this wheel
/// 4. LoopBreak: Ignore this wheel and subsequent objects, can only be executed in order
pub mod loops;

/// variable // ! Not yet
/// Two components
/// 1. VariableValue
/// 2. VariableAssign
pub mod variable;

/// output
pub mod output;

/// combined
pub mod combined;

use call::ComponentCall;
use code::ComponentCode;
use combined::ComponentCombined;
use condition::ComponentCondition;
use constant::ComponentConst;
use form::ComponentForm;
use identity::{ComponentIdentity, IdentityInnerMetadata, IdentityMetadata};
use interaction::ComponentInteraction;
use output::ComponentOutput;
use param::ComponentParam;
use view::ComponentView;

/// Component
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LinkComponent {
    /// parameter
    #[serde(rename = "param")]
    Param(ComponentParam),

    /// constant
    #[serde(rename = "const")]
    Const(ComponentConst),

    /// form
    #[serde(rename = "form")]
    Form(ComponentForm),

    /// code
    #[serde(rename = "code")]
    Code(ComponentCode),

    /// identity
    #[serde(rename = "identity")]
    Identity(ComponentIdentity),

    /// call
    #[serde(rename = "call")]
    Call(ComponentCall),

    /// interaction
    #[serde(rename = "interaction")]
    Interaction(ComponentInteraction),

    /// view
    #[serde(rename = "view")]
    View(ComponentView),

    /// condition
    #[serde(rename = "condition")]
    Condition(ComponentCondition),

    /// output
    #[serde(rename = "output")]
    Output(ComponentOutput),

    /// combined
    #[serde(rename = "combined")]
    Combined(ComponentCombined),
}

impl LinkComponent {
    /// id
    pub fn id(&self) -> ComponentId {
        match self {
            LinkComponent::Param(param) => param.id,
            LinkComponent::Const(constant) => constant.id,
            LinkComponent::Form(form) => form.id,
            LinkComponent::Code(code) => code.id,
            LinkComponent::Identity(identity) => identity.id,
            LinkComponent::Call(call) => call.id,
            LinkComponent::Interaction(interaction) => interaction.id,
            LinkComponent::View(view) => view.id,
            LinkComponent::Condition(condition) => condition.id,
            LinkComponent::Output(output) => output.id,
            LinkComponent::Combined(combined) => combined.id,
        }
    }

    /// Get the introduction point
    pub fn get_inlets(&self) -> Option<&Vec<Endpoint>> {
        match self {
            LinkComponent::Param(_) => None,
            LinkComponent::Const(_) => None,
            LinkComponent::Form(form) => form.get_inlets(),
            LinkComponent::Code(code) => code.get_inlets(),
            LinkComponent::Identity(identity) => identity.get_inlets(),
            LinkComponent::Call(call) => call.get_inlets(),
            LinkComponent::Interaction(interaction) => interaction.get_inlets(),
            LinkComponent::View(view) => view.get_inlets(),
            LinkComponent::Condition(condition) => condition.get_inlets(),
            LinkComponent::Output(output) => output.get_inlets(),
            LinkComponent::Combined(combined) => combined.get_inlets(),
        }
    }

    /// Calculate the output point of the component
    pub fn count_outputs(&self) -> u32 {
        match self {
            LinkComponent::Param(_) => 1,
            LinkComponent::Const(_) => 1,
            LinkComponent::Form(_) => 1,
            LinkComponent::Code(_) => 1,
            LinkComponent::Identity(_) => 1,
            LinkComponent::Call(_) => 1,
            LinkComponent::Interaction(_) => 1,
            LinkComponent::View(_) => 1, // 1 access, but no data
            LinkComponent::Condition(condition) => condition.count_outputs(), // Multiple access, but no data
            LinkComponent::Output(_) => 1,
            LinkComponent::Combined(_) => 1, // 1 access, do you have any data to see if the combined has an output component
        }
    }

    /// get code anchor
    pub fn get_code_anchors(&self) -> Vec<CodeDataAnchor> {
        match self {
            LinkComponent::Param(_) => vec![],
            LinkComponent::Const(_) => vec![],
            LinkComponent::Form(form) => form.get_code_anchors(),
            LinkComponent::Code(code) => code.get_code_anchors(),
            LinkComponent::Identity(_) => vec![],
            LinkComponent::Call(call) => call.get_code_anchors(),
            LinkComponent::Interaction(interaction) => interaction.get_code_anchors(),
            LinkComponent::View(_) => vec![],
            LinkComponent::Condition(_) => vec![],
            LinkComponent::Output(_) => vec![],
            LinkComponent::Combined(_) => vec![], // What has been submitted is that there is no need to handle CODE
        }
    }

    /// get apis anchor
    pub fn get_apis_anchors(&self) -> Vec<ApiDataAnchor> {
        match self {
            LinkComponent::Param(_) => vec![],
            LinkComponent::Const(_) => vec![],
            LinkComponent::Form(_) => vec![],
            LinkComponent::Code(_) => vec![],
            LinkComponent::Identity(_) => vec![],
            LinkComponent::Call(call) => call.get_apis_anchors(),
            LinkComponent::Interaction(_) => vec![],
            LinkComponent::View(_) => vec![],
            LinkComponent::Condition(_) => vec![],
            LinkComponent::Output(_) => vec![],
            LinkComponent::Combined(_) => vec![], // What has been submitted is no need to handle the API
        }
    }

    /// get combined anchor
    pub fn get_combined_anchors(&self) -> Vec<CombinedAnchor> {
        match self {
            LinkComponent::Param(_) => vec![],
            LinkComponent::Const(_) => vec![],
            LinkComponent::Form(_) => vec![],
            LinkComponent::Code(_) => vec![],
            LinkComponent::Identity(_) => vec![],
            LinkComponent::Call(_) => vec![],
            LinkComponent::Interaction(_) => vec![],
            LinkComponent::View(_) => vec![],
            LinkComponent::Condition(_) => vec![],
            LinkComponent::Output(_) => vec![],
            LinkComponent::Combined(combined) => combined.get_combined_anchors(),
        }
    }

    /// Find the front access point of the component
    pub fn get_all_endpoints<'a>(
        &self,
        components: &'a HashMap<ComponentId, &LinkComponent>,
        colors: &HashMap<ComponentId, ComponentColor<'_>>,
        direct: bool,
    ) -> Option<AllEndpoints<'a>> {
        let inlets = self.get_inlets()?;

        let mut endpoints = Vec::with_capacity(inlets.len());

        for inlet in inlets {
            if let Some(c) = components.get(&inlet.id) {
                endpoints.push(AllEndpoint {
                    id: inlet.id,
                    index: inlet.index.unwrap_or_default(),
                    component: c,
                    inlets: c.get_all_endpoints(components, colors, false),
                });
            }
        }

        // The conflict directly introduced will be filtered by color
        if !direct {
            // Filter the conflict node
            let info = colors.get(&self.id());
            if let Some(info) = info {
                endpoints.retain(|endpoint| {
                    for id in &info.conflict {
                        if endpoint.find(*id) {
                            return false; // If the introduction node contains conflict nodes, then refuse to use the node data
                        }
                    }
                    true
                });
            }
        }

        Some(AllEndpoints { endpoints })
    }

    /// Find the output of the specified point
    pub fn get_output_type<'a>(&'a self, index: u32, from: &ComponentId) -> Result<Cow<'a, LinkType>, LinkError> {
        let max = self.count_outputs(); // Output point number
        if max <= index {
            return Err(LinkError::InvalidEndpoint {
                from: *from,
                inlet: Endpoint {
                    id: self.id(),
                    index: Some(index),
                },
            });
        }
        let ty = match self {
            LinkComponent::Param(_) => Cow::Owned(LinkType::Text),
            LinkComponent::Const(constant) => Cow::Borrowed(&constant.output),
            LinkComponent::Form(form) => Cow::Borrowed(&form.output),
            LinkComponent::Code(code) => Cow::Borrowed(&code.output),
            LinkComponent::Identity(identity) => identity.get_output_type(),
            LinkComponent::Call(call) => Cow::Borrowed(&call.output),
            LinkComponent::Interaction(interaction) => interaction.get_output_type(),
            LinkComponent::View(_) => {
                return Err(LinkError::ReferNoOutputComponent {
                    from: *from,
                    refer: self.id(),
                });
            } // ! No exact value is allowed
            LinkComponent::Condition(_) => {
                return Err(LinkError::ReferNoOutputComponent {
                    from: *from,
                    refer: self.id(),
                });
            } // ! No exact value is allowed
            LinkComponent::Output(output) => Cow::Borrowed(&output.output),
            LinkComponent::Combined(combined) => match combined.get_output_type() {
                Some(output) => output,
                None => {
                    return Err(LinkError::ReferNoOutputComponent {
                        from: *from,
                        refer: self.id(),
                    }); // ! If there is no output, the specific value is not allowed
                }
            },
        };
        Ok(ty)
    }

    /// get origin code
    pub fn get_origin_codes<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
    ) -> Result<Vec<CheckedCodeItem>, LinkError> {
        let codes = match self {
            LinkComponent::Param(_) => vec![],
            LinkComponent::Const(_) => vec![],
            LinkComponent::Form(form) => form.get_origin_codes(),
            LinkComponent::Code(code) => code.get_origin_codes(endpoints)?, // Parameters may be wrong
            LinkComponent::Identity(_) => vec![],
            LinkComponent::Call(call) => call.get_origin_codes(endpoints, fetch)?,
            LinkComponent::Interaction(interaction) => interaction.get_origin_codes(),
            LinkComponent::View(_) => vec![],
            LinkComponent::Condition(_) => vec![],
            LinkComponent::Output(_) => vec![],
            LinkComponent::Combined(_) => vec![], // What has been submitted is that there is no need to handle CODE
        };
        Ok(codes)
    }

    /// Get the parameter name
    pub fn get_param_name(&self) -> Option<&String> {
        if let LinkComponent::Param(param) = self {
            return Some(param.get_param_name());
        }
        None
    }

    /// Get the parameter name
    pub fn get_form_name(&self) -> Option<&String> {
        if let LinkComponent::Form(form) = self {
            return form.get_form_name();
        }
        None
    }

    /// Get the parameter name
    pub fn get_identity_name(&self) -> Option<&String> {
        if let LinkComponent::Identity(identity) = self {
            return identity.get_identity_name();
        }
        None
    }

    /// Get the parameter name
    pub fn get_interaction_name(&self) -> Option<&String> {
        if let LinkComponent::Interaction(interaction) = self {
            return interaction.get_interaction_name();
        }
        None
    }

    /// Query can be empty introduced point
    pub fn get_nullable_endpoints(&self) -> Option<Vec<Endpoint>> {
        match self {
            LinkComponent::Param(_) => None,
            LinkComponent::Const(_) => None,
            LinkComponent::Form(_) => None,
            LinkComponent::Code(_) => None,
            LinkComponent::Identity(_) => None,
            LinkComponent::Call(_) => None,
            LinkComponent::Interaction(_) => None,
            LinkComponent::View(_) => None,
            LinkComponent::Condition(condition) => condition.get_nullable_endpoints(),
            LinkComponent::Output(_) => None,
            LinkComponent::Combined(_) => None,
        }
    }

    /// Check whether the component is effective
    pub fn check<F: CheckFunction>(
        &self,
        endpoints: &Option<AllEndpoints<'_>>,
        fetch: &F,
        triggers: &mut HashMap<ComponentId, ComponentTriggered>,
        codes: &mut HashMap<CodeDataAnchor, CodeData>,
        apis: &mut HashMap<ApiDataAnchor, ApiData>,
    ) -> Result<LinkComponent, LinkError> {
        let component = match self {
            LinkComponent::Param(param) => LinkComponent::Param(param.check()?),
            LinkComponent::Const(constant) => LinkComponent::Const(constant.check()?),
            LinkComponent::Form(form) => LinkComponent::Form(form.check(endpoints, fetch, codes)?),
            LinkComponent::Code(code) => LinkComponent::Code(code.check(endpoints, fetch, codes)?),
            LinkComponent::Identity(identity) => LinkComponent::Identity(identity.check(endpoints, triggers)?),
            LinkComponent::Call(call) => LinkComponent::Call(call.check(endpoints, fetch, triggers, codes, apis)?),
            LinkComponent::Interaction(interaction) => {
                LinkComponent::Interaction(interaction.check(endpoints, fetch, triggers, codes)?)
            }
            LinkComponent::View(view) => LinkComponent::View(view.check(endpoints)?),
            LinkComponent::Condition(condition) => LinkComponent::Condition(condition.check(endpoints)?),
            LinkComponent::Output(output) => LinkComponent::Output(output.check(endpoints)?),
            LinkComponent::Combined(combined) => LinkComponent::Combined(combined.check(endpoints, fetch)?),
        };
        Ok(component)
    }

    pub(super) fn get_param_required(&self) -> Option<ComponentParamRequired> {
        if let LinkComponent::Param(param) = self {
            return Some(param.get_required());
        }
        None
    }
    pub(super) fn get_form_required(&self) -> Option<ComponentFormRequired> {
        if let LinkComponent::Form(form) = self {
            return Some(form.get_required());
        }
        None
    }
    pub(super) fn get_identity_required(&self) -> Option<ComponentIdentityRequired> {
        if let LinkComponent::Identity(identity) = self {
            return identity.get_required();
        }
        None
    }
    pub(super) fn get_interaction_required(&self) -> Option<ComponentInteractionRequired> {
        if let LinkComponent::Interaction(interaction) = self {
            return Some(interaction.get_required());
        }
        None
    }

    /// get call chain
    pub fn get_call_chain(&self) -> Option<CallChain> {
        if let LinkComponent::Identity(identity) = &self {
            return Some(identity.get_call_chain());
        }
        if let LinkComponent::Call(call) = self {
            return Some(call.get_call_chain());
        }
        None
    }

    // /// Whether to match your identity
    // pub fn is_match_identity(&self, metadata: &ComponentIdentityRequired) -> bool {
    //     match self {
    //         LinkComponent::Param(_) => false,
    //         LinkComponent::Const(_) => false,
    //         LinkComponent::Form(_) => false,
    //         LinkComponent::Code(_) => false,
    //         LinkComponent::Identity(identity) => identity.is_match_identity(metadata),
    //         LinkComponent::Call(_) => false,
    //         LinkComponent::Interaction(_) => false,
    //         LinkComponent::View(_) => false,
    //         LinkComponent::Condition(_) => false,
    //         LinkComponent::Output(_) => false,
    //         LinkComponent::Combined(_) => false,
    //     }
    // }
}
