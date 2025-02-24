import { ethers } from 'ethers';
import {
    check_evm_gas_limit,
    check_evm_gas_price,
    check_evm_nonce,
    handle_evm_wallet_error,
} from '..';
import { get_cached_call_result } from '../../..';
import { deepClone } from '../../../../../../common/clones';
import { CallingData, EvmActionData } from '../../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { input_value_get_used_component, InputValue } from '../../../../../common/refer';
import { EvmWallet } from '../../../../../common/wallet/evm';
import { AbiItem } from '../../../../../types/abi';
import {
    checkEvmValue,
    evm_address_check,
    evm_param_to_type,
    evm_tx_check,
} from '../../../../../types/abi/types';
import { EvmChain } from '../../../../../types/evm';
import { ComponentIdentityEvmValue } from '../../../../identity/evm';
import { check_evm_abi } from './abi';
import { check_evm_bytecode } from './bytecode';
import {
    check_evm_deploy_initial,
    evm_action_deploy_initial_get_used_component,
    EvmDeployInitial,
} from './initial';

export type ExecuteEvmActionDeploy = (param: {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    gas_limit: number | undefined;
    gas_price: string | undefined;
    nonce: number | undefined;
    abi: string;
    bytecode: string;
    initial: any[] | undefined;
}) => Promise<{ tx: string; address: string }>;

export interface EvmActionDeploy {
    // pay_value?: InputValue; // Unable to transfer contracts cannot be transferred
    gas_limit?: InputValue;
    gas_price?: InputValue;
    nonce?: InputValue;
    abi: InputValue;
    bytecode: InputValue;
    initial?: EvmDeployInitial;
}

export const evm_action_deploy_get_used_component = (self: EvmActionDeploy): ComponentId[] => {
    const used: ComponentId[] = [];
    // if (self.pay_value) used.push(...input_value_get_used_component(deploy.pay_value));
    if (self.gas_limit) used.push(...input_value_get_used_component(self.gas_limit));
    if (self.gas_price) used.push(...input_value_get_used_component(self.gas_price));
    if (self.nonce) used.push(...input_value_get_used_component(self.nonce));
    used.push(...input_value_get_used_component(self.abi));
    used.push(...input_value_get_used_component(self.bytecode));
    if (self.initial) used.push(...evm_action_deploy_initial_get_used_component(self.initial));
    return used;
};

