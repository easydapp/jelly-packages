import { unwrapOptionMap, unwrapRustResultMap, wrapOptionMap } from '@choptop/haw';
import { Principal } from '@dfinity/principal';
import bs58 from 'bs58';

import { sha256 } from '../../common/hash';
import { array2hex } from '../../common/hex';
import { ApiData, ApiDataAnchor } from '../../store/api';
import { CodeData, CodeDataAnchor } from '../../store/code';
import { Combined, CombinedAnchor } from '../../store/combined';
import { DappAnchor, DappView } from '../../store/dapp';
import { DappAccessView, DappVerified } from '../../store/dapp/access';
import { Publisher, PublisherAnchor } from '../../store/publisher';
import anonymous from '../anonymous';
import { idlFactory } from './candid';
import { _SERVICE } from './candid.d';

// https://dashboard.internetcomputer.org/canister/zhyj6-vaaaa-aaaai-q3luq-cai

export type IncrementCombinedCall = (anchor: CombinedAnchor) => Promise<void>;
export type IncrementDappCalledByToken = (id: string, verified?: DappVerified) => Promise<void>;

const proxy_query = async <T>(api: string): Promise<T | undefined> => {
    const response = await fetch(`https://p.easydapp.ai${api}`);
    const json = await response.json();
    return json.data;
};

// ================= publisher =================

const publisher_query = async (canister_id: string, anchor: PublisherAnchor): Promise<Publisher | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.publisher_query(anchor);
    return unwrapOptionMap(r, JSON.parse);
};

export const query_publisher = async (anchor: PublisherAnchor): Promise<Publisher | undefined> => {
    const canister_id = anchor.split('#')[1];
    if (!canister_id) throw new Error('invalid user anchor');
    return publisher_query(canister_id, anchor);
};

export const proxy_query_publisher = async (anchor: PublisherAnchor): Promise<Publisher | undefined> => {
    return (
        (await proxy_query<Publisher>(`/storage/publisher/${encodeURIComponent(anchor)}`)) ??
        (await query_publisher(anchor))
    );
};

// ================= code =================

const code_query = async (canister_id: string, anchor: CodeDataAnchor): Promise<CodeData | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.code_query(anchor);
    return unwrapOptionMap(r, JSON.parse);
};

export const query_code = async (anchor: CodeDataAnchor): Promise<CodeData | undefined> => {
    const canister_id = anchor.split('#')[1];
    if (!canister_id) throw new Error('invalid code anchor');
    return code_query(canister_id, anchor);
};

export const proxy_query_code = async (anchor: CodeDataAnchor): Promise<CodeData | undefined> => {
    return (await proxy_query<CodeData>(`/storage/code/${encodeURIComponent(anchor)}`)) ?? (await query_code(anchor));
};

// ================= apis =================

const api_query = async (canister_id: string, anchor: ApiDataAnchor): Promise<ApiData | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.api_query(anchor);
    return unwrapOptionMap(r, JSON.parse);
};

export const query_api = async (anchor: ApiDataAnchor): Promise<ApiData | undefined> => {
    const canister_id = anchor.split('#')[1];
    if (!canister_id) throw new Error('invalid api anchor');
    return api_query(canister_id, anchor);
};

export const proxy_query_api = async (anchor: ApiDataAnchor): Promise<ApiData | undefined> => {
    return (await proxy_query<ApiData>(`/storage/api/${encodeURIComponent(anchor)}`)) ?? (await query_api(anchor));
};

// ================= combined =================

const combined_increment_called = async (canister_id: string, anchor: CombinedAnchor): Promise<void> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    await actor.combined_increment_called(anchor);
};
export const increment_combined_called = async (anchor: CombinedAnchor): Promise<void> => {
    const canister_id = anchor.split('#')[1];
    if (canister_id === 'aaaaa-aa') return;
    if (!canister_id) throw new Error('invalid combined anchor');
    return combined_increment_called(canister_id, anchor);
};

const combined_query = async (canister_id: string, anchor: CombinedAnchor): Promise<Combined | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.combined_query(anchor);
    return unwrapOptionMap(r, JSON.parse);
};

export const query_combined = async (anchor: CombinedAnchor): Promise<Combined | undefined> => {
    const canister_id = anchor.split('#')[1];
    if (!canister_id) throw new Error('invalid combined anchor');
    return combined_query(canister_id, anchor);
};

export const proxy_query_combined = async (anchor: CombinedAnchor): Promise<Combined | undefined> => {
    return (
        (await proxy_query<Combined>(`/storage/combined/${encodeURIComponent(anchor)}`)) ??
        (await query_combined(anchor))
    );
};

