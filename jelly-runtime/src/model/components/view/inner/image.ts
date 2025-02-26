import { LinkType } from '@jellypack/types/lib/types';

import { same } from '../../../../common/same';
import { ViewImageMetadataStyle } from '../image';

// image

export interface InnerViewImageMetadata {
    style?: string;
}

export type ImageViewSupportedType = 'text' | { array: 'integer' };

export const image_view_supported_types = (): LinkType[] => {
    return ['text', { array: 'integer' }];
};

export const image_view_is_supported_type = (ty: LinkType): boolean => {
    return image_view_supported_types().find((s) => same(s, ty)) !== undefined;
};

// ========================= style =========================

export type InnerViewImageMetadataStyle = ViewImageMetadataStyle;
