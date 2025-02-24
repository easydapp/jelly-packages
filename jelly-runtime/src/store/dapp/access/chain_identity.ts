import { Chain } from './chain';

export interface ChainIdentity {
    chain: Chain; // chain
    identity: string; // Identity address
}

export interface VerifiedChainIdentity {
    chain: Chain; // chain
    identity: string; // Identity address

    // =============== Verification information ===============
    message: string;
    signature: string;
}
