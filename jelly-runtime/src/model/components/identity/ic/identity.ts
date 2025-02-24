import { Actor, HttpAgent, Identity } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { IC_HTTP_HOST } from '../../../../canisters/host';
import { hex2array } from '../../../../common/hex';
import { ComponentIdentityIcValue } from '../ic';
import { principal2account } from './account';
import { ActorCreator } from './types';

export const getAnonymousAgent = (): HttpAgent => {
    return HttpAgent.createSync({ host: IC_HTTP_HOST });
};
export const getAnonymousActorCreator = (): ActorCreator => {
    return async <T>(idlFactory: IDL.InterfaceFactory, canisterId: string) => {
        const agent = getAnonymousAgent();
        return Actor.createActor<T>(idlFactory, { agent, canisterId });
    };
};

const getIdentityFromSecretKey = async (secret_key: string): Promise<Identity> => {
    const seed = hex2array(secret_key);
    const identity = Ed25519KeyIdentity.fromSecretKey(new Uint8Array(seed));
    return identity;
};

export const getActorCreatorByAgent = (agent: HttpAgent): ActorCreator => {
    return async <T>(idlFactory: IDL.InterfaceFactory, canisterId: string) => {
        return Actor.createActor<T>(idlFactory, { agent, canisterId });
    };
};

export const getIdentityBySecretKey = async (
    secret_key: string,
): Promise<ComponentIdentityIcValue> => {
    const identity = await getIdentityFromSecretKey(secret_key);
    const owner = identity.getPrincipal().toText();
    const account_id = principal2account(owner);
    const agent = HttpAgent.createSync({
        host: IC_HTTP_HOST,
        identity: identity,
    });
    return {
        wallet: { any: {} },
        secret: secret_key, // undefined means wallet

        is_connected: async () => {
            console.error('agent', agent);
            console.error('agent', await agent.status());
            throw new Error('todo');
        },
        creator: getActorCreatorByAgent(agent),

        owner,
        account_id,
    };
};
