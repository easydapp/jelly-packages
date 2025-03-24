import { link_type_is_match_js_value, LinkType } from '@jellypack/types/lib/types';
import { link_value_to_js_value } from '@jellypack/types/lib/values';

import {
    increment_combined_called,
    increment_dapp_called_by_token,
    IncrementCombinedCall,
    IncrementDappCalledByToken,
} from '../canisters/storage';
import { deepClone } from '../common/clones';
import { same } from '../common/same';
import { LinkComponent } from '../model';
import { match_component_call_trigger } from '../model/common/call_trigger';
import { ComponentId } from '../model/common/identity';
import {
    all_endpoints_find_all_trigger_interrupt_by_form,
    all_endpoints_has_output,
    AllBranches,
    AllEndpoints,
} from '../model/common/lets';
import { InputValue, match_input_value, refer_value_get_value } from '../model/common/refer';
import { match_validate_form_async } from '../model/common/validate';
import {
    link_component_count_outputs,
    link_component_get_all_branches,
    link_component_get_all_endpoints,
    link_component_get_id,
    link_component_get_output_type,
    link_component_get_validate_form,
    match_link_component,
    match_link_component_async,
} from '../model/components';
import { component_call_get_identity, component_call_get_trigger, get_call_value } from '../model/components/call';
import { ExecuteEvmActionCall } from '../model/components/call/evm/action/call';
import { ExecuteEvmActionDeploy } from '../model/components/call/evm/action/deploy';
import {
    ExecuteEvmActionTransaction,
    ExecuteEvmActionTransactionEstimateGas,
} from '../model/components/call/evm/action/transaction';
import { ExecuteEvmActionTransfer } from '../model/components/call/evm/action/transfer';
import { ExecuteHttpCall } from '../model/components/call/http';
import { ExecuteIcActionCall } from '../model/components/call/ic/action/call';
import { doFunctionTransformByCodeContent, get_code_value } from '../model/components/code';
import { component_condition_get_expected_component, get_condition_index } from '../model/components/condition';
import {
    component_identity_get_connect,
    component_identity_get_output_value,
    component_identity_has_value,
    ComponentIdentity,
    ComponentIdentityValue,
    match_identity_inner_metadata,
    PlainComponentIdentityValue,
} from '../model/components/identity';
import { ApiData, ApiDataAnchor } from '../store/api';
import { CodeData, CodeDataAnchor } from '../store/code';
import { Combined, CombinedAnchor } from '../store/combined';
import { DappAnchor } from '../store/dapp';
import { DappVerified } from '../store/dapp/access';
import { Publisher } from '../store/publisher';
import { CodeExecutor, ParseFuncCandid, ParseServiceCandid } from '../wasm';
import { CallingData } from './calling';
import { ComponentInfo } from './info';
import { EndpointOutput, RuntimeValues } from './value';

export class CombinedRuntime {
    dapp_anchor: DappAnchor; // dapp key
    verified?: DappVerified;
    publisher: Publisher; // Publisher
    combined: Combined; // combined
    combined_links: ComponentId[]; // all component id
    param: Record<string, string>; // query parameter
    identity: Record<ComponentId, ComponentIdentityValue>; // Preset identity
    identity_fetched: Record<ComponentId, PlainComponentIdentityValue>; // fetched identity
    codes: Record<CodeDataAnchor, CodeData>; // stored code
    apis: Record<ApiDataAnchor, ApiData>; // stored API
    combines: Record<CombinedAnchor, Combined>; // stored combined
    calling: Record<ComponentId, CallingData>; // record call data
    call_components: ComponentId[]; // all call id
    connecting: Record<ComponentId, boolean>; // Record whether the wallet is connected
    call: () => void; // After calling, the interface is adjusted
    update: () => void; // After updating the data, the interface is adjusted
    onError: (error: string) => void; // After an error occurs, the interface is adjusted

    components: Record<ComponentId, ComponentInfo>; // Details of each component

    clock_timer: Record<ComponentId, NodeJS.Timeout>; // All timing tasks
    runtime_values: RuntimeValues; // The value of all nodes running

    updating: boolean; // Whether the record is being updated
    timer?: NodeJS.Timeout; // Update

    latest_called: number;

