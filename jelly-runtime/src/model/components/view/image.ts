import { ComponentId } from '../../common/identity';
import { input_value_get_used_component, InputValue } from '../../common/refer';

export interface ViewImageMetadata {
    value: InputValue;
    href?: InputValue;
    style?: string;
}

export const view_image_metadata_get_used_component = (self: ViewImageMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.value));
    if (self.href) used.push(...input_value_get_used_component(self.href));
    return used;
};

// ========================= style =========================

export interface ViewImageMetadataStyle {
    style?: {
        borderRadius?: string;
    };
}
