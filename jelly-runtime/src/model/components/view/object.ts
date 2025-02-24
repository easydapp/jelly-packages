import { ComponentId } from '../../common/identity';
import { input_value_get_used_component, InputValue } from '../../common/refer';
import { InnerViewObjectItem } from './inner/object';

export interface ViewObjectMetadata {
    value: InputValue;
    inner: InnerViewObjectItem[];
    style?: string;
}

export const view_object_metadata_get_used_component = (
    self: ViewObjectMetadata,
): ComponentId[] => {
    return input_value_get_used_component(self.value);
};

// ========================= style =========================

export interface ViewObjectMetadataStyle {
    direction?: 'row' | 'column';
}
