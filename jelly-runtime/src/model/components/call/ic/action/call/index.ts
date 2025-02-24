import { get_cached_call_result } from '../../..';
import { deepClone } from '../../../../../../common/clones';
import { CallingData, IcActionData } from '../../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../../../store/api';
import { InternetComputerApi } from '../../../../../../store/api/content/ic';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor, ParseFuncCandid, ParseServiceCandid } from '../../../../../../wasm';
import {
    check_wrapped_candid_value,
    mapping_func,
    WrappedCandidType,
    WrappedCandidTypeFunction,
} from '../../../../../../wasm/candid';
import { IcCallApi, match_ic_call_api } from '../../../../../common/api/ic';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { input_value_get_used_component, InputValue } from '../../../../../common/refer';
import { transform_result } from '../../../../../common/transform';
import { IcWallet } from '../../../../../common/wallet/ic';
import { ComponentIdentityIcValue } from '../../../../identity/ic';
import { parse_ic_api } from './api';
import { ic_call_arg_get_used_component, IcCallArg, parse_ic_call_action_arg } from './arg';
import { CanisterInfo } from './info';
import { IcCallRet, parse_by_ic_call_ret } from './ret';

export type ExecuteIcActionCall = (param: {
    wallet: IcWallet;
    secret: string | undefined; // undefined means wallet

    canister_id: string;
    api: InternetComputerApi;
    unwrapped: any[];
}) => Promise<any>;

export interface IcActionCall {
    canister_id: InputValue;
    info?: CanisterInfo;
    api: IcCallApi;
    arg?: IcCallArg;
    ret?: IcCallRet;
}

export const ic_action_call_get_used_component = (self: IcActionCall): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.canister_id));
    if (self.arg) used.push(...ic_call_arg_get_used_component(self.arg));
    return used;
};

export const call_ic_call_action = async (
    self: IcActionCall,
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
    // 1. Target jar
    const canister_id = runtime_values.find_input_value<string>(self.canister_id, 'text');
    if (canister_id === undefined) return undefined;

    // 2. get api
    const api = match_ic_call_api<InternetComputerApi>(self.api, {
        api: (api) => api,
        anchor: (anchor) => {
            const api = apis[anchor];
            if (!api) throw new Error(`can not find api data by anchor: ${anchor}`);
            if ('ic' in api.content) return api.content.ic;
            throw new Error(`can not find ic api data by anchor: ${anchor}`);
        },
    });

    // 3. parse api

    let _method_: string | undefined = undefined;
    let _func_: WrappedCandidTypeFunction | undefined = undefined;

    await parse_ic_api(
        api,
        (m, f) => {
            _method_ = m;
            _func_ = f;
        },
        parse_service_candid,
        parse_func_candid,
    );

    if (_method_ === undefined) throw new Error('can not get method by candid');
    if (_func_ === undefined) throw new Error('can not get func by candid');

    const method: string = _method_;
    const func: WrappedCandidTypeFunction = _func_;
    const arg: WrappedCandidType[] = func.args ?? [];
    const ret: WrappedCandidType[] = func.rets ?? [];

    console.debug('call ic', canister_id, method, func, arg, ret);

    const idl = {
        idlFactory: ({ IDL }: any) => IDL.Service({ [method]: mapping_func(IDL, func) }),
    };

    // 4. build actor
    let actor: any = undefined;
    try {
        actor = await identity_metadata.creator(idl.idlFactory, canister_id); // Get ACTOR
    } catch (e) {
        throw handle_error(identity_metadata, e);
    }

    // 5. Constructive request parameters
    // get argument
    // const arg = parseTypes((func as IDL.FuncClass).argTypes);
    let data_of_args: any | undefined = undefined;
    const unwrapped: any[] | undefined = await parse_ic_call_action_arg(
        self.arg,
        endpoints,
        runtime_values,
        codes,
        arg,
        (data) => (data_of_args = data),
        code_executor,
    );
    if (unwrapped === undefined) return undefined;

    // ? Do you check whether the parameter matches the corresponding type?
    for (let i = 0; i < arg.length; i++) {
        if (!check_wrapped_candid_value(arg[i], unwrapped[i])) {
            console.error('candid value not match 1', arg[i], unwrapped[i]);
            throw new Error('candid value not match');
        }
    }

    // 6. Request
    const key: IcActionData = {
        call: {
            caller: identity_metadata.owner,
            canister_id,
            method,
            args: unwrapped,
        },
    };
    const response = await get_cached_call_result(
        id,
        key,
        alive,
        () => calling.start({ ic: deepClone(key) }), // ! Start call
        (call_index: number, response: any) => calling.result(call_index, deepClone(response)), // ! Save the call result
        (call_index: number) => calling.over(call_index), // ! End call
        async () => {
            console.debug(`🚀 ~ call canister key:`, key);
            let response;
            try {
                response = await (async () => {
                    // ! Agent intercept execution, if necessary
                    if (execute_ic_action_call && identity_metadata.secret === undefined) {
                        return execute_ic_action_call({
                            wallet: identity_metadata.wallet,
                            secret: identity_metadata.secret,
                            canister_id,
                            api,
                            unwrapped,
                        });
                    }

                    return await actor[method](...deepClone(unwrapped));
                })();
            } catch (e) {
                throw handle_error(identity_metadata, e);
            }
            // console.debug(`call canister ~ response:`, canister_id, method, unwrapped, response);
            return transform_result(response);
        },
    );

    // ? Does the test match match the corresponding type?
    if (response) {
        if (ret.length === 1) {
            if (!check_wrapped_candid_value(ret[0], response)) {
                console.error('candid value not match 2', ret[0], response);
                throw new Error('candid value not match');
            }
        } else {
            for (let i = 0; i < ret.length; i++) {
                if (!check_wrapped_candid_value(ret[i], response[i])) {
                    console.error('candid value not match 3', ret[i], response[i]);
                    throw new Error('candid value not match');
                }
            }
        }
    }

    let result: any = undefined;
    // const results = parseTypes(func.retTypes);
    // 7. Post -treatment
    if (self.ret) {
        result = await parse_by_ic_call_ret(
            self.ret,
            codes,
            arg,
            {
                response,
                unwrapped,
                data_of_args,
            },
            code_executor,
        );
    } else {
        result = response; // don't need a tuple if it's a single result
    }

    if (result == undefined) throw new Error(`result of internet-computer call can not be undefined`);
    return result;
};

const handle_error = (identity_metadata: ComponentIdentityIcValue, e: any): any => {
    const msg = `${e}`;
    if ('plug' in identity_metadata.wallet && msg === 'No keychain found for account') {
        console.error('handle_error', e);
        return new Error(`Plug wallet is locked. Please unlock plug and try again.`);
    }
    if ('plug' in identity_metadata.wallet && msg.startsWith('Call was returned undefined, but type')) {
        console.error('handle_error', e);
        return new Error(`Plug wallet call failed, return undefined.`);
    }
    return e;
};
