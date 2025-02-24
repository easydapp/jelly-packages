import { Actor, HttpAgent } from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import { IC_HTTP_HOST } from './host';

const anonymous_creator = async <T>(idlFactory: IDL.InterfaceFactory, canisterId: string) => {
    const agent = HttpAgent.createSync({ host: IC_HTTP_HOST });
    return Actor.createActor<T>(idlFactory, { agent, canisterId });
};

export default { creator: anonymous_creator };
