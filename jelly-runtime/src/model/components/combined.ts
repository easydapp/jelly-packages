import { LinkType } from '@jellypack/types/lib/types';

import { CombinedAnchor } from '../../store/combined';
import { CombinedMetadata } from '../combined';
import { ComponentId } from '../common/identity';
import { Endpoint } from '../common/lets';

export interface ComponentCombined {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: ComponentCombinedMetadata;
}

export interface ComponentCombinedMetadata {
    anchor: CombinedAnchor;

    metadata?: CombinedMetadata;

    // hidden?: boolean;

    // params?: CombinedParamItem[];
    // identities?: CombinedIdentityItem[];
    // forms?: CombinedFormItem[];
    // interactions?: CombinedInteractionItem[];
}

// export type CombinedParamItem = {
//     inner: ComponentId;
//     value: InputValue;
// };

// export type CombinedIdentityItem = {
//     inner: ComponentId;
//     identity: ComponentId;
// };

// export type CombinedFormItem = {
//     inner: ComponentId;
//     value: InputValue;
// };

// export type CombinedInteractionItem = {
//     inner: ComponentId;
//     value?: InputValue;
// };

export const component_combined_get_output_type = (self: ComponentCombined): LinkType | undefined => {
    return self.metadata.metadata?.output;
};

export const component_combined_get_used_component = (_self: ComponentCombined): ComponentId[] => {
    throw new Error('unimplemented');
    // const used = [];
    // for (const p of self.metadata.params ?? [])
    //     used.push(...input_value_get_used_component(p.value));
    // for (const i of self.metadata.identities ?? []) used.push(i.identity);
    // for (const f of self.metadata.forms ?? [])
    //     used.push(...input_value_get_used_component(f.value));
    // for (const i of self.metadata.interactions ?? [])
    //     if (i.value) used.push(...input_value_get_used_component(i.value));
    // return used;
};
