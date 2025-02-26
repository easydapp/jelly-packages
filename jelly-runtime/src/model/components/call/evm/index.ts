import { get_identity_value_by_id } from '..';
import { CallingData } from '../../../../runtime/calling';
import { RuntimeValues } from '../../../../runtime/value';
import { ApiData, ApiDataAnchor } from '../../../../store/api';
import { CodeData, CodeDataAnchor } from '../../../../store/code';
import { CodeExecutor } from '../../../../wasm';
import {
    call_trigger_get_used_component,
    ComponentCallTrigger,
    match_component_call_trigger,
} from '../../../common/call_trigger';
import { ComponentId } from '../../../common/identity';
import { AllEndpoints } from '../../../common/lets';
import { EvmChain } from '../../../types/evm';
import { ComponentIdentityValue } from '../../identity';
import { ComponentIdentityEvmValue, identity_evm_metadata_get_anonymous_value } from '../../identity/evm';
import { call_evm_action, evm_action_get_used_component, EvmAction, match_evm_action } from './action';
import { ExecuteEvmActionCall } from './action/call';
import { ExecuteEvmActionDeploy } from './action/deploy';
import { ExecuteEvmActionTransaction, ExecuteEvmActionTransactionEstimateGas } from './action/transaction';
import { ExecuteEvmActionTransfer } from './action/transfer';

export interface CallEvmMetadata {
    trigger: ComponentCallTrigger;
    identity?: ComponentId;
    chain: EvmChain;
    action: EvmAction;
}

export const call_evm_metadata_get_used_component = (self: CallEvmMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...call_trigger_get_used_component(self.trigger));
    if (self.identity) used.push(self.identity);
    used.push(...evm_action_get_used_component(self.action));
    return used;
};

export const get_call_evm_value = async (
    self: CallEvmMetadata,
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
    execute_evm_action_call: ExecuteEvmActionCall | undefined,
    execute_evm_action_transaction_estimate_gas: ExecuteEvmActionTransactionEstimateGas | undefined,
    execute_evm_action_transaction: ExecuteEvmActionTransaction | undefined,
    execute_evm_action_deploy: ExecuteEvmActionDeploy | undefined,
    execute_evm_action_transfer: ExecuteEvmActionTransfer | undefined,
): Promise<any> => {
    // 0. cache
    const alive = match_component_call_trigger(self.trigger, {
        loading: (loading) =>
            match_evm_action(self.action, {
                call: () => loading.alive ?? 120000,
                sign: () => 0,
                transaction: () => 0,
                deploy: () => 0,
                transfer: () => 0,
            }),
        clock: (clock) => (id !== trigger ? undefined : clock.sleep),
        click: () => (id !== trigger ? undefined : 0),
    });
    if (alive === undefined) return undefined;

    // 1. identity
    if (identity_triggered[self.identity ?? id]) return undefined; // do not trigger again this time
    identity_triggered[self.identity ?? id] = true;

    calling.set_connecting(true); // ! Identity link
    let identity_metadata: ComponentIdentityEvmValue | undefined;
    try {
        identity_metadata =
            self.identity === undefined
                ? identity_evm_metadata_get_anonymous_value(self.chain)
                : (await get_identity_value_by_id<{ evm: ComponentIdentityEvmValue }>(self.identity, identity))?.evm;
    } finally {
        calling.set_connecting(false); // ! Identity link
    }

    if (identity_metadata === undefined) return undefined;

    calling.set_identity_value({ evm: identity_metadata }); // ! Save identity

    // Check if the wallet is online
    if (!(await identity_metadata.is_connected())) throw new Error(`Wallet ${identity_metadata.wallet} is lost.`);

    // 2. do action
    const value = await call_evm_action(
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
        execute_evm_action_call,
        execute_evm_action_transaction_estimate_gas,
        execute_evm_action_transaction,
        execute_evm_action_deploy,
        execute_evm_action_transfer,
    );

    return value;
};
