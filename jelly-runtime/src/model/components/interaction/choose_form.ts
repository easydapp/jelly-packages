import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../common/identity';
import { named_value_get_used_component, NamedValue } from '../../common/refer';
import { ValidateForm } from '../../common/validate';

export interface InteractionChooseFormMetadata {
    values: NamedValue[];
    default?: string; // Default value The default value of the input box. Because there is a "Trigger" button, the default here is only an auxiliary filling. The confirmation button can only be triggered
    confirm?: string;
    validate?: ValidateForm;
    style?: string;
}

// get output type
export const interaction_choose_form_metadata_get_output_type = (_self: InteractionChooseFormMetadata): LinkType => {
    return 'text';
};

export const interaction_choose_form_metadata_get_used_component = (
    self: InteractionChooseFormMetadata,
): ComponentId[] => {
    return self.values.flatMap((named_value) => named_value_get_used_component(named_value));
};

// ========================= style =========================

// Button and input box
export interface InteractionChooseFormMetadataStyle {
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
