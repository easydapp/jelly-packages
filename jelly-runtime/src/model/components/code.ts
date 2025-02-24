import { LinkType } from '@jellypack/types/lib/types';
import { deepClone } from '../../common/clones';
import { RuntimeValues } from '../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../store/code';
import { CodeContent, match_code_content } from '../common/code';
import { ComponentId } from '../common/identity';
import { AllEndpoints, Endpoint } from '../common/lets';
import { code_value_get_used_component, CodeValue } from '../common/refer';
// import { doExecuteByEvalToModuleDirectly } from './code/by_eval_to_module';
// import { doExecuteByFunctionDirectly } from './code/by_function';
// import { doExecuteBySaferEvalDirectly } from './code/by_safer_eval';
// import { doExecuteBySaferEvalAndFunction } from './code/by_safer_eval_and_function';
// import { doExecuteByWasm } from './code/by_wasm';
// import { doExecuteByWasmAndCached } from './code/by_wasm_and_cached';
// import { doExecuteByRemote } from './code/by_remote';
import { doExecuteByRemoteAndCached } from './code/by_remote_and_cached';

export interface ComponentCode {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: CodeMetadata;
    output: LinkType;
}

export interface CodeMetadata {
    data?: CodeValue[];
    code: CodeContent;
}

export const component_code_get_used_component = (self: ComponentCode): ComponentId[] => {
    return (self.metadata.data ?? []).flatMap((code_value) =>
        code_value_get_used_component(code_value),
    );
};

export const get_code_value = async (
    self: ComponentCode,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    code_executor = doExecuteByRemoteAndCached,
): Promise<any> => {
    // 1. get arguments
    const data = runtime_values.find_data(endpoints, self.metadata.data ?? []);
    if (data === undefined) return undefined;
    // 2. execute
    let value = undefined;
    try {
        value = await doFunctionTransformByCodeContent(
            self.metadata.code,
            codes,
            [['data', data]],
            code_executor,
        );
    } catch (e) {
        console.error('execute code failed', e);
    }
    // 3. return
    return value;
};

export const doFunctionTransformByCodeContent = async (
    code_content: CodeContent,
    codes: Record<CodeDataAnchor, CodeData>,
    args: [string, any][],
    code_executor = doExecuteByRemoteAndCached,
): Promise<any> => {
    const code = match_code_content(code_content, {
        code: ({ js }) => js,
        anchor: (anchor) => {
            const code = codes[anchor];
            if (!code) throw new Error(`can not find code data by anchor: ${anchor}`);
            return code.js;
        },
    });
    return await doFunctionTransform(code, deepClone(args), code_executor);
};

export const doFunctionTransform = async (
    code: string,
    args: [string, any][], // real value // * note: BigInt and Principal type
    code_executor = doExecuteByRemoteAndCached,
): Promise<any> => {
    let result: any;

    const debug = false;
    if (debug) console.group(`do function transform: ${code}`);
    if (debug) console.debug('args:', args);

    try {
        // ? eval
        // const value = await doExecuteByEvalToModuleDirectly(code, args, debug);
        // ? Function
        // const value = await doExecuteByFunctionDirectly(code, args, debug);
        // ? SaferEval
        // const value = await doExecuteBySaferEvalDirectly(code, args, debug);
        // ? SaferEval and Function most safe way
        // const value = await doExecuteBySaferEvalAndFunction(code, args, debug);
        // ? Wasm
        // const value = await doExecuteByWasm(code, args, debug);
        // ? Wasm and cached
        // const value = await doExecuteByWasmAndCached(code, args, debug);
        // ? Remote
        // const value = await doExecuteByRemote(code, args, debug);
        // ? Remote and cached
        // const value = await doExecuteByRemoteAndCached(code, args, debug);

        // ? form upper
        const s = Date.now();
        const value = await code_executor(code, args, debug);
        const e = Date.now();
        const spend = e - s;
        if (30 < spend) {
            console.debug('doFunctionTransform spend', spend, 'ms', [code, args, code_executor]);
        }

        if (debug) console.log('result:', value);

        // value can not be undefined
        if (value === undefined)
            throw new Error('The result of function transform can not be undefined. code: ' + code);

        // return result
        result = value; // wrap value
    } catch (e) {
        console.error('doFunctionTransform failed', code, args, e);
        throw e;
    }

    if (debug) console.groupEnd();

    return result;
};
