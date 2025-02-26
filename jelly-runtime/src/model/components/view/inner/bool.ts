import { LinkType } from '@jellypack/types/lib/types';

import { same } from '../../../../common/same';
import { ViewBoolMetadataStyle } from '../bool';

// bool

export interface InnerViewBoolMetadata {
    style?: string;
}

export type BoolViewSupportedType = 'bool';

export const bool_view_supported_types = (): LinkType[] => {
    return ['bool'];
};

export const bool_view_is_supported_type = (ty: LinkType): boolean => {
    return bool_view_supported_types().find((s) => same(s, ty)) !== undefined;
};

// ========================= style =========================

export type InnerViewBoolMetadataStyle = ViewBoolMetadataStyle;