export const call_evm_deploy_action = async (
    self: EvmActionDeploy,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    calling: CallingData,
    alive: number,
    identity_metadata: ComponentIdentityEvmValue,
    code_executor: CodeExecutor | undefined,
    execute_evm_action_deploy: ExecuteEvmActionDeploy | undefined,
) => {
    // const pay_value = check_pay_value(deploy.pay_value, runtime_values);

    // 1. Limit GAS
    const gas_limit = check_evm_gas_limit(self.gas_limit, runtime_values);

    // 2. Limit Gas Price
    const gas_price = check_evm_gas_price(self.gas_price, runtime_values);

    // 3. get nonce
    const nonce = check_evm_nonce(self.nonce, runtime_values);

    // 4. get ABI
    const abi = check_evm_abi(self.abi, runtime_values);
    if (abi === undefined) return undefined;

    // 5. get bytecode
    const bytecode = check_evm_bytecode(self.bytecode, runtime_values);
    if (bytecode === undefined) return undefined;

    // 6. get initial
    let initial: any[] | undefined = undefined;
    if (self.initial) {
        initial = await check_evm_deploy_initial(
            self.initial,
            runtime_values,
            endpoints,
            codes,
            code_executor,
        );
        if (initial === undefined) return undefined;
    }

    // 7. check contractor
    check_contract_contractor(abi, initial);

    // 8. build actor
    const actor = new ethers.ContractFactory(JSON.parse(abi), bytecode, identity_metadata.signer);
    console.debug(
        `🚀 ~ call: ~ actor:`,
        actor,
        gas_limit,
        gas_price,
        nonce,
        abi,
        bytecode,
        initial,
    );

    // 9. Request
    const key: EvmActionData = {
        deploy: {
            chain: identity_metadata.chain,
            account: identity_metadata.account,
            abi,
            bytecode,
            initial,
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
        (call_index: number, result: { tx: string; address: string }) =>
            calling.result(call_index, deepClone(result)), // ! Save the call result
        (call_index: number) => calling.over(call_index), // ! End call
        async () => {
            console.debug(`🚀 ~ call evm key:`, key);

            let response: { tx: string; address: string };
            try {
                console.error('before evm deploy', bytecode);

                response = await (async () => {
                    // ! Agent intercept execution, if necessary
                    if (execute_evm_action_deploy && identity_metadata.secret === undefined) {
                        return await execute_evm_action_deploy({
                            chain: identity_metadata.chain,
                            chain_id: identity_metadata.chain_id,
                            wallet: identity_metadata.wallet,
                            secret: identity_metadata.secret,
                            gas_limit,
                            gas_price,
                            nonce,
                            abi,
                            bytecode,
                            initial,
                        });
                    }

                    const contract =
                        initial !== undefined
                            ? await actor.deploy(...initial, {
                                  //   value: pay_value,
                                  gasLimit: gas_limit,
                                  gasPrice: gas_price,
                                  nonce: nonce,
                              })
                            : await actor.deploy({
                                  //   value: pay_value,
                                  gasLimit: gas_limit,
                                  gasPrice: gas_price,
                                  nonce: nonce,
                              });

                    console.debug('waiting for confirms', contract.deploymentTransaction()?.hash);

                    // * Waiting for confirmation
                    await Promise.race([
                        contract.waitForDeployment(),
                        new Promise((_, reject) => setTimeout(reject, 180000)),
                    ]);

                    const tx = contract.deploymentTransaction()?.hash;
                    // 0x2eb65adba3d6d79fff5b518241658e9632b72e97a9ebc0aff8e11fbb01356423
                    if (tx === undefined || !evm_tx_check(tx)) {
                        console.error('deploy contract failed', tx, contract);
                        throw new Error(`deploy contract failed: ${tx}`);
                    }
                    // 0xe1abbaf8cb8f4Da3658182fAe1A27E4f123d6634
                    const address =
                        typeof contract.target === 'string'
                            ? contract.target
                            : await contract.target.getAddress();
                    if (!evm_address_check(address)) {
                        console.error('deploy contract failed', address, contract);
                        throw new Error(`deploy contract failed: ${address}`);
                    }

                    const result = { tx, address };
                    console.debug(`🚀 ~ result ~ result:`, result);
                    console.error('after evm deploy', contract);
                    return result;
                })();
            } catch (e) {
                console.error(`🚀 ~ call evm failed:`, e);
                if (handle_evm_wallet_error(e)) return undefined;
                throw e;
            } finally {
                console.error('finally evm deploy', bytecode);
            }
            console.debug(`call evm ~ response:`, response);

            return response;
        },
    );

    // if (result == undefined)
    //     throw new Error(`result of evm transaction can not be undefined`);
    return result;
};

const check_contract_contractor = (abi: string, initial: any[] | undefined) => {
    const abi_items: AbiItem[] = JSON.parse(abi);
    const contractor_item = abi_items.find((item) => item.type === 'constructor');
    if (contractor_item && contractor_item.inputs) {
        if (initial === undefined) throw Error('contract deploy needs initial data');
        for (let i = 0; i < contractor_item.inputs.length; i++) {
            const type = evm_param_to_type(contractor_item.inputs[i]);
            if (!checkEvmValue(type, initial[i])) {
                console.error('evm initial value not match', type, initial[i]);
                throw new Error('evm initial value not match');
            }
        }
    } else {
        if (initial) throw Error('contract deploy has no initial data');
    }
};
