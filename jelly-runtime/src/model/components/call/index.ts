import { LinkType } from '@jellypack/types/lib/types';
import { deepClone } from '../../../common/clones';
import { same } from '../../../common/same';
import { CallingData } from '../../../runtime/calling';
import { RuntimeValues } from '../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../store/api';
import { CodeData, CodeDataAnchor } from '../../../store/code';
import { CodeExecutor, ParseFuncCandid, ParseServiceCandid } from '../../../wasm';
import { ComponentCallTrigger } from '../../common/call_trigger';
import { ComponentId } from '../../common/identity';
import { AllEndpoints, Endpoint } from '../../common/lets';
import { ComponentIdentityValue } from '../identity';
import { call_evm_metadata_get_used_component, CallEvmMetadata, get_call_evm_value } from './evm';
import { ExecuteEvmActionCall } from './evm/action/call';
import { ExecuteEvmActionDeploy } from './evm/action/deploy';
import {
    ExecuteEvmActionTransaction,
    ExecuteEvmActionTransactionEstimateGas,
} from './evm/action/transaction';
import { ExecuteEvmActionTransfer } from './evm/action/transfer';
import {
    call_http_metadata_get_used_component,
    CallHttpMetadata,
    ExecuteHttpCall,
    get_call_http_value,
} from './http';
import { call_ic_metadata_get_used_component, CallIcMetadata, get_call_ic_value } from './ic';
import { ExecuteIcActionCall } from './ic/action/call';

export interface ComponentCall {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: CallMetadata;
    output: LinkType;
}

export type CallMetadata =
    | { http: CallHttpMetadata }
    | { ic: CallIcMetadata }
    | { evm: CallEvmMetadata };

export const match_call_metadata = <T>(
    self: CallMetadata,
    {
        http,
        ic,
        evm,
    }: {
        http: (http: CallHttpMetadata) => T;
        ic: (ic: CallIcMetadata) => T;
        evm: (evm: CallEvmMetadata) => T;
    },
): T => {
    if ('http' in self) return http(self.http);
    if ('ic' in self) return ic(self.ic);
    if ('evm' in self) return evm(self.evm);
    throw new Error('invalid call metadata');
};

export const match_call_metadata_async = <T>(
    self: CallMetadata,
    {
        http,
        ic,
        evm,
    }: {
        http: (http: CallHttpMetadata) => Promise<T>;
        ic: (ic: CallIcMetadata) => Promise<T>;
        evm: (evm: CallEvmMetadata) => Promise<T>;
    },
): Promise<T> => {
    if ('http' in self) return http(self.http);
    if ('ic' in self) return ic(self.ic);
    if ('evm' in self) return evm(self.evm);
    throw new Error('invalid call metadata');
};

export const component_call_get_trigger = (self: ComponentCall): ComponentCallTrigger => {
    return match_call_metadata(self.metadata, {
        http: (http) => http.trigger,
        ic: (ic) => ic.trigger,
        evm: (evm) => evm.trigger,
    });
};

export const component_call_get_used_component = (self: ComponentCall): ComponentId[] => {
    return match_call_metadata(self.metadata, {
        http: (http) => call_http_metadata_get_used_component(http),
        ic: (ic) => call_ic_metadata_get_used_component(ic),
        evm: (evm) => call_evm_metadata_get_used_component(evm),
    });
};

export const component_call_get_identity = (self: ComponentCall): ComponentId | undefined => {
    return match_call_metadata(self.metadata, {
        http: (http) => http.identity,
        ic: (ic) => ic.identity,
        evm: (evm) => evm.identity,
    });
};

export const get_identity_value_by_id = async <T>(
    id: ComponentId,
    identity: Record<ComponentId, ComponentIdentityValue>,
): Promise<T | undefined> => {
    const value = identity[id];
    if (typeof value === 'function') return (await value()) as T;
    console.debug('get_identity_value_by_id', id, value);
    return value as T;
};

export const get_call_value = async (
    self: ComponentCall,
    endpoints: AllEndpoints | undefined,
    trigger: ComponentId | undefined,
    set_identity_triggered: (identity: ComponentId) => void,
    identity: Record<ComponentId, ComponentIdentityValue>,
    runtime_values: RuntimeValues,
    apis: Record<ApiDataAnchor, ApiData>,
    codes: Record<CodeDataAnchor, CodeData>,
    calling: CallingData,
    code_executor: CodeExecutor | undefined,
    parse_service_candid: ParseServiceCandid | undefined,
    parse_func_candid: ParseFuncCandid | undefined,
    execute_http_call: ExecuteHttpCall | undefined,
    execute_ic_action_call: ExecuteIcActionCall | undefined,
    execute_evm_action_call: ExecuteEvmActionCall | undefined,
    execute_evm_action_transaction_estimate_gas: ExecuteEvmActionTransactionEstimateGas | undefined,
    execute_evm_action_transaction: ExecuteEvmActionTransaction | undefined,
    execute_evm_action_deploy: ExecuteEvmActionDeploy | undefined,
    execute_evm_action_transfer: ExecuteEvmActionTransfer | undefined,
): Promise<any> => {
    return await match_call_metadata_async(self.metadata, {
        http: async (http) => {
            return await get_call_http_value(
                http,
                self.id,
                endpoints,
                trigger,
                set_identity_triggered,
                identity,
                runtime_values,
                codes,
                calling,
                code_executor,
                execute_http_call,
            );
        },
        ic: async (ic) => {
            return await get_call_ic_value(
                ic,
                self.id,
                endpoints,
                trigger,
                set_identity_triggered,
                identity,
                runtime_values,
                codes,
                apis,
                calling,
                code_executor,
                parse_service_candid,
                parse_func_candid,
                execute_ic_action_call,
            );
        },
        evm: async (evm) => {
            return await get_call_evm_value(
                evm,
                self.id,
                endpoints,
                trigger,
                set_identity_triggered,
                identity,
                runtime_values,
                codes,
                apis,
                calling,
                code_executor,
                execute_evm_action_call,
                execute_evm_action_transaction_estimate_gas,
                execute_evm_action_transaction,
                execute_evm_action_deploy,
                execute_evm_action_transfer,
            );
        },
    });
};

// cached

const CACHED: Record<
    ComponentId,
    {
        key: any;
        created: number;
        result: any;
    }[]
> = {};

export const get_cached_call_result = async (
    id: ComponentId,
    key: any,
    alive: number,
    before: () => number,
    handle: (call_index: number, result: any) => void,
    after: (call_index: number) => void,
    call: () => Promise<any>,
): Promise<any> => {
    const now = Date.now();
    const cached = CACHED[id];
    if (cached !== undefined) {
        // remove all timeouts
        for (let i = cached.length - 1; 0 <= i; i--) {
            if (cached[i].created + alive < now) {
                cached.splice(i, 1);
            }
        }
        for (let i = cached.length - 1; 0 <= i; i--) {
            if (same(cached[i].key, key)) {
                const call_index = before();
                const result = deepClone(cached[i].result);
                handle(call_index, result);
                after(call_index);
                return result;
            }
        }
    }

    const call_index = before();
    let result: any | undefined = undefined;
    try {
        result = await call();
    } finally {
        if (result !== undefined) {
            handle(call_index, result);
        }
        after(call_index);
    }

    CACHED[id] = CACHED[id] ?? [];
    CACHED[id].push({
        key: deepClone(key),
        created: now,
        result: deepClone(result),
    });
    return result;
};
