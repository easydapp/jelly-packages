import { LinkType } from '@jellypack/types/lib/types';
import { ComponentId } from '../../common/identity';
import { named_value_get_used_component, NamedValue } from '../../common/refer';

export interface InteractionChooseMetadata {
    values: NamedValue[];
    style?: string;
}

// get output type
export const interaction_choose_metadata_get_output_type = (
    _self: InteractionChooseMetadata,
): LinkType => {
    return 'text';
};

export const interaction_choose_metadata_get_used_component = (
    self: InteractionChooseMetadata,
): ComponentId[] => {
    return self.values.flatMap((named_value) => named_value_get_used_component(named_value));
};

// ========================= style =========================

// All buttons
export interface InteractionChooseMetadataStyle {
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
