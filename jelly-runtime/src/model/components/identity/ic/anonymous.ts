import { ComponentIdentityIcValue } from '../ic';
import { getAnonymousActorCreator } from './identity';

// Globally shared anonymous
export const anonymous: ComponentIdentityIcValue = {
    wallet: { any: {} },
    secret: '', // undefined means wallet

    is_connected: async () => true,
    creator: getAnonymousActorCreator(),

    owner: '2vxsx-fae', // anonymous identity // cspell: disable-line
    account_id: '1c7a48ba6a562aa9eaa2481a9049cdf0433b9738c992d698c31d8abf89cadc79', // anonymous identity
};
