import { format_integer, format_number, unit } from '../../open/open-number';

test('test open-number', () => {
    expect(format_integer('123123123')).toBe('123,123,123');
    expect(format_integer('123123123.12312312')).toBe('123,123,123.12312312');
    expect(format_number('123123123.12312312')).toBe('123,123,123.123,123,12');
    expect(unit(8)).toBe('100000000');
});
