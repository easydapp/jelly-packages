import { ethers } from 'ethers';

import {
    check_evm_gas_limit,
    check_evm_gas_price,
    check_evm_nonce,
    check_evm_pay_value,
    handle_evm_wallet_error,
} from '.';
import { get_cached_call_result } from '../..';
import { deepClone } from '../../../../../common/clones';
import { CallingData, EvmActionData } from '../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../../store/api';
import { EvmApi } from '../../../../../store/api/content/evm';
import { CodeData, CodeDataAnchor } from '../../../../../store/code';
import { CodeExecutor } from '../../../../../wasm';
import { EvmCallApi } from '../../../../common/api/evm';
import { ComponentId } from '../../../../common/identity';
import { AllEndpoints } from '../../../../common/lets';
import { input_value_get_used_component, InputValue } from '../../../../common/refer';
import { EvmWallet } from '../../../../common/wallet/evm';
import { AbiItem } from '../../../../types/abi';
import { EvmChain } from '../../../../types/evm';
import { ComponentIdentityEvmValue } from '../../../identity/evm';
import { check_evm_abi_item } from './call/abi';
import { check_evm_api } from './call/api';
import { check_evm_args, evm_call_arg_get_used_component, EvmCallArg } from './call/arg';
import { check_evm_contract } from './call/contract';

export type ExecuteEvmActionTransactionEstimateGas = (param: {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    contract: string;
    pay_value: string | undefined;
    gas_limit: number | undefined;
    gas_price: string | undefined;
    nonce: number | undefined;
    api: EvmApi;
    unwrapped: any[];
}) => Promise<void>;

export type ExecuteEvmActionTransaction = (param: {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    contract: string;
    pay_value: string | undefined;
    gas_limit: number | undefined;
    gas_price: string | undefined;
    nonce: number | undefined;
    api: EvmApi;
    unwrapped: any[];
}) => Promise<string>;

export interface EvmActionTransaction {
    contract: InputValue;
    pay_value?: InputValue;
    gas_limit?: InputValue;
    gas_price?: InputValue;
    nonce?: InputValue;
    api: EvmCallApi;
    arg?: EvmCallArg;
    // ret?: EvmCallRet; // No post-treatment
}

export const evm_action_transaction_get_used_component = (self: EvmActionTransaction): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.contract));
    if (self.pay_value) used.push(...input_value_get_used_component(self.pay_value));
    if (self.gas_limit) used.push(...input_value_get_used_component(self.gas_limit));
    if (self.gas_price) used.push(...input_value_get_used_component(self.gas_price));
    if (self.nonce) used.push(...input_value_get_used_component(self.nonce));
    if (self.arg) used.push(...evm_call_arg_get_used_component(self.arg));
    return used;
};

