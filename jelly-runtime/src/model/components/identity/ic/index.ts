import { LinkType } from '@jellypack/types/lib/types';

import { ComponentIdentityValue, PlainComponentIdentityValue } from '..';
import { ComponentId } from '../../../common/identity';
import { input_value_get_used_component, InputValue } from '../../../common/refer';
import { IcWallet } from '../../../common/wallet/ic';
import { anonymous } from './anonymous';
import { ActorCreator } from './types';

export interface IdentityIcMetadata {
    includes?: IcWallet[];
    excludes?: IcWallet[];
    connect?: InputValue;
}

// IC login saved value
export interface ComponentIdentityIcValue {
    wallet: IcWallet;
    secret: string | undefined; // undefined means wallet

    is_connected: () => Promise<boolean>;
    creator: ActorCreator;

    owner: string;
    account_id: string;
}

// IC output type
export interface IdentityIcOutput {
    wallet: string;

    owner: string;
    account_id: string;
}

// get output type
export const identity_ic_metadata_get_output_type = (_self: IdentityIcMetadata): LinkType => {
    return {
        object: [
            { key: 'wallet', ty: 'text' },
            { key: 'owner', ty: 'text' },
            { key: 'account_id', ty: 'text' },
        ],
    };
};

// Whether it has been logged in
export const identity_ic_metadata_has_value = (
    self: IdentityIcMetadata,
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
export const identity_ic_metadata_get_anonymous_value = (): ComponentIdentityIcValue => anonymous;

// get default output
export const identity_ic_metadata_get_anonymous_output_value = (): IdentityIcOutput => ({
    wallet: Object.keys(anonymous.wallet)[0],
    owner: anonymous.owner,
    account_id: anonymous.account_id,
});

// get or connect
export const identity_ic_metadata_get_value = async (
    self: IdentityIcMetadata,
    id: ComponentId,
    identity: Record<ComponentId, ComponentIdentityValue>,
    connecting: (id: ComponentId, connecting: boolean) => void,
): Promise<IdentityIcOutput | undefined> => {
    let value: PlainComponentIdentityValue | undefined;
    if (!self.includes?.length) {
        value = { ic: anonymous };
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
    if ('ic' in value) {
        identity[id] = value;
        return {
            wallet: Object.keys(value.ic.wallet)[0],
            owner: value.ic.owner,
            account_id: value.ic.account_id,
        };
    }
    throw new Error('Invalid identity value');
};

// get used component
export const identity_ic_metadata_get_used_component = (self: IdentityIcMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    if (self.connect) used.push(...input_value_get_used_component(self.connect));
    return used;
};
