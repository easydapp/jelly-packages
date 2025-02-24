import { ethers } from 'ethers';

import { get_cached_call_result } from '../../..';
import { deepClone } from '../../../../../../common/clones';
import { CallingData, EvmActionData } from '../../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../../../store/api';
import { EvmApi } from '../../../../../../store/api/content/evm';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { EvmCallApi } from '../../../../../common/api/evm';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { input_value_get_used_component, InputValue } from '../../../../../common/refer';
import { transform_result } from '../../../../../common/transform';
import { EvmWallet } from '../../../../../common/wallet/evm';
import { EvmChain } from '../../../../../types/evm';
import { ComponentIdentityEvmValue } from '../../../../identity/evm';
import { check_evm_abi_item } from './abi';
import { check_evm_api } from './api';
import { check_evm_args, evm_call_arg_get_used_component, EvmCallArg } from './arg';
import { check_evm_contract } from './contract';
import { check_evm_outputs, check_evm_ret, EvmCallRet } from './ret';

export type ExecuteEvmActionCall = (param: {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    contract: string;
    api: EvmApi;
    unwrapped: any[];
}) => Promise<any>;

export interface EvmActionCall {
    contract: InputValue;

    api: EvmCallApi;

    arg?: EvmCallArg;

    ret?: EvmCallRet;
}

export const evm_action_call_get_used_component = (self: EvmActionCall): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.contract));
    if (self.arg) used.push(...evm_call_arg_get_used_component(self.arg));
    return used;
};

export const call_evm_call_action = async (
    self: EvmActionCall,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    apis: Record<ApiDataAnchor, ApiData>,
    calling: CallingData,
    alive: number,
    identity_metadata: ComponentIdentityEvmValue,
    code_executor: CodeExecutor | undefined,
    execute_evm_action_call: ExecuteEvmActionCall | undefined,
) => {
    // 1. Target contract
    const contract = check_evm_contract(self.contract, runtime_values);
    if (contract === undefined) return undefined;

    // 2. get api
    const api = check_evm_api(self.api, apis);

    // 3. parse api
    const item = check_evm_abi_item(api);

    // 4. build actor
    const actor = new ethers.Contract(contract, [item], identity_metadata.provider);
    console.debug(`🚀 ~ call: ~ actor:`, actor, api, item);

    // 5. Constructive request parameters
    let data_of_args: any | undefined = undefined;
    const unwrapped = await check_evm_args(
        item,
        self.arg,
        runtime_values,
        endpoints,
        codes,
        (data) => (data_of_args = data),
        code_executor,
    );
    if (unwrapped === undefined) return undefined;

    // 6. Request
    const key: EvmActionData = {
        call: {
            chain: identity_metadata.chain,
            account: identity_metadata.account,
            contract,
            method: item.name ?? '',
            args: unwrapped,
        },
    };
    let result = await get_cached_call_result(
        id,
        key,
        alive,
        () => calling.start({ evm: deepClone(key) }), // ! Start call
        (call_index: number, result: any) => calling.result(call_index, deepClone(result)), // ! Save the call result
        (call_index: number) => calling.over(call_index), // ! End call
        async () => {
            console.debug(`🚀 ~ call evm key:`, key);
            const result = await (async () => {
                try {
                    // ! Agent intercept execution, if necessary
                    if (execute_evm_action_call && identity_metadata.secret === undefined) {
                        return await execute_evm_action_call({
                            chain: identity_metadata.chain,
                            chain_id: identity_metadata.chain_id,
                            wallet: identity_metadata.wallet,
                            secret: identity_metadata.secret,
                            contract,
                            api,
                            unwrapped,
                        });
                    }

                    console.error('before evm call', contract, unwrapped, deepClone(unwrapped));
                    const result: any = await new Promise((resolve, reject) => {
                        actor[item.name ?? '']
                            .staticCall(...deepClone(unwrapped))
                            .then(resolve)
                            .catch(reject);
                    });
                    console.debug(`🚀 ~ result ~ result:`, result);
                    console.error('after evm call', contract);
                    return result;
                } catch (e) {
                    console.error(`🚀 ~ call evm failed:`, e);
                    throw e;
                } finally {
                    console.error('finally evm call', contract);
                }
            })();
            return transform_result(result);
        },
    );

    // 7. Converting result
    result = check_evm_outputs(item, result);

    console.debug(`call evm ~ response:`, contract, item, unwrapped, result);

    // 8. Post -treatment
    result = check_evm_ret(self.ret, result, codes, unwrapped, data_of_args, code_executor);

    if (result == undefined) throw new Error(`result of evm call can not be undefined`);
    return result;
};
