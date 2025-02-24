import { LinkComponent } from '../model';
import { ComponentId } from '../model/common/identity';
import { AllBranches, AllEndpoints } from '../model/common/lets';
import {
    link_component_get_used_component,
    link_component_is_triggered,
} from '../model/components';
import { match_identity_inner_metadata } from '../model/components/identity';

export class ComponentInfo {
    id: ComponentId;
    component: LinkComponent; // self component
    endpoints: AllEndpoints | undefined; // Reference
    used: Set<ComponentId>; // need values
    branches: AllBranches | undefined; // Referenced
    // The current component is an identity, and there are only one trigger in the future, so the trigger of this Identity is to which Trigger binding in the follow -up
    identity_triggers: Set<ComponentId> | undefined; // Requires any subsequent component to trigger to run
    // The component of the trigger conditions of this component should be discharged,
    // 1. The front is identity. You need to trigger this call component to call the front IDentity. Therefore, when judging the display, you should not consider the output of Identity
    // 2. This component is a condition, and there is NULL judgment, so the dependence of this component should abandon the reference node of NULL
    excepted: Set<ComponentId> | undefined; // This component data is not required for presentation // from trigger
    constructor(
        id: ComponentId,
        component: LinkComponent,
        endpoints: AllEndpoints | undefined,
        branches: AllBranches | undefined,
    ) {
        this.id = id;
        this.component = component;
        this.endpoints = endpoints;
        this.used = link_component_get_used_component(component);
        this.branches = branches;
        if (this.branches && 'identity' in component) {
            if (
                match_identity_inner_metadata(component.identity.metadata.metadata, {
                    http: () => false, // http not blocking
                    ic: (ic) => !!ic.includes?.length && !ic.connect, // block if use wallet
                    evm: (evm) => !!evm.includes?.length && !evm.connect, // block if use wallet
                })
            ) {
                const no_triggers = new Set<ComponentId>();
                const triggers = new Set<ComponentId>();
                for (const b of this.branches.branches) {
                    if (link_component_is_triggered(b.component)) triggers.add(b.id);
                    else no_triggers.add(b.id);
                    // Indirect reference is not checked
                }
                this.identity_triggers =
                    no_triggers.size || triggers.size !== 1 ? undefined : triggers;
            }
        }
    }
}
