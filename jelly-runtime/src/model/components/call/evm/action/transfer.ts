import {
    check_evm_gas_price,
    check_evm_nonce,
    check_evm_pay_value,
    handle_evm_wallet_error,
} from '.';
import { get_cached_call_result } from '../..';
import { deepClone } from '../../../../../common/clones';
import { CallingData, EvmActionData } from '../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../runtime/value';
import { ComponentId } from '../../../../common/identity';
import { input_value_get_used_component, InputValue } from '../../../../common/refer';
import { EvmWallet } from '../../../../common/wallet/evm';
import { evm_address_check, evm_tx_check } from '../../../../types/abi/types';
import { EvmChain } from '../../../../types/evm';
import { ComponentIdentityEvmValue } from '../../../identity/evm';

export type ExecuteEvmActionTransfer = (param: {
    chain: EvmChain;
    chain_id: number;
    wallet: EvmWallet;
    secret: string | undefined; // undefined means wallet

    transfer_to: string;
    pay_value: string;
    gas_price: string | undefined;
}) => Promise<string>;

export interface EvmActionTransfer {
    transfer_to: InputValue;
    pay_value: InputValue;
    gas_price?: InputValue;
    nonce?: InputValue;
}

export const evm_action_transfer_get_used_component = (self: EvmActionTransfer): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...input_value_get_used_component(self.transfer_to));
    used.push(...input_value_get_used_component(self.pay_value));
    if (self.gas_price) used.push(...input_value_get_used_component(self.gas_price));
    if (self.nonce) used.push(...input_value_get_used_component(self.nonce));
    return used;
};

export const call_evm_transfer_action = async (
    self: EvmActionTransfer,
    id: ComponentId,
    runtime_values: RuntimeValues,
    calling: CallingData,
    alive: number,
    identity_metadata: ComponentIdentityEvmValue,
    execute_evm_action_transfer: ExecuteEvmActionTransfer | undefined,
) => {
    const transfer_to = check_evm_transfer_to(self.transfer_to, runtime_values);
    if (transfer_to == undefined) return undefined;

    const pay_value = check_evm_pay_value(self.pay_value, runtime_values);
    if (pay_value == undefined) return undefined;

    const gas_price = check_evm_gas_price(self.gas_price, runtime_values);

    const nonce = check_evm_nonce(self.nonce, runtime_values);

    const tx = {
        to: transfer_to,
        value: pay_value,
        gasLimit: 21000, // Transfer to death
        gasPrice: gas_price,
        nonce: nonce,
    };
    console.debug(`🚀 ~ transfer: ~ tx:`, tx);

    const key: EvmActionData = {
        transfer: {
            chain: identity_metadata.chain,
            account: identity_metadata.account,
            transfer_to,
            pay_value,
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
                console.error('before evm transfer', tx);

                response = await (async () => {
                    // ! Agent intercept execution, if necessary
                    if (execute_evm_action_transfer && identity_metadata.secret === undefined) {
                        return await execute_evm_action_transfer({
                            chain: identity_metadata.chain,
                            chain_id: identity_metadata.chain_id,
                            wallet: identity_metadata.wallet,
                            secret: identity_metadata.secret,
                            transfer_to,
                            pay_value,
                            gas_price,
                        });
                    }

                    const transaction = await identity_metadata.signer.sendTransaction(tx);

                    console.debug('waiting for confirms', transaction.hash);
                    await transaction.wait(1, 180000); // * Wait 1 confirmation

                    const result = transaction.hash;

                    console.debug(`🚀 ~ result ~ result:`, result);

                    if (!evm_tx_check(result)) {
                        console.error('transfer failed', result, result);
                        throw new Error(`transfer failed: ${result}`);
                    }
                    console.error('after evm transfer', transaction);
                    return result;
                })();
            } catch (e) {
                console.error(`🚀 ~ call evm failed:`, e);
                if (handle_evm_wallet_error(e)) return undefined;
                throw e;
            } finally {
                console.error('finally evm transfer', tx);
            }
            console.debug(`call evm ~ response:`, response);
            return response;
        },
    );

    // if (result == undefined)
    //     throw new Error(`result of evm transaction can not be undefined`);
    return result;
};

const check_evm_transfer_to = (
    ref_transfer_to: InputValue,
    runtime_values: RuntimeValues,
): string | undefined => {
    const transfer_to = runtime_values.find_input_value<string>(ref_transfer_to, 'text');
    if (transfer_to === undefined) return undefined;
    if (!evm_address_check(transfer_to)) {
        throw new Error(`invalid transfer to address: ${transfer_to}`);
    }
    return transfer_to;
};
