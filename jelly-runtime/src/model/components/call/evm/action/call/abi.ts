import { EvmApi, match_evm_api } from '../../../../../../store/api/content/evm';
import { AbiItem } from '../../../../../types/abi';

export const check_evm_abi_item = (api: EvmApi, call: boolean): AbiItem => {
    return match_evm_api<AbiItem>(api, {
        single: (single) => JSON.parse(single.api),
        origin: (origin) => {
            let items: AbiItem[] = JSON.parse(origin.abi);

            // ! try match name
            const found = items.filter((item) => item.name === origin.name);
            if (found.length === 1) return found[0]; // ! must be unique

            // ! use index
            items = (call ? filter_call_methods : filter_transaction_methods)(items);
            const item =
                origin.index !== undefined ? items[origin.index] : items.find((item) => item.name === origin.name);
            if (!item) throw new Error('can not find function');
            return item;
        },
    });
};

export const filter_call_methods = (methods: AbiItem[]) => {
    return methods.filter(
        (m) =>
            m.name &&
            m.type === 'function' &&
            [
                'pure',
                'view',
                'nonpayable', // call can support modified calls, but not on the chain
                'payable', // call can support modified calls, but not on the chain
            ].includes(m.stateMutability ?? ''),
    );
};

export const filter_transaction_methods = (methods: AbiItem[]) => {
    return methods.filter(
        (m) => m.name && m.type === 'function' && ['nonpayable', 'payable'].includes(m.stateMutability ?? ''),
    );
};
