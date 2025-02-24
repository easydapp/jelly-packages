import { WalletEmptySettings } from '.';

export type EvmWallet =
    | { any: WalletEmptySettings }
    | { metamask: WalletEmptySettings }
    | { rainbow: WalletEmptySettings };
