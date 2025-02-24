import { array2hex, hex2array } from '../../open/open-hex';

test('test open-ic', () => {
    expect(hex2array('0x0102')).toStrictEqual([1, 2]);
    expect(array2hex([1, 2])).toBe('0102');
});
