import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../common/identity';
import { input_value_get_used_component, InputValue } from '../../common/refer';

export interface InteractionChooseTipMetadata {
    values: InputValue;
    tips?: InputValue;
    style?: string;
}

// get output type
export const interaction_choose_tip_metadata_get_output_type = (_self: InteractionChooseTipMetadata): LinkType => {
    return 'integer';
};

export const interaction_choose_tip_metadata_get_used_component = (
    self: InteractionChooseTipMetadata,
): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.values));
    if (self.tips) {
        used.push(...input_value_get_used_component(self.tips));
    }
    return used;
};

// ========================= style =========================

// Selector
export interface InteractionChooseTipMetadataStyle {
    placeholder?: string;

    outputLabel?: string; // After the user confirms, whether to prompt the label on the left

    style?: {
        paddingTop?: string;
        paddingBottom?: string;
    };
}