    code_executor?: CodeExecutor;
    parse_service_candid?: ParseServiceCandid;
    parse_func_candid?: ParseFuncCandid;
    increment_combined_called?: IncrementCombinedCall;
    increment_dapp_called_by_token?: IncrementDappCalledByToken;
    execute_http_call?: ExecuteHttpCall;
    execute_ic_action_call?: ExecuteIcActionCall;
    execute_evm_action_call?: ExecuteEvmActionCall;
    execute_evm_action_transaction_estimate_gas?: ExecuteEvmActionTransactionEstimateGas;
    execute_evm_action_transaction?: ExecuteEvmActionTransaction;
    execute_evm_action_deploy?: ExecuteEvmActionDeploy;
    execute_evm_action_transfer?: ExecuteEvmActionTransfer;

    constructor(
        dapp_anchor: DappAnchor,
        verified: DappVerified,
        combined: Combined,
        publisher: Publisher,
        codes: Record<CodeDataAnchor, CodeData>,
        apis: Record<ApiDataAnchor, ApiData>,
        combines: Record<CombinedAnchor, Combined>,
        param: Record<string, string>,
        identity: Record<ComponentId, ComponentIdentityValue>,
        update: () => void,
        call: () => void,
        onError: (error: string) => void,
        proxy?: {
            code_executor?: CodeExecutor;
            parse_service_candid?: ParseServiceCandid;
            parse_func_candid?: ParseFuncCandid;
            increment_combined_called?: IncrementCombinedCall;
            increment_dapp_called_by_token?: IncrementDappCalledByToken;
            execute_http_call?: ExecuteHttpCall;
            execute_ic_action_call?: ExecuteIcActionCall;
            execute_evm_action_call?: ExecuteEvmActionCall;
            execute_evm_action_transaction_estimate_gas?: ExecuteEvmActionTransactionEstimateGas;
            execute_evm_action_transaction?: ExecuteEvmActionTransaction;
            execute_evm_action_deploy?: ExecuteEvmActionDeploy;
            execute_evm_action_transfer?: ExecuteEvmActionTransfer;
        },
    ) {
        this.dapp_anchor = dapp_anchor;
        this.verified = verified;
        this.publisher = publisher;
        this.combined = combined;
        this.combined_links = [];
        this.param = param;
        this.identity = identity;
        this.identity_fetched = {};
        this.codes = codes;
        this.apis = apis;
        this.combines = combines;
        this.calling = {};
        this.call_components = [];
        this.connecting = {};
        this.call = call;
        this.update = update;
        this.onError = onError;

        // component information
        this.components = {};
        const components: Record<ComponentId, LinkComponent> = {};
        for (const component of combined.components) {
            const id = link_component_get_id(component);
            components[id] = component;
            this.combined_links.push(id);
        }
        console.log('got all_component_id', this.combined_links);
        const all_endpoints: Record<ComponentId, AllEndpoints> = {};
        for (const id of this.combined_links) {
            const component = components[id];
            const endpoints = link_component_get_all_endpoints(component, components);
            if (endpoints) all_endpoints[id] = endpoints;
        }
        console.log('got all_endpoints', all_endpoints);
        const cached: Record<ComponentId, AllBranches> = {};
        const triggers = new Set<ComponentId>();
        for (const id of this.combined_links) {
            const component = components[id];

            // get all triggers
            if (
                match_link_component(component, {
                    param: () => false,
                    constant: () => false,
                    form: () => false,
                    code: () => false,
                    identity: (identity) =>
                        match_identity_inner_metadata(identity.metadata.metadata, {
                            http: () => false,
                            ic: (ic) => ic.connect !== undefined,
                            evm: (evm) => evm.connect !== undefined,
                        }),
                    call: (call) =>
                        match_component_call_trigger(component_call_get_trigger(call), {
                            loading: () => false,
                            clock: () => false,
                            click: () => true,
                        }),
                    interaction: () => true,
                    view: () => false,
                    output: () => false,
                    condition: () => false,
                    combined: () => false,
                })
            ) {
                triggers.add(id);
            }

            if ('call' in component) {
                const identity_id = component_call_get_identity(component.call);
                const identity_component = identity_id !== undefined ? components[identity_id] : undefined;
                this.calling[id] = new CallingData(
                    id,
                    component.call,
                    identity_id !== undefined
                        ? {
                              id: identity_id,
                              identity: (identity_component as { identity: ComponentIdentity }).identity,
                          }
                        : undefined,
                    call,
                ); // Initialization Data
                this.call_components.push(id);
            }

            const endpoints = all_endpoints[id];
            const branches = link_component_get_all_branches(id, combined.components, cached);
            this.components[id] = new ComponentInfo(id, component, endpoints, branches);
        }

        // set triggered_components
        for (const id of this.combined_links) {
            const component = components[id];

            if ('call' in component) {
                const trigger = component_call_get_trigger(component.call);
                if (!('loading' in trigger)) continue;

                const identity_id = component_call_get_identity(component.call);
                // find trigger
                const endpoints = all_endpoints[id];
                const ts = all_endpoints_find_all_trigger_interrupt_by_form(endpoints, triggers, identity_id);
                let triggered_component: ComponentId | undefined = undefined;
                if (ts.size === 1) {
                    ts.forEach((id) => (triggered_component = id));
                } else if (ts.size < 1 && identity_id !== undefined && triggers.has(identity_id)) {
                    triggered_component = identity_id;
                }
                if (triggered_component !== undefined) {
                    const info = this.components[triggered_component];
                    const set = info.triggered_components ?? new Set();
                    set.add(id);
                    info.triggered_components = set;
                }
            }
        }
        console.log('this.components', this.components);
        // set excepted
        for (const current of this.combined_links) {
            const excepted = new Set<ComponentId>();

            // Set the reference node check of this component through the follow -up TRIGGER
            for (const id of this.combined_links) {
                if (current === id) continue;
                if (this.components[id]?.identity_triggers?.has(current)) excepted.add(id);
            }

            // Conditional inspection
            const component = components[current];
            if ('condition' in component) {
                for (const id of component_condition_get_expected_component(component.condition)) {
                    excepted.add(id);
                }
            }

            if (excepted.size) this.components[current].excepted = excepted;
        }
        console.log('got excepted', this.components);

        this.clock_timer = {};
        this.runtime_values = new RuntimeValues();

        this.updating = false;
        this.timer = undefined;

        this.latest_called = 0;

        this.code_executor = proxy?.code_executor;
        this.parse_service_candid = proxy?.parse_service_candid;
        this.parse_func_candid = proxy?.parse_func_candid;
        this.increment_combined_called = proxy?.increment_combined_called;
        this.increment_dapp_called_by_token = proxy?.increment_dapp_called_by_token;
        this.execute_http_call = proxy?.execute_http_call;
        this.execute_ic_action_call = proxy?.execute_ic_action_call;
        this.execute_evm_action_call = proxy?.execute_evm_action_call;
        this.execute_evm_action_transaction_estimate_gas = proxy?.execute_evm_action_transaction_estimate_gas;
        this.execute_evm_action_transaction = proxy?.execute_evm_action_transaction;
        this.execute_evm_action_deploy = proxy?.execute_evm_action_deploy;
        this.execute_evm_action_transfer = proxy?.execute_evm_action_transfer;

        // Try get the value on first time
        this.refresh(false);
    }

