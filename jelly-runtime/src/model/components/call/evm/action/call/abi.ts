import { EvmApi, match_evm_api } from '../../../../../../store/api/content/evm';
import { AbiItem } from '../../../../../types/abi';

export const check_evm_abi_item = (api: EvmApi): AbiItem => {
    return match_evm_api<AbiItem>(api, {
        single: (single) => JSON.parse(single.api),
        origin: (origin) => {
            const items: AbiItem[] = JSON.parse(origin.abi);
            const item =
                origin.index !== undefined
                    ? items[origin.index]
                    : items.find((item) => item.name === origin.name);
            if (!item) throw new Error('can not find function');
            return item;
        },
    });
};
