import { ApiDataAnchor } from '../../../store/api';
import { InternetComputerApi } from '../../../store/api/content/ic';

export type IcCallApi = { api: InternetComputerApi } | { anchor: ApiDataAnchor };

export const match_ic_call_api = <T>(
    self: IcCallApi,
    { api, anchor }: { api: (api: InternetComputerApi) => T; anchor: (anchor: ApiDataAnchor) => T },
): T => {
    if ('api' in self) return api(self.api);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('invalid ic call api');
};

export const match_ic_call_api_async = async <T>(
    self: IcCallApi,
    {
        api,
        anchor,
    }: {
        api: (api: InternetComputerApi) => Promise<T>;
        anchor: (anchor: ApiDataAnchor) => Promise<T>;
    },
): Promise<T> => {
    if ('api' in self) return api(self.api);
    if ('anchor' in self) return anchor(self.anchor);
    throw new Error('invalid ic call api');
};