    public start_clock() {
        for (const component of this.combined.components) {
            if ('call' in component) {
                const trigger = component_call_get_trigger(component.call);
                if ('clock' in trigger) {
                    const clock = trigger.clock;
                    const id = link_component_get_id(component);
                    if (clock.loading) {
                        setTimeout(() => {
                            console.debug('trigger call by clock loading', id, new Date());
                            this.refresh(false, id);
                        }, 333);
                    }
                    const timer = setInterval(() => {
                        console.debug('trigger call by clock', id, new Date());
                        this.refresh(false, id);
                    }, clock.sleep);
                    this.clock_timer[id] = timer;
                }
            }
        }
    }
    public stop_clock() {
        for (const timer of Object.values(this.clock_timer)) {
            clearInterval(timer);
        }
    }
    public stop_timer(): undefined {
        clearTimeout(this.timer);
        this.stop_clock();
        return undefined;
    }

    public set_update(update: () => void) {
        this.update = update;
    }
    public set_on_error(onError: (error: string) => void) {
        this.onError = onError;
    }

    /// trigger
    private called() {
        if (0 < this.latest_called) return;
        const now = new Date().getTime();
        this.latest_called = now;
        (this.increment_dapp_called_by_token ?? increment_dapp_called_by_token)(this.dapp_anchor, this.verified);
        (this.increment_combined_called ?? increment_combined_called)(this.combined.anchor);
    }

