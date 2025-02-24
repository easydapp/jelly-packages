import { CallingData } from '../../../../../runtime/calling';
import { RuntimeValues } from '../../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../../store/api';
import { CodeData, CodeDataAnchor } from '../../../../../store/code';
import { CodeExecutor } from '../../../../../wasm';
import { ComponentId } from '../../../../common/identity';
import { AllEndpoints } from '../../../../common/lets';
import { input_value_get_used_component, InputValue } from '../../../../common/refer';
import { evm_value_exp } from '../../../../types/abi/types';
import { ComponentIdentityEvmValue } from '../../../identity/evm';
import {
    call_evm_call_action,
    evm_action_call_get_used_component,
    EvmActionCall,
    ExecuteEvmActionCall,
} from './call';
import {
    call_evm_deploy_action,
    evm_action_deploy_get_used_component,
    EvmActionDeploy,
    ExecuteEvmActionDeploy,
} from './deploy';
import { call_evm_sign_action } from './sign';
import {
    call_evm_transaction_action,
    evm_action_transaction_get_used_component,
    EvmActionTransaction,
    ExecuteEvmActionTransaction,
    ExecuteEvmActionTransactionEstimateGas,
} from './transaction';
import {
    call_evm_transfer_action,
    evm_action_transfer_get_used_component,
    EvmActionTransfer,
    ExecuteEvmActionTransfer,
} from './transfer';

export type EvmAction =
    | { call: EvmActionCall }
    | { sign: InputValue }
    | { transaction: EvmActionTransaction }
    | { deploy: EvmActionDeploy }
    | { transfer: EvmActionTransfer };

export const match_evm_action = <T>(
    self: EvmAction,
    {
        call,
        sign,
        transaction,
        deploy,
        transfer,
    }: {
        call: (call: EvmActionCall) => T;
        sign: (sign: InputValue) => T;
        transaction: (transaction: EvmActionTransaction) => T;
        deploy: (deploy: EvmActionDeploy) => T;
        transfer: (transfer: EvmActionTransfer) => T;
    },
): T => {
    if ('call' in self) return call(self.call);
    if ('sign' in self) return sign(self.sign);
    if ('transaction' in self) return transaction(self.transaction);
    if ('deploy' in self) return deploy(self.deploy);
    if ('transfer' in self) return transfer(self.transfer);
    throw new Error('invalid evm action');
};

export const match_evm_action_async = async <T>(
    self: EvmAction,
    {
        call,
        sign,
        transaction,
        deploy,
        transfer,
    }: {
        call: (call: EvmActionCall) => Promise<T>;
        sign: (sign: InputValue) => Promise<T>;
        transaction: (transaction: EvmActionTransaction) => Promise<T>;
        deploy: (deploy: EvmActionDeploy) => Promise<T>;
        transfer: (transfer: EvmActionTransfer) => Promise<T>;
    },
): Promise<T> => {
    if ('call' in self) return call(self.call);
    if ('sign' in self) return sign(self.sign);
    if ('transaction' in self) return transaction(self.transaction);
    if ('deploy' in self) return deploy(self.deploy);
    if ('transfer' in self) return transfer(self.transfer);
    throw new Error('invalid evm action');
};

export const evm_action_get_used_component = (self: EvmAction): ComponentId[] => {
    const used: ComponentId[] = [];
    match_evm_action(self, {
        call: (call) => used.push(...evm_action_call_get_used_component(call)),
        sign: (sign) => used.push(...input_value_get_used_component(sign)),
        transaction: (transaction) =>
            used.push(...evm_action_transaction_get_used_component(transaction)),
        deploy: (deploy) => used.push(...evm_action_deploy_get_used_component(deploy)),
        transfer: (transfer) => used.push(...evm_action_transfer_get_used_component(transfer)),
    });
    return used;
};

