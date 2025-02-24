import { CodeExecutor } from '../../../wasm';

export const doExecuteByWasmFactory = (execute_code: CodeExecutor): CodeExecutor => {
    return async (code: string, args: [string, any][], debug: boolean): Promise<any> => {
        return execute_code(code, args, debug);
    };
};
