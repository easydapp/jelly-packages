import { RuntimeValues } from '../../../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../../../store/code';
import { CodeExecutor } from '../../../../../../wasm';
import { CodeContent } from '../../../../../common/code';
import { ComponentId } from '../../../../../common/identity';
import { AllEndpoints } from '../../../../../common/lets';
import { code_value_get_used_component, CodeValue } from '../../../../../common/refer';
import { doFunctionTransformByCodeContent } from '../../../../code';

export interface EvmDeployInitialCode {
    data?: CodeValue[];
    code: CodeContent;
}

export const evm_deploy_initial_code_get_used_component = (self: EvmDeployInitialCode): ComponentId[] => {
    const used: ComponentId[] = [];
    for (const code_value of self.data ?? []) {
        used.push(...code_value_get_used_component(code_value));
    }
    return used;
};

export interface EvmDeployInitial {
    code: EvmDeployInitialCode;
}

export const match_evm_deploy_initial = <T>(
    self: EvmDeployInitial,
    { code }: { code: (code: { data?: CodeValue[]; code: CodeContent }) => T },
): T => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm deploy initial');
};

export const match_evm_deploy_initial_async = async <T>(
    self: EvmDeployInitial,
    { code }: { code: (code: { data?: CodeValue[]; code: CodeContent }) => Promise<T> },
): Promise<T> => {
    if ('code' in self) return code(self.code);
    throw new Error('invalid evm deploy initial');
};

export const evm_action_deploy_initial_get_used_component = (self: EvmDeployInitial): ComponentId[] => {
    const used: ComponentId[] = [];
    match_evm_deploy_initial(self, {
        code: (code) => used.push(...evm_deploy_initial_code_get_used_component(code)),
    });
    return used;
};

export const check_evm_deploy_initial = async (
    initial: EvmDeployInitial,
    runtime_values: RuntimeValues,
    endpoints: AllEndpoints | undefined,
    codes: Record<CodeDataAnchor, CodeData>,
    code_executor: CodeExecutor | undefined,
): Promise<any[] | undefined> => {
    const unwrapped: any[] = [];
    const flag = await match_evm_deploy_initial_async(initial, {
        code: async (code) => {
            const data = runtime_values.find_data(endpoints, code.data ?? []);
            if (data === undefined) return undefined;
            const value = await doFunctionTransformByCodeContent(code.code, codes, [['data', data]], code_executor);
            for (const v of value) unwrapped.push(v);
            return unwrapped;
        },
    });
    if (flag === undefined) return undefined; // failed
    return unwrapped;
};
