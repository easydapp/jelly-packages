import { LinkType } from '@jellypack/types/lib/types';

import { match_component_call_trigger } from '../common/call_trigger';
import { ComponentId } from '../common/identity';
import { AllBranch, AllBranches, AllEndpoints, Endpoint } from '../common/lets';
import { ValidateForm } from '../common/validate';
import { component_call_get_trigger, component_call_get_used_component, ComponentCall } from './call';
import { component_code_get_used_component, ComponentCode } from './code';
import {
    component_combined_get_output_type,
    component_combined_get_used_component,
    ComponentCombined,
} from './combined';
import {
    component_condition_count_outputs,
    component_condition_get_used_component,
    ComponentCondition,
} from './condition';
import { ComponentConst } from './constant';
import { component_form_get_used_component, ComponentForm } from './form';
import {
    component_identity_get_output_type,
    component_identity_get_used_component,
    ComponentIdentity,
} from './identity';
import {
    component_interaction_get_output_type,
    component_interaction_get_used_component,
    component_interaction_get_validate_form,
    ComponentInteraction,
} from './interaction';
import { component_output_get_used_component, ComponentOutput } from './output';
import { ComponentParam } from './param';
import { component_view_get_used_component, ComponentView } from './view';

export type LinkComponent =
    | { param: ComponentParam }
    | { const: ComponentConst }
    | { form: ComponentForm }
    | { code: ComponentCode }
    | { identity: ComponentIdentity }
    | { call: ComponentCall }
    | { interaction: ComponentInteraction }
    | { view: ComponentView }
    | { condition: ComponentCondition }
    | { output: ComponentOutput }
    | { combined: ComponentCombined };

export const match_link_component = <T>(
    self: LinkComponent,
    {
        param,
        constant,
        form,
        code,
        identity,
        call,
        interaction,
        view,
        condition,
        output,
        combined,
    }: {
        param: (param: ComponentParam) => T;
        constant: (constant: ComponentConst) => T;
        form: (form: ComponentForm) => T;
        code: (code: ComponentCode) => T;
        identity: (identity: ComponentIdentity) => T;
        call: (call: ComponentCall) => T;
        interaction: (interaction: ComponentInteraction) => T;
        view: (view: ComponentView) => T;
        condition: (condition: ComponentCondition) => T;
        output: (output: ComponentOutput) => T;
        combined: (combined: ComponentCombined) => T;
    },
): T => {
    if ('param' in self) return param(self.param);
    if ('const' in self) return constant(self.const);
    if ('form' in self) return form(self.form);
    if ('code' in self) return code(self.code);
    if ('identity' in self) return identity(self.identity);
    if ('call' in self) return call(self.call);
    if ('interaction' in self) return interaction(self.interaction);
    if ('view' in self) return view(self.view);
    if ('condition' in self) return condition(self.condition);
    if ('output' in self) return output(self.output);
    if ('combined' in self) return combined(self.combined);
    throw new Error(`unknown component: ${Object.keys(self)[0]}`);
};

export const match_link_component_async = async <T>(
    self: LinkComponent,
    {
        param,
        constant,
        form,
        code,
        identity,
        call,
        interaction,
        view,
        condition,
        output,
        combined,
    }: {
        param: (param: ComponentParam) => Promise<T>;
        constant: (constant: ComponentConst) => Promise<T>;
        form: (form: ComponentForm) => Promise<T>;
        code: (code: ComponentCode) => Promise<T>;
        identity: (identity: ComponentIdentity) => Promise<T>;
        call: (call: ComponentCall) => Promise<T>;
        interaction: (interaction: ComponentInteraction) => Promise<T>;
        view: (view: ComponentView) => Promise<T>;
        condition: (condition: ComponentCondition) => Promise<T>;
        output: (output: ComponentOutput) => Promise<T>;
        combined: (combined: ComponentCombined) => Promise<T>;
    },
): Promise<T> => {
    if ('param' in self) return param(self.param);
    if ('const' in self) return constant(self.const);
    if ('form' in self) return form(self.form);
    if ('code' in self) return code(self.code);
    if ('identity' in self) return identity(self.identity);
    if ('call' in self) return call(self.call);
    if ('interaction' in self) return interaction(self.interaction);
    if ('view' in self) return view(self.view);
    if ('condition' in self) return condition(self.condition);
    if ('output' in self) return output(self.output);
    if ('combined' in self) return combined(self.combined);
    throw new Error(`unknown component: ${Object.keys(self)[0]}`);
};

export const link_component_get_id = (self: LinkComponent): ComponentId => {
    return match_link_component<ComponentId>(self, {
        param: (param) => param.id,
        constant: (constant) => constant.id,
        form: (form) => form.id,
        code: (code) => code.id,
        identity: (identity) => identity.id,
        call: (call) => call.id,
        interaction: (interaction) => interaction.id,
        view: (view) => view.id,
        condition: (condition) => condition.id,
        output: (output) => output.id,
        combined: (combined) => combined.id,
    });
};

export const link_component_get_inlets = (self: LinkComponent): Endpoint[] | undefined => {
    return match_link_component<Endpoint[] | undefined>(self, {
        param: () => undefined,
        constant: () => undefined,
        form: (form) => form.inlets,
        code: (code) => code.inlets,
        identity: (identity) => identity.inlets,
        call: (call) => call.inlets,
        interaction: (interaction) => interaction.inlets,
        view: (view) => view.inlets,
        output: (output) => output.inlets,
        condition: (condition) => condition.inlets,
        combined: (combined) => combined.inlets,
    });
};

