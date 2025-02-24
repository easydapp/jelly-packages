import { LinkType } from '@jellypack/types/lib/types';
import { ApiDataAnchor } from '../store/api';
import { CodeDataAnchor } from '../store/code';
import { CombinedAnchor } from '../store/combined';
import { ComponentId } from './common/identity';
import { IdentityInnerMetadata } from './components/identity';
import { InteractionInnerMetadata } from './components/interaction';

export interface CombinedMetadata {
    params?: ComponentParamRequired[];
    identities?: ComponentIdentityRequired[];

    forms?: ComponentFormRequired[];
    interactions?: ComponentInteractionRequired[];

    code_anchors?: CodeDataAnchor[];
    apis_anchors?: ApiDataAnchor[];
    combined_anchors?: CombinedAnchor[];

    output?: LinkType;
}

// param
export interface ComponentParamRequired {
    id: ComponentId;
    name: string;
    default?: string;
}

// form
export interface ComponentFormRequired {
    id: ComponentId;
    name?: string;
    output: LinkType;
}

// identity
export interface ComponentIdentityRequired {
    id: ComponentId;
    name?: string;
    metadata: IdentityInnerMetadata;
}

// interaction
export interface ComponentInteractionRequired {
    id: ComponentId;
    name?: string;
    metadata: InteractionInnerMetadata;
}
