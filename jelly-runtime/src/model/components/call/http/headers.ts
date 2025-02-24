import { RuntimeValues } from '../../../../runtime/value';
import { NamedValue } from '../../../common/refer';

export const get_http_headers_value = (
    self: NamedValue[] | undefined,
    runtime_values: RuntimeValues,
): [string, string][] | undefined => {
    let headers: [string, string][] | undefined = undefined;
    if (self) {
        headers = [];
        for (const header of self) {
            const value = runtime_values.find_input_value<string>(header.value, 'text');
            if (value === undefined) return undefined;
            headers.push([header.name, value]);
        }
    }
    return headers;
};
