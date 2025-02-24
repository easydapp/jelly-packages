import { LinkType } from '@jellypack/types/lib/types';
import { ComponentId } from '../common/identity';
import { Endpoint } from '../common/lets';
import { code_value_get_used_component, CodeValue } from '../common/refer';

export interface ComponentOutput {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata?: OutputMetadata;
    output: LinkType;
}

export interface OutputMetadata {
    data?: CodeValue[];
}

export const component_output_get_used_component = (self: ComponentOutput): ComponentId[] => {
    const used = [];
    for (const code_value of self.metadata?.data ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    return used;
};
