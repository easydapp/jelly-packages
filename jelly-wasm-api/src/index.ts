import { MotokoResult } from '@choptop/haw';
import { LinkComponent } from '@jellypack/runtime/lib/model/components';
import { TrimmedNode } from '@jellypack/runtime/lib/model/node';
import {
    ApisCheckFunction,
    CheckedAnchors,
    CheckedCodeItem,
    CheckedCombined,
    handle_wasm_code_result,
} from '@jellypack/runtime/lib/wasm';
import {
    WrappedCandidType,
    WrappedCandidTypeFunction,
    WrappedCandidTypeService,
} from '@jellypack/runtime/lib/wasm/candid';
import { stringify_factory } from '@jellypack/types/lib/open/open-json';
import { LinkValue } from '@jellypack/types/lib/values';
import init, * as wasm from '@jellypack/wasm';
import wasmURL from '@jellypack/wasm/wasm_bg.wasm?url';

// https://juejin.cn/post/7187279730264473656

let initializing: Promise<any>;

try {
    // initializing = init(); // website and chrome extension and deny wasmURL

    initializing = init(wasmURL); // dev and website

    // initializing = init('../../../node_modules/@jellypack/wasm/wasm_bg.wasm'); // dev
} catch (e) {
    // noop
    console.debug(`🚀 ~ initializing jellypack-wasm:`, e);
}

// ================ code ================

export const execute_code = async (code: string, args: [string, any][], debug: boolean): Promise<any | undefined> => {
    await initializing;

    const stringify = stringify_factory(JSON.stringify);

    const args_mapping = args.map(([name, value]) => [name, value === undefined ? '' : stringify(value)]);
    const args_json = JSON.stringify(args_mapping);

    if (debug) {
        console.debug('args mapping: ', args_mapping);
        console.debug('args [code and json]: ', [code, args_json]);
    }

    const s = Date.now();
    let value: any = wasm.execute_code(code, args_json);
    const e = Date.now();
    const spend = e - s;
    if (debug || 30 < spend) console.debug('execute_code wasm spend', spend, 'ms', [code, args]);

    if (debug) {
        console.debug('execute result [stringify]: ', [value]);
    }

    value = handle_wasm_code_result(value);

    if (debug) {
        console.debug('execute result [real]: ', [value]);
    }

    return value;
};

export const execute_validate_code = async (
    code: string,
    link_value: LinkValue,
    debug: boolean,
): Promise<string | undefined> => {
    await initializing;

    const stringify = stringify_factory(JSON.stringify);

    const value_json = stringify(link_value);

    if (debug) {
        console.debug('value [code and json]: ', [code, value_json]);
    }

    const s = Date.now();
    let value: any = wasm.execute_validate_code(code, value_json);
    const e = Date.now();
    if (debug) console.debug('execute_validate_code wasm spend', e - s, 'ms', [code, value_json]);

    if (debug) {
        console.debug('validate result [stringify]: ', [value]);
    }

    value = handle_wasm_code_result(value);

    if (debug) {
        console.debug('validate result [real]: ', [value]);
    }

    return value;
};

// ================ candid ================

