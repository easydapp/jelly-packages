import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { CodeContent } from '../../../../../common/code';
import { AbiItem, AbiParam } from '../../../../../types/abi';
import { checkEvmValue, evm_param_to_type } from '../../../../../types/abi/types';
import { doFunctionTransformByCodeContent } from '../../../../code';

export interface EvmCallRet {
    code: CodeContent;
}

export const match_evm_call_ret = <T>(self: EvmCallRet, { code }: { code: (code: CodeContent) => T }): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm call ret');
};

export const match_evm_call_ret_async = async <T>(
    self: EvmCallRet,
    { code }: { code: (code: CodeContent) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm call ret');
};

const covert2link_type = (param: AbiParam, value: any): any => {
    const type = evm_param_to_type(param);
    if (!checkEvmValue(type, value)) {
        console.error('evm value not match', type, value);
        throw new Error('evm value not match ' + type);
    }
    return value;
};

// If a single result, you need to extract it
export const check_evm_outputs = (item: AbiItem, result: any) => {
    let response;
    if (item.outputs !== undefined) {
        if (item.outputs.length === 1) {
            response = covert2link_type(item.outputs[0], result);
        } else {
            // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
            response = (result as any[]).map((v, index) => covert2link_type(item.outputs![index], v));
        }
    } else {
        response = [];
    }
    return response;
};

export const check_evm_ret = async (
    ret: EvmCallRet | undefined,
    response: any,
    codes: Record<CodeDataAnchor, CodeData>,
    unwrapped: any[] | undefined,
    data_of_args: any | undefined,
    code_executor: CodeExecutor | undefined,
) => {
    let result: any = undefined;
    // const results = parseTypes(func.retTypes);
    if (ret) {
        result = await match_evm_call_ret_async(ret, {
            code: async (code) => {
                return await doFunctionTransformByCodeContent(
                    code,
                    codes,
                    [
                        ['data', response],
                        ['args', unwrapped],
                        ['data_of_args', data_of_args],
                    ],
                    code_executor,
                );
            },
        });
    } else {
        result = response; // don't need a tuple if it's a single result
    }
    return result;
};
