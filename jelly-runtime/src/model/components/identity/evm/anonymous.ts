import { ethers, SigningKey, Wallet } from 'ethers';
import { hex2array } from '../../../../common/hex';
import {
    EvmChain,
    get_evm_chain_id_by_chain,
    get_evm_default_rpc_by_chain,
} from '../../../types/evm';
import { ComponentIdentityEvmValue } from '../evm';

const ANONYMOUS_SECRET = '0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef';

// Globally shared anonymous
export const get_evm_anonymous = (chain: EvmChain): ComponentIdentityEvmValue => {
    const chain_id = get_evm_chain_id_by_chain(chain);
    // const provider = ethers.getDefaultProvider(chain_id);
    const provider = ethers.getDefaultProvider(get_evm_default_rpc_by_chain(chain));
    const signer = new Wallet(
        new SigningKey(new Uint8Array(hex2array(ANONYMOUS_SECRET))),
        provider,
    );
    const account = signer.address; // "0xFCAd0B19bB29D4674531d6f115237E16AfCE377c"
    return {
        chain,
        chain_id,
        wallet: { any: {} },
        secret: ANONYMOUS_SECRET, // undefined means wallet

        is_connected: async () => true,
        provider,
        signer,

        account,
    };
};