export const link_component_count_outputs = (self: LinkComponent): number => {
    return match_link_component<number>(self, {
        param: () => 1,
        constant: () => 1,
        form: () => 1,
        code: () => 1,
        identity: () => 1,
        call: () => 1,
        interaction: () => 1,
        view: () => 1, // 1 access, but no data
        output: () => 1,
        condition: (condition) => component_condition_count_outputs(condition), // Multiple access, but no data
        combined: () => 1,
    });
};

export const link_component_get_all_endpoints = (
    self: LinkComponent,
    components: Record<ComponentId, LinkComponent>,
): AllEndpoints | undefined => {
    const inlets = link_component_get_inlets(self);
    if (!inlets) return undefined;

    const endpoints = [];
    for (const inlet of inlets) {
        const component = components[inlet.id];
        if (component) {
            endpoints.push({
                id: inlet.id,
                index: inlet.index ?? 0,
                component,
                inlets: link_component_get_all_endpoints(component, components),
            });
        }
    }

    return endpoints.length ? { endpoints } : undefined;
};

export const link_component_get_output_type = (self: LinkComponent, index?: number): LinkType => {
    index = index ?? 0;
    const max = link_component_count_outputs(self);
    if (max <= index) {
        console.error('get_output_type index is not exist');
        throw new Error('invalid endpoint. index is not exist');
    }
    return match_link_component<LinkType>(self, {
        param: () => 'text',
        constant: (constant) => constant.output,
        form: (form) => form.output,
        code: (code) => code.output,
        identity: (identity) => component_identity_get_output_type(identity),
        call: (call) => call.output,
        interaction: (interaction) => component_interaction_get_output_type(interaction),
        view: () => {
            throw new Error('unreachable');
        }, // ? no output
        condition: () => {
            throw new Error('unreachable');
        }, // ? no output
        output: (output) => output.output,
        combined: (combined) => {
            const output = component_combined_get_output_type(combined);
            if (output) return output;
            throw new Error('unreachable'); // ? no output If there is no output, the specific value is not allowed
        },
    });
};

export const link_component_get_used_component = (self: LinkComponent): Set<ComponentId> => {
    const used = match_link_component<ComponentId[]>(self, {
        param: () => [],
        constant: () => [],
        form: (form) => component_form_get_used_component(form),
        code: (code) => component_code_get_used_component(code),
        identity: (identity) => component_identity_get_used_component(identity),
        call: (call) => component_call_get_used_component(call),
        interaction: (interaction) => component_interaction_get_used_component(interaction),
        view: (view) => component_view_get_used_component(view),
        condition: (condition) => component_condition_get_used_component(condition),
        output: (output) => component_output_get_used_component(output),
        combined: (combined) => component_combined_get_used_component(combined),
    });
    const set = new Set<ComponentId>();
    for (const id of used) set.add(id);
    return set;
};

export const link_component_is_triggered = (self: LinkComponent): boolean => {
    if ('call' in self) {
        const trigger = component_call_get_trigger(self.call);
        return match_component_call_trigger(trigger, {
            loading: () => false,
            clock: (clock) => clock.loading === false,
            click: () => true,
        });
    }
    return false;
};

export const link_component_get_all_branches = (
    self_id: ComponentId,
    components: LinkComponent[],
    cached: Record<ComponentId, AllBranches>,
): AllBranches | undefined => {
    if (cached[self_id]) return cached[self_id];

    const branches: AllBranch[] = [];
    for (const component of components) {
        const id = link_component_get_id(component);
        if (self_id === id) continue;

        const inlets = link_component_get_inlets(component);
        if (!inlets) continue;

        for (const inlet of inlets) {
            if (inlet.id === self_id) {
                branches.push({
                    id,
                    component,
                    inlets: link_component_get_all_branches(id, components, cached),
                });
            }
        }
    }
    const r = branches.length ? { branches } : undefined;

    if (r) cached[self_id] = r;

    return r;
};

export const link_component_get_validate_form = (self: LinkComponent): ValidateForm | undefined => {
    return match_link_component<ValidateForm | undefined>(self, {
        param: () => undefined,
        constant: () => undefined,
        form: (form) => form.metadata?.validate,
        code: () => undefined,
        identity: () => undefined,
        call: () => undefined,
        interaction: (interaction) => component_interaction_get_validate_form(interaction),
        view: () => undefined,
        output: () => undefined,
        condition: () => undefined,
        combined: () => undefined,
    });
};

export const link_component_get_param_name = (self: LinkComponent): string | undefined => {
    if ('param' in self) return self.param.metadata.name;
    return undefined;
};

export const link_component_get_form_name = (self: LinkComponent): string | undefined => {
    if ('form' in self) return self.form.metadata?.name;
    return undefined;
};

export const link_component_get_identity_name = (self: LinkComponent): string | undefined => {
    if ('identity' in self) return self.identity.metadata.name;
    return undefined;
};

export const link_component_get_interaction_name = (self: LinkComponent): string | undefined => {
    if ('interaction' in self) return self.interaction.metadata.name;
    return undefined;
};