    private connect(id: ComponentId, connecting: boolean) {
        this.connecting[id] = connecting;
        this.call();
    }

    public should_show(id: ComponentId): boolean {
        const info = this.components[id];
        if (info === undefined) return true;
        if (info.endpoints === undefined) return true;
        return all_endpoints_has_output(info.endpoints, this.runtime_values, info.used, info.excepted);
    }

    public identity_triggered(id: ComponentId, trigger?: ComponentId): boolean {
        const info = this.components[id];
        if (info?.branches === undefined) return true;
        if (info?.identity_triggers === undefined) return true;

        if (trigger === undefined) return false;

        return info.identity_triggers.has(trigger);
    }

    private replace_value(
        id: ComponentId,
        index: number,
        value: any, // js value, not link value
        output: LinkType,
        name: string,
        spend: number, // time spend
        do_changed: () => void,
        do_marked: () => void,
    ) {
        if (
            this.runtime_values.replace(id, index, value, output, name, spend, (last, value) => {
                console.log(`component(${name}) ${id}.${index}`, last, '->', value, `spend: ${spend}ms`);
                do_changed();
                do_marked();
            })
        ) {
            if (['identity', 'call', 'condition'].includes(name)) {
                this.update();
            }
        }
    }
    private clean_condition(id: ComponentId, do_changed: () => void, do_marked: () => void) {
        const value = this.runtime_values.delete(id); // reset condition outputs
        if (value) {
            for (let index = 0; index < value.length(); index++) {
                const v = value.get(index);
                if (v !== undefined) {
                    console.log(`component(condition) ${id}.${index}`, v, '->', undefined);
                }
            }
            do_changed();
            do_marked();
            this.update();
        }
    }
    private async refresh_single_component(
        trigger: ComponentId | undefined,
        component: LinkComponent,
        do_changed: () => void,
        do_marked: () => void,
        refresh_values: RuntimeValues,
        inlets_values: Record<ComponentId, any[]>,
        identity_triggered: Record<ComponentId, boolean>,
    ) {
        const id = link_component_get_id(component);
        // ! If the reference component has no output, no output value should be release
        const info = this.components[id];
        if (
            info.endpoints &&
            !all_endpoints_has_output(
                info.endpoints,
                this.runtime_values,
                info.used,
                (() => {
                    if ('call' in component && trigger === id) return undefined; // The trigger of the call component must also consider exclusion items
                    return info.excepted;
                })(),
            )
        ) {
            const name = match_link_component(component, {
                param: () => undefined, // * unreachable
                constant: () => undefined, // * unreachable
                form: () => undefined, // ! The information entered by the user cannot be lost
                code: () => 'code',
                identity: () => undefined, // * unreachable
                call: () => 'call',
                interaction: () => undefined, // ! The information entered by the user cannot be lost
                view: () => 'view', // no output
                condition: () => 'condition', // No value can also enter
                output: () => 'output',
                combined: () => 'combined',
            });
            if (name === undefined) return;
            if (name === 'view') {
                this.replace_value(id, 0, undefined, 'bool', 'view', 0, do_changed, do_marked); // ? Once output
                return;
            }
            if (name === 'condition') {
                this.clean_condition(id, do_changed, do_marked);
                return;
            }
            // ! Only 1 output, do not consider multiple outputs
            const output_type = link_component_get_output_type(component);
            this.replace_value(id, 0, undefined, output_type, name, 0, do_changed, do_marked);
            return;
        }
        // try to find every outputs
        await match_link_component_async(component, {
            param: async (param) => {
                if (!this.runtime_values.has_component(id)) {
                    const output_type = link_component_get_output_type(component);
                    const value = deepClone(this.param[param.metadata.name] ?? param.metadata.default);
                    this.replace_value(id, 0, value, output_type, 'param', 0, do_changed, do_marked); // ? Only 1 output
                }
            },
            constant: async (constant) => {
                if (!this.runtime_values.has_component(id)) {
                    const output_type = link_component_get_output_type(component);
                    const value = deepClone(link_value_to_js_value(constant.metadata.value));
                    this.replace_value(id, 0, value, output_type, 'const', 0, do_changed, do_marked); // ? Only 1 output
                }
            },
            form: async (form) => {
                if (!this.runtime_values.has_component(id) && form.metadata?.default !== undefined) {
                    const output_type = link_component_get_output_type(component);
                    const value = deepClone(link_value_to_js_value(form.metadata.default));
                    this.replace_value(id, 0, value, output_type, 'form', 0, do_changed, do_marked); // ? Only 1 output
                }
            },
            code: async (code) => {
                // execute output only once
                const inlets = this.runtime_values.get_inlets_outputs(code.inlets);
                if (!refresh_values.has_component(id) || !same(inlets, inlets_values[id])) {
                    inlets_values[id] = inlets;
                    const output_type = link_component_get_output_type(component);
                    const start = new Date().getTime();
                    const value = await get_code_value(
                        code,
                        this.components[id]?.endpoints,
                        this.runtime_values,
                        this.codes,
                        this.code_executor,
                    );
                    const end = new Date().getTime();
                    const spend = end - start;
                    this.replace_value(id, 0, value, output_type, 'code', spend, do_changed, do_marked); // ? Only 1 output
                    refresh_values.set(id, 0, value, spend);
                }
            },
            identity: async (identity) => {
                if (!this.runtime_values.has_component(id)) {
                    // ! waiting for trigger if needs
                    if (
                        component_identity_has_value(identity, this.identity) || // Value can trigger
                        (trigger === id && !identity_triggered[id]) || // User click can trigger
                        (trigger && info.identity_triggers?.has(trigger)) || // The only dependencies can trigger
                        (!component_identity_get_connect(identity) && // No need to click on, and it is currently triggered by follow -up, and has not tried this round
                            this.identity_triggered(id, trigger) &&
                            !identity_triggered[id])
                    ) {
                        identity_triggered[id] = true;
                        const output_type = link_component_get_output_type(component);
                        const value = await component_identity_get_output_value(
                            identity,
                            this.identity,
                            this.identity_fetched,
                            (id: ComponentId, connecting: boolean) => this.connect(id, connecting),
                        );
                        this.call(); // Notification status column
                        this.replace_value(id, 0, value, output_type, 'identity', 0, do_changed, do_marked); // ? Only 1 output
                    }
                }
            },
            call: async (call) => {
                // execute output only once
                const component_trigger = component_call_get_trigger(call);
                const output = this.runtime_values.get_output(id, 0);
                if ('click' in component_trigger && trigger !== id && this.runtime_values.has_component(id) && output) {
                    // If it is not necessary, the result of the call of the button is not reset
                    refresh_values.set(id, 0, output.value, output.spend);
                } else {
                    const inlets = this.runtime_values.get_inlets_outputs(call.inlets);
                    if (!refresh_values.has_component(id) || !same(inlets, inlets_values[id])) {
                        inlets_values[id] = inlets;
                        const output_type = link_component_get_output_type(component);
                        const start = new Date().getTime();
                        let value = await get_call_value(
                            call,
                            this.components[id]?.endpoints,
                            trigger,
                            identity_triggered,
                            this.identity,
                            this.identity_fetched,
                            this.runtime_values,
                            this.apis,
                            this.codes,
                            this.calling[id],
                            this.code_executor,
                            this.parse_service_candid,
                            this.parse_func_candid,
                            this.execute_http_call,
                            this.execute_ic_action_call,
                            this.execute_evm_action_call,
                            this.execute_evm_action_transaction_estimate_gas,
                            this.execute_evm_action_transaction,
                            this.execute_evm_action_deploy,
                            this.execute_evm_action_transfer,
                        );
                        const end = new Date().getTime();
                        let spend = end - start;
                        if (value === undefined && id !== trigger && this.runtime_values.has_output(id, 0)) {
                            // console.error('call value is undefined', component);
                            const trigger = component_call_get_trigger(call);
                            if ('clock' in trigger) {
                                const output = this.runtime_values.get_output(id, 0);
                                if (output !== undefined) {
                                    value = output.value;
                                    spend = output.spend;
                                }
                            }
                        }
                        this.replace_value(id, 0, value, output_type, 'call', spend, do_changed, do_marked); // ? Only 1 output
                        refresh_values.set(id, 0, value, spend);
                    }
                }
            },
            interaction: async () => {
                // User action
                /* do nothing */
            },
            view: async () => {
                if (!refresh_values.has_component(id)) {
                    const output_type = 'bool';
                    this.replace_value(id, 0, true, output_type, 'view', 0, do_changed, do_marked); // ? Once output
                    refresh_values.set(id, 0, true, 0);
                }
            }, // no output
            condition: async (condition) => {
                // ! Once the conditional component changes the condition, you need to be re -calculated, so you need to record the input data of the conditional component, and then compare the comparison
                // execute output only once
                const inlets = this.runtime_values.get_inlets_outputs(condition.inlets);
                if (!refresh_values.has_component(id) || !same(inlets, inlets_values[id])) {
                    inlets_values[id] = inlets;
                    const output_type = 'bool';
                    const start = new Date().getTime();
                    const endpoints = this.components[id]?.endpoints;
                    let index = get_condition_index(condition, endpoints, this.runtime_values);
                    if (index === undefined) {
                        const end = new Date().getTime();
                        const spend = end - start;
                        this.clean_condition(id, do_changed, do_marked);
                        index = link_component_count_outputs(component);
                        // replace_value(id, index, true, output_type, 'condition', spend); // ? Multiple output
                        refresh_values.set(id, index, true, spend);
                    } else if (0 <= index && index < link_component_count_outputs(component)) {
                        const end = new Date().getTime();
                        const spend = end - start;
                        this.clean_condition(id, do_changed, do_marked);
                        this.replace_value(id, index, true, output_type, 'condition', spend, do_changed, do_marked); // ? Multiple output
                        refresh_values.set(id, index, true, spend);
                    }
                }
            },
            output: async (output) => {
                // execute output only once
                const inlets = this.runtime_values.get_inlets_outputs(output.inlets);
                if (!refresh_values.has_component(id) || !same(inlets, inlets_values[id])) {
                    inlets_values[id] = inlets;
                    const output_type = link_component_get_output_type(component);
                    const start = new Date().getTime();
                    const value = this.runtime_values.find_data(
                        this.components[id]?.endpoints,
                        output.metadata?.data ?? [],
                    );
                    const end = new Date().getTime();
                    const spend = end - start;
                    this.replace_value(id, 0, value, output_type, 'output', spend, do_changed, do_marked); // ? Only 1 output
                    refresh_values.set(id, 0, value, spend);
                }
            },
            combined: async () => {
                throw new Error(`unimplemented`);
            },
        });
    }