export const parse_service_candid = async <T>(
    candid: string,
    mapping: (service: WrappedCandidTypeService) => T,
    debug: boolean,
): Promise<T> => {
    await initializing;

    if (debug) {
        console.debug('origin [candid]: ', [candid]);
    }

    const s = Date.now();
    let value: any = wasm.parse_service_candid(candid);
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

export const parse_func_candid = async <T>(
    func: string,
    mapping: (func: [string, WrappedCandidTypeFunction]) => T,
    debug: boolean,
): Promise<T> => {
    await initializing;

    if (debug) {
        console.debug('origin func [candid]: ', [func]);
    }

    const s = Date.now();
    let value: any = wasm.parse_func_candid(func);
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

export const parse_candid_type_to_text = async <T>(ty: WrappedCandidType, debug: boolean): Promise<T> => {
    await initializing;

    if (debug) {
        console.debug('origin type [candid]: ', [ty]);
    }

    const s = Date.now();
    let value: any = wasm.parse_candid_type_to_text(JSON.stringify(ty));
    const e = Date.now();
    if (debug) console.debug('parse_func_candid wasm spend', e - s, 'ms', [ty]);

    if (debug) {
        console.debug('func candid result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = result.ok;
    }

    if (debug) {
        console.debug('func candid result real: ', value);
    }

    return value;
};

// ================ check ================

export const find_all_anchors = async (components: LinkComponent[], debug: boolean): Promise<CheckedAnchors> => {
    await initializing;

    if (debug) {
        console.debug('find_all_anchors components: ', components);
    }

    const components_json = JSON.stringify(components);

    if (debug) {
        console.debug('find_origin_codes components [stringify]: ', [components_json]);
    }

    const s = Date.now();
    let value: any = wasm.find_all_anchors(components_json);
    const e = Date.now();
    if (debug) console.debug('find_all_anchors wasm spend', e - s, 'ms', [components_json]);

    if (debug) {
        console.debug('find_all_anchors result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('find_all_anchors result real: ', value);
    }

    return value;
};

export const find_origin_codes = async (
    components: LinkComponent[],
    fetch: ApisCheckFunction,
    debug: boolean,
): Promise<CheckedCodeItem[]> => {
    await initializing;

    if (debug) {
        console.debug('find_origin_codes components: ', components, fetch);
    }

    const components_json = JSON.stringify(components);
    const fetch_json = JSON.stringify(fetch);

    if (debug) {
        console.debug('find_origin_codes components [stringify]: ', [components_json, fetch_json]);
    }

    const s = Date.now();
    let value: any = wasm.find_origin_codes(components_json, fetch_json);
    const e = Date.now();
    if (debug) console.debug('find_origin_codes wasm spend', e - s, 'ms', [components_json, fetch_json]);

    if (debug) {
        console.debug('find_origin_codes result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('find_origin_codes result real: ', value);
    }

    return value;
};

export const find_template_origin_codes = async (nodes: TrimmedNode[], debug: boolean): Promise<CheckedCodeItem[]> => {
    await initializing;

    if (debug) {
        console.debug('find_template_origin_codes nodes: ', nodes, fetch);
    }

    const nodes_json = JSON.stringify(nodes);

    if (debug) {
        console.debug('find_template_origin_codes nodes [stringify]: ', [nodes_json]);
    }

    const s = Date.now();
    let value: any = wasm.find_template_origin_codes(nodes_json);
    const e = Date.now();
    if (debug) console.debug('find_template_origin_codes wasm spend', e - s, 'ms', [nodes_json]);

    if (debug) {
        console.debug('find_template_origin_codes result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('find_template_origin_codes result real: ', value);
    }

    return value;
};

export const check = async (
    components: LinkComponent[],
    fetch: ApisCheckFunction,
    debug: boolean,
): Promise<CheckedCombined> => {
    await initializing;

    if (debug) {
        console.debug('check components: ', [components, fetch]);
    }

    const components_json = JSON.stringify(components);
    const fetch_json = JSON.stringify(fetch);

    if (debug) {
        console.debug('check components [stringify]: ', [components_json, fetch_json]);
    }

    const s = Date.now();
    let value: any = wasm.check(components_json, fetch_json);
    const e = Date.now();
    if (debug) console.debug('check wasm spend', e - s, 'ms', [components_json, fetch_json]);

    if (debug) {
        console.debug('check result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('check result real: ', value);
    }

    return value;
};

export const check_template = async (
    nodes: TrimmedNode[],
    checked: CheckedCombined,
    fetch: ApisCheckFunction,
    debug: boolean,
): Promise<number> => {
    await initializing;

    if (debug) {
        console.debug('check_template nodes: ', [nodes, checked, fetch]);
    }

    const nodes_json = JSON.stringify(nodes);
    const checked_json = JSON.stringify(checked);
    const fetch_json = JSON.stringify(fetch);

    if (debug) {
        console.debug('check_template components [stringify]: ', [nodes_json, checked_json, fetch_json]);
    }

    const s = Date.now();
    let value: any = wasm.check_template(nodes_json, checked_json, fetch_json);
    const e = Date.now();
    if (debug) console.debug('check_template wasm spend', e - s, 'ms', [nodes_json, checked_json, fetch_json]);

    if (debug) {
        console.debug('check_template result [stringify]: ', [value]);
    }

    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) throw new Error(result.err);
        else value = JSON.parse(result.ok);
    }

    if (debug) {
        console.debug('check_template result real: ', value);
    }

    return value;
};
