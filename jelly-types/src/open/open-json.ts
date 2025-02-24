import { Principal } from './principal';

const OPEN_TYPE = '__open_type__';

// const check_reviver = (
//     value: any,
//     reviver?: (key: string | undefined, value: any) => any,
//     key?: string,
// ): any => {
//     if (reviver === undefined) return value;
//     return reviver(key, value);
// };

// const get_space = (space: string | number): string => {
//     if (typeof space === 'string') return space;
//     let s = '';
//     while (space-- > 0) s += ' ';
//     return s;
// };

// /**
//  * Converts a JavaScript Object Notation (JSON) string into an object.
//  * @param text A valid JSON string.
//  * @param reviver A function that transforms the results. This function is called for each member of the object.
//  * If a member contains nested objects, the nested objects are transformed before the parent object is.
//  */
// const custom_parse = (
//     json: string,
//     reviver?: (key: string | undefined, value: any) => any,
// ): any => {
//     const inner_parse = (
//         json: string,
//         reviver?: (key: string | undefined, value: any) => any,
//     ): [any, string] => {
//         if (json === undefined) return [check_reviver(undefined, reviver), ''];

//         if (typeof json !== 'string') throw new Error('json must be a string');

//         json = json.trim();

//         // check null
//         if (json.startsWith('null')) return [check_reviver(null, reviver), json.substring(4)];

//         // check boolean
//         if (json.startsWith('true')) return [check_reviver(true, reviver), json.substring(4)];
//         if (json.startsWith('false')) return [check_reviver(false, reviver), json.substring(5)];

//         // check number
//         const match = json.match(/^[-+]?\d+(\.\d+)?([eE][-+]?\d+)?/);
//         if (match) {
//             return [check_reviver(parseFloat(match[0]), reviver), json.substring(match[0].length)];
//         }

//         // check string
//         if (json[0] === '"') {
//             let s = '';
//             let i = 1;
//             while (i < json.length && json[i] !== '"') {
//                 if (json[i] === '\\') {
//                     i++;
//                     switch (json[i]) {
//                         case '"': {
//                             s += '"';
//                             break;
//                         }
//                         case '\\': {
//                             s += '\\';
//                             break;
//                         }
//                         case '/': {
//                             s += '/';
//                             break;
//                         }
//                         case 'b': {
//                             s += '\b';
//                             break;
//                         }
//                         case 'f': {
//                             s += '\f';
//                             break;
//                         }
//                         case 'n': {
//                             s += '\n';
//                             break;
//                         }
//                         case 'r': {
//                             s += '\r';
//                             break;
//                         }
//                         case 't': {
//                             s += '\t';
//                             break;
//                         }
//                         case 'u': {
//                             const code = parseInt(json.substring(i + 1, i + 5), 16);
//                             s += String.fromCharCode(code);
//                             i += 4;
//                             break;
//                         }
//                     }
//                 } else {
//                     s += json[i];
//                 }
//                 i++;
//             }
//             return [check_reviver(s, reviver), json.substring(i + 1)];
//         }

//         // check array
//         if (json[0] === '[') {
//             const arr: any[] = [];
//             let s = json.substring(1);
//             while (s[0] !== ']') {
//                 const [value, rest] = inner_parse(s, reviver);
//                 arr.push(value);
//                 s = rest;
//                 s = s.trim();
//                 if (s[0] === ',') s = s.substring(1);
//                 s = s.trim();
//             }
//             return [check_reviver(arr, reviver), s.substring(1)];
//         }

//         // check object
//         if (json[0] === '{') {
//             const obj: Record<string, any> = {};
//             let s = json.substring(1);
//             while (s[0] !== '}') {
//                 // key
//                 const [key, rest1] = inner_parse(s);
//                 if (typeof key !== 'string') throw new Error(`invalid key: ${key}`);
//                 s = rest1;
//                 // :
//                 s = s.trim();
//                 if (s[0] !== ':') throw new Error(`invalid key-value separator: ${s[0]}`);
//                 s = s.substring(1).trim();
//                 // value
//                 const [value, rest2] = inner_parse(s, reviver);
//                 obj[key] = check_reviver(value, reviver, key);
//                 s = rest2;
//                 if (s[0] === ',') s = s.substring(1);
//                 s = s.trim();
//             }
//             return [check_reviver(obj, reviver), s.substring(1)];
//         }

//         throw new Error(`can not parse: ${json}`);
//     };

//     const [v, rest] = inner_parse(json, reviver);

