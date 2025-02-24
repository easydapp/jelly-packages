import { EvmApi } from './content/evm';
import { InternetComputerApi } from './content/ic';

export type ApiDataContent = { ic: InternetComputerApi } | { evm: EvmApi };

export const match_api_data_content = <T>(
    self: ApiDataContent,
    { ic, evm }: { ic: (ic: InternetComputerApi) => T; evm: (evm: EvmApi) => T },
): T => {
    if ('ic' in self) return ic(self.ic);
    if ('evm' in self) return evm(self.evm);
    throw new Error('invalid api data content');
};
