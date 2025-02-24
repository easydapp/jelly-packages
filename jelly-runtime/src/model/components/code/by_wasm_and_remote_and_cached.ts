import { stringify_factory } from '@jellypack/types/lib/open/open-json';
import { deepClone } from '../../../common/clones';
import { sha256 } from '../../../common/hash';
import { CodeExecutor } from '../../../wasm';

const stringify = stringify_factory(JSON.stringify);

const CACHED: Record<string, any> = {};

export const doExecuteByWasmAndRemoteAndCachedFactory = (
    wasm_execute_code: CodeExecutor,
    remote_execute_code: CodeExecutor,
): CodeExecutor => {
    return async (code: string, args: [string, any][], debug: boolean): Promise<any> => {
        const hash_text = stringify({ code, args });
        const hash = await sha256(hash_text);

        let cached = CACHED[hash];
        if (cached !== undefined) return deepClone(cached);

        if (1024 * 64 < hash_text.length) {
            cached = await remote_execute_code(code, args, debug);
        } else {
            cached = await wasm_execute_code(code, args, debug);
        }

        if (cached === undefined) return cached;

        CACHED[hash] = deepClone(cached);

        return cached;
    };
};
