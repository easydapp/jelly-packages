import { link_value_to_js_value } from '../../open/open-type';

test('test open-type', () => {
    expect(link_value_to_js_value({ text: 'text' })).toBe('text');
    expect(link_value_to_js_value({ integer: 123 })).toBe(123);
});
