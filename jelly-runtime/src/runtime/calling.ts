// ================= http =================

import { ComponentId } from '../model/common/identity';
import { ComponentCall } from '../model/components/call';
import { ComponentIdentity, PlainComponentIdentityValue } from '../model/components/identity';
import { EvmChain } from '../model/types/evm';

export interface HttpActionData {
    url: string;
    method: 'GET' | 'POST' | 'PUT' | 'DELETE';
    headers?: [string, string][];
    body?: any;
    cached?: number;
}

// ================= ic =================

export interface IcCallActionData {
    caller: string;
    canister_id: string;
    method: string;
    args: any;
}

export interface IcActionData {
    call: IcCallActionData;
}

// ================= evm =================

export interface EvmCallActionData {
    chain: EvmChain;
    account: string;
    contract: string;
    method: string;
    args: any;
}

export interface EvmSignActionData {
    chain: EvmChain;
    account: string;
    message: string;
}

export interface EvmTransactionActionData {
    chain: EvmChain;
    account: string;
    contract: string;
    method: string;
    args: any;
    pay_value: any;
    gas_limit: any;
    gas_price: any;
    nonce: any;
}

export interface EvmDeployActionData {
    chain: EvmChain;
    account: string;
    abi: any;
    bytecode: any;
    initial: any;
    gas_limit: any;
    gas_price: any;
    nonce: any;
}

export interface EvmTransferActionData {
    chain: EvmChain;
    account: string;
    transfer_to: any;
    pay_value: any;
    gas_price: any;
    nonce: any;
}

export type EvmActionData =
    | { call: EvmCallActionData }
    | { sign: EvmSignActionData }
    | { transaction: EvmTransactionActionData }
    | { deploy: EvmDeployActionData }
    | { transfer: EvmTransferActionData };

export type CallData = { http: HttpActionData } | { ic: IcActionData } | { evm: EvmActionData };

export class CallingData {
    id: ComponentId;
    component: ComponentCall;
    identity?: {
        id: ComponentId;
        identity: ComponentIdentity;
    };
    call: () => void;
    identity_value?: PlainComponentIdentityValue;
    connecting: boolean;
    calling: boolean;
    data: {
        start: number;
        end: number;
        data: CallData;
        result: any;
    }[];

    constructor(
        id: ComponentId,
        component: ComponentCall,
        identity:
            | {
                  id: ComponentId;
                  identity: ComponentIdentity;
              }
            | undefined,
        call: () => void,
    ) {
        this.id = id;
        this.component = component;
        this.identity = identity;
        this.call = call;
        this.identity_value = undefined;
        this.connecting = false;
        this.calling = false;
        this.data = [];
    }

    start(data: CallData): number {
        const index = this.data.length;
        this.data[index] = { start: new Date().getTime(), end: 0, data, result: null };
        this.calling = true;
        this.call();
        return index;
    }

    result(index: number, result: any) {
        this.data[index].result = result;
    }

    over(index: number) {
        this.data[index].end = new Date().getTime();
        this.calling = false;
        this.call();
    }

    set_identity_value(identity_value: PlainComponentIdentityValue) {
        this.identity_value = identity_value;
    }

    set_connecting(connecting: boolean) {
        if (this.identity && connecting) return;
        this.connecting = connecting;
        this.call();
    }

    is_pending(): boolean {
        return this.calling || this.connecting;
    }
}
