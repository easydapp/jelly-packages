import { WalletEmptySettings } from '.';

export type IcWallet =
    | { any: WalletEmptySettings }
    | { ii: WalletEmptySettings }
    | { plug: WalletEmptySettings }
    | { me: WalletEmptySettings }
    | { bitfinity: WalletEmptySettings }
    | { nfid: WalletEmptySettings }
    | { stoic: WalletEmptySettings };
