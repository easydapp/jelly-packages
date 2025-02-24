import { RuntimeValues } from '../../../../../../runtime/value';
import { InputValue } from '../../../../../common/refer';

export const check_evm_abi = (
    ref_abi: InputValue | undefined,
    runtime_values: RuntimeValues,
): string | undefined => {
    let abi: string | undefined = undefined;
    if (ref_abi) {
        abi = runtime_values.find_input_value<string>(ref_abi, 'text');
        if (abi === undefined) return undefined;
        // How to check ABI JSON?
    }
    return abi;
};
