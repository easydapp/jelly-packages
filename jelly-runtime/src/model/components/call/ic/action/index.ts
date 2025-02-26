import { CallingData } from '../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../../store/api';
import { CodeData, CodeDataAnchor } from '../../../../../store/code';
import { CodeExecutor, ParseFuncCandid, ParseServiceCandid } from '../../../../../wasm';
import { ComponentId } from '../../../../common/identity';
import { AllEndpoints } from '../../../../common/lets';
import { ComponentIdentityIcValue } from '../../../identity/ic';
import { call_ic_call_action, ExecuteIcActionCall, ic_action_call_get_used_component, IcActionCall } from './call';

export interface IcAction {
    call: IcActionCall;
}

export const match_ic_action = <T>(
    self: IcAction,
    {
        call,
    }: {
        call: (call: IcActionCall) => T;
    },
): T => {
    if ('call' in self) return call(self.call);
    throw new Error('invalid ic action');
};

export const match_ic_action_async = async <T>(
    self: IcAction,
    {
        call,
    }: {
        call: (call: IcActionCall) => Promise<T>;
    },
): Promise<T> => {
    if ('call' in self) return call(self.call);
    throw new Error('invalid ic action');
};

export const ic_action_get_used_component = (self: IcAction): ComponentId[] => {
    const used: ComponentId[] = [];
    match_ic_action(self, {
        call: (call) => used.push(...ic_action_call_get_used_component(call)),
    });
    return used;
};

export const call_ic_action = async (
    self: IcAction,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    apis: Record<ApiDataAnchor, ApiData>,
    calling: CallingData,
    alive: number,
    identity_metadata: ComponentIdentityIcValue,
    code_executor: CodeExecutor | undefined,
    parse_service_candid: ParseServiceCandid | undefined,
    parse_func_candid: ParseFuncCandid | undefined,
    execute_ic_action_call: ExecuteIcActionCall | undefined,
) => {
    return await match_ic_action_async(self, {
        call: async (call) =>
            await call_ic_call_action(
                call,
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
            ),
    });
};
