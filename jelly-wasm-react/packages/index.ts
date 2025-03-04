import { LinkComponent } from '@jellypack/runtime/lib/model/components';
import { TrimmedNode } from '@jellypack/runtime/lib/model/node';
import { ApisCheckFunction, CheckedAnchors, CheckedCodeItem, CheckedCombined } from '@jellypack/runtime/lib/wasm';
import {
    WrappedCandidType,
    WrappedCandidTypeFunction,
    WrappedCandidTypeService,
} from '@jellypack/runtime/lib/wasm/candid';
import { LinkValue } from '@jellypack/types/lib/values';
import {
    check as _check,
    check_template as _check_template,
    execute_code as _execute_code,
    execute_validate_code as _execute_validate_code,
    find_all_anchors as _find_all_anchors,
    find_origin_codes as _find_origin_codes,
    find_template_origin_codes as _find_template_origin_codes,
    parse_candid_type_to_text as _parse_candid_type_to_text,
    parse_func_candid as _parse_func_candid,
    parse_service_candid as _parse_service_candid,
} from '@jellypack/wasm-api';

export const execute_code: (code: string, args: [string, any][], debug: boolean) => Promise<any | undefined> =
    _execute_code;
export const execute_validate_code: (
    code: string,
    link_value: LinkValue,
    debug: boolean,
) => Promise<string | undefined> = _execute_validate_code;

export const parse_service_candid: <T>(
    candid: string,
    mapping: (service: WrappedCandidTypeService) => T,
    debug: boolean,
) => Promise<T> = _parse_service_candid;
export const parse_func_candid: <T>(
    func: string,
    mapping: (func: [string, WrappedCandidTypeFunction]) => T,
    debug: boolean,
) => Promise<T> = _parse_func_candid;
export const parse_candid_type_to_text: <T>(ty: WrappedCandidType, debug: boolean) => Promise<T> =
    _parse_candid_type_to_text;

export const find_all_anchors: (components: LinkComponent[], debug: boolean) => Promise<CheckedAnchors> =
    _find_all_anchors;
export const find_origin_codes: (
    components: LinkComponent[],
    fetch: ApisCheckFunction,
    debug: boolean,
) => Promise<CheckedCodeItem[]> = _find_origin_codes;
export const find_template_origin_codes: (nodes: TrimmedNode[], debug: boolean) => Promise<CheckedCodeItem[]> =
    _find_template_origin_codes;

export const check: (
    components: LinkComponent[],
    fetch: ApisCheckFunction,
    debug: boolean,
) => Promise<CheckedCombined> = _check;
export const check_template: (
    nodes: TrimmedNode[],
    checked: CheckedCombined,
    fetch: ApisCheckFunction,
    debug: boolean,
) => Promise<number> = _check_template;
