import { CodeItem } from '@jellypack/types';
import { TimestampMills } from '../common/time';

export type CodeDataAnchor = string; // code key

export interface CodeData {
    anchor: CodeDataAnchor;
    created: TimestampMills;
    code: CodeItem;
    js: string;
}
