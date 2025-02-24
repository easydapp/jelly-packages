import { RuntimeValues } from '../../../../../../runtime/value';
import { InputValue } from '../../../../../common/refer';
import { evm_address_check } from '../../../../../types/abi/types';

export const check_evm_contract = (
    ref_contract: InputValue,
    runtime_values: RuntimeValues,
): string | undefined => {
    const contract = runtime_values.find_input_value<string>(ref_contract, 'text');
    if (contract === undefined) return undefined;
    if (!evm_address_check(contract)) {
        throw new Error(`invalid contract address: ${contract}`);
    }
    return contract;
};
