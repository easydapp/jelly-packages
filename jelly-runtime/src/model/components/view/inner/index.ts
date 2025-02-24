import { LinkType } from '@jellypack/types/lib/types';
import _ from 'lodash';
import { array_view_is_supported_type, InnerViewArrayMetadata } from './array';
import { bool_view_is_supported_type, InnerViewBoolMetadata } from './bool';
import { html_view_is_supported_type, InnerViewHtmlMetadata } from './html';
import { image_view_is_supported_type, InnerViewImageMetadata } from './image';
import { InnerViewObjectMetadata, object_view_is_supported_type } from './object';
import { InnerViewTableMetadata, table_view_is_supported_type } from './table';
import { InnerViewTextMetadata, text_view_is_supported_type } from './text';

// view metadata

export type InnerViewMetadata =
    | { text: InnerViewTextMetadata }
    | { bool: InnerViewBoolMetadata }
    | { image: InnerViewImageMetadata }
    | { table: InnerViewTableMetadata }
    | { html: InnerViewHtmlMetadata }
    | { array: InnerViewArrayMetadata }
    | { object: InnerViewObjectMetadata };

export const match_inner_view_metadata = <T>(
    self: InnerViewMetadata,
    {
        text,
        bool,
        image,
        table,
        html,
        array,
        object,
    }: {
        text: (text: InnerViewTextMetadata) => T;
        bool: (bool: InnerViewBoolMetadata) => T;
        image: (image: InnerViewImageMetadata) => T;
        table: (table: InnerViewTableMetadata) => T;
        html: (html: InnerViewHtmlMetadata) => T;
        array: (array: InnerViewArrayMetadata) => T;
        object: (object: InnerViewObjectMetadata) => T;
    },
): T => {
    if ('text' in self) return text(self.text);
    if ('bool' in self) return bool(self.bool);
    if ('image' in self) return image(self.image);
    if ('table' in self) return table(self.table);
    if ('html' in self) return html(self.html);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('invalid inner view metadata');
};

export const inner_view_metadata_is_supported_type = (
    self: InnerViewMetadata,
    ty: LinkType,
): boolean => {
    return match_inner_view_metadata(self, {
        text: (_) => text_view_is_supported_type(ty),
        bool: (_) => bool_view_is_supported_type(ty),
        image: (_) => image_view_is_supported_type(ty),
        table: (_) => table_view_is_supported_type(ty),
        html: (_) => html_view_is_supported_type(ty),
        array: (array) => array_view_is_supported_type(array, ty),
        object: (object) => object_view_is_supported_type(object, ty),
    });
};