    public async refresh(
        changed: boolean, // changed content
        trigger?: ComponentId, // which Triggered component
    ) {
        if (this.updating) {
            console.log('setTimeout refresh');
            clearTimeout(this.timer);
            this.timer = setTimeout(() => this.refresh(changed, trigger), 100);
            return;
        }
        // do update
        this.updating = true;
        let mark = true;
        const do_changed = () => (changed = true);
        const do_mark = () => (mark = true);
        const refresh_values = new RuntimeValues();
        const inlets_values: Record<ComponentId, any[]> = {};
        const identity_triggered: Record<ComponentId, boolean> = {};
        const start = Date.now();
        try {
            do {
                mark = false;

                // * Single thread
                for (const component of this.combined.components) {
                    const s = Date.now();
                    await this.refresh_single_component(
                        trigger,
                        component,
                        do_changed,
                        do_mark,
                        refresh_values,
                        inlets_values,
                        identity_triggered,
                    );
                    const e = Date.now();
                    const spend = e - s;
                    if (30 < spend) {
                        console.debug('refresh_single_component spend', link_component_get_id(component), spend, 'ms');
                    }
                }

                // // * Concurrent execution
                // await Promise.all(
                //     this.combined.components.map((component) =>
                //         this.refresh_single_component(
                //             trigger,
                //             component,
                //             do_changed,
                //             do_mark,
                //             refresh_values,
                //             inlets_values,
                //             identity_triggered,
                //         ),
                //     ),
                // );
            } while (mark);
        } catch (e: any) {
            console.error('catch runtime error', e);
            this.onError(e.message);
        } finally {
            const end = Date.now();
            console.debug('refresh spend', end - start, 'ms');
        }
        console.error('runtime_values', this.runtime_values.outputs);
        this.updating = false;
        if (changed) {
            this.called();
            this.update();
        }
    }

