import { deepClone } from '../../../../../../common/clones';
import { RuntimeValues } from '../../../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { CodeContent } from '../../../../../common/code';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { code_value_get_used_component, CodeValue } from '../../../../../common/refer';
import { AbiItem } from '../../../../../types/abi';
import { checkEvmValue, evm_param_to_type } from '../../../../../types/abi/types';
import { doFunctionTransformByCodeContent } from '../../../../code';

export interface EvmCallArgCode {
    data?: CodeValue[];
    code: CodeContent;
}

export const evm_call_arg_code_get_used_component = (self: EvmCallArgCode): ComponentId[] => {
    const used: ComponentId[] = [];
    for (const code_value of self.data ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    return used;
};

export interface EvmCallArg {
    code: EvmCallArgCode;
}

export const match_evm_call_arg = <T>(
    self: EvmCallArg,
    { code }: { code: (code: { data?: CodeValue[]; code: CodeContent }) => T },
): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm call arg');
};

export const match_evm_call_arg_async = async <T>(
    self: EvmCallArg,
    { code }: { code: (code: { data?: CodeValue[]; code: CodeContent }) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm call arg');
};

export const evm_call_arg_get_used_component = (self: EvmCallArg): ComponentId[] => {
    const used: ComponentId[] = [];
    match_evm_call_arg(self, {
        code: (code) => used.push(...evm_call_arg_code_get_used_component(code)),
    });
    return used;
};

export const check_evm_args = async (
    item: AbiItem,
    arg: EvmCallArg | undefined,
    runtime_values: RuntimeValues,
    endpoints: AllEndpoints | undefined,
    codes: Record<CodeDataAnchor, CodeData>,
    handle: (data_of_args: any) => void,
    code_executor: CodeExecutor | undefined,
): Promise<any[] | undefined> => {
    // get argument
    const unwrapped: any[] = [];
    switch (item.inputs?.length ?? 0) {
        case 0:
            /// no arg
            break;
        case 1: {
            /// single arg
            if (!arg) throw new Error('arg is missing');
            // get arg
            const flag = await match_evm_call_arg_async(arg, {
                code: async (code) => {
                    const data = runtime_values.find_data(endpoints, code.data ?? []);
                    if (data === undefined) return undefined;
                    handle(deepClone(data));
                    const value = await doFunctionTransformByCodeContent(
                        code.code,
                        codes,
                        [['data', data]],
                        code_executor,
                    );
                    unwrapped.push(value); // returns directly
                    return unwrapped;
                },
            });
            if (flag === undefined) return undefined; // failed
            break;
        }
        default: {
            if (!arg) throw new Error('arg is missing');
            // get args
            const flag = await match_evm_call_arg_async(arg, {
                code: async (code) => {
                    const data = runtime_values.find_data(endpoints, code.data ?? []);
                    if (data === undefined) return undefined;
                    handle(deepClone(data));
                    const value = await doFunctionTransformByCodeContent(
                        code.code,
                        codes,
                        [['data', data]],
                        code_executor,
                    );
                    for (const v of value) unwrapped.push(v);
                    return unwrapped;
                },
            });
            if (flag === undefined) return undefined; // failed
        }
    }

    // Do you check whether the parameter matches the corresponding type?
    if (item.inputs) {
        for (let i = 0; i < item.inputs.length; i++) {
            const type = evm_param_to_type(item.inputs[i]);
            if (!checkEvmValue(type, unwrapped[i])) {
                console.error('evm value not match', type, unwrapped[i]);
                throw new Error(
                    `evm value not match: ${JSON.stringify(type)} -> ${JSON.stringify(unwrapped[i], (_, v) => (typeof v === 'bigint' ? v.toString() : v))}`,
                );
            }
        }
    }
    return unwrapped;
};
