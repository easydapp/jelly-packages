import { ComponentId } from './identity';

export interface TriggeredComponentIdentity {
    // Whether to trigger
    click: boolean;
}

export interface TriggeredComponentCall {
    // Referenced identity
    identity?: ComponentId;
    // Is your own trigger?
    click: boolean;
    // Whether it is update
    update: boolean;
}

// Component information with trigger conditions
export type TriggeredComponent =
    | { identity: TriggeredComponentIdentity }
    | { call: TriggeredComponentCall }
    // eslint-disable-next-line @typescript-eslint/no-empty-object-type
    | { interaction: {} };

// Trigger inspection
export interface ComponentTriggered {
    id: ComponentId;
    triggered: TriggeredComponent;
    clickable?: boolean;
}
