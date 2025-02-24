import type { ActorSubclass } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';

export type ActorCreator = <T>(
    idlFactory: IDL.InterfaceFactory, // candid interface
    canister_id: string, // target
) => Promise<ActorSubclass<T>>;