export const call_evm_transaction_action = async (
    self: EvmActionTransaction,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    apis: Record<ApiDataAnchor, ApiData>,
    calling: CallingData,
    alive: number,
    identity_metadata: ComponentIdentityEvmValue,
    code_executor: CodeExecutor | undefined,
    execute_evm_action_transaction_estimate_gas: ExecuteEvmActionTransactionEstimateGas | undefined,
    execute_evm_action_transaction: ExecuteEvmActionTransaction | undefined,
) => {
    // 1. Target contract
    const contract = check_evm_contract(self.contract, runtime_values);
    if (contract === undefined) return undefined;

    // 2. transfer
    const pay_value = check_evm_pay_value(self.pay_value, runtime_values);

    // 3. Limit GAS
    const gas_limit = check_evm_gas_limit(self.gas_limit, runtime_values);

    // 4. Limit Gas Price
    const gas_price = check_evm_gas_price(self.gas_price, runtime_values);

    // 5. Specify nonce
    const nonce = check_evm_nonce(self.nonce, runtime_values);

    // 6. get API
    const api = check_evm_api(self.api, apis);

    // 7. parse API
    const item = check_evm_abi_item(api, false);

    // 8. build actor
    const actor = new ethers.Contract(contract, [item], identity_metadata.signer);

    console.debug(`🚀 ~ call: ~ actor:`, actor, pay_value, gas_limit, gas_price, nonce, api, item);

    // 9. Constructive request parameters
    let _data_of_args: any | undefined = undefined;
    const unwrapped = await check_evm_args(
        item,
        self.arg,
        runtime_values,
        endpoints,
        codes,
        (data) => (_data_of_args = data),
        code_executor,
    );
    if (unwrapped === undefined) return undefined;

    console.debug(`🚀 ~ call: ~ unwrapped:`, actor, unwrapped);

    // 10. Estimated Gas Limit
    // ! Agent intercept execution, if necessary
    if (execute_evm_action_transaction_estimate_gas && identity_metadata.secret === undefined) {
        await execute_evm_action_transaction_estimate_gas({
            chain: identity_metadata.chain,
            chain_id: identity_metadata.chain_id,
            wallet: identity_metadata.wallet,
            secret: identity_metadata.secret,
            contract,
            pay_value,
            gas_limit,
            gas_price,
            nonce,
            api,
            unwrapped,
        });
    } else {
        await estimate_gas(gas_limit, actor, item, unwrapped, pay_value);
    }

    // 11. Request
    const key: EvmActionData = {
        transaction: {
            chain: identity_metadata.chain,
            account: identity_metadata.account,
            contract,
            method: item.name ?? '',
            args: unwrapped,
            pay_value,
            gas_limit,
            gas_price,
            nonce,
        },
    };
    const result = await get_cached_call_result(
        id,
        key,
        alive,
        () => calling.start({ evm: deepClone(key) }), // ! Start call
        (call_index: number, hash: string) => calling.result(call_index, deepClone(hash)), // ! Save the call result
        (call_index: number) => calling.over(call_index), // ! End call
        async () => {
            console.debug(`🚀 ~ call evm key:`, key);

            let response: string;
            try {
                console.error('before evm transaction', contract);

                response = await (async () => {
                    // ! Agent intercept execution, if necessary
                    if (execute_evm_action_transaction && identity_metadata.secret === undefined) {
                        return await execute_evm_action_transaction({
                            chain: identity_metadata.chain,
                            chain_id: identity_metadata.chain_id,
                            wallet: identity_metadata.wallet,
                            secret: identity_metadata.secret,
                            contract,
                            pay_value,
                            gas_limit,
                            gas_price,
                            nonce,
                            api,
                            unwrapped,
                        });
                    }

                    const result: ethers.ContractTransactionResponse = await new Promise((resolve, reject) => {
                        actor[item.name ?? ''](...unwrapped, {
                            value: pay_value,
                            gasLimit: gas_limit,
                            gasPrice: gas_price,
                            nonce: nonce,
                        })
                            .then(resolve)
                            .catch(reject);
                    });
                    console.debug('waiting for confirms', result.hash);

                    await result.wait(1, 180000); // * Wait 1 confirmation

                    console.debug(`🚀 ~ result ~ result:`, result);

                    return result.hash;
                })();
                console.error('after evm transaction', contract);
            } catch (e) {
                console.error(`🚀 ~ call evm failed:`, e);
                if (handle_evm_wallet_error(e)) return undefined;
                throw e;
            } finally {
                console.error('finally evm transaction', contract);
            }
            console.debug(`call evm ~ response:`, contract, item, unwrapped, response);

            // return check_ret(transaction.ret, response, codes); // No post-treatment
            return response;
        },
    );

    // if (result == undefined)
    //     throw new Error(`result of evm transaction can not be undefined`);
    return result;
};

export const estimate_gas = async (
    gas_limit: number | undefined,
    actor: ethers.Contract,
    item: AbiItem,
    unwrapped: any[],
    pay_value: string | undefined,
) => {
    let estimate_gas_limit: bigint | undefined = undefined;
    if (gas_limit !== undefined) {
        try {
            estimate_gas_limit = await actor[item.name ?? ''].estimateGas(...unwrapped, { value: pay_value });
        } catch (e) {
            console.error(`🚀 ~ call evm failed:`, e);
            if (handle_evm_wallet_error(e)) return undefined;
            throw e;
        }
        if (BigInt(`${gas_limit}`) < estimate_gas_limit) {
            const error = `estimate gas limit ${estimate_gas_limit} is less than input gas limit ${gas_limit}`;
            console.error(error);
            throw new Error(error);
        }
    }
    console.debug(`🚀 ~ transaction: ~ estimate_gas_limit:`, estimate_gas_limit);
};
