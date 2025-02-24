import { Sha256 } from '@aws-crypto/sha256-js';
import { array2hex } from './hex';

export const sha256 = async (text: string): Promise<string> => {
    const hash = new Sha256();
    hash.update(text);
    const result = await hash.digest();
    return array2hex(result);
};
