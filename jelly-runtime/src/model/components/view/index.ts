import { ComponentId } from '../../common/identity';
import { Endpoint } from '../../common/lets';
import { view_array_metadata_get_used_component, ViewArrayMetadata } from './array';
import { view_bool_metadata_get_used_component, ViewBoolMetadata } from './bool';
import { view_html_metadata_get_used_component, ViewHtmlMetadata } from './html';
import { view_image_metadata_get_used_component, ViewImageMetadata } from './image';
import { view_object_metadata_get_used_component, ViewObjectMetadata } from './object';
import { view_table_metadata_get_used_component, ViewTableMetadata } from './table';
import { view_text_metadata_get_used_component, ViewTextMetadata } from './text';

export interface ComponentView {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: ViewMetadata;
}

export type ViewMetadata =
    | { text: ViewTextMetadata }
    | { bool: ViewBoolMetadata }
    | { image: ViewImageMetadata }
    | { table: ViewTableMetadata }
    | { html: ViewHtmlMetadata }
    | { array: ViewArrayMetadata }
    | { object: ViewObjectMetadata };

export const match_view_metadata = <T>(
    self: ViewMetadata,
    {
        text,
        bool,
        image,
        table,
        html,
        array,
        object,
    }: {
        text: (text: ViewTextMetadata) => T;
        bool: (bool: ViewBoolMetadata) => T;
        image: (image: ViewImageMetadata) => T;
        table: (table: ViewTableMetadata) => T;
        html: (html: ViewHtmlMetadata) => T;
        array: (array: ViewArrayMetadata) => T;
        object: (object: ViewObjectMetadata) => T;
    },
): T => {
    if ('text' in self) return text(self.text);
    if ('bool' in self) return bool(self.bool);
    if ('image' in self) return image(self.image);
    if ('table' in self) return table(self.table);
    if ('html' in self) return html(self.html);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('Invalid view metadata');
};

export const component_view_get_used_component = (self: ComponentView): ComponentId[] => {
    return match_view_metadata(self.metadata, {
        text: (text) => view_text_metadata_get_used_component(text),
        bool: (bool) => view_bool_metadata_get_used_component(bool),
        image: (image) => view_image_metadata_get_used_component(image),
        table: (table) => view_table_metadata_get_used_component(table),
        html: (html) => view_html_metadata_get_used_component(html),
        array: (array) => view_array_metadata_get_used_component(array),
        object: (object) => view_object_metadata_get_used_component(object),
    });
};
