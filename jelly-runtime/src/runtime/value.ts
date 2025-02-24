import { link_type_is_match_js_value, LinkType } from '@jellypack/types/lib/types';
import { link_value_to_js_value } from '@jellypack/types/lib/values';
import { deepClone } from '../common/clones';
import { same } from '../common/same';
import { ComponentId } from '../model/common/identity';
import { all_endpoints_find_endpoint, AllEndpoints, Endpoint } from '../model/common/lets';
import {
    CodeValue,
    InputValue,
    match_input_value,
    refer_value_get_type,
    refer_value_get_value,
} from '../model/common/refer';
import { link_component_get_output_type } from '../model/components';

// Introduce the output of nodes
export interface EndpointOutput<T> { value: T | undefined; spend: number }

export class RuntimeValue {
    values: any[];
    spend: number;
    constructor() {
        this.values = [];
        this.spend = 0;
    }

    length(): number {
        return this.values.length;
    }

    get(index: number): any {
        return this.values[index];
    }

    set(index: number, value: any, spend: number) {
        this.values[index] = value;
        this.spend = spend;
    }
}
export class RuntimeValues {
    outputs: Record<ComponentId, RuntimeValue>;
    constructor() {
        this.outputs = {};
    }

    has_component(id: ComponentId): boolean {
        return this.outputs[id] !== undefined;
    }
    has_output(id: ComponentId, index: number): boolean {
        return this.outputs[id]?.get(index) !== undefined;
    }

    get_inlets_outputs(inlets: Endpoint[] | undefined): any[] {
        const values: any[] = [];
        for (const inlet of inlets ?? []) {
            values.push(this.get_outputs(inlet.id));
        }
        return values;
    }
    get_outputs(id: ComponentId): any[] | undefined {
        return this.outputs[id]?.values;
    }
    get_output<T>(id: ComponentId, index: number): EndpointOutput<T> | undefined {
        const value = this.outputs[id]?.get(index);
        if (value === undefined) return undefined;
        return { value, spend: this.outputs[id]?.spend ?? 0 };
    }

    set(id: ComponentId, index: number, value: any, spend: number) {
        if (this.outputs[id] === undefined) {
            this.outputs[id] = new RuntimeValue();
        }
        this.outputs[id].set(index, value, spend);
    }
    delete(id: ComponentId): RuntimeValue | undefined {
        const value = this.outputs[id];
        // eslint-disable-next-line @typescript-eslint/no-dynamic-delete
        delete this.outputs[id];
        return value;
    }
    replace(
        id: ComponentId,
        index: number,
        value: any, // js value
        output: LinkType,
        name: string,
        spend: number, // time spend
        callback?: (last: any, value: any) => void,
    ): boolean {
        const last = this.outputs[id]?.get(index);
        if (value === undefined) {
            if (last === undefined) return false;
            this.set(id, index, value, spend);
        } else {
            if (!link_type_is_match_js_value(output, value)) {
                console.error('type is mismatch:', output, value);
                throw new Error(`${name} value is not match type`);
            }
            if (last === undefined) {
                this.set(id, index, value, spend);
            } else {
                if (same(last, value)) return false;
                this.set(id, index, value, spend);
            }
        }
        if (callback) callback(last, value);
        return true;
    }

    find_data(endpoints: AllEndpoints | undefined, data: CodeValue[]): any {
        if (!endpoints) {
            if (data.length === 0) return {};
            for (const d of data) {
                if ('refer' in d.value) {
                    console.error('find data endpoints is undefined');
                    throw new Error('invalid endpoint');
                }
            }
            endpoints = { endpoints: [] }; // What needs is constant, no reference is required
        }
        const params: Record<string, any> = {};
        let done = true;
        for (const param of data) {
            const key = param.key;
            const value = param.value;
            params[key] = match_input_value<any>(value, {
                constant: (constant) => link_value_to_js_value(constant),
                refer: (refer) => {
                    // 1. Check whether the reference component generates a value
                    const outputs = this.get_outputs(refer.endpoint.id);
                    if (outputs === undefined) {
                        done = false;
                        return;
                    }
                    // 2. Check whether to link to the reference output point
                    const endpoint = all_endpoints_find_endpoint(endpoints, refer.endpoint);
                    if (endpoint === undefined) throw new Error(`can not find endpoint`);
                    // 3. Check whether the reference output point has output
                    let output = outputs[endpoint.index];
                    if (output === undefined) {
                        done = false;
                        return;
                    }
                    // 4. get type of output
                    let ty = link_component_get_output_type(endpoint.component, endpoint.index);
                    if (ty === undefined) throw new Error(`can not find endpoint type`);
                    ty = refer_value_get_type(refer, ty);
                    // 5. read value
                    output = refer_value_get_value(refer, output);
                    if (!link_type_is_match_js_value(ty, output))
                        throw new Error(`can not find endpoint type`);
                    return deepClone(output);
                },
            });
            if (!done) break;
        }
        return done ? params : undefined;
    }

    find_input_value<T>(input: InputValue, ty?: LinkType): T | undefined {
        const value = match_input_value<T | undefined>(input, {
            constant: (constant) => link_value_to_js_value(constant),
            refer: (refer) => {
                const outputs = this.get_outputs(refer.endpoint.id);
                if (outputs === undefined) return undefined;
                const output = outputs[refer.endpoint.index ?? 0];
                if (output === undefined) return undefined;
                const value = refer_value_get_value(refer, output);
                return deepClone(value);
            },
        });
        if (value !== undefined && ty !== undefined) {
            if (!link_type_is_match_js_value(ty, value)) throw new Error(`type is mismatch`);
        }
        return value;
    }
}