export const call_evm_action = async (
    self: EvmAction,
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
    execute_evm_action_transaction_estimate_gas: ExecuteEvmActionTransactionEstimateGas | undefined,
    execute_evm_action_transaction: ExecuteEvmActionTransaction | undefined,
    execute_evm_action_deploy: ExecuteEvmActionDeploy | undefined,
    execute_evm_action_transfer: ExecuteEvmActionTransfer | undefined,
) => {
    return await match_evm_action_async(self, {
        call: async (call) =>
            await call_evm_call_action(
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
                execute_evm_action_call,
            ),
        sign: async (sign) =>
            await call_evm_sign_action(sign, runtime_values, calling, identity_metadata),
        transaction: async (transaction) =>
            await call_evm_transaction_action(
                transaction,
                id,
                endpoints,
                runtime_values,
                codes,
                apis,
                calling,
                alive,
                identity_metadata,
                code_executor,
                execute_evm_action_transaction_estimate_gas,
                execute_evm_action_transaction,
            ),
        deploy: async (deploy) =>
            await call_evm_deploy_action(
                deploy,
                id,
                endpoints,
                runtime_values,
                codes,
                calling,
                alive,
                identity_metadata,
                code_executor,
                execute_evm_action_deploy,
            ),
        transfer: async (transfer) =>
            await call_evm_transfer_action(
                transfer,
                id,
                runtime_values,
                calling,
                alive,
                identity_metadata,
                execute_evm_action_transfer,
            ),
    });
};

export const check_evm_pay_value = (
    ref_pay_value: InputValue | undefined,
    runtime_values: RuntimeValues,
): string | undefined => {
    let pay_value: string | undefined = undefined;
    if (ref_pay_value) {
        pay_value = runtime_values.find_input_value<string>(ref_pay_value, 'text');
        if (pay_value === undefined) return undefined;
        // ! unit is ETH
        pay_value = evm_value_exp(pay_value, 18);
        if (!pay_value) throw new Error(`invalid pay value: ${pay_value}`);
    }
    return pay_value;
};

export const check_evm_gas_limit = (
    ref_gas_limit: InputValue | undefined,
    runtime_values: RuntimeValues,
): number | undefined => {
    let gas_limit: number | undefined = undefined;
    if (ref_gas_limit) {
        gas_limit = runtime_values.find_input_value<number>(ref_gas_limit, 'integer');
        if (gas_limit === undefined) return undefined;
        if (gas_limit <= 0) {
            throw new Error(`invalid gas limit: ${gas_limit}`);
        }
    }
    return gas_limit;
};

export const check_evm_gas_price = (
    ref_gas_price: InputValue | undefined,
    runtime_values: RuntimeValues,
): string | undefined => {
    let gas_price: string | undefined = undefined;
    if (ref_gas_price) {
        gas_price = runtime_values.find_input_value<string>(ref_gas_price, 'text');
        if (gas_price === undefined) return undefined;
        // ! uint is GWei
        gas_price = evm_value_exp(gas_price, 9);
        if (!gas_price) throw new Error(`invalid gas price: ${gas_price}`);
    }
    return gas_price;
};

export const check_evm_nonce = (
    ref_nonce: InputValue | undefined,
    runtime_values: RuntimeValues,
): number | undefined => {
    let nonce: number | undefined = undefined;
    if (ref_nonce) {
        nonce = runtime_values.find_input_value<number>(ref_nonce, 'integer');
        if (nonce === undefined) return undefined;
        if (nonce < 0) {
            throw new Error(`invalid nonce: ${nonce}`);
        }
    }
    return nonce;
};

export const handle_evm_wallet_error = (e: any): boolean => {
    const error = `${e}`;
    if (error.includes('execution reverted')) throw new Error('transaction would be failed');
    if (error.includes('ACTION_REJECTED')) return true;
    // if (error.includes('REPLACEMENT_UNDERPRICED')) return true; // Gas price is too low and cannot be replaced
    // if (error.includes('NONCE_EXPIRED')) return true; // NONCE has used
    // if (error.includes('UNKNOWN_ERROR')) return true;
    return false;
};
