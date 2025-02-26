import { LinkType } from '@jellypack/types/lib/types';
import { LinkValue } from '@jellypack/types/lib/values';

import { ComponentId } from './identity';
import { Endpoint } from './lets';

export interface NamedValue {
    name: string;
    value: InputValue;
}

export interface CodeValue {
    key: string;
    value: InputValue;
}

export const named_value_get_used_component = (self: NamedValue): ComponentId[] => {
    return input_value_get_used_component(self.value);
};

export const code_value_get_used_component = (self: CodeValue): ComponentId[] => {
    return input_value_get_used_component(self.value);
};

export const input_value_get_used_component = (self: InputValue): ComponentId[] => {
    return match_input_value(self, { constant: () => [], refer: (refer) => [refer.endpoint.id] });
};

export type InputValue = { const: LinkValue } | { refer: ReferValue };

export const match_input_value = <T>(
    self: InputValue,
    { constant, refer }: { constant: (constant: LinkValue) => T; refer: (refer: ReferValue) => T },
): T => {
    if ('const' in self) return constant(self.const);
    if ('refer' in self) return refer(self.refer);
    throw new Error('Invalid InputValue');
};
export const match_input_value_async = async <T>(
    self: InputValue,
    { constant, refer }: { constant: (constant: LinkValue) => Promise<T>; refer: (refer: ReferValue) => Promise<T> },
): Promise<T> => {
    if ('const' in self) return constant(self.const);
    if ('refer' in self) return refer(self.refer);
    throw new Error('Invalid InputValue');
};

export interface ReferValue {
    endpoint: Endpoint;
    refer?: KeyRefer;
}

export const refer_value_get_type = (self: ReferValue, ty: LinkType): LinkType => {
    if (self.refer === undefined) return ty;
    return key_refer_get_output_type(self.refer, ty);
};

export const refer_value_get_value = (self: ReferValue, value: any): any => {
    if (self.refer === undefined) return value;
    return key_refer_get_value(self.refer, value);
};

export interface KeyRefer {
    key: string;
    refer?: KeyRefer;
}

export const key_refer_get_output_type = (self: KeyRefer, ty: LinkType): LinkType => {
    if (typeof ty === 'object' && 'object' in ty) {
        const subitem = ty.object.find((item) => item.key === self.key);
        if (!subitem) throw new Error('KeyRefer not found');
        const out = subitem.ty;
        if (self.refer === undefined) return out;
        else return key_refer_get_output_type(self.refer, out);
    }
    throw new Error('KeyRefer not found');
};

export const key_refer_get_value = (self: KeyRefer, value: any): any => {
    if (typeof value !== 'object') return undefined;
    const subitem = value[self.key];
    // console.error('subitem', subitem, key);
    if (subitem === undefined) return undefined;
    if (self.refer === undefined) return subitem;
    return key_refer_get_value(self.refer, subitem);
};
