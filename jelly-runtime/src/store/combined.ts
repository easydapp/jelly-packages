import { TimestampMills } from '../common/time';
import { CombinedMetadata, LinkComponent } from '../model';

export type CombinedAnchor = string; // combined key

export interface Combined {
    anchor: CombinedAnchor;

    created: TimestampMills;

    called: number;

    version: string;

    components: LinkComponent[];

    metadata?: CombinedMetadata;
}
