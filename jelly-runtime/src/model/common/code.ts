import { CodeItem } from '@jellypack/types/lib/code';
import { CodeDataAnchor } from '../../store/code';

export interface OriginCodeContent {
    code: CodeItem;
    js: string;
}

export type CodeContent = { code: OriginCodeContent } | { anchor: CodeDataAnchor };

export const match_code_content = <T>(
    self: CodeContent,
    {
        code,
        anchor,
    }: {
        code: (code: OriginCodeContent) => T;
        anchor: (anchor: CodeDataAnchor) => T;
    },
): T => {
    if ('code' in self) return code(self.code);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('Invalid code content');
};

export const match_code_content_async = async <T>(
    self: CodeContent,
    {
        code,
        anchor,
    }: {
        code: (code: OriginCodeContent) => Promise<T>;
        anchor: (anchor: CodeDataAnchor) => Promise<T>;
    },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('Invalid code content');
};
