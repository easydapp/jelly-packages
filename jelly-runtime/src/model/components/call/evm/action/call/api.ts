import { ApiData, ApiDataAnchor } from '../../../../../../store/api';
import { EvmApi } from '../../../../../../store/api/content/evm';
import { EvmCallApi, match_evm_call_api } from '../../../../../common/api/evm';

export const check_evm_api = (api: EvmCallApi, apis: Record<ApiDataAnchor, ApiData>): EvmApi => {
    return match_evm_call_api<EvmApi>(api, {
        api: (api) => api,
        anchor: (anchor) => {
            const api = apis[anchor];
            if (!api) throw new Error(`can not find api data by anchor: ${anchor}`);
            if ('evm' in api.content) return api.content.evm;
            throw new Error(`can not find evm api data by anchor: ${anchor}`);
        },
    });
};
