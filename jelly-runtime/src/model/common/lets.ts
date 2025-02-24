import { LinkType } from '@jellypack/types/lib/types';

import { LinkComponent } from '..';
import { RuntimeValues } from '../../runtime/value';
import { link_component_get_output_type } from '../components';
import { ComponentId } from './identity';
import { key_refer_get_output_type, KeyRefer } from './refer';

export interface Endpoint {
    id: ComponentId;
    index?: number; // 0 if undefined
}

export interface AllEndpoint {
    id: ComponentId;
    index: number;
    component: LinkComponent;
    inlets?: AllEndpoints;
}

export interface AllEndpoints {
    endpoints: AllEndpoint[];
}

// Find the introduction component
export const all_endpoints_find_endpoint = (self: AllEndpoints, endpoint: Endpoint): AllEndpoint | undefined => {
    for (const e of self.endpoints) {
        if (e.id === endpoint.id && e.index === (endpoint.index ?? 0)) return e;
        if (e.inlets) {
            const found = all_endpoints_find_endpoint(e.inlets, endpoint);
            if (found) return found;
        }
    }
    return undefined;
};

const inner_all_endpoints_has_output = (
    self: AllEndpoints,
    runtime_values: RuntimeValues,
    used: Set<ComponentId>,
    excepted?: Set<ComponentId>,
): boolean => {
    for (const e of self.endpoints) {
        if (used.has(e.id) && !excepted?.has(e.id)) {
            if (!runtime_values.has_output(e.id, e.index)) return false;
        }
        if (e.inlets && !inner_all_endpoints_has_output(e.inlets, runtime_values, used, excepted)) return false;
    }
    return true;
};

export const all_endpoints_has_output = (
    self: AllEndpoints,
    runtime_values: RuntimeValues,
    used: Set<ComponentId>,
    excepted?: Set<ComponentId>,
): boolean => {
    const resort = new Map<ComponentId, number[]>();
    for (const e of self.endpoints) {
        if (!resort.has(e.id)) resort.set(e.id, []);
        resort.get(e.id)?.push(e.index ?? 0);
    }

    for (const [id, indexes] of resort) {
        if (!excepted || !excepted.has(id)) {
            if (indexes.length == 1) {
                if (!runtime_values.has_output(id, indexes[0])) return false;
            }
            if (1 < indexes.length) {
                let passed = false;
                for (const index of indexes) {
                    if (runtime_values.has_output(id, index)) {
                        passed = true;
                        break;
                    }
                }
                if (!passed) return false;
            }
        }
    }

    // for (const e of self.endpoints) {
    //     if (!excepted || !excepted.has(e.id)) {
    //         if (!runtime_values.has_output(e.id, e.index)) return false;
    //     }
    // }

    for (const e of self.endpoints) {
        if (e.inlets && !inner_all_endpoints_has_output(e.inlets, runtime_values, used, excepted)) return false;
    }
    return true;
};

export const all_endpoints_find_output_type = (self: AllEndpoints, endpoint: Endpoint, refer?: KeyRefer): LinkType => {
    const all_endpoint = all_endpoints_find_endpoint(self, endpoint);
    if (!all_endpoint) throw new Error('Endpoint not found');
    const output = link_component_get_output_type(all_endpoint.component, endpoint.index ?? 0);
    if (!output) throw new Error('Endpoint not found');
    if (refer) return key_refer_get_output_type(refer, output);
    return output;
};

// export const all_endpoints_has_component = (self: AllEndpoints, id: ComponentId): boolean => {
//     for (const e of self.endpoints) {
//         if (e.id === id) return true;
//         if (e.inlets && all_endpoints_has_component(e.inlets, id)) return true;
//     }
//     return false;
// };

// ================== Sub-branch ==================

export interface AllBranch {
    id: ComponentId;
    component: LinkComponent;
    inlets?: AllBranches;
}

export interface AllBranches {
    branches: AllBranch[];
}
