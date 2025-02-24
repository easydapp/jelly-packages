import { CodeType } from '@jellypack/types/lib/code';
import { LinkType, match_link_type } from '@jellypack/types/lib/types';

import { match_wrapped_candid_type, WrappedCandidType } from '../../wasm/candid';
import { AbiParam } from '../types/abi';
import { evm_param_to_type, EvmType, match_evm_type } from '../types/abi/types';

const MAX_LENGTH = 50;

export const combine_typescript_object = (key_and_types: [string, string][]): string => {
    if (key_and_types.length === 0) return '{}';
    const single = `{ ${key_and_types.map((sub) => `${sub[0]}: ${sub[1]}`).join('; ')} }`;
    if (single.length <= MAX_LENGTH && !key_and_types.find((sub) => sub[1].includes('\n'))) return single;
    return `{
${key_and_types.map((sub) => `  ${sub[0]}: ${sub[1].split('\n').join('\n  ')};`).join('\n')}
}`;
};

export const combine_typescript_tuple = (types: [string][]): string => {
    if (types.length === 0) return '[]';
    const single = `[${types.join(', ')}]`;
    if (single.length <= MAX_LENGTH && !types.find((sub) => sub[0].includes('\n'))) return single;
    return `[
${types.map((t) => `  ${t[0].split('\n').join('\n  ')},`).join('\n')}
]`;
};

export const combine_typescript_option = (ty: string): string => {
    const single = `([] | [${ty}])`;
    if (single.length <= MAX_LENGTH && !ty.includes('\n')) return single;
    return `(
  | []
  | [
      ${ty.split('\n').join('\n      ')}
    ]
)`;
};

export const combine_typescript_variant = (key_and_types: [string, string][]): string => {
    if (key_and_types.length === 0) return '{}';
    const ty2: string[] = key_and_types.map((t) => combine_typescript_object([t]));
    const single = `(${ty2.join(' | ')})`;
    if (single.length <= MAX_LENGTH && !ty2.find((sub) => sub.includes('\n'))) return single;
    return `(
${ty2.map((t) => `  | ${t.split('\n').join('\n    ')}`).join('\n')}
)`;
};

// =========== link type ===========

export const link_type_to_typescript = (self: LinkType): string => {
    return match_link_type(self, {
        text: () => 'string',
        bool: () => 'boolean',
        integer: () => 'number',
        number: () => 'number',
        array: (array) => `${link_type_to_typescript(array)}[]`,
        object: (object) => {
            const key_and_types: [string, string][] = object.map((sub) => [sub.key, link_type_to_typescript(sub.ty)]);
            return combine_typescript_object(key_and_types);
        },
    });
};

// =========== candid type ===========

export const candid_to_typescript = (type: WrappedCandidType): CodeType => {
    return match_wrapped_candid_type(type, {
        bool: () => ({ ty: 'boolean' }),
        nat: () => ({ ty: 'bigint' }),
        int: () => ({ ty: 'bigint' }),
        nat8: () => ({ ty: 'number' }),
        nat16: () => ({ ty: 'number' }),
        nat32: () => ({ ty: 'number' }),
        nat64: () => ({ ty: 'bigint' }),
        int8: () => ({ ty: 'number' }),
        int16: () => ({ ty: 'number' }),
        int32: () => ({ ty: 'number' }),
        int64: () => ({ ty: 'bigint' }),
        float32: () => ({ ty: 'number' }),
        float64: () => ({ ty: 'number' }),
        none: () => ({ ty: 'null' }),
        text: () => ({ ty: 'string' }),
        principal: () => ({ ty: 'Principal' }),
        vec: (vec) => {
            if ('nat8' in vec.subtype) return { ty: '(Uint8Array | number[])' };
            const sub = candid_to_typescript(vec.subtype);
            return { ty: `${sub.ty}[]`, types: sub.types };
        },
        opt: (opt) => {
            const sub = candid_to_typescript(opt.subtype);
            return {
                ty: combine_typescript_option(sub.ty),
                types: sub.types,
            };
        },
        record: (record) => {
            const ty: [string, string][] = [];
            const ty_types: string[] = [];
            for (const item of record.subitems) {
                const sub = candid_to_typescript(item[1]);
                ty.push([item[0], sub.ty]);
                if (sub.types) ty_types.push(...sub.types);
            }
            return {
                ty: combine_typescript_object(ty),
                types: ty_types.length ? ty_types : undefined,
            };
        },
        variant: (variant) => {
            const ty: [string, string][] = [];
            const ty_types: string[] = [];
            for (const item of variant.subitems) {
                if (item[1]) {
                    const sub = candid_to_typescript(item[1]);
                    ty.push([item[0], sub.ty]);
                    if (sub.types) ty_types.push(...sub.types);
                } else {
                    ty.push([item[0], 'null']);
                }
            }
            return {
                ty: combine_typescript_variant(ty),
                types: ty_types.length ? ty_types : undefined,
            };
        },
        tuple: (tuple) => {
            const ty: [string][] = [];
            const ty_types: string[] = [];
            for (const item of tuple.subitems) {
                const sub = candid_to_typescript(item);
                ty.push([sub.ty]);
                if (sub.types) ty_types.push(...sub.types);
            }
            return {
                ty: combine_typescript_tuple(ty),
                types: ty_types.length ? ty_types : undefined,
            };
        },
        unknown: () => {
            return { ty: 'unknown' };
            throw new Error('candid type `unknown` can not transform to typescript');
        },
        empty: () => {
            return { ty: 'any' };
            throw new Error('candid type `empty` can not transform to typescript');
        },
        reserved: () => {
            return { ty: 'any' };
            throw new Error('candid type `reserved` can not transform to typescript');
        }, // any type
        func: () => {
            return { ty: 'any' };
            throw new Error('candid type `func` can not transform to typescript');
        }, // ! Don't know how to see the examination
        service: () => {
            throw new Error('candid type `service` can not transform to typescript');
        }, // ! Don't know how to see the examination
        rec: (rec) => {
            const sub = candid_to_typescript(rec.ty);
            const ty_types = sub.types ?? [];
            ty_types.push(`type ${rec.name} = ${sub.ty};`);
            return {
                ty: rec.name ?? '',
                types: ty_types,
            };
        },
        ref: (ref) => ({ ty: ref.name ?? '' }),
    });
};

