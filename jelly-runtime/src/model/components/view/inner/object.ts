import { LinkType } from '@jellypack/types/lib/types';

import { inner_view_metadata_is_supported_type, InnerViewMetadata } from '.';
import { ViewObjectMetadataStyle } from '../object';

// object

export interface InnerViewObjectItem {
    key: string;
    inner: InnerViewMetadata;
}
export interface InnerViewObjectMetadata {
    inner: InnerViewObjectItem[];
    style?: string;
}

export const object_view_is_supported_type = (self: InnerViewObjectMetadata, ty: LinkType): boolean => {
    if (typeof ty === 'object' && 'object' in ty) {
        const subitems = ty.object;
        if (self.inner.length != subitems.length) return false;
        for (let i = 0; i < self.inner.length; i++) {
            if (self.inner[i].key !== subitems[i].key) return false;
            if (!inner_view_metadata_is_supported_type(self.inner[i].inner, subitems[i].ty)) return false;
        }
        return true;
    }
    return false;
};

// ========================= style =========================

export type InnerViewObjectMetadataStyle = ViewObjectMetadataStyle;