//     if (rest.length > 0) throw new Error(`invalid json: ${json}`);
//     return v;
// };

// export const parse = (
//     json: string,
//     reviver?: (key: string | undefined, value: any) => any,
//     off?: any,
// ): any => {
//     return off(json);
//     return custom_parse(
//         json,
//         reviver ??
//             ((_, v) => {
//                 if (typeof v === 'object' && v !== null) {
//                     const open_type = v[OPEN_TYPE];
//                     switch (open_type) {
//                         case 'bigint':
//                             return BigInt(v.value);
//                         case 'Uint8Array':
//                             return new Uint8Array(v.value);
//                         case 'Principal':
//                             return Principal.fromText(v.value);
//                     }
//                 }
//                 return v;
//             }),
//     );
// };

// /**
//  * Converts a JavaScript value to a JavaScript Object Notation (JSON) string.
//  * @param value A JavaScript value, usually an object or array, to be converted.
//  * @param replacer A function that transforms the results.
//  * @param space Adds indentation, white space, and line break characters to the return-value JSON text to make it easier to read.
//  */
// const custom_stringify = (
//     value: any,
//     replacer?: (key: string | undefined, value: any) => any,
//     space?: string | number,
// ): string | undefined => {
//     if (replacer) value = replacer(undefined, value); // replace

//     if (value === undefined) return undefined;

//     if (value === null) return 'null';

//     switch (typeof value) {
//         case 'string': {
//             return `"${value
//                 .replace(/\\/g, '\\\\')
//                 .replace(/"/g, '\\"')
//                 .replace(/\n/g, '\\n')
//                 .replace(/\r/g, '\\r')
//                 .replace(/\t/g, '\\t')
//                 // .replace(/\b/g, '\b')
//                 .replace(/\f/g, '\\f')
//                 .replace(/'/g, "'")
//                 .replace(/\\u([0-9a-fA-F]{4})/g, '\\\\u$1')}"`;
//         }
//         case 'number':
//         case 'boolean':
//             return `${value}`;
//         case 'object':
//             break;
//         case 'bigint':
//             throw new Error(`can not stringify bigint`);
//         case 'function':
//             throw new Error(`can not stringify function`);
//         default:
//             throw new Error(`can not stringify`);
//     }

//     if (Array.isArray(value)) {
//         // * Array
//         if (replacer) value = value.map((v) => replacer(undefined, v));
//         if (space === undefined) {
//             return `[${value.map((item: any) => custom_stringify(item, replacer, space)).join(',')}]`;
//         }
//         const s = get_space(space);
//         return `[
// ${s}${value
//             .map((item: any) => {
//                 let v = custom_stringify(item, replacer, space);
//                 if (v !== undefined) v = v.split('\n').join(`\n${s}`);
//                 return v;
//             })
//             .join(`,${s}`)}
// ]`;
//     }

//     // * Object
//     if (replacer) {
//         const old = value;
//         value = {};
//         for (const [key, v] of Object.entries(old)) {
//             value[key] = replacer(key, v);
//         }
//     }
//     if (space === undefined) {
//         return `{${Object.keys(value)
//             .map((key) => `"${key}":${custom_stringify(value[key], replacer, space)}`)
//             .join(',')}}`;
//     }
//     const s = get_space(space);
//     return `{
// ${s}${Object.keys(value)
//         .map((key) => {
//             let v = custom_stringify(value[key], replacer, space);
//             if (v !== undefined) v = v.split('\n').join(`\n${s}`);
//             return `"${key}" : ${v}`;
//         })
//         .join(',')}}`;
// };

// export const stringify = (
//     value: any,
//     replacer?: (key: string | undefined, value: any) => any,
//     space?: string | number,
// ): string | undefined => {
//     return custom_stringify(
//         value,
//         replacer ??
//             ((_, v) => {
//                 switch (typeof v) {
//                     case 'bigint': {
//                         return { [OPEN_TYPE]: 'bigint', value: `${v}` };
//                     }
//                     case 'object': {
//                         if (v === null) break;
//                         if (v instanceof Uint8Array) {
//                             return { [OPEN_TYPE]: 'Uint8Array', value: Array.from(v) };
//                         }
//                         if ('_arr' in v && '_isPrincipal' in v && v._isPrincipal && 'toText' in v) {
//                             return { [OPEN_TYPE]: 'Principal', value: v.toText() };
//                         }
//                     }
//                 }
//                 return v;
//             }),
//         space,
//     );
// };

