import { Principal } from '../../open/principal';

test('test open-ic', () => {
    expect(Principal.fromText('chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe').toText()).toBe(
        'chdqb-y6zw7-nozh2-hfq26-gr4om-szect-pnlo4-oazpk-zxyxw-xsfaj-7qe',
    );
});