    public async validate_form(id: ComponentId, value: any): Promise<string | undefined> {
        if (value === undefined) return undefined;
        const info = this.components[id];
        const validate = link_component_get_validate_form(info.component);
        if (validate === undefined) return undefined;
        const output = link_component_get_output_type(info.component);
        if (!link_type_is_match_js_value(output, value)) return 'Wrong value.';
        return match_validate_form_async(validate, {
            code: async (code) => {
                return await doFunctionTransformByCodeContent(code, this.codes, [['data', value]], this.code_executor);
            },
        });
    }

    public async refresh_form(id: ComponentId, value: any) {
        const info = this.components[id];
        if (!('form' in info.component)) throw new Error(`not a form: ${id}`);
        console.debug(`form`, id, '->', value);
        if (
            this.runtime_values.replace(
                id,
                0,
                undefined, // Preceding air
                link_component_get_output_type(info.component),
                'form',
                0,
            )
        ) {
            await this.refresh(true);
        }
        if (this.runtime_values.replace(id, 0, value, link_component_get_output_type(info.component), 'form', 0)) {
            await this.refresh(true);
        }
    }

    public async refresh_interaction(id: ComponentId, value: any) {
        const info = this.components[id];
        if (!('interaction' in info.component)) throw new Error(`not a interaction: ${id}`);
        console.debug(`interaction`, id, '->', value);
        if (
            this.runtime_values.replace(
                id,
                0,
                undefined, // Preceding air
                link_component_get_output_type(info.component),
                'interaction',
                0,
            )
        ) {
            await this.refresh(true);
        }
        if (
            this.runtime_values.replace(id, 0, value, link_component_get_output_type(info.component), 'interaction', 0)
        ) {
            await this.refresh(true);
        }
    }

