/* eslint-disable @typescript-eslint/array-type */
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';
import type { Principal } from '@dfinity/principal';

export interface CanisterStatusResponse {
    status: CanisterStatusType;
    memory_size: bigint;
    cycles: bigint;
    settings: DefiniteCanisterSettings;
    query_stats: QueryStats;
    idle_cycles_burned_per_day: bigint;
    module_hash: [] | [Uint8Array | number[]];
    reserved_cycles: bigint;
}
export type CanisterStatusType = { stopped: null } | { stopping: null } | { running: null };
export interface DefiniteCanisterSettings {
    freezing_threshold: bigint;
    controllers: Array<Principal>;
    reserved_cycles_limit: bigint;
    log_visibility: LogVisibility;
    wasm_memory_limit: bigint;
    memory_allocation: bigint;
    compute_allocation: bigint;
}
export type LogVisibility = { controllers: null } | { public: null };
export interface QueryStats {
    response_payload_bytes_total: bigint;
    num_instructions_total: bigint;
    num_calls_total: bigint;
    request_payload_bytes_total: bigint;
}
export type Result = { Ok: string } | { Err: string };
export interface _SERVICE {
    __get_candid_interface_tmp_hack: ActorMethod<[], string>;
    admin_add: ActorMethod<[Principal], undefined>;
    admin_query: ActorMethod<[], Array<Principal>>;
    admin_remove: ActorMethod<[Principal], undefined>;
    api_query: ActorMethod<[string], [] | [string]>;
    api_update: ActorMethod<[string], undefined>;
    canister_status: ActorMethod<[], CanisterStatusResponse>;
    code_query: ActorMethod<[string], [] | [string]>;
    code_update: ActorMethod<[string], undefined>;
    combined_increment_called: ActorMethod<[string], undefined>;
    combined_query: ActorMethod<[string], [] | [string]>;
    combined_update: ActorMethod<[string], undefined>;
    dapp_increment_called_by_admin: ActorMethod<[string], undefined>;
    dapp_increment_called_by_token: ActorMethod<[string, [] | [string]], undefined>;
    dapp_query_access: ActorMethod<[string], Result>;
    dapp_query_by_admin: ActorMethod<[string], Result>;
    dapp_query_by_token: ActorMethod<[string, [] | [string]], Result>;
    dapp_update: ActorMethod<[string], undefined>;
    dapp_update_collected: ActorMethod<[string, bigint], undefined>;
    publisher_query: ActorMethod<[string], [] | [string]>;
    publisher_update: ActorMethod<[string], undefined>;
    wallet_balance: ActorMethod<[], bigint>;
    whoami: ActorMethod<[], Principal>;
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
