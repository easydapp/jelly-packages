import { LinkType } from '@jellypack/types/lib/types';
import { same } from '../../../../common/same';
import { ViewTableMetadataStyle } from '../table';

// table

export interface InnerViewTableMetadata {
    style?: string;
}

export interface TableViewSupportedType {
    object: [
        { key: 'headers'; ty: { array: 'text' } },
        { key: 'rows'; ty: { array: { array: 'text' } } },
    ];
}

export const table_view_supported_types = (): LinkType[] => {
    return [
        {
            object: [
                { key: 'headers', ty: { array: 'text' } },
                { key: 'rows', ty: { array: { array: 'text' } } },
            ],
        },
    ];
};

export const table_view_is_supported_type = (ty: LinkType): boolean => {
    return table_view_supported_types().find((s) => same(s, ty)) !== undefined;
};

// ========================= style =========================

export type InnerViewTableMetadataStyle = ViewTableMetadataStyle;
