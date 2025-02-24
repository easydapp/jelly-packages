use serde::Serialize;

use crate::{
    model::common::{identity::ComponentId, lets::Endpoint},
    store::code::item::CodeItem,
};

use super::{refer::KeyRefer, types::LinkType, values::LinkValue};

/// Error of examination
#[derive(Debug, Serialize)]
pub enum LinkError {
    /// Code bug
    SystemError {
        /// error message
        message: String,
    },

    // ==================== Error between components ====================
    /// No component
    EmptyComponents {
        /// error message
        message: String,
    },

    /// Invalid ID
    InvalidComponentId {
        /// Invalid ID
        id: ComponentId,
    },

    /// Repeat your own ID
    DuplicateComponentId {
        /// Repeated ID
        id: ComponentId,
    },

    /// Cyclic reference
    CircularReference {
        /// referenced ID
        id: ComponentId,
    },

    /// Multiple outputs of a certain component are gathered together
    AffluxComponentId {
        /// The ID of the collection
        from: ComponentId,
        /// Multiple output components
        afflux: ComponentId,
    },

    /// Unknown component or not introduced this component
    UnknownComponentOrNotRefer {
        /// The required component
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<ComponentId>,
        /// Can't find a component
        id: ComponentId,
    },

    /// Invalid introduction point
    InvalidEndpoint {
        /// The required component
        from: ComponentId,
        /// Introduced point
        inlet: Endpoint,
    },

    /// Inferior introduction point View and Condition are no output data
    ReferNoOutputComponent {
        /// The required component
        from: ComponentId,
        /// Introduced point
        refer: ComponentId,
    },

    // ==================== An element error ====================

    // ------------ tip name ------------
    /// Param parameters are repeated
    DuplicateParamName {
        /// Repeated parameter name
        name: String,
    },

    /// Form parameter is repeated
    DuplicateFormName {
        /// Repeated parameter name
        name: String,
    },

    /// Identity parameters are repeated
    DuplicateIdentityName {
        /// Repeated parameter name
        name: String,
    },

    /// The INTERACTION parameter is repeated
    DuplicateInteractionName {
        /// Repeated parameter name
        name: String,
    },

    // ------------ link and value ------------
    /// Types and values ​​do not correspond
    MismatchedLinkValueType {
        /// Repeated ID
        from: ComponentId,
        /// Corresponding value
        value: LinkValue,
    },

    /// Wrong quotation
    WrongLinkTypeForRefer {
        /// The required component
        from: ComponentId,
        /// Introduced point
        inlet: Endpoint,
        /// Introduction
        refer: KeyRefer,
    },

    // ------------ object ------------
    /// Object's key repeat
    DuplicateObjectKey {
        /// Repeated ID
        from: ComponentId,
        /// Repeated KEY
        key: String,
    },

    /// Object's key is invalid
    InvalidObjectKey {
        /// Repeated ID
        from: ComponentId,
        /// Invalid KEY
        key: String,
    },

    // ------------ code value ------------
    /// Invalid variable name
    InvalidVariantKey {
        /// The required component
        from: ComponentId,
        /// Invalid variable name
        key: String,
    },

    /// Repeated variable name
    DuplicateVariantKey {
        /// The required component
        from: ComponentId,
        /// Repeated variable name
        key: String,
    },

    // ------------ named value ------------
    /// Empty variable name
    InvalidName {
        /// The required component
        from: ComponentId,
        /// Repeated variable name
        name: String,
    },

    /// Repeated variable name
    DuplicateName {
        /// The required component
        from: ComponentId,
        /// Repeated variable name
        name: String,
    },

    /// Invalid variable name
    InvalidNamedValueType(CommonLinkError),

    // ------------ refer non-matching ------------
    /// Quote parameters do not match Inlets and data inconsistency
    MismatchedInlets {
        /// The required component
        from: ComponentId,
    },

    // ------------ The output does not match ------------
    /// Output of the output
    MismatchedOutput {
        /// The required component
        from: ComponentId,
    },

    // ------------ Code error ------------
    /// Wrong code
    WrongCode(Box<LinkErrorWrongCode>),

    /// Wrong code
    ValidateCodeFailed(Box<LinkErrorValidateCodeFailed>),

    // ------------ Confirm button ------------
    /// Error confirmation button text
    InvalidConfirmText {
        /// The required component
        from: ComponentId,
    },
    // ==================== Const error ====================
    /// Invalid constant value
    MismatchedConstValue {
        /// The required component
        from: ComponentId,
        /// Required type
        output: LinkType,
        /// Non -matching type
        value: LinkValue,
    },
    /// Infernal constant value Check out errors
    WrongConstValue(CommonLinkError),

    // ==================== Form error ====================
    /// mismatch form recognition value
    MismatchedFormDefaultValue {
        /// The required component
        from: ComponentId,
        /// Required type
        output: LinkType,
        /// Non -matching type
        value: LinkValue,
    },
    /// mismatch form recognition value
    MismatchedFormSuffixValue {
        /// The required component
        from: ComponentId,
        /// Non-matching type
        value: LinkType,
    },

