import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../common/identity';
import { Endpoint } from '../../common/lets';
import { InputValue } from '../../common/refer';
import {
    ComponentIdentityEvmValue,
    identity_evm_metadata_get_output_type,
    identity_evm_metadata_get_used_component,
    identity_evm_metadata_get_value,
    identity_evm_metadata_has_value,
    IdentityEvmMetadata,
    IdentityEvmOutput,
} from './evm';
import {
    ComponentIdentityHttpValue,
    identity_http_metadata_get_output_type,
    identity_http_metadata_get_used_component,
    identity_http_metadata_get_value,
    identity_http_metadata_has_value,
    IdentityHttpMetadata,
    IdentityHttpOutput,
} from './http';
import {
    ComponentIdentityIcValue,
    identity_ic_metadata_get_output_type,
    identity_ic_metadata_get_used_component,
    identity_ic_metadata_get_value,
    identity_ic_metadata_has_value,
    IdentityIcMetadata,
    IdentityIcOutput,
} from './ic';

export interface ComponentIdentity {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: IdentityMetadata;
}

export interface IdentityMetadata {
    name?: string;
    metadata: IdentityInnerMetadata;
}

export type IdentityInnerMetadata =
    | { http: IdentityHttpMetadata }
    | { ic: IdentityIcMetadata }
    | { evm: IdentityEvmMetadata };

export type PlainComponentIdentityValue =
    | { http: ComponentIdentityHttpValue }
    | { ic: ComponentIdentityIcValue }
    | { evm: ComponentIdentityEvmValue };

export type ComponentIdentityValue =
    | PlainComponentIdentityValue
    | (() => Promise<PlainComponentIdentityValue | undefined>)
    | undefined;

export const match_identity_inner_metadata = <T>(
    self: IdentityInnerMetadata,
    {
        http,
        ic,
        evm,
    }: {
        http: (http: IdentityHttpMetadata) => T;
        ic: (ic: IdentityIcMetadata) => T;
        evm: (evm: IdentityEvmMetadata) => T;
    },
): T => {
    if ('http' in self) return http(self.http);
    if ('ic' in self) return ic(self.ic);
    if ('evm' in self) return evm(self.evm);
    throw new Error('Invalid identity metadata');
};

export const match_identity_inner_metadata_async = async <T>(
    self: IdentityInnerMetadata,
    {
        http,
        ic,
        evm,
    }: {
        http: (http: IdentityHttpMetadata) => Promise<T>;
        ic: (ic: IdentityIcMetadata) => Promise<T>;
        evm: (evm: IdentityEvmMetadata) => Promise<T>;
    },
): Promise<T> => {
    if ('http' in self) return http(self.http);
    if ('ic' in self) return ic(self.ic);
    if ('evm' in self) return evm(self.evm);
    throw new Error('Invalid identity metadata');
};

export const component_identity_get_output_type = (self: ComponentIdentity): LinkType => {
    return match_identity_inner_metadata<LinkType>(self.metadata.metadata, {
        http: (http) => identity_http_metadata_get_output_type(http),
        ic: (ic) => identity_ic_metadata_get_output_type(ic),
        evm: (evm) => identity_evm_metadata_get_output_type(evm),
    });
};

export const component_identity_has_value = (
    self: ComponentIdentity,
    identity: Record<ComponentId, ComponentIdentityValue>,
): boolean => {
    return match_identity_inner_metadata<boolean>(self.metadata.metadata, {
        http: (http) => identity_http_metadata_has_value(http),
        ic: (ic) => identity_ic_metadata_has_value(ic, self.id, identity),
        evm: (evm) => identity_evm_metadata_has_value(evm, self.id, identity),
    });
};

type IdentityOutput = IdentityHttpOutput | IdentityIcOutput | IdentityEvmOutput;

export const component_identity_get_output_value = async (
    self: ComponentIdentity,
    identity: Record<ComponentId, ComponentIdentityValue>,
    identity_fetched: Record<ComponentId, PlainComponentIdentityValue>,
    connecting: (id: ComponentId, connecting: boolean) => void,
): Promise<IdentityOutput | undefined> => {
    return await match_identity_inner_metadata_async<IdentityOutput | undefined>(self.metadata.metadata, {
        http: async (http) => identity_http_metadata_get_value(http),
        ic: async (ic) => await identity_ic_metadata_get_value(ic, self.id, identity, identity_fetched, connecting),
        evm: async (evm) => await identity_evm_metadata_get_value(evm, self.id, identity, identity_fetched, connecting),
    });
};

export const component_identity_get_used_component = (self: ComponentIdentity): ComponentId[] => {
    return match_identity_inner_metadata(self.metadata.metadata, {
        http: (http) => identity_http_metadata_get_used_component(http),
        ic: (ic) => identity_ic_metadata_get_used_component(ic),
        evm: (evm) => identity_evm_metadata_get_used_component(evm),
    });
};

export const component_identity_get_connect = (self: ComponentIdentity): InputValue | undefined => {
    return match_identity_inner_metadata(self.metadata.metadata, {
        http: () => undefined,
        ic: (ic) => ic.connect,
        evm: (evm) => evm.connect,
    });
};
