// import { Principal } from '../../node_modules/@dfinity/principal/lib/esm/index';
// import { getCrc32 } from '../../node_modules/@dfinity/principal/lib/esm/utils/getCrc';
// import { sha224 } from '../../node_modules/@dfinity/principal/lib/esm/utils/sha224';
import { Principal } from '@dfinity/principal';
import { getCrc32 } from '@dfinity/principal/lib/cjs/utils/getCrc';
import { sha224 } from '@dfinity/principal/lib/cjs/utils/sha224';

import { array2hex } from './open-hex';

function str2UTF8(str: string): number[] {
    const bytes = [];
    let c;
    const len = str.length;
    for (let i = 0; i < len; i++) {
        c = str.charCodeAt(i);
        if (c >= 0x010000 && c <= 0x10ffff) {
            bytes.push(((c >> 18) & 0x07) | 0xf0);
            bytes.push(((c >> 12) & 0x3f) | 0x80);
            bytes.push(((c >> 6) & 0x3f) | 0x80);
            bytes.push((c & 0x3f) | 0x80);
        } else if (c >= 0x000800 && c <= 0x00ffff) {
            bytes.push(((c >> 12) & 0x0f) | 0xe0);
            bytes.push(((c >> 6) & 0x3f) | 0x80);
            bytes.push((c & 0x3f) | 0x80);
        } else if (c >= 0x000080 && c <= 0x0007ff) {
            bytes.push(((c >> 6) & 0x1f) | 0xc0);
            bytes.push((c & 0x3f) | 0x80);
        } else {
            bytes.push(c & 0xff);
        }
    }
    return bytes;
}

const principal2account_array = (principal: string, subaccount?: number | number[]): number[] => {
    if (typeof subaccount === 'number') {
        subaccount = [
            (subaccount >> 24) & 0xff,
            (subaccount >> 16) & 0xff,
            (subaccount >> 8) & 0xff,
            subaccount & 0xff,
        ];
    }
    if (subaccount === undefined) subaccount = [];
    while (subaccount.length < 32) subaccount = [0, ...subaccount];
    if (subaccount.length !== 32) throw new Error(`wrong subaccount: ${subaccount}`);

    const buffer: number[] = [
        ...str2UTF8('\x0Aaccount-id'),
        ...Principal.fromText(principal).toUint8Array(),
        ...subaccount,
    ];

    const hash = sha224(new Uint8Array(buffer));
    const checksum = getCrc32(hash);

    const result = [
        (checksum >> 24) & 0xff,
        (checksum >> 16) & 0xff,
        (checksum >> 8) & 0xff,
        (checksum >> 0) & 0xff,
        ...hash,
    ];

    return result;
};

// ext standard. prefix for id
const TDS = [10, 116, 105, 100]; //b"\x0Atid"

// principal to account id
export const principal2account_id = (principal: string, subaccount?: number | number[]): string => {
    return array2hex(principal2account_array(principal, subaccount));
};

// ext standard. index to nft identifier
export const ext_index2identifier = (collection: string, token_index: number): string => {
    const buffer: number[] = [
        ...TDS,
        ...Principal.fromText(collection).toUint8Array(),
        (token_index >> 24) & 0xff,
        (token_index >> 16) & 0xff,
        (token_index >> 8) & 0xff,
        token_index & 0xff,
    ];

    return Principal.fromUint8Array(new Uint8Array(buffer)).toText();
};

// ext standard. nft identifier to index
export const ext_identifier2index = (token_identifier: string): number => {
    const buffer = Principal.fromText(token_identifier).toUint8Array();

    if (buffer.length != 18) throw new Error(`can not parse token index by token identifier 1: ${token_identifier}`);

    if (buffer[0] != TDS[0] || buffer[1] != TDS[1] || buffer[2] != TDS[2] || buffer[3] != TDS[3]) {
        throw new Error(`can not parse token index by token identifier 2: ${token_identifier}`);
    }

    const inner_collection = Principal.fromUint8Array(buffer.subarray(4, buffer.length - 4)).toText();
    if (inner_collection.length !== 27) {
        throw new Error(`can not parse token index by token identifier 3: ${token_identifier}`);
    }

    const index = buffer.subarray(buffer.length - 4, buffer.length);

    return ((index[0] & 0xff) << 24) | ((index[1] & 0xff) << 16) | ((index[2] & 0xff) << 8) | ((index[3] & 0xff) << 0);
};
