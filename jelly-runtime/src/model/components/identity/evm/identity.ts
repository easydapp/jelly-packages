import { ethers, SigningKey, Wallet } from 'ethers';
import { hex2array } from '../../../../common/hex';
import {
    EvmChain,
    get_evm_chain_id_by_chain,
    get_evm_default_rpc_by_chain,
} from '../../../types/evm';
import { ComponentIdentityEvmValue } from '../evm';

export const get_evm_identity = (chain: EvmChain, secret: string): ComponentIdentityEvmValue => {
    const chain_id = get_evm_chain_id_by_chain(chain);
    // const provider = ethers.getDefaultProvider(chain_id);
    const provider = ethers.getDefaultProvider(get_evm_default_rpc_by_chain(chain));
    const signer = new Wallet(new SigningKey(new Uint8Array(hex2array(secret))), provider);
    const account = signer.address; // "xx"
    return {
        chain,
        chain_id,
        wallet: { any: {} },
        secret: secret, // undefined means wallet

        is_connected: async () => true,
        provider,
        signer,

        account,
    };
};
