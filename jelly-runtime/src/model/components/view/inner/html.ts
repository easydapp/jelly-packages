import { LinkType } from '@jellypack/types/lib/types';
import _ from 'lodash';

import { ViewHtmlMetadataStyle } from '../html';
import { image_view_is_supported_type } from './image';

// html

export interface InnerViewHtmlMetadata {
    template: string;
    style?: string;
}

export const html_view_is_supported_type = (ty: LinkType): boolean => {
    if (typeof ty === 'object' && 'object' in ty) {
        const subitems = ty.object;

        // check image
        const image = subitems.find((s) => s.key === 'image');
        let images: string[] = [];
        if (image) {
            if (typeof image.ty === 'object' && 'object' in image.ty) {
                const subitems = image.ty.object;
                for (const item of subitems) {
                    if (!image_view_is_supported_type(item.ty)) return false;
                }
                images = subitems.map((item) => item.key);
            } else {
                return false;
            }
        }

        // check text
        const text = subitems.find((s) => s.key === 'text');
        let texts: string[] = [];
        if (text) {
            if (typeof text.ty === 'object' && 'object' in text.ty) {
                const subitems = text.ty.object;
                for (const item of subitems) {
                    if (item.ty !== 'text') return false;
                }
                texts = subitems.map((item) => item.key);
            } else {
                return false;
            }
        }

        // check image and text
        if (images.length !== _.uniq(images).length) return false;
        if (texts.length !== _.uniq(texts).length) return false;

        for (const name of images) if (texts.includes(name)) return false;
        for (const name of texts) if (images.includes(name)) return false;

        return true;
    }
    return false;
};

// ========================= style =========================

export type InnerViewHtmlMetadataStyle = ViewHtmlMetadataStyle;
