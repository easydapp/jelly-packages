import { LinkType } from '@jellypack/types/lib/types';
import { same } from '../../../../common/same';
import { ViewTextMetadataStyle } from '../text';

// text

export interface InnerViewTextMetadata {
    style?: string;
}

export type TextViewSupportedType = 'text' | 'integer' | 'number';

export const text_view_supported_types = (): LinkType[] => {
    return ['text', 'integer', 'number'];
};

export const text_view_is_supported_type = (ty: LinkType): boolean => {
    return text_view_supported_types().find((s) => same(s, ty)) !== undefined;
};

// ========================= style =========================

export type InnerViewTextMetadataStyle = ViewTextMetadataStyle;
