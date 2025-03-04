import type { IDL } from '@dfinity/candid';
import { Principal } from '@dfinity/principal';

export interface WrappedCandidTypeName {
    name?: string;
}

export interface WrappedCandidTypeSubtype {
    subtype: WrappedCandidType;
    name?: string;
}

export interface WrappedCandidTypeRecord {
    subitems: [string, WrappedCandidType][];
    name?: string;
}

export interface WrappedCandidTypeVariant {
    subitems: [string, WrappedCandidType | undefined][];
    name?: string;
}

export interface WrappedCandidTypeTuple {
    subitems: WrappedCandidType[];
    name?: string;
}

export type FunctionAnnotation = 'query' | 'oneway';

export interface WrappedCandidTypeFunction {
    args?: WrappedCandidType[];
    rets?: WrappedCandidType[];
    annotation?: FunctionAnnotation;
    name?: string;
}

export interface WrappedCandidTypeService {
    args?: WrappedCandidType[];
    methods?: [string, WrappedCandidTypeFunction][];
    name?: string;
}

export interface WrappedCandidTypeRecursion {
    ty: WrappedCandidType;
    id: number;
    name?: string;
}

export interface WrappedCandidTypeReference {
    id: number;
    name?: string;
}

export type WrappedCandidType =
    | { bool: WrappedCandidTypeName }
    | { nat: WrappedCandidTypeName }
    | { int: WrappedCandidTypeName }
    | { nat8: WrappedCandidTypeName }
    | { nat16: WrappedCandidTypeName }
    | { nat32: WrappedCandidTypeName }
    | { nat64: WrappedCandidTypeName }
    | { int8: WrappedCandidTypeName }
    | { int16: WrappedCandidTypeName }
    | { int32: WrappedCandidTypeName }
    | { int64: WrappedCandidTypeName }
    | { float32: WrappedCandidTypeName }
    | { float64: WrappedCandidTypeName }
    | { null: WrappedCandidTypeName }
    | { text: WrappedCandidTypeName }
    | { principal: WrappedCandidTypeName }
    | { vec: WrappedCandidTypeSubtype }
    | { opt: WrappedCandidTypeSubtype }
    | { record: WrappedCandidTypeRecord }
    | { variant: WrappedCandidTypeVariant }
    | { tuple: WrappedCandidTypeTuple }
    | { unknown: WrappedCandidTypeName }
    | { empty: WrappedCandidTypeName }
    | { reserved: WrappedCandidTypeName }
    | { func: WrappedCandidTypeFunction }
    | { service: WrappedCandidTypeService }
    | { rec: WrappedCandidTypeRecursion }
    | { ref: WrappedCandidTypeReference };

export const match_wrapped_candid_type = <T>(
    self: WrappedCandidType,
    {
        bool,
        nat,
        int,
        nat8,
        nat16,
        nat32,
        nat64,
        int8,
        int16,
        int32,
        int64,
        float32,
        float64,
        none,
        text,
        principal,
        vec,
        opt,
        record,
        variant,
        tuple,
        unknown,
        empty,
        reserved,
        func,
        service,
        rec,
        ref,
    }: {
        bool: (bool: WrappedCandidTypeName) => T;
        nat: (nat: WrappedCandidTypeName) => T;
        int: (int: WrappedCandidTypeName) => T;
        nat8: (nat8: WrappedCandidTypeName) => T;
        nat16: (nat16: WrappedCandidTypeName) => T;
        nat32: (nat32: WrappedCandidTypeName) => T;
        nat64: (nat64: WrappedCandidTypeName) => T;
        int8: (int8: WrappedCandidTypeName) => T;
        int16: (int16: WrappedCandidTypeName) => T;
        int32: (int32: WrappedCandidTypeName) => T;
        int64: (int64: WrappedCandidTypeName) => T;
        float32: (float32: WrappedCandidTypeName) => T;
        float64: (float64: WrappedCandidTypeName) => T;
        none: (none: WrappedCandidTypeName) => T;
        text: (text: WrappedCandidTypeName) => T;
        principal: (principal: WrappedCandidTypeName) => T;
        vec: (vec: WrappedCandidTypeSubtype) => T;
        opt: (opt: WrappedCandidTypeSubtype) => T;
        record: (record: WrappedCandidTypeRecord) => T;
        variant: (variant: WrappedCandidTypeVariant) => T;
        tuple: (tuple: WrappedCandidTypeTuple) => T;
        unknown: (unknown: WrappedCandidTypeName) => T;
        empty: (empty: WrappedCandidTypeName) => T;
        reserved: (reserved: WrappedCandidTypeName) => T;
        func: (func: WrappedCandidTypeFunction) => T;
        service: (service: WrappedCandidTypeService) => T;
        rec: (rec: WrappedCandidTypeRecursion) => T;
        ref: (ref: WrappedCandidTypeReference) => T;
    },
): T => {
    if ('bool' in self) return bool(self.bool);
    if ('nat' in self) return nat(self.nat);
    if ('int' in self) return int(self.int);
    if ('nat8' in self) return nat8(self.nat8);
    if ('nat16' in self) return nat16(self.nat16);
    if ('nat32' in self) return nat32(self.nat32);
    if ('nat64' in self) return nat64(self.nat64);
    if ('int8' in self) return int8(self.int8);
    if ('int16' in self) return int16(self.int16);
    if ('int32' in self) return int32(self.int32);
    if ('int64' in self) return int64(self.int64);
    if ('float32' in self) return float32(self.float32);
    if ('float64' in self) return float64(self.float64);
    if ('null' in self) return none(self.null);
    if ('text' in self) return text(self.text);
    if ('principal' in self) return principal(self.principal);

    if ('vec' in self) return vec(self.vec);
    if ('opt' in self) return opt(self.opt);
    if ('record' in self) return record(self.record);
    if ('variant' in self) return variant(self.variant);
    if ('tuple' in self) return tuple(self.tuple);

    if ('unknown' in self) return unknown(self.unknown);
    if ('empty' in self) return empty(self.empty);
    if ('reserved' in self) return reserved(self.reserved);

    if ('func' in self) return func(self.func);
    if ('service' in self) return service(self.service);
    if ('rec' in self) return rec(self.rec);
    if ('ref' in self) return ref(self.ref);

    console.error('invalid wrapped candid type', self);
    throw new Error('invalid wrapped candid type');
};

