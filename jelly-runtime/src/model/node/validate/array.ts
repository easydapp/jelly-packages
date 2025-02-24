import { NodeTemplateValidate } from '.';

export interface NodeTemplateValidateArray {
    subtype?: NodeTemplateValidate; // Sub-type constraint

    min_length?: number;
    max_length?: number;
    code?: string;
}
