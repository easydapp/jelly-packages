import queryString from 'query-string';

// parse parameter to object
export const parse_param = (id: string): Record<string, string> => {
    const index = id.indexOf('?');
    const search = 0 <= index ? id.substring(index) : '';
    const param = queryString.parse(search);
    return param as any;
};
