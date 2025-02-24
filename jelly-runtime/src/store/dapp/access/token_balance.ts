import { Chain } from './chain';

export interface TokenBalance {
    chain: Chain; // chain
    address?: string; // Main currency or designated token
    balance: number; // Minimum ownership
}

export interface VerifiedTokenBalance {
    chain: Chain; // chain
    address?: string; // Main currency or designated token
    balance: number; // Minimum ownership

    // =============== Verification information ===============
    identity: string; // Identity address
    message: string;
    signature: string;
}
