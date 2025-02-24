import { LinkType } from '@jellypack/types/lib/types';
import { LinkValue } from '@jellypack/types/lib/values';
import { ComponentId } from '../common/identity';

export interface ComponentConst {
    id: ComponentId;
    metadata: ConstMetadata;
    output: LinkType;
}

export interface ConstMetadata {
    value: LinkValue;
}
