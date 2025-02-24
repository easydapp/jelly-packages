import { ApiDataAnchor } from '../../../store/api';
import { EvmApi } from '../../../store/api/content/evm';

export type EvmCallApi = { api: EvmApi } | { anchor: ApiDataAnchor };

export const match_evm_call_api = <T>(
    self: EvmCallApi,
    { api, anchor }: { api: (api: EvmApi) => T; anchor: (anchor: ApiDataAnchor) => T },
): T => {
    if ('api' in self) return api(self.api);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('invalid evm call api');
};

export const match_evm_call_api_async = async <T>(
    self: EvmCallApi,
    {
        api,
        anchor,
    }: { api: (api: EvmApi) => Promise<T>; anchor: (anchor: ApiDataAnchor) => Promise<T> },
): Promise<T> => {
    if ('api' in self) return api(self.api);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('invalid evm call api');
};