    public links(): ComponentId[] {
        return this.combined_links;
    }

    public calls(): ComponentId[] {
        return this.call_components;
    }

    public get_call(id: ComponentId): CallingData {
        return this.calling[id];
    }

    public get_connect(id: ComponentId): boolean {
        return !!this.connecting[id];
    }

    public component(id: ComponentId): LinkComponent {
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        return this.combined.components.find((c) => link_component_get_id(c) === id)!;
    }

    public get_component_info(id: ComponentId): ComponentInfo {
        return this.components[id];
    }

    public update_component(_id: ComponentId, _updated: number) {
        // console.debug(`🚀 ~ CombinedRuntime ~ update_component ~ id:`, id, _updated);
        // this.updated[id] = true;
    }

    public find_value<T>(id: ComponentId, index: number): EndpointOutput<T> | undefined {
        return this.runtime_values.get_output(id, index);
    }

    public input_value<T>(input_value: InputValue, needs: LinkType[] | ((value: any) => boolean)): T | undefined {
        const value = match_input_value(input_value, {
            constant: (constant) => link_value_to_js_value(constant),
            refer: (refer) => {
                //  1. Check whether the reference component generates a value
                const outputs = this.runtime_values.get_outputs(refer.endpoint.id);
                if (outputs === undefined) return undefined;

                // 2. which output
                const index = refer.endpoint.index ?? 0;

                // 3. check output
                const output = outputs[index];
                if (output === undefined) return undefined;

                // 4. read
                return refer_value_get_value(refer, output);
            },
        });

        // 5. check type of value
        if (value !== undefined) {
            if (typeof needs === 'function') {
                if (needs(value)) return value;
            } else {
                for (const ty of needs) {
                    if (link_type_is_match_js_value(ty, value)) return value;
                }
            }
        }

        console.error('type is mismatch:', input_value, needs, value);
        match_input_value(input_value, {
            constant: (constant) => {
                throw new Error(`wrong value type for ${JSON.stringify(constant)}`);
            },
            refer: (refer) => {
                throw new Error(`wrong value type for ${refer.endpoint.id}.${refer.endpoint.index ?? 0}`);
            },
        });
    }
}
