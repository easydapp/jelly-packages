import open from '@jellypack/types/lib/open';
import { link_value_to_js_value } from '@jellypack/types/lib/values';

// =================== tool ===================

export const EXCLUDES = ['eval', 'import']; // ! strict mode execute Error

export const MASKS: [string, string][] = [
    ['document', 'undefined'],
    ['window', 'undefined'],
    ['globalThis', 'undefined'],

    ['Function', 'undefined'],
    ['eval', 'undefined'],
    ['XMLHttpRequest', 'undefined'],
    ['import', 'undefined'],
    ['require', 'undefined'],

    ['console', 'undefined'],

    ['JSON', '{ stringify: env.JSON.stringify, parse: env.JSON.parse }'],

    ['OpenJSON', 'env.OpenJSON'],
    ['OpenType', 'env.OpenType'],
    ['OpenNumber', 'env.OpenNumber'],
    ['OpenHex', 'env.OpenHex'],

    ['Principal', 'env.Principal'],
    ['OpenIc', 'env.OpenIc'],
];

// =================== tool ===================

export const OpenJSON = {
    stringify: open.OpenJSON.stringify_factory(JSON.stringify),
    parse: open.OpenJSON.parse_factory(JSON.parse),
};

export const OpenType = {
    link_value_to_js_value,
};

export const OpenNumber: {
    format_number: (text_number: string, count?: number) => string;
    format_integer: (text_number: string, count?: number) => string;
    unit: (decimal: number) => string;
} = open.OpenNumber;

export const OpenHex: {
    hex2array: (hex: string) => number[];
    array2hex: (value: number[] | Uint8Array) => string;
} = open.OpenHex;

// =================== ic ===================

export const Principal = open.Principal;

export const OpenIc: {
    // Provide basic tools
    principal2account_id: (principal: string, subaccount?: number | number[]) => string; // principal to account_id
    ext_index2identifier: (collection: string, token_index: number) => string; // collection and id to token_id
    ext_identifier2index: (token_identifier: string) => number; // token_id to id
} = open.OpenIc;
