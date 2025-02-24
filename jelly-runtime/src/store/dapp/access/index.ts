import { ChainIdentity, VerifiedChainIdentity } from './chain_identity';
import { AccessDuration, VerifiedAccessDuration } from './duration';
import { NFTOwner, VerifiedNFTOwner } from './nft_owner';
import { TokenBalance, VerifiedTokenBalance } from './token_balance';

export type DappAccessItem =
    | { duration: AccessDuration }
    | { times: number }
    | { token: string }
    | { chain_identity: ChainIdentity }
    | { token_balance: TokenBalance }
    | { nft_owner: NFTOwner };

export type DappAccess =
    | 'none'
    | 'exclusive'
    | { required: DappAccessItem }
    | { deny: DappAccessItem }
    | { all: DappAccess[] }
    | { any: DappAccess[] }
    | { not: DappAccess[] };

// ================== view ==================

export type DappAccessItemView =
    | { duration: AccessDuration }
    | { times: number }
    | 'token'
    | { chain_identity: ChainIdentity }
    | { token_balance: TokenBalance }
    | { nft_owner: NFTOwner };

export type DappAccessView =
    | 'none'
    | 'exclusive'
    | { required: DappAccessItemView }
    | { deny: DappAccessItemView }
    | { all: DappAccessView[] }
    | { any: DappAccessView[] }
    | { not: DappAccessView[] };

// ================== verified ==================

export type DappVerifiedItem =
    | { duration: VerifiedAccessDuration }
    | 'times'
    | { token: string }
    | { chain_identity: VerifiedChainIdentity }
    | { token_balance: VerifiedTokenBalance }
    | { nft_owner: VerifiedNFTOwner };

export type DappVerified =
    | 'none'
    | 'exclusive'
    | { required: DappVerifiedItem }
    | { deny: DappVerifiedItem }
    | { all: DappVerified[] }
    | { any: DappVerified[] }
    | { not: DappVerified[] };
