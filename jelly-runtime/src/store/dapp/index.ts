import { CallChain } from '../../common/chain';
import { TimestampMills } from '../../common/time';
import { CombinedMetadata } from '../../model';
import { CombinedAnchor } from '../combined';
import { PublisherAnchor } from '../publisher';
import { DappAccess, DappAccessView } from './access';
import { DappCategory } from './category';
import { DappInfo } from './info';

export type DappAnchor = string; // dapp key

export interface Dapp {
    id: DappAnchor;

    created: TimestampMills;
    updated: TimestampMills;
    frozen?: TimestampMills;
    reason: string;

    access: DappAccess;

    accessed: number;
    called: number;
    collected: number;

    category: DappCategory;

    info: DappInfo;

    publisher: PublisherAnchor;

    combined: CombinedAnchor;

    chains?: CallChain[];

    metadata?: CombinedMetadata;
}

// ================== view ==================

export interface DappView {
    id: DappAnchor;

    created: TimestampMills;
    updated: TimestampMills;
    frozen?: TimestampMills;
    reason: string;

    access: DappAccessView;

    accessed: number;
    called: number;
    collected: number;

    category: DappCategory;

    info: DappInfo;

    publisher: PublisherAnchor;

    combined: CombinedAnchor;

    chains?: CallChain[];

    metadata?: CombinedMetadata;
}

// ================== view ==================

export interface DappMetadata {
    accessed: number;
    called: number;
    collected: number;

    category: DappCategory;

    info: DappInfo;
}
