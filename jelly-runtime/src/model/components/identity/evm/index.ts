import { LinkType } from '@jellypack/types/lib/types';
import { AbstractSigner, Provider } from 'ethers';

import { ComponentIdentityValue, PlainComponentIdentityValue } from '..';
import { ComponentId } from '../../../common/identity';
import { input_value_get_used_component, InputValue } from '../../../common/refer';
import { EvmWallet } from '../../../common/wallet/evm';
import { EvmChain } from '../../../types/evm';
import { get_evm_anonymous } from './anonymous';

export interface IdentityEvmMetadata {
    chain: EvmChain;
    includes?: EvmWallet[];
    excludes?: EvmWallet[];
    connect?: InputValue;
}

// EVM login saved value
export interface ComponentIdentityEvmValue {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    is_connected: () => Promise<boolean>;
    provider: Provider; // connect to blockchain
    signer: AbstractSigner; // account

    account: string;
}

// EVM output type
export interface IdentityEvmOutput {
    chain: EvmChain;
    chain_id: number;
    wallet: string;

    account: string;
}

// get output type
export const identity_evm_metadata_get_output_type = (_self: IdentityEvmMetadata): LinkType => {
    return {
        object: [
            { key: 'chain', ty: 'text' },
            { key: 'chain_id', ty: 'integer' },
            { key: 'wallet', ty: 'text' },
            { key: 'account', ty: 'text' },
        ],
    };
};

// Whether it has been logged in
export const identity_evm_metadata_has_value = (
    self: IdentityEvmMetadata,
    id: ComponentId,
    identity: Record<ComponentId, ComponentIdentityValue>,
): boolean => {
    if (!self.includes?.length) {
        return true;
    } else {
        const identity_value = identity[id];
        if (!identity_value) throw new Error('Identity not found');
        return typeof identity_value !== 'function';
    }
};

// get default value
export const identity_evm_metadata_get_anonymous_value = (chain: EvmChain): ComponentIdentityEvmValue =>
    get_evm_anonymous(chain);

// get default output
export const identity_evm_metadata_get_anonymous_output_value = (chain: EvmChain): IdentityEvmOutput => {
    const anonymous = get_evm_anonymous(chain);
    return {
        chain: anonymous.chain,
        chain_id: anonymous.chain_id,
        wallet: Object.keys(anonymous.wallet)[0],
        account: anonymous.account,
    };
};

// Login in the query value
export const identity_evm_metadata_get_value = async (
    self: IdentityEvmMetadata,
    id: ComponentId,
    identity: Record<ComponentId, ComponentIdentityValue>,
    connecting: (id: ComponentId, connecting: boolean) => void,
): Promise<IdentityEvmOutput | undefined> => {
    let value: PlainComponentIdentityValue | undefined;
    if (!self.includes?.length) {
        value = { evm: get_evm_anonymous(self.chain) };
    } else {
        const identity_value = identity[id];
        if (!identity_value) throw new Error('Identity not found');
        value = await (async () => {
            if (typeof identity_value === 'function') {
                try {
                    connecting(id, true);
                    return await identity_value();
                } finally {
                    connecting(id, false);
                }
            }
            return identity_value;
        })();
    }
    if (value === undefined) return undefined;
    if ('evm' in value) {
        identity[id] = value;
        return {
            chain: value.evm.chain,
            chain_id: value.evm.chain_id,
            wallet: Object.keys(value.evm.wallet)[0],
            account: value.evm.account,
        };
    }
    throw new Error('Invalid identity value');
};

// get used component
export const identity_evm_metadata_get_used_component = (self: IdentityEvmMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    if (self.connect) used.push(...input_value_get_used_component(self.connect));
    return used;
};