export const candid_types_to_typescript = (items: WrappedCandidType[]): CodeType => {
    if (items.length === 0) return { ty: '[]' };

    if (items.length === 1) return candid_to_typescript(items[0]);

    const ty: [string][] = [];
    const ty_types: string[] = [];
    for (const item of items) {
        const code_type = candid_to_typescript(item);
        ty.push([code_type.ty]);
        if (code_type.types) ty_types.push(...code_type.types);
    }
    return {
        ty: combine_typescript_tuple(ty),
        types: ty_types.length ? ty_types : undefined,
    };
};

// =========== abi type ===========

export const abi_param_to_typescript = (type: EvmType): CodeType => {
    return match_evm_type(type, {
        bool: () => ({ ty: 'boolean' }),
        int: () => ({ ty: 'bigint' }),
        uint: () => ({ ty: 'bigint' }),
        fixed: () => ({ ty: 'string' }),
        ufixed: () => ({ ty: 'string' }),
        address: () => ({ ty: 'string' }),
        bytes: () => ({ ty: '(string | Uint8Array)' }),
        string: () => ({ ty: 'string' }),
        func: () => ({ ty: 'string' }),
        array: (array) => {
            const sub = abi_param_to_typescript(array.type);
            return { ty: `${sub.ty}[]`, types: sub.types };
        },
        mapping: (mapping) => {
            const key = abi_param_to_typescript(mapping.key);
            const value = abi_param_to_typescript(mapping.value);
            const ty_types: string[] = [];
            if (key.types) ty_types.push(...key.types);
            if (value.types) ty_types.push(...value.types);
            return {
                ty: `Record<${key.ty},${value.ty}>`,
                types: ty_types.length ? ty_types : undefined,
            };
        },
        tuple: (tuple) => {
            const ty: [string][] = [];
            const ty_types: string[] = [];
            for (const item of tuple) {
                const sub = abi_param_to_typescript(item);
                ty.push([sub.ty]);
                if (sub.types) ty_types.push(...sub.types);
            }
            return {
                ty: combine_typescript_tuple(ty),
                types: ty_types.length ? ty_types : undefined,
            };
        },
    });
};

export const abi_params_to_typescript = (items: AbiParam[]): CodeType => {
    if (items.length === 0) return { ty: '[]' };

    if (items.length === 1) return abi_param_to_typescript(evm_param_to_type(items[0]));

    const ty: [string][] = [];
    const ty_types: string[] = [];
    for (const item of items) {
        const code_type = abi_param_to_typescript(evm_param_to_type(item));
        ty.push([code_type.ty]);
        if (code_type.types) ty_types.push(...code_type.types);
    }
    return {
        ty: combine_typescript_tuple(ty),
        types: ty_types.length ? ty_types : undefined,
    };
};
