import { parse_factory, stringify_factory } from '../../open/open-json';
import { Principal } from '../../open/principal';

test('test open-json', () => {
    const parse = parse_factory(JSON.parse);
    const stringify = stringify_factory(JSON.stringify);

    expect(stringify(undefined)).toBeUndefined();
    expect(stringify(null)).toBe('null');
    expect(stringify('text')).toBe('"text"');
    expect(stringify(123)).toBe('123');
    expect(stringify(123.123)).toBe('123.123');
    expect(stringify(123.123e-3)).toBe('0.123123');
    expect(stringify(true)).toBe('true');
    expect(stringify(false)).toBe('false');
    expect(stringify(1n)).toBe('{"__open_type__":"bigint","value":"1"}');
    expect(() => stringify(() => undefined)).toThrow('can not stringify function');
    expect(stringify([1, 2, 3])).toBe('[1,2,3]');
    expect(stringify([1, '2', true, 4n])).toBe('[1,"2",true,{"__open_type__":"bigint","value":"4"}]');
    expect(stringify(new Uint8Array([1, 2, 3]))).toBe('{"__open_type__":"Uint8Array","value":[1,2,3]}');
    expect(stringify(new Uint16Array([1, 2, 3]))).toBe('{"__open_type__":"Uint16Array","value":[1,2,3]}');
    expect(stringify(new Uint32Array([1, 2, 3]))).toBe('{"__open_type__":"Uint32Array","value":[1,2,3]}');
    expect(stringify(new BigUint64Array([1n, 2n, 3n]))).toBe(
        '{"__open_type__":"BigUint64Array","value":["1","2","3"]}',
    );
    expect(stringify(new Int8Array([1, 2, 3]))).toBe('{"__open_type__":"Int8Array","value":[1,2,3]}');
    expect(stringify(new Int16Array([1, 2, 3]))).toBe('{"__open_type__":"Int16Array","value":[1,2,3]}');
    expect(stringify(new Int32Array([1, 2, 3]))).toBe('{"__open_type__":"Int32Array","value":[1,2,3]}');
    expect(stringify(new BigInt64Array([1n, 2n, 3n]))).toBe('{"__open_type__":"BigInt64Array","value":["1","2","3"]}');
    expect(stringify(Principal.fromText('udtw4-baaaa-aaaah-abc3q-cai'))).toBe(
        '{"__open_type__":"Principal","value":"udtw4-baaaa-aaaah-abc3q-cai"}',
    );
    expect(stringify({ a: 1, b: true, c: 3n, d: [1, 2, 3] })).toBe(
        '{"a":1,"b":true,"c":{"__open_type__":"bigint","value":"3"},"d":[1,2,3]}',
    );

    expect(() => parse(null as unknown as any)).toThrow('json must be a string');
    expect(() => parse((() => undefined) as unknown as any)).toThrow('json must be a string');
    expect(parse('null')).toBe(null);
    expect(parse('"text"')).toBe('text');
    expect(parse('123')).toBe(123);
    expect(parse('123.123')).toBe(123.123);
    expect(parse('123.123e-3')).toBe(0.123123);
    expect(parse('true')).toBe(true);
    expect(parse('false')).toBe(false);
    expect(parse('{"__open_type__":"bigint","value":"1"}')).toBe(1n);
    expect(parse('[1,2,3]')).toStrictEqual([1, 2, 3]);
    expect(parse('[1,"2",true,{"__open_type__":"bigint","value":"4"}]')).toStrictEqual([1, '2', true, 4n]);
    expect(parse('{"__open_type__":"Uint8Array","value":[1,2,3]}')).toStrictEqual(new Uint8Array([1, 2, 3]));
    expect(parse('{"__open_type__":"Uint16Array","value":[1,2,3]}')).toStrictEqual(new Uint16Array([1, 2, 3]));
    expect(parse('{"__open_type__":"Uint32Array","value":[1,2,3]}')).toStrictEqual(new Uint32Array([1, 2, 3]));
    expect(parse('{"__open_type__":"BigUint64Array","value":["1","2","3"]}').length).toStrictEqual(3);
    expect(parse('{"__open_type__":"Int8Array","value":[1,2,3]}')).toStrictEqual(new Int8Array([1, 2, 3]));
    expect(parse('{"__open_type__":"Int16Array","value":[1,2,3]}')).toStrictEqual(new Int16Array([1, 2, 3]));
    expect(parse('{"__open_type__":"Int32Array","value":[1,2,3]}')).toStrictEqual(new Int32Array([1, 2, 3]));
    expect(parse('{"__open_type__":"BigInt64Array","value":["1","2","3"]}').length).toStrictEqual(3);
    expect((parse('{"__open_type__":"Principal","value":"udtw4-baaaa-aaaah-abc3q-cai"}') as Principal).toText()).toBe(
        'udtw4-baaaa-aaaah-abc3q-cai',
    );
    expect(parse('{"a":1,"b":true,"c":{"__open_type__":"bigint","value":"3"},"d":[1,2,3]}')).toStrictEqual({
        a: 1,
        b: true,
        c: 3n,
        d: [1, 2, 3],
    });

    const full = '123 こんにちは,"\n\r\t\f\'\\\u3456';
    const custom = stringify(full);
    const official = JSON.stringify(full);
    // console.error('custom', stringify(full));
    // console.error('official', JSON.stringify(full));
    expect(custom).toStrictEqual(official);
    expect(parse(custom)).toStrictEqual(JSON.parse(official));
});
