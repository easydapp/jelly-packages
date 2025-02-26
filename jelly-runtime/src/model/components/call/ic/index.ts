import _ from 'lodash';

import { get_identity_value_by_id } from '..';
import { CallingData } from '../../../../runtime/calling';
import { RuntimeValues } from '../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../store/api';
import { CodeData, CodeDataAnchor } from '../../../../store/code';
import { CodeExecutor, ParseFuncCandid, ParseServiceCandid } from '../../../../wasm';
import {
    call_trigger_get_used_component,
    ComponentCallTrigger,
    match_component_call_trigger,
} from '../../../common/call_trigger';
import { ComponentId } from '../../../common/identity';
import { AllEndpoints } from '../../../common/lets';
import { ComponentIdentityValue } from '../../identity';
import { ComponentIdentityIcValue, identity_ic_metadata_get_anonymous_value } from '../../identity/ic';
import { call_ic_action, ic_action_get_used_component, IcAction } from './action';
import { ExecuteIcActionCall } from './action/call';

export interface CallIcMetadata {
    trigger: ComponentCallTrigger;
    identity?: ComponentId;
    action: IcAction;
}

export const call_ic_metadata_get_used_component = (self: CallIcMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...call_trigger_get_used_component(self.trigger));
    if (self.identity) used.push(self.identity);
    used.push(...ic_action_get_used_component(self.action));
    return used;
};

export const get_call_ic_value = async (
    self: CallIcMetadata,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    trigger: ComponentId | undefined,
    identity_triggered: Record<ComponentId, boolean>,
    identity: Record<ComponentId, ComponentIdentityValue>,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    apis: Record<ApiDataAnchor, ApiData>,
    calling: CallingData,
    code_executor: CodeExecutor | undefined,
    parse_service_candid: ParseServiceCandid | undefined,
    parse_func_candid: ParseFuncCandid | undefined,
    execute_ic_action_call: ExecuteIcActionCall | undefined,
): Promise<any> => {
    // 0. cache
    const alive = match_component_call_trigger(self.trigger, {
        loading: (loading) => loading.alive ?? 120000,
        clock: (clock) => (id !== trigger ? undefined : clock.sleep),
        click: () => (id !== trigger ? undefined : 0),
    });
    if (alive === undefined) return undefined;

    // 1. identity
    if (identity_triggered[self.identity ?? id]) return undefined; // do not trigger again this time
    identity_triggered[self.identity ?? id] = true;

    calling.set_connecting(true); // ! Identity link
    let identity_metadata: ComponentIdentityIcValue | undefined;
    try {
        identity_metadata =
            self.identity === undefined
                ? identity_ic_metadata_get_anonymous_value()
                : (await get_identity_value_by_id<{ ic: ComponentIdentityIcValue }>(self.identity, identity))?.ic;
    } finally {
        calling.set_connecting(false); // ! Identity link
    }

    if (identity_metadata === undefined) return undefined;

    calling.set_identity_value({ ic: identity_metadata }); // ! Save identity

    // Check if the wallet is online
    if (!(await identity_metadata.is_connected())) throw new Error(`Wallet ${identity_metadata.wallet} is lost.`);

    // 2. do action
    const value = call_ic_action(
        self.action,
        id,
        endpoints,
        runtime_values,
        codes,
        apis,
        calling,
        alive,
        identity_metadata,
        code_executor,
        parse_service_candid,
        parse_func_candid,
        execute_ic_action_call,
    );

    return value;
};
