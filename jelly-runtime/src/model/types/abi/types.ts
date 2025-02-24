import { AbiParam } from '.';

const evm_hash_check = (value: string, length = 20): boolean => {
    return new RegExp(`^0[x|X][0-9a-fA-F]{${length * 2}}$`).test(value);
};
export const evm_address_check = (address: string): boolean => {
    return evm_hash_check(address, 20);
};
export const evm_func_check = (address: string): boolean => {
    return evm_hash_check(address, 24);
};
export const evm_tx_check = (tx: string): boolean => {
    return evm_hash_check(tx, 32);
};
export const evm_bytecode_check = (bytecode: string): boolean => {
    return /^0[x|X]([0-9a-fA-F][0-9a-fA-F])+$/.test(bytecode) && bytecode.length % 2 === 0;
};

export const evm_value_exp = (value: string, exp: number): string => {
    const s = value.split('.');
    if (2 < s.length) return '';
    if (1 < s.length && exp < s[1].length) return '';
    if (!value.includes('.')) value += '.'; // Add without a decimal point
    while (value.split('.')[1].length < exp) value += '0'; // fix 0
    value = value.replace('.', ''); // Remove the decimal point
    while (value.startsWith('0')) value = value.substring(1);
    let v;
    try {
        v = BigInt(value);
    } catch (e) {
        console.debug(`🚀 ~ e:`, e);
        return '';
    }
    if (`${v}` !== value) return '';
    return value;
};

/**
 * boolean -> bool -> boolean ✅
 * bigint -> int -> bigint
 * ✅ bigint -> uint -> bigint ✅
 * string -> fixed -> string
 * string -> ufixed -> string
 * ✅ string -> address -> string
 * ✅ (string | Uint8Array) -> bytes -> (string | Uint8Array) ✅
 * string -> string -> string ✅
 * string -> function -> string
 * any*[] -> array -> any*[] ✅
 * Record<any*, any*> -> mapping -> Record<any*, any*>
 * ✅ [any*,..] -> tuple -> [any*,..]
 */

export type EvmType =
    // Boolean
    | 'bool'
    // Integer
    | {
          int: {
              type: string; // int | int8 .. int256
              bits: number; // 8 .. 256
          };
      }
    | {
          uint: {
              type: string; // uint | uint8 .. uint256
              bits: number; // 8 .. 256
          };
      }
    // Fixed floating point type
    | {
          fixed: {
              type: string; // fixed | fixedMxN
              m: number; // 8 .. 256
              n: number; // 0 .. 80
          };
      }
    | {
          ufixed: {
              type: string; // ufixed | ufixedMxN
              m: number; // 8 .. 256
              n: number; // 0 .. 80
          };
      }
    // address
    | {
          address: {
              type: 'address' | 'address payable';
              payable: boolean;
          };
      }
    // Fixed-length byte array
    | {
          bytes: {
              type: string;
              length?: number; // Maybe it is not sure
          };
      }
    // Changing byte array
    | 'string'
    // Functional type
    | 'function'
    // Array
    | {
          array: {
              type: EvmType;
              length?: number; // Maybe it is not sure
          };
      }
    // Mapping
    | {
          mapping: {
              key: EvmType;
              value: EvmType;
          };
      }
    // Tuple
    | {
          tuple: EvmType[];
      };

export const match_evm_type = <T>(
    self: EvmType,
    {
        bool,
        int,
        uint,
        fixed,
        ufixed,
        address,
        bytes,
        string,
        func,
        array,
        mapping,
        tuple,
    }: {
        bool: () => T;
        int: (int: {
            type: string; // int | int8 .. int256
            bits: number; // 8 .. 256
        }) => T;
        uint: (uint: {
            type: string; // uint | uint8 .. uint256
            bits: number; // 8 .. 256
        }) => T;
        fixed: (fixed: {
            type: string; // fixed | fixedMxN
            m: number; // 8 .. 256
            n: number; // 0 .. 80
        }) => T;
        ufixed: (ufixed: {
            type: string; // ufixed | ufixedMxN
            m: number; // 8 .. 256
            n: number; // 0 .. 80
        }) => T;
        address: (address: { type: 'address' | 'address payable'; payable: boolean }) => T;
        bytes: (bytes: {
            type: string;
            length?: number; // Maybe it is not sure
        }) => T;
        string: () => T;
        func: () => T;
        array: (array: {
            type: EvmType;
            length?: number; // Maybe it is not sure
        }) => T;
        mapping: (mapping: { key: EvmType; value: EvmType }) => T;
        tuple: (tuple: EvmType[]) => T;
    },
) => {
    if (self === 'bool') return bool();
    if (self === 'string') return string();
    if (self === 'function') return func();
    if (typeof self === 'object') {
        if ('int' in self) return int(self.int);
        if ('uint' in self) return uint(self.uint);
        if ('fixed' in self) return fixed(self.fixed);
        if ('ufixed' in self) return ufixed(self.ufixed);
        if ('address' in self) return address(self.address);
        if ('bytes' in self) return bytes(self.bytes);
        if ('array' in self) return array(self.array);
        if ('mapping' in self) return mapping(self.mapping);
        if ('tuple' in self) return tuple(self.tuple);
    }
    throw new Error('invalid evm type');
};

