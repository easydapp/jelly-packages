import { ComponentId } from '../common/identity';

export interface ComponentParam {
    id: ComponentId;
    metadata: ParamMetadata;
}

export interface ParamMetadata {
    name: string;
    default?: string;
}
