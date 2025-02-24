import { LinkType } from '@jellypack/types';
import { NodeTemplateValidate } from './validate';

export interface NodeTemplate {
    node_id: string;
    output: LinkType;
    title: string;
    description: string;
    validate?: NodeTemplateValidate;
}
