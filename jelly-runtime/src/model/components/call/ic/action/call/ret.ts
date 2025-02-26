import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { WrappedCandidType } from '../../../../../../wasm/candid';
import { CodeContent } from '../../../../../common/code';
import { doFunctionTransformByCodeContent } from '../../../../code';

export interface IcCallRet {
    code: CodeContent;
}

export const match_ic_call_ret = <T>(self: IcCallRet, { code }: { code: (code: CodeContent) => T }): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid ic call ret');
};
export const match_ic_call_ret_async = async <T>(
    self: IcCallRet,
    { code }: { code: (code: CodeContent) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid ic call ret');
};

export const parse_by_ic_call_ret = async (
    self: IcCallRet,
    codes: Record<CodeDataAnchor, CodeData>,
    arg: WrappedCandidType[],
    { response, unwrapped, data_of_args }: { response: any; unwrapped: any[]; data_of_args: any | undefined },
    code_executor: CodeExecutor | undefined,
) => {
    return await match_ic_call_ret_async(self, {
        code: async (code) => {
            return await doFunctionTransformByCodeContent(
                code,
                codes,
                [
                    ['data', response],
                    ['args', arg.length === 1 ? unwrapped[0] : unwrapped],
                    ['data_of_args', data_of_args],
                ],
                code_executor,
            );
        },
    });
};
