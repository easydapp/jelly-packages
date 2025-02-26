import { LinkType } from '@jellypack/types/lib/types';

import { inner_view_metadata_is_supported_type, InnerViewMetadata } from '.';
import { ViewArrayMetadataStyle } from '../array';

// array

export interface InnerViewArrayMetadata {
    inner: InnerViewMetadata;
    style?: string;
}

export const array_view_is_supported_type = (self: InnerViewArrayMetadata, ty: LinkType): boolean => {
    if (typeof ty === 'object' && 'array' in ty) return inner_view_metadata_is_supported_type(self.inner, ty.array);
    return false;
};

// ========================= style =========================

export type InnerViewArrayMetadataStyle = ViewArrayMetadataStyle;