export const parse_factory = (
    upper_parse: (json: string, reviver?: (key: string | undefined, value: any) => any) => any,
): ((json: string, reviver?: (key: string | undefined, value: any) => any) => any) => {
    return (json: string, reviver?: (key: string | undefined, value: any) => any): any => {
        if (json === undefined) return reviver ? reviver(undefined, undefined) : undefined;

        if (typeof json !== 'string') throw new Error('json must be a string');
        return upper_parse(
            json,
            reviver ??
                ((_, v) => {
                    if (typeof v === 'object' && v !== null) {
                        const open_type = v[OPEN_TYPE];
                        switch (open_type) {
                            case 'bigint':
                                return BigInt(v.value);
                            case 'Uint8Array':
                                return new Uint8Array(v.value);
                            case 'Uint16Array':
                                return new Uint16Array(v.value);
                            case 'Uint32Array':
                                return new Uint32Array(v.value);
                            case 'BigUint64Array':
                                return new BigUint64Array(v.value.map((n: string) => BigInt(n)));
                            case 'Int8Array':
                                return new Int8Array(v.value);
                            case 'Int16Array':
                                return new Int16Array(v.value);
                            case 'Int32Array':
                                return new Int32Array(v.value);
                            case 'BigInt64Array':
                                return new BigInt64Array(v.value.map((n: string) => BigInt(n)));
                            case 'ArrayBuffer': {
                                const buffer = new ArrayBuffer(v.value.length);
                                const view = new DataView(buffer);
                                for (let i = 0; i < v.value.length; i++) view.setUint8(i, v.value[i]);
                                return buffer;
                            }
                            case 'Principal':
                                return Principal.fromText(v.value);
                        }
                    }
                    return v;
                }),
        );
    };
};

export const stringify_factory = (
    upper_stringify: (
        value: any,
        replacer?: (key: string | undefined, value: any) => any,
        space?: string | number,
    ) => string,
): ((value: any, replacer?: (key: string | undefined, value: any) => any, space?: string | number) => string) => {
    return (value: any, replacer?: (key: string | undefined, value: any) => any, space?: string | number): string => {
        return upper_stringify(
            value,
            replacer ??
                ((_, v) => {
                    switch (typeof v) {
                        case 'bigint': {
                            return { [OPEN_TYPE]: 'bigint', value: `${v}` };
                        }
                        case 'object': {
                            if (v === null) break;
                            if (v instanceof Uint8Array) {
                                return { [OPEN_TYPE]: 'Uint8Array', value: Array.from(v) };
                            }
                            if (v instanceof Uint16Array) {
                                return { [OPEN_TYPE]: 'Uint16Array', value: Array.from(v) };
                            }
                            if (v instanceof Uint32Array) {
                                return { [OPEN_TYPE]: 'Uint32Array', value: Array.from(v) };
                            }
                            if (v instanceof BigUint64Array) {
                                return {
                                    [OPEN_TYPE]: 'BigUint64Array',
                                    value: Array.from(v).map((n) => `${n}`),
                                };
                            }
                            if (v instanceof Int8Array) {
                                return { [OPEN_TYPE]: 'Int8Array', value: Array.from(v) };
                            }
                            if (v instanceof Int16Array) {
                                return { [OPEN_TYPE]: 'Int16Array', value: Array.from(v) };
                            }
                            if (v instanceof Int32Array) {
                                return { [OPEN_TYPE]: 'Int32Array', value: Array.from(v) };
                            }
                            if (v instanceof BigInt64Array) {
                                return {
                                    [OPEN_TYPE]: 'BigInt64Array',
                                    value: Array.from(v).map((n) => `${n}`),
                                };
                            }
                            if (v instanceof ArrayBuffer) {
                                const buffer = v as ArrayBuffer;
                                const view = new DataView(buffer);
                                const value: number[] = [];
                                for (let i = 0; i < buffer.byteLength; i++) value.push(view.getUint8(i));
                                return { [OPEN_TYPE]: 'ArrayBuffer', value };
                            }
                            if (v['_arr'] && v['_isPrincipal']) {
                                return { [OPEN_TYPE]: 'Principal', value: v.toText() };
                            }
                            if (v['__principal__']) {
                                return { [OPEN_TYPE]: 'Principal', value: v['__principal__'] };
                            }
                            break;
                        }
                        case 'function': {
                            throw new Error(`can not stringify function`);
                        }
                    }
                    return v;
                }),
            space,
        );
    };
};
