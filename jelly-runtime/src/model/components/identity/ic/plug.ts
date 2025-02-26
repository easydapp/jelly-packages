import type { ActorSubclass, HttpAgent } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';

// Plug interface
export interface PlugInterface {
    requestConnect: (_?: {
        whitelist?: string[]; // ['canister-id'],
        host?: string; // 'https://network-address', // https://icp0.io
        timeout?: number; // ms, default 2 minutes
    }) => Promise<void>;
    isConnected: () => Promise<boolean>;
    disconnect: () => Promise<void>;

    agent?: HttpAgent;
    isWalletLocked?: boolean;
    principalId?: string;
    accountId?: string;

    createActor: <T>(_: { canisterId: string; interfaceFactory: IDL.InterfaceFactory }) => Promise<ActorSubclass<T>>;
}
