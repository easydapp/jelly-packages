import { CodeItem, CodeType } from '@jellypack/types/lib/code';
import { LinkType } from '@jellypack/types/lib/types';
import { CodeContent } from './code';
import { link_type_to_typescript } from './to_typescript';

export interface ValidateForm {
    code: CodeContent;
}

export const match_validate_form = <T>(
    self: ValidateForm,
    { code }: { code: (code: CodeContent) => T },
): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid validate form');
};

export const match_validate_form_async = async <T>(
    self: ValidateForm,
    { code }: { code: (code: CodeContent) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid validate form');
};

export const get_validate_code_ret = (): CodeType => {
    return {
        ty: 'ErrorMessage',
        types: ["type ErrorMessage = string; // pass if return ''"],
    };
};

export const get_validate_code_item = (output: LinkType, code: string): CodeItem => {
    return {
        code,
        args: [{ name: 'data', ty: { ty: link_type_to_typescript(output) } }],
        ret: get_validate_code_ret(),
    };
};
