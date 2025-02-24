import { ext_identifier2index, ext_index2identifier, principal2account_id } from '../../open/open-ic';

test('test open-ic', () => {
    expect(principal2account_id('chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe')).toBe(
        'ec267d2835aef4dc6b69007ad51f49efcd0850324c059ea314cc81849480beb4',
    );
    expect(ext_index2identifier('n5yqx-uqaaa-aaaap-aatja-cai', 1)).toBe('ukbxy-zykor-uwiaa-aaaaa-dyae2-iaqca-aaaaa-q');
    expect(ext_identifier2index('ukbxy-zykor-uwiaa-aaaaa-dyae2-iaqca-aaaaa-q')).toBe(1);
});