export const evm_param_to_type = (param: AbiParam): EvmType => {
    const ty = param.type.trim();
    if (ty.endsWith(']')) {
        const index = ty.lastIndexOf('[');
        const type = ty.substring(0, index);
        const p: AbiParam = {
            name: param.name,
            type,
            internalType: type,
            components: param.components,
        };
        const length = ty.substring(index + 1, ty.length - 1);
        return {
            array: {
                type: evm_param_to_type(p),
                length: length === '' ? undefined : Number(length),
            },
        };
    } else if (ty.startsWith('mapping')) {
        if (param.type.split('=>').length != 2) {
            throw new Error(`invalid mapping type: ${param.type}`);
        }
        const types = param.type
            .substring(8, param.type.length - 1)
            .split('=>')
            .map((s) => s.trim());
        const p1: AbiParam = {
            name: param.name,
            type: types[0],
            internalType: types[0],
            components: param.components,
        };
        const p2: AbiParam = {
            name: param.name,
            type: types[1],
            internalType: types[1],
            components: param.components,
        };
        return {
            mapping: {
                key: evm_param_to_type(p1),
                value: evm_param_to_type(p2),
            },
        };
    } else {
        if (ty === 'bool') return 'bool';

        if (ty.startsWith('int')) {
            const bits = ty.substring(3);
            return {
                int: {
                    type: ty,
                    bits: bits ? Number(bits) : 256,
                },
            };
        }
        if (ty.startsWith('uint')) {
            const bits = ty.substring(4);
            return {
                uint: {
                    type: ty,
                    bits: bits ? Number(bits) : 256,
                },
            };
        }

        if (ty === 'address')
            return { address: { type: ty, payable: param.internalType === 'address payable' } };
        if (ty === 'address payable') return { address: { type: ty, payable: true } };

        if (ty.startsWith('bytes')) {
            const length = ty.substring(5);
            return {
                bytes: {
                    type: ty,
                    length: length ? Number(length) : undefined,
                },
            };
        }
        if (ty === 'string') return 'string';

        if (ty === 'tuple') {
            const types: EvmType[] = [];
            for (const component of param.components ?? []) {
                types.push(evm_param_to_type(component));
            }
            return {
                tuple: types,
            };
        }

        throw new Error(`unsupported evm type: ${ty}`);
    }
    throw new Error(`invalid evm type: ${ty}`);
};

export const checkEvmValue = (type: EvmType, value: any): boolean => {
    return match_evm_type(type, {
        bool: () => typeof value === 'boolean',
        int: () => typeof value === 'bigint',
        uint: () => typeof value === 'bigint' && 0 <= BigInt(value),
        fixed: () => typeof value === 'string' && `${BigInt(value)}` === value,
        ufixed: () => typeof value === 'string' && `${BigInt(value)}` === value,
        address: () => typeof value === 'string' && evm_address_check(value),
        bytes: (bytes) => {
            if (typeof value === 'string') {
                return (
                    evm_bytecode_check(value) &&
                    (!bytes.length || (value.length - 2) / 2 === bytes.length)
                );
            }
            return value instanceof Uint8Array && (!bytes.length || value.length === bytes.length);
        },
        string: () => typeof value === 'string',
        func: () => typeof value === 'string' && evm_func_check(value),
        array: (array) =>
            Array.isArray(value) &&
            value.every((v) => checkEvmValue(array.type, v)) &&
            (!array.length || value.length === array.length),
        mapping: (mapping) => {
            for (const key in value) {
                if (!checkEvmValue(mapping.key, key)) return false;
                if (!checkEvmValue(mapping.value, value[key])) return false;
            }
            return true;
        },
        tuple: (tuple) => {
            for (let i = 0; i < tuple.length; i++) {
                if (!checkEvmValue(tuple[i], value[i])) return false;
            }
            return true;
        },
    });
};
