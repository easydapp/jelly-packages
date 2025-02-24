import { ComponentId } from '../../common/identity';
import { code_value_get_used_component, CodeValue } from '../../common/refer';

export interface ViewHtmlMetadata {
    image?: CodeValue[];
    text?: CodeValue[];
    template: string;
    style?: string;
}

export const view_html_metadata_get_used_component = (self: ViewHtmlMetadata): ComponentId[] => {
    const used = [];
    for (const code_value of self.image ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    for (const code_value of self.text ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    return used;
};

// ========================= style =========================

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface ViewHtmlMetadataStyle {}
