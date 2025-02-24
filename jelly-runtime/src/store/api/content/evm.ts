export interface SingleEvmApi {
    api: string;
}

export interface OriginEvmApi {
    abi: string;
    name: string;
    index?: number;
}

export type EvmApi = { single: SingleEvmApi } | { origin: OriginEvmApi };

export const match_evm_api = <T>(
    self: EvmApi,
    {
        single,
        origin,
    }: {
        single: (single: SingleEvmApi) => T;
        origin: (origin: OriginEvmApi) => T;
    },
): T => {
    if ('single' in self) return single(self.single);
    if ('origin' in self) return origin(self.origin);
    throw new Error('invalid evm api');
};
