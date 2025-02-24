import { ComponentId } from './common/identity';
import { NodeTemplate } from './node/template';

export interface TrimmedLinkComponent {
    id: ComponentId;
}

export interface TrimmedNodeDataTemplate {
    component?: TrimmedLinkComponent;
    template?: NodeTemplate;
}

export interface TrimmedNodeData {
    node_id: string;
    data: TrimmedNodeDataTemplate;
}

export interface TrimmedNode {
    data: TrimmedNodeData;
}