// ============================= check value =============================

const NAT_MIN = '0';
const NAT8_MAX = '255';
const NAT16_MAX = '65535';
const NAT32_MAX = '4294967295';
const NAT64_MAX = '18446744073709551615';

const INT8_MAX = '127';
const INT8_MIN = '-128';
const INT16_MAX = '32767';
const INT16_MIN = '-32768';
const INT32_MAX = '2147483647';
const INT32_MIN = '-2147483648';
const INT64_MAX = '9223372036854775807';
const INT64_MIN = '-9223372036854775808';

const isArray = (value: any): boolean => {
    if (value === undefined) return false;
    if (value === null) return false;
    const flag = Object.prototype.toString.call(value) === '[object Array]';
    if (flag) return flag;
    if (typeof value !== 'object') return false;
    // some array like {length: 1, 0: 1}, obviously it is object
    let length = (value as any)['length'];
    if (!['number', 'undefined'].includes(typeof length)) return false;
    const keys = Object.keys(value)
        .filter((key) => key !== 'length')
        .map((key) => parseInt(key));
    if (typeof length === 'number' && length !== keys.length) return false;
    length = keys.length;
    for (let i = 0; i < length; i++) {
        const index = keys.indexOf(i);
        if (index >= 0) keys.splice(index, 1);
    }
    if (keys.length) return false;
    if ((value as any)['length'] === undefined) (value as any)['length'] = length; // add length filed
    return true;
};
const isBool = (value: any): boolean => typeof value === 'boolean';
const isNat = (value: any): boolean => typeof value === 'bigint';
const isInt = (value: any): boolean => typeof value === 'bigint';
const isNat8 = (value: any): boolean =>
    typeof value === 'number' && Number(NAT_MIN) <= value && value <= Number(NAT8_MAX);
const isNat16 = (value: any): boolean =>
    typeof value === 'number' && Number(NAT_MIN) <= value && value <= Number(NAT16_MAX);
const isNat32 = (value: any): boolean =>
    typeof value === 'number' && Number(NAT_MIN) <= value && value <= Number(NAT32_MAX);
const isNat64 = (value: any): boolean => {
    if (!isNat(value)) return false;
    const v = value as bigint;
    return BigInt(`${NAT_MIN}`) <= v && v <= BigInt(NAT64_MAX);
};
const isInt8 = (value: any): boolean =>
    typeof value === 'number' && Number(INT8_MIN) <= value && value <= Number(INT8_MAX);
const isInt16 = (value: any): boolean =>
    typeof value === 'number' && Number(INT16_MIN) <= value && value <= Number(INT16_MAX);
const isInt32 = (value: any): boolean =>
    typeof value === 'number' && Number(INT32_MIN) <= value && value <= Number(INT32_MAX);
const isInt64 = (value: any): boolean => {
    if (!isInt(value)) return false;
    const v = value as bigint;
    return BigInt(INT64_MIN) <= v && v <= BigInt(INT64_MAX);
};
const isFloat32 = (value: any): boolean =>
    // typeof value === "number" && !!`${value}`.match(FLOAT32_REGEX);
    typeof value === 'number'; // ? decimal ???
