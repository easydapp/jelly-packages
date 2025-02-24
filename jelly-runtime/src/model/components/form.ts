import { LinkType } from '@jellypack/types/lib/types';
import { LinkValue } from '@jellypack/types/lib/values';
import { ComponentId } from '../common/identity';
import { Endpoint } from '../common/lets';
import { InputValue } from '../common/refer';
import { ValidateForm } from '../common/validate';

export interface ComponentForm {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata?: FormMetadata;
    output: LinkType;
}

export interface FormMetadata {
    name?: string;
    default?: LinkValue;
    suffix?: InputValue;
    validate?: ValidateForm;
    style?: string;
}

export const component_form_get_used_component = (self: ComponentForm): ComponentId[] => {
    if (self.metadata?.suffix && 'refer' in self.metadata.suffix)
        return [self.metadata.suffix.refer.endpoint.id];
    return [];
};

// ========================= style =========================

export interface FormMetadataTextStyle {
    label?: string;
    placeholder?: string;
    suffix?: string;
    style?: {
        borderRadius?: string;
        borderStyle?: string;
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export interface FormMetadataBoolStyle {
    label?: string;
    trueText?: string;
    falseText?: string;
    style?: {
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export interface FormMetadataIntegerStyle {
    label?: string;
    placeholder?: string;
    suffix?: string;
    style?: {
        borderRadius?: string;
        borderStyle?: string;
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export interface FormMetadataNumberStyle {
    label?: string;
    placeholder?: string;
    suffix?: string;
    style?: {
        borderRadius?: string;
        borderStyle?: string;
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export interface FormMetadataArrayStyle {
    label?: string;
    showIndex?: boolean;
    subtype?: string; // FormMetadataStyle; // Sub-type style
    style?: {
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export interface FormMetadataObjectStyle {
    label?: string;
    subitems?: string[]; // FormMetadataStyle[]; // Sub-type style
    style?: {
        paddingTop?: string;
        paddingBottom?: string;
    };
}

export type FormMetadataStyle =
    | FormMetadataTextStyle
    | FormMetadataBoolStyle
    | FormMetadataIntegerStyle
    | FormMetadataNumberStyle
    | FormMetadataArrayStyle
    | FormMetadataObjectStyle;
