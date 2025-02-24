export interface SingleInternetComputerApi {
    api: string;
}

export interface OriginInternetComputerApi {
    candid: string;
    method: string;
}

export type InternetComputerApi =
    | { single: SingleInternetComputerApi }
    | { origin: OriginInternetComputerApi };

export const match_internet_computer_api = <T>(
    self: InternetComputerApi,
    {
        single,
        origin,
    }: {
        single: (single: SingleInternetComputerApi) => T;
        origin: (origin: OriginInternetComputerApi) => T;
    },
): T => {
    if ('single' in self) return single(self.single);
    if ('origin' in self) return origin(self.origin);
    throw new Error('invalid internet computer api');
};

export const match_internet_computer_api_async = async <T>(
    self: InternetComputerApi,
    {
        single,
        origin,
    }: {
        single: (single: SingleInternetComputerApi) => Promise<T>;
        origin: (origin: OriginInternetComputerApi) => Promise<T>;
    },
): Promise<T> => {
    if ('single' in self) return single(self.single);
    if ('origin' in self) return origin(self.origin);
    throw new Error('invalid internet computer api');
};
