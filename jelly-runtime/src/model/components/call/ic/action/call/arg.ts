import { deepClone } from '../../../../../../common/clones';
import { RuntimeValues } from '../../../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { WrappedCandidType } from '../../../../../../wasm/candid';
import { CodeContent } from '../../../../../common/code';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { code_value_get_used_component, CodeValue } from '../../../../../common/refer';
import { doFunctionTransformByCodeContent } from '../../../../code';

export interface IcCallArgCode {
    data?: CodeValue[];
    code: CodeContent;
}

export const ic_call_arg_code_get_used_component = (self: IcCallArgCode): ComponentId[] => {
    const used: ComponentId[] = [];
    for (const code_value of self.data ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    return used;
};

export interface IcCallArg {
    code: IcCallArgCode;
}

export const match_ic_call_arg = <T>(self: IcCallArg, { code }: { code: (code: IcCallArgCode) => T }): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid ic call arg');
};

export const match_ic_call_arg_async = async <T>(
    self: IcCallArg,
    { code }: { code: (code: IcCallArgCode) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid ic call arg');
};

export const ic_call_arg_get_used_component = (self: IcCallArg): ComponentId[] => {
    const used: ComponentId[] = [];
    match_ic_call_arg(self, {
        code: (code) => used.push(...ic_call_arg_code_get_used_component(code)),
    });
    return used;
};

export const parse_ic_call_action_arg = async (
    self: IcCallArg | undefined,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    arg: WrappedCandidType[],
    handle: (data_of_args: any) => void,
    code_executor: CodeExecutor | undefined,
) => {
    const unwrapped: any[] = [];
    switch (arg.length) {
        case 0: {
            /// no arg
            break;
        }
        case 1: {
            /// single arg
            if (self) {
                // get arg
                const flag = await match_ic_call_arg_async(self, {
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
            } else {
                unwrapped.push([]); // opt -> null -> []
            }
            break;
        }
        default: {
            if (self) {
                // get args
                const flag = await match_ic_call_arg_async(self, {
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
            } else {
                for (const _ of arg) {
                    unwrapped.push([]); // opt -> null -> []
                }
            }
        }
    }
    return unwrapped;
};
