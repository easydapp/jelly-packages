import { stringify_factory } from '@jellypack/types/lib/open/open-json';

import { handle_wasm_code_result } from '../../../wasm';

export const doExecuteByRemote = async (code: string, args: [string, any][], debug: boolean): Promise<any> => {
    const stringify = stringify_factory(JSON.stringify);

    const args_mapping = args.map(([name, value]) => [name, value === undefined ? '' : stringify(value)]);
    const args_json = JSON.stringify(args_mapping);

    if (debug) {
        console.debug('remote args mapping: ', args_mapping);
        console.debug('remote args [code and json]: ', [code, args_json]);
    }

    const s = Date.now();
    let value: any = await fetch_remote(code, args_json, debug);
    const e = Date.now();
    if (debug) console.debug('remote spend', e - s, 'ms', [code, args_json]);

    if (debug) {
        console.debug('remote execute result [stringify]: ', [value]);
    }

    value = handle_wasm_code_result(value);

    if (debug) {
        console.debug('remote execute result [real]: ', [value]);
    }

    return value;
};

// const executor_url = `https://wasm-api.easydapp.ai/execute_code`;
const executor_url = `https://p.easydapp.ai/execute_code`;
const fetch_remote = async (code: string, data: string, debug: boolean): Promise<string> => {
    const response = await fetch(executor_url, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code, data }),
    });
    const result: {
        code: number;
        created: string;
        message: string;
        data?: {
            spend: number;
            result: string;
        };
    } = await response.json();

    if (result.data === undefined) {
        throw new Error(result.message);
    }

    if (debug) {
        console.debug('remote result: ', result.data.spend, result);
    }

    return result.data.result;
};
