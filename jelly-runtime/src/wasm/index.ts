import { MotokoResult } from '@choptop/haw';
import { CodeItem } from '@jellypack/types/lib/code';
import { parse_factory } from '@jellypack/types/lib/open/open-json';

import { CombinedMetadata, LinkComponent } from '../model';
import { ComponentId } from '../model/common/identity';
import { ApiData, ApiDataAnchor } from '../store/api';
import { CodeData, CodeDataAnchor } from '../store/code';
import { Combined, CombinedAnchor } from '../store/combined';
import { WrappedCandidTypeFunction, WrappedCandidTypeService } from './candid';

export type CodeExecutor = (code: string, args: [string, any][], debug: boolean) => Promise<any>;
export type ParseServiceCandid = <T>(
    candid: string,
    mapping: (service: WrappedCandidTypeService) => T,
    debug: boolean,
) => Promise<T>;
export type ParseFuncCandid = <T>(
    func: string,
    mapping: (func: [string, WrappedCandidTypeFunction]) => T,
    debug: boolean,
) => Promise<T>;

// ================ code ================

export const handle_wasm_code_result = (value: any) => {
    if (value !== undefined) {
        const result: MotokoResult<string, string> = JSON.parse(value);
        if (result.err !== undefined) {
            if (result.err.indexOf('Undefined') < 0) {
                // InvalidArgs // Pass parameters cannot be parsed
                // InvalidOutput // Results Type string coding error
                // WrongOutput // The result type is not string
                // ExecuteError // Execute code error
                throw new Error(value);
            }
            value = undefined; // The result is undefined
        } else {
            const parse = parse_factory(JSON.parse);
            value = parse(result.ok);
        }
    }
    return value;
};

// ================ candid ================

// ================ check ================

export interface CheckedAnchors {
    code_anchors?: CodeDataAnchor[];
    api_anchors?: ApiDataAnchor[];
    combined_anchors?: CombinedAnchor[];
}

export interface CombinedOriginApis {
    hash_origins: Record<string, string>; // There is no contract name through the file, so you can only record haSh
    key_hashes: Record<string, string>; // The API that query through the contract, first point to Hash, and then query the specific content
}

export interface ApisCheckFunction {
    // The latest jar
    canister_id: string;

    // stored codes
    codes: Record<CodeDataAnchor, CodeData>;
    // stored apis
    apis: Record<ApiDataAnchor, ApiData>;
    // stored combined
    combines: Record<CombinedAnchor, Combined>;

    // cached api
    origin_apis: CombinedOriginApis;

    // compiled code
    compiled: [CodeItem, string][];
}

export interface CheckedCodeItem {
    from: ComponentId;

    index: number;

    mark: string;

    code: CodeItem;
}

export interface CheckedCombined {
    // store code
    codes: Record<CodeDataAnchor, CodeData>;
    // store api
    apis: Record<ApiDataAnchor, ApiData>;

    // new components
    components: LinkComponent[];

    combined_anchor: CombinedAnchor;

    // metadata
    metadata?: CombinedMetadata;
}
