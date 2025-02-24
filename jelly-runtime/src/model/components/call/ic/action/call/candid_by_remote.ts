import { MotokoResult } from '@choptop/haw';
import { deepClone } from '../../../../../../common/clones';
import { sha256 } from '../../../../../../common/hash';
import { WrappedCandidTypeFunction, WrappedCandidTypeService } from '../../../../../../wasm/candid';

const CACHED: Record<string, string> = {};

const fetch_remote_by_candid = async (candid: string, debug: boolean): Promise<string> => {
    const hash = `candid:${await sha256(candid)}`;
    const cached = CACHED[hash];
    if (cached !== undefined) return deepClone(cached); // ? cached

    const response = await fetch(`https://wasm-api.easydapp.ai/parse_service_candid`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ candid }),
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

    CACHED[hash] = deepClone(result.data.result); // ? cached

    return result.data.result;
};

const fetch_remote_by_func = async (func: string, debug: boolean): Promise<string> => {
    const hash = `func:${await sha256(func)}`;
    const cached = CACHED[hash];
    if (cached !== undefined) return deepClone(cached); // ? cached

    const response = await fetch(`https://wasm-api.easydapp.ai/parse_func_candid`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ func }),
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

    CACHED[hash] = deepClone(result.data.result); // ? cached

    return result.data.result;
};

export const parse_service_candid_by_remote = async <T>(
    candid: string,
    mapping: (service: WrappedCandidTypeService) => T,
    debug: boolean,
): Promise<T> => {
    if (debug) {
        console.debug('origin [candid]: ', [candid]);
    }

    const s = Date.now();
    let value: any = await fetch_remote_by_candid(candid, debug);
    const e = Date.now();
    if (debug) console.debug('parse_service_candid wasm spend', e - s, 'ms', [candid]);

    if (debug) {
        console.debug('candid result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('candid result real: ', value);
    }

    if (value !== undefined) {
        value = mapping(value);

        if (debug) {
            console.debug('candid result mapping: ', value);
        }
    }

    return value;
};

export const parse_func_candid_by_remote = async <T>(
    func: string,
    mapping: (func: [string, WrappedCandidTypeFunction]) => T,
    debug: boolean,
): Promise<T> => {
    if (debug) {
        console.debug('origin func [candid]: ', [func]);
    }

    const s = Date.now();
    let value: any = fetch_remote_by_func(func, debug);
    const e = Date.now();
    if (debug) console.debug('parse_func_candid wasm spend', e - s, 'ms', [func]);

    if (debug) {
        console.debug('func candid result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else {
            value = JSON.parse(result.ok);

            if (debug) {
                console.debug('func candid result service: ', value);
            }

            value = (value.methods ?? [])[0];
        }
    }

    if (debug) {
        console.debug('func candid result real: ', value);
    }

    if (value !== undefined) {
        value = mapping(value);

        if (debug) {
            console.debug('func candid result mapping: ', value);
        }
    }

    return value;
};
