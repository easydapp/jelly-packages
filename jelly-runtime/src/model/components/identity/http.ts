import { LinkType } from '@jellypack/types/lib/types';

import { ComponentId } from '../../common/identity';

export interface IdentityHttpMetadata {
    proxy?: string;
}

// http login saved value
export interface ComponentIdentityHttpValue {
    proxy?: string;
}

// HTTP output type
export interface IdentityHttpOutput {
    proxy: string;
}

// get output type
export const identity_http_metadata_get_output_type = (_self: IdentityHttpMetadata): LinkType => {
    return { object: [{ key: 'proxy', ty: 'text' }] };
};

// Whether it has been logged in
export const identity_http_metadata_has_value = (_self: IdentityHttpMetadata): boolean => {
    return true;
};

// get default value
export const identity_http_metadata_get_anonymous_value = (): IdentityHttpOutput => {
    return {
        proxy: 'https://p.easydapp.ai',
    };
};

// get value
export const identity_http_metadata_get_value = (self: IdentityHttpMetadata): IdentityHttpOutput => {
    return {
        proxy: self.proxy ?? 'https://p.easydapp.ai',
    };
};

// get used component
export const identity_http_metadata_get_used_component = (_self: IdentityHttpMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    return used;
};
