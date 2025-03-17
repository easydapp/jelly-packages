export type EvmChain =
    // ethereum
    | 'ethereum'
    | 'ethereum-test-sepolia'
    // bsc
    | 'bsc'
    | 'bsc-test'
    // hsk
    | 'hsk'
    | 'hsk-test'
    // polygon
    | 'polygon'
    | 'polygon-test-amoy';

export const match_evm_chain = <T>(
    self: EvmChain,
    {
        ethereum,
        ethereum_test_sepolia,

        bsc,
        bsc_test,

        hsk,
        hsk_test,

        polygon,
        polygon_test_amoy,
    }: {
        ethereum: () => T;
        ethereum_test_sepolia: () => T;

        bsc: () => T;
        bsc_test: () => T;

        hsk: () => T;
        hsk_test: () => T;

        polygon: () => T;
        polygon_test_amoy: () => T;
    },
): T => {
    switch (self) {
        case 'ethereum':
            return ethereum();
        case 'ethereum-test-sepolia':
            return ethereum_test_sepolia();

        case 'bsc':
            return bsc();
        case 'bsc-test':
            return bsc_test();

        case 'hsk':
            return hsk();
        case 'hsk-test':
            return hsk_test();

        case 'polygon':
            return polygon();
        case 'polygon-test-amoy':
            return polygon_test_amoy();

        default:
            throw new Error(`Unsupported chain: ${self}`);
    }
};

export const get_evm_chain_id_by_chain = (chain: EvmChain): number => {
    return match_evm_chain(chain, {
        ethereum: () => 1, // 0x1
        ethereum_test_sepolia: () => 11155111, // 0xaa36a7

        bsc: () => 56, // 0x38
        bsc_test: () => 97, // 0x61

        hsk: () => 177, // 0xb1
        hsk_test: () => 133, // 0x85

        polygon: () => 137, // 0x89
        polygon_test_amoy: () => 80002, // 0x13882
    });
};

export const get_evm_default_rpc_by_chain = (chain: EvmChain): string => {
    return match_evm_chain(chain, {
        ethereum: () => 'https://ethereum-rpc.publicnode.com',
        ethereum_test_sepolia: () => 'https://ethereum-sepolia-rpc.publicnode.com',

        bsc: () => 'https://bsc-rpc.publicnode.com',
        bsc_test: () => 'https://bsc-testnet-rpc.publicnode.com',

        hsk: () => 'https://mainnet.hsk.xyz',
        hsk_test: () => 'https://hashkeychain-testnet.alt.technology',

        polygon: () => 'https://polygon-bor-rpc.publicnode.com',
        polygon_test_amoy: () => 'https://polygon-amoy-bor-rpc.publicnode.com',
    });
};

/**
 * POST https://ethereum-rpc.publicnode.com
    {
        "jsonrpc": "2.0",
        "method": "eth_chainId",
        "params": [],
        "id": 1
    }

    {
        "jsonrpc": "2.0",
        "result": "0x89",
        "id": 1
    }
 */