const isFloat64 = (value: any): boolean =>
    // typeof value === "number" && !!`${value}`.match(FLOAT64_REGEX);
    typeof value === 'number'; // ? decimal ???
const isNull = (value: any): boolean => value === null;
const isText = (value: any): boolean => typeof value === 'string';
const isPrincipal = (value: any): boolean => {
    if (typeof value !== 'object') return false;
    const v = value as Principal;
    if (typeof v.toText !== 'function') return false;
    const text = v.toText();
    return Principal.fromText(text).toText() === text;
};
// const isBlob = (value: any): boolean => {
//     if (!isArray(value)) return false;
//     const length = (value as any)['length'];
//     if (typeof length !== 'number') return false;
//     for (let i = 0; i < length; i++) if (!isNat8((value as any)[i])) return false;
//     return true;
// };
const isVec = (value: any, subtype: WrappedCandidType, recursions: Record<number, WrappedCandidType>): boolean => {
    if (!isArray(value)) return false;
    const length = (value as any)['length'];
    if (typeof length !== 'number') return false;
    for (let i = 0; i < length; i++)
        if (!check_wrapped_candid_value(subtype, (value as any)[i], recursions)) return false;
    return true;
};
export const isOpt = (
    value: any,
    subtype: WrappedCandidType,
    recursions: Record<number, WrappedCandidType>,
): boolean => {
    if (!isArray(value)) return false;
    const length = (value as any)['length'];
    if (typeof length !== 'number') return false;
    switch (length) {
        case 0:
            return true;
        case 1:
            return check_wrapped_candid_value(subtype, (value as any)[0], recursions);
    }
    return false;
};
export const isRecord = (
    value: any,
    subitems: [string, WrappedCandidType][],
    recursions: Record<number, WrappedCandidType>,
): boolean => {
    if (typeof value !== 'object') return false;
    const valueKeys = Object.keys(value);
    if (valueKeys.length !== subitems.length) return false;
    for (const subitem of subitems) {
        if (!check_wrapped_candid_value(subitem[1], (value as any)[subitem[0]], recursions)) return false;
        valueKeys.splice(
            valueKeys.findIndex((key) => key === subitem[0]),
            1,
        );
    }
    return valueKeys.length === 0;
};
export const isVariant = (
    value: any,
    subitems: [string, WrappedCandidType | undefined][],
    recursions: Record<number, WrappedCandidType>,
): boolean => {
    if (typeof value !== 'object') return false;
    const valueKeys = Object.keys(value);
    if (valueKeys.length > 1) return false;
    if (subitems.length > 0) {
        if (valueKeys.length > 0) {
            const key = valueKeys[0]; // the only key
            const findSubitems = subitems.filter((subitem) => subitem[0] === key); // find the chosen key
            if (findSubitems.length !== 1) return false;
            const subitem = findSubitems[0];
            return check_wrapped_candid_value(subitem[1] ?? { null: {} }, (value as any)[key], recursions);
        } else {
            return false;
        }
    } else {
        if (valueKeys.length > 0) {
            return false;
        } else {
            return true;
        }
    }
};
export const isTuple = (
    value: any,
    subitems: WrappedCandidType[],
    recursions: Record<number, WrappedCandidType>,
): boolean => {
    if (!isArray(value)) return false;
    const length = (value as any)['length'];
    if (typeof length !== 'number') return false;
    if (length !== subitems.length) return false;
    for (let i = 0; i < length; i++) {
        if (!check_wrapped_candid_value(subitems[i], (value as any)[i], recursions)) return false;
    }
    return true;
};

export const check_wrapped_candid_value = (
    self: WrappedCandidType,
    value: any,
    recursions: Record<number, WrappedCandidType> = {},
): boolean => {
    if (value === undefined) return false;
    return match_wrapped_candid_type(self, {
        bool: () => isBool(value),
        nat: () => isNat(value),
        int: () => isInt(value),
        nat8: () => isNat8(value),
        nat16: () => isNat16(value),
        nat32: () => isNat32(value),
        nat64: () => isNat64(value),
        int8: () => isInt8(value),
        int16: () => isInt16(value),
        int32: () => isInt32(value),
        int64: () => isInt64(value),
        float32: () => isFloat32(value),
        float64: () => isFloat64(value),
        none: () => isNull(value),
        text: () => isText(value),
        principal: () => isPrincipal(value),
        vec: (vec) => isVec(value, vec.subtype, recursions),
        opt: (opt) => isOpt(value, opt.subtype, recursions),
        record: (record) => isRecord(value, record.subitems, recursions),
        variant: (variant) => isVariant(value, variant.subitems, recursions),
        tuple: (tuple) => isTuple(value, tuple.subitems, recursions),
        unknown: () => false,
        empty: () => false,
        reserved: () => true, // any type
        func: () => false, // ! Don't know how to see the examination
        service: () => false, // ! Don't know how to see the examination
        rec: (rec) => {
            recursions[rec.id] = rec.ty;
            return check_wrapped_candid_value(rec.ty, value, recursions);
        },
        ref: (ref) => {
            const rec_ty = recursions[ref.id];
            if (rec_ty === undefined) throw new Error('invalid ref');
            return check_wrapped_candid_value(rec_ty, value, recursions);
        },
    });
};