    // ==================== Identity error ====================
    /// Invalid identity
    InvalidIdentity(CommonLinkError),

    /// invalid proxy
    InvalidIdentityHttpProxy {
        /// The required component
        from: ComponentId,
        /// The wrong proxy
        proxy: String,
    },

    // ==================== Call error ====================
    /// Invalid trigger
    InvalidCallTrigger(CommonLinkError),

    /// Invalid identity
    InvalidCallIdentity(CommonLinkError),

    /// Effective call output type
    InvalidCallOutputType(CommonLinkError),

    // ------------ call http ------------
    /// Invalid http url // There should be no name
    NeedlessCallHttpName {
        /// The required component
        from: ComponentId,
    },

    /// Invalid http url
    InvalidCallHttpUrl(CommonLinkError),

    // ------------ call ic ------------
    /// Invalid ic canister_id
    InvalidCallIcCanisterId(CommonLinkError),

    /// Invalid ic api Cyclic reference
    InvalidCallIcApi {
        /// The required component
        from: ComponentId,
    },

    /// Compiled IC failure
    CompileCallIcCandid {
        /// The required component
        from: ComponentId,
        /// candid
        candid: String,
        /// message
        message: String,
    },

    /// IC type that is not supported for the time being
    CompileCallIcCandidTypeUnsupported {
        /// The required component
        from: ComponentId,
        /// ty
        ty: String,
    },

    /// Invalid ic api arg
    InvalidCallIcApiArg(CommonLinkError),

    /// Invalid ic api ret
    InvalidCallIcApiRet(CommonLinkError),

    // ------------ call evm ------------
    /// Invalid evm action Contract address
    InvalidCallEvmActionContract(CommonLinkError),

    /// Invalid evm api
    InvalidCallEvmActionApi(CommonLinkError),

    /// Invalid evm api arg
    InvalidCallEvmActionArg(CommonLinkError),

    /// Invalid evm api ret
    InvalidCallEvmActionRet(CommonLinkError),

    /// Invalid evm action sign
    InvalidCallEvmActionSign(CommonLinkError),

    /// Invalid evm action value
    InvalidCallEvmActionPayValue(CommonLinkError),

    /// Invalid evm action value
    InvalidCallEvmActionGasLimit(CommonLinkError),

    /// Invalid evm action value
    InvalidCallEvmActionGasPrice(CommonLinkError),

    /// Invalid evm action value
    InvalidCallEvmActionNonce(CommonLinkError),

    /// Invalid evm action contract code
    InvalidCallEvmActionAbi(CommonLinkError),

    /// Invalid evm action contract code
    InvalidCallEvmActionBytecode(CommonLinkError),

    /// Invalid evm action contract code
    InvalidCallEvmActionTransferTo(CommonLinkError),

    /// Invalid evm action contract code
    InvalidCallEvmActionOutput(CommonLinkError),

    // ==================== Interaction error ====================
    /// Invalid interaction
    InvalidInteractionComponent(CommonLinkError),

    // ==================== View error ====================
    /// Invalid view
    InvalidViewComponent(CommonLinkError),

    // ==================== Output error ====================
    /// Multiple output
    MultipleOutput,

    // ==================== Condition error ====================
    /// Invalid conditions
    InvalidCondition(CommonLinkError),
    // // ==================== Combined error ====================
    // /// Invalid conditions
    // MismatchedCombinedMetadata {
    //     /// The required component
    //     from: ComponentId,
    //     /// Introduction of Combined
    //     anchor: CombinedAnchor,
    // },

    // /// Invalid conditions
    // InvalidCombinedRefer {
    //     /// The required component
    //     from: ComponentId,
    //     /// Introduction of Combined
    //     anchor: CombinedAnchor,
    //     /// Error message
    //     message: String,
    // },
}

/// The wrong code object is too large to place the stack on the stack
#[derive(Debug, Serialize)]
pub struct LinkErrorWrongCode {
    /// The required component
    pub from: ComponentId,
    /// Code
    pub code: CodeItem,
    /// error message
    pub message: String,
}

/// The wrong code object is too large to place the stack on the stack
#[derive(Debug, Serialize)]
pub struct LinkErrorValidateCodeFailed {
    /// The required component
    pub from: ComponentId,
    /// Code
    pub code: CodeItem,
    /// Compiled JS code
    pub js: String,
    /// Verification parameter
    pub value: LinkValue,
    /// error message
    pub message: String,
}

/// Wrong public part
#[derive(Debug, Serialize)]
pub struct CommonLinkError {
    /// The required component
    pub from: ComponentId,
    /// Error message
    pub message: String,
}

impl From<(ComponentId, String)> for CommonLinkError {
    fn from((from, message): (ComponentId, String)) -> Self {
        Self { from, message }
    }
}

/// System error
pub fn system_error(message: String) -> LinkError {
    LinkError::SystemError { message }
}
