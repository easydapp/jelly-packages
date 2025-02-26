import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../../common/identity';
import { input_value_get_used_component, InputValue } from '../../../common/refer';
import { ChooseFullForm } from './form';

export interface InteractionChooseFullMetadata {
    values: InputValue;
    form?: ChooseFullForm;
    style?: string;
}

// get output type
export const interaction_choose_full_metadata_get_output_type = (_self: InteractionChooseFullMetadata): LinkType => {
    return 'text';
};

export const interaction_choose_full_metadata_get_used_component = (
    self: InteractionChooseFullMetadata,
): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.values));
    return used;
};

// ========================= style =========================

// Button and input box
export interface InteractionChooseFullMetadataStyle {
    formPlaceholder?: string;
    formSuffix?: string;

    outputLabel?: string; // After the user confirms, whether to prompt the label on the left

    style?: {
        gridTemplateColumns?: string;
        paddingTop?: string;
        paddingBottom?: string;
        backgroundColor?: string;
        color?: string;
        borderRadius?: string;
        fontWeight?: string;
    };
}
