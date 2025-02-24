import { TimestampMills } from '../../common/time';
import { ApiDataContent } from './content';

export type ApiDataAnchor = string; // api key

export interface ApiData {
    anchor: ApiDataAnchor;
    created: TimestampMills;
    content: ApiDataContent;
}