// ============================= mapping =============================

export const mapping_service = (
    _IDL: typeof IDL,
    service: WrappedCandidTypeService,
    rec_records: Record<number, IDL.RecClass> = {},
): { name: string; func: IDL.FuncClass }[] => {
    return (service.methods ?? []).map(([name, func]) => ({
        name,
        func: mapping_func(_IDL, func, rec_records),
    }));
};

export const mapping_func = (
    _IDL: typeof IDL,
    func: WrappedCandidTypeFunction,
    rec_records: Record<number, IDL.RecClass> = {},
): IDL.FuncClass => {
    const args = (func.args ?? []).map((arg) => mapping_type_to_idl_type(_IDL, arg, rec_records));
    const rets = (func.rets ?? []).map((ret) => mapping_type_to_idl_type(_IDL, ret, rec_records));
    return _IDL.Func(
        args,
        rets,
        (() => {
            switch (func.annotation) {
                case 'query':
                    return ['query'];
                case 'oneway':
                    return ['oneway'];
                default:
                    return [];
            }
        })(),
    );
};

export const mapping_type_to_idl_type = (
    _IDL: typeof IDL,
    ty: WrappedCandidType,
    rec_records: Record<number, IDL.RecClass> = {},
): IDL.Type => {
    return match_wrapped_candid_type<IDL.Type>(ty, {
        bool: () => _IDL.Bool,
        nat: () => _IDL.Nat,
        int: () => _IDL.Int,
        nat8: () => _IDL.Nat8,
        nat16: () => _IDL.Nat16,
        nat32: () => _IDL.Nat32,
        nat64: () => _IDL.Nat64,
        int8: () => _IDL.Int8,
        int16: () => _IDL.Int16,
        int32: () => _IDL.Int32,
        int64: () => _IDL.Int64,
        float32: () => _IDL.Float32,
        float64: () => _IDL.Float64,
        none: () => _IDL.Null,
        text: () => _IDL.Text,
        principal: () => _IDL.Principal,
        vec: (vec) => _IDL.Vec(mapping_type_to_idl_type(_IDL, vec.subtype, rec_records)),
        opt: (opt) => _IDL.Opt(mapping_type_to_idl_type(_IDL, opt.subtype, rec_records)),
        record: (record) => {
            const fields: Record<string, IDL.Type> = {};
            for (const [name, type] of record.subitems) {
                fields[name] = mapping_type_to_idl_type(_IDL, type, rec_records);
            }
            return _IDL.Record(fields);
        },
        variant: (variant) => {
            const fields: Record<string, IDL.Type> = {};
            for (const [name, type] of variant.subitems) {
                fields[name] = type ? mapping_type_to_idl_type(_IDL, type, rec_records) : _IDL.Null;
            }
            return _IDL.Variant(fields);
        },
        tuple: (tuple) =>
            _IDL.Tuple(...tuple.subitems.map((type) => mapping_type_to_idl_type(_IDL, type, rec_records))),
        unknown: () => _IDL.Unknown,
        empty: () => _IDL.Empty,
        reserved: () => _IDL.Reserved,
        func: (func) => mapping_func(_IDL, func, rec_records),
        service: (service) => {
            const methods: Record<string, IDL.FuncClass> = {};
            for (const [name, func] of service.methods ?? []) {
                methods[name] = mapping_func(_IDL, func, rec_records);
            }
            return _IDL.Service(methods);
        },
        rec: (rec) => {
            if (rec.id in rec_records) return rec_records[rec.id];
            const rec_ty = _IDL.Rec();
            rec_records[rec.id] = rec_ty;
            rec_ty.fill(mapping_type_to_idl_type(_IDL, rec.ty, rec_records));
            return rec_ty;
        },
        ref: (ref) => {
            const rec_ty = rec_records[ref.id];
            if (rec_ty === undefined) throw new Error('invalid ref');
            return rec_ty;
        },
    });
};
