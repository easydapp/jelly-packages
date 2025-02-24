import { RuntimeValues } from '../../../../../../runtime/value';
import { InputValue } from '../../../../../common/refer';
import { evm_bytecode_check } from '../../../../../types/abi/types';

export const check_evm_bytecode = (
    ref_bytecode: InputValue | undefined,
    runtime_values: RuntimeValues,
): string | undefined => {
    let bytecode: string | undefined = undefined;
    if (ref_bytecode) {
        bytecode = runtime_values.find_input_value<string>(ref_bytecode, 'text');
        if (bytecode === undefined) return undefined;
        if (!evm_bytecode_check(bytecode)) {
            throw new Error(`invalid bytecode: ${bytecode}`);
        }
    }
    return bytecode;
};