// ================= dapp =================
export const get_dapp_canister_id_by_anchor = async (anchor: DappAnchor): Promise<string | undefined> => {
    try {
        return await parse_dapp_canister_id(anchor);
        // eslint-disable-next-line @typescript-eslint/no-unused-vars, no-empty
    } catch (error) {}
    return undefined;
};
const parse_dapp_canister_id = async (id: string) => {
    if (!id || !id.startsWith('in')) throw new Error('invalid dapp id');
    const data = bs58.decode(id.substring(2));

    // Check the verification
    if (data.length < 4) throw new Error('invalid dapp id');
    const hex = array2hex(data);
    const hash = await sha256(hex.substring(0, hex.length - 8));
    if (hash.substring(0, 8) !== hex.substring(hex.length - 8)) throw new Error('invalid dapp id');

    const length = (data[0] >> 4) & 0x0f;
    const len = data[0] & 0x0f;
    if (data.length < len + 1) throw new Error('invalid dapp id');

    const canister_id_bytes = new Uint8Array(length);
    for (let i = 0; i < length; i++) canister_id_bytes[i] = 0;
    for (let i = 0; i < len; i++) canister_id_bytes[i + length - len] = data[i + 1];
    const canister_id = Principal.fromUint8Array(canister_id_bytes).toText();

    return canister_id;
};
const handle_wrong_canister_id_error = async <T>(
    id: string,
    call: (canister_id: string) => Promise<T>,
): Promise<T | undefined> => {
    const canister_id = await parse_dapp_canister_id(id);
    try {
        return await call(canister_id);
    } catch (error: any) {
        // Invalid certificate: Invalid signature from replica signed query: no matching node key found.
        if (error.message && 0 <= error.message.indexOf('no matching node key found')) {
            return undefined;
        }
        if (error.message && error.message.startsWith('Frozen: ')) {
            throw error;
        }
        console.error(`handle_wrong_canister_id_error 🚀 ~ canister_id:`, canister_id);
        console.error('handle_wrong_canister_id_error', error);
    }
    return undefined;
};

const dapp_increment_called_by_token = async (
    canister_id: string,
    anchor: DappAnchor,
    verified?: DappVerified,
): Promise<void> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    await actor.dapp_increment_called_by_token(anchor, wrapOptionMap(verified, JSON.stringify));
};

export const increment_dapp_called_by_token = async (id: string, verified?: DappVerified): Promise<void> => {
    try {
        return handle_wrong_canister_id_error(id, (canister_id) =>
            dapp_increment_called_by_token(canister_id, id, verified),
        );
        // eslint-disable-next-line @typescript-eslint/no-unused-vars, no-empty
    } catch (error) {}
};

const dapp_query_by_token = async (
    canister_id: string,
    anchor: DappAnchor,
    verified?: DappVerified,
): Promise<DappView | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.dapp_query_by_token(anchor, wrapOptionMap(verified, JSON.stringify));
    return unwrapRustResultMap(
        r,
        (json) => {
            const dapp: DappView = JSON.parse(json);
            return dapp;
        },
        (error) => {
            if (error.startsWith('Frozen: ')) throw new Error(error);
            console.error(`query dapp by ${anchor} got error: ${error}`);
            return undefined;
        },
    );
};

export const query_dapp_by_token = async (id: string, verified?: DappVerified): Promise<DappView | undefined> => {
    return handle_wrong_canister_id_error(id, (canister_id) => dapp_query_by_token(canister_id, id, verified));
};

export const proxy_query_dapp_by_token = async (id: string, verified?: DappVerified): Promise<DappView | undefined> => {
    if (!verified) return query_dapp_by_token(id, verified);
    return (
        (await proxy_query<DappView>(`/storage/dapp/${encodeURIComponent(id)}`)) ??
        (await query_dapp_by_token(id, verified))
    );
};

// ================= dapp access =================

const dapp_query_access = async (canister_id: string, anchor: DappAnchor): Promise<DappAccessView | undefined> => {
    const { creator } = anonymous;
    const actor: _SERVICE = await creator(idlFactory, canister_id);
    const r = await actor.dapp_query_access(anchor);
    return unwrapRustResultMap(
        r,
        (json) => {
            const dapp: DappAccessView = JSON.parse(json);
            return dapp;
        },
        (error) => {
            console.error(`query dapp access by ${anchor} got error: ${error}`);
            return undefined;
        },
    );
};

export const query_dapp_access_by_id = async (id: string): Promise<DappAccessView | undefined> => {
    return handle_wrong_canister_id_error(id, (canister_id) => dapp_query_access(canister_id, id));
};

export const proxy_query_dapp_access_by_id = async (id: string): Promise<DappAccessView | undefined> => {
    return (
        (await proxy_query<DappAccessView>(`/storage/dapp/access/${encodeURIComponent(id)}`)) ??
        (await query_dapp_access_by_id(id))
    );
};
