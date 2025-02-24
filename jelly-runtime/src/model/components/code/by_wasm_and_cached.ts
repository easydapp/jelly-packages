import { stringify_factory } from '@jellypack/types/lib/open/open-json';
import { deepClone } from '../../../common/clones';
import { sha256 } from '../../../common/hash';
import { CodeExecutor } from '../../../wasm';

const stringify = stringify_factory(JSON.stringify);

const CACHED: Record<string, any> = {};

export const doExecuteByWasmAndCachedFactory = (execute_code: CodeExecutor): CodeExecutor => {
    return async (code: string, args: [string, any][], debug: boolean): Promise<any> => {
        const hash = await sha256(stringify({ code, args }));

        let cached = CACHED[hash];
        if (cached !== undefined) return deepClone(cached);

        cached = await execute_code(code, args, debug);
        if (cached === undefined) return cached;

        CACHED[hash] = deepClone(cached);

        return cached;
    };
};
