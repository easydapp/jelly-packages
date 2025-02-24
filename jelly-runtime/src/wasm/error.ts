import { CodeItem, LinkType, LinkValue } from '@jellypack/types';
import { ComponentId } from '../model/common/identity';
import { Endpoint } from '../model/common/lets';
import { KeyRefer } from '../model/common/refer';

export interface CommonLinkError {
    from: ComponentId;
    message: string;
}

export interface LinkErrorWrongCode {
    from: ComponentId;
    code: CodeItem;
    js: string;
    message: string;
}

export interface LinkErrorValidateCodeFailed {
    from: ComponentId;
    code: CodeItem;
    js: string;
    value: LinkValue;
    message: string;
}

export type LinkError =
    | { SystemError: { message: string } }
    // ==================== Error between components ====================
    | { EmptyComponents: { message: string } }
    | { InvalidComponentId: { id: ComponentId } }
    | { DuplicateComponentId: { id: ComponentId } }
    | { CircularReference: { id: ComponentId } }
    | { AffluxComponentId: { from: ComponentId; afflux: ComponentId } }
    | { UnknownComponentOrNotRefer: { from?: ComponentId; id: ComponentId } }
    | { InvalidEndpoint: { from: ComponentId; inlet: Endpoint } }
    | { ReferNoOutputComponent: { from: ComponentId; refer: ComponentId } }
    // ==================== An element error ====================
    // ------------ tip name ------------
    | { DuplicateParamName: { name: string } }
    | { DuplicateFormName: { name: string } }
    | { DuplicateIdentityName: { name: string } }
    | { DuplicateInteractionName: { name: string } }
    // ------------ link and value ------------
    | { MismatchedLinkValueType: { from: ComponentId; value: LinkValue } }
    | { WrongLinkTypeForRefer: { from: ComponentId; inlet: Endpoint; refer: KeyRefer } }
    // ------------ object ------------
    | { DuplicateObjectKey: { from: ComponentId; key: string } }
    | { InvalidObjectKey: { from: ComponentId; key: string } }
    // ------------ code value ------------
    | { InvalidVariantKey: { from: ComponentId; key: string } }
    | { DuplicateVariantKey: { from: ComponentId; key: string } }
    // ------------ named value ------------
    | { InvalidName: { from: ComponentId; name: string } }
    | { DuplicateName: { from: ComponentId; name: string } }
    | { InvalidNamedValueType: CommonLinkError }
    // ------------ Quote non -matching ------------
    | { MismatchedInlets: { from: ComponentId } }
    // ------------ The output does not match ------------
    | { MismatchedOutput: { from: ComponentId } }
    // ------------ Code error ------------
    | { WrongCode: LinkErrorWrongCode }
    | { ValidateCodeFailed: LinkErrorValidateCodeFailed }
    // ------------ Confirm button ------------
    | { InvalidConfirmText: { from: ComponentId } }
    // ==================== Const error ====================
    | { MismatchedConstValue: { from: ComponentId; output: LinkType; value: LinkValue } }
    | { WrongConstValue: CommonLinkError }
    // ==================== Form error ====================
    | { MismatchedFormDefaultValue: { from: ComponentId; output: LinkType; value: LinkValue } }
    // ==================== Identity error ====================
    | { InvalidIdentity: CommonLinkError }
    | { InvalidIdentityHttpProxy: { from: ComponentId; proxy: string } }
    // ==================== Call error ====================
    | { InvalidCallTrigger: CommonLinkError }
    | { InvalidCallIdentity: CommonLinkError }
    | { InvalidCallOutputType: CommonLinkError }
    // ------------ call http ------------
    | { NeedlessCallHttpName: { from: ComponentId } }
    | { InvalidCallHttpUrl: CommonLinkError }
    // ------------ call ic ------------
    | { InvalidCallIcCanisterId: CommonLinkError }
    | { InvalidCallIcApi: { from: ComponentId } }
    | { CompileCallIcCandid: { from: ComponentId; candid: string; message: string } }
    | { CompileCallIcCandidTypeUnsupported: { from: ComponentId; ty: string } }
    | { InvalidCallIcApiArg: CommonLinkError }
    | { InvalidCallIcApiRet: CommonLinkError }
    // ------------ call evm ------------
    | { InvalidCallEvmActionContract: CommonLinkError }
    | { InvalidCallEvmActionApi: CommonLinkError }
    | { InvalidCallEvmActionArg: CommonLinkError }
    | { InvalidCallEvmActionRet: CommonLinkError }
    | { InvalidCallEvmActionSign: CommonLinkError }
    | { InvalidCallEvmActionPayValue: CommonLinkError }
    | { InvalidCallEvmActionGasLimit: CommonLinkError }
    | { InvalidCallEvmActionGasPrice: CommonLinkError }
    | { InvalidCallEvmActionNonce: CommonLinkError }
    | { InvalidCallEvmActionAbi: CommonLinkError }
    | { InvalidCallEvmActionBytecode: CommonLinkError }
    | { InvalidCallEvmActionTransferTo: CommonLinkError }
    | { InvalidCallEvmActionOutput: CommonLinkError }
    // ==================== Interaction error ====================
    | { InvalidInteractionComponent: CommonLinkError }
    // ==================== View error ====================
    | { InvalidViewComponent: CommonLinkError }
    // ==================== Output error ====================
    // eslint-disable-next-line @typescript-eslint/no-empty-object-type
    | { MultipleOutput: {} }
    // ==================== Condition error ====================
    | { InvalidCondition: CommonLinkError };
// ==================== Combined error ====================
// | { MismatchedCombinedMetadata: { from: ComponentId; anchor: CombinedAnchor } }
// | { InvalidCombinedRefer: { from: ComponentId; anchor: CombinedAnchor; message: string } };
