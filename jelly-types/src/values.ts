import { clone_link_type, LinkType, match_link_type } from './types';

// Support values
export type LinkValue =
    | { text: string } // text, Corresponding to JS string
    | { bool: boolean } // boolean, Corresponding to JS boolean
    | { integer: number } // integer, Corresponding to JS number // ! Attention to the scope of safety. Number.MIN_SAFE_INTEGER <= x <= Number.MAX_SAFE_INTEGER
    | { number: number } // float, Corresponding to JS number
    | { array: ArrayLinkValue } // array, corresponding to JS Array
    | { object: ObjectSubitemValue[] }; // object, Corresponding to JS object

// Array value
export interface ArrayLinkValue {
    ty: LinkType; // If the array is empty, it will lose the subtype. Here to record subtype
    values: LinkValue[]; // array value
}

// sub value of object
export interface ObjectSubitemValue {
    key: string; // key
    value: LinkValue; // sub value
}

// match value
export const match_link_value = <T>(
    self: LinkValue,
    {
        text,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: (text: string) => T;
        bool: (bool: boolean) => T;
        integer: (integer: number) => T;
        number: (number: number) => T;
        array: (array: ArrayLinkValue) => T;
        object: (object: ObjectSubitemValue[]) => T;
    },
): T => {
    if ('text' in self) return text(self.text);
    if ('bool' in self) return bool(self.bool);
    if ('integer' in self) return integer(self.integer);
    if ('number' in self) return number(self.number);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('unknown link value');
};

// match value
export const match_link_value_async = async <T>(
    self: LinkValue,
    {
        text,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: (text: string) => Promise<T>;
        bool: (bool: boolean) => Promise<T>;
        integer: (integer: number) => Promise<T>;
        number: (number: number) => Promise<T>;
        array: (array: ArrayLinkValue) => Promise<T>;
        object: (object: ObjectSubitemValue[]) => Promise<T>;
    },
): Promise<T> => {
    if ('text' in self) return text(self.text);
    if ('bool' in self) return bool(self.bool);
    if ('integer' in self) return integer(self.integer);
    if ('number' in self) return number(self.number);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('unknown link value');
};

// The link value converts to JS value
export const link_value_to_js_value = (self: LinkValue): any => {
    return match_link_value<any>(self, {
        text: (text) => text,
        bool: (bool) => bool,
        integer: (integer) => integer,
        number: (number) => number,
        array: (array) => array.values.map(link_value_to_js_value),
        object: (object) =>
            object.reduce(
                (acc, item) => {
                    acc[item.key] = link_value_to_js_value(item.value);
                    return acc;
                },
                {} as Record<string, any>,
            ),
    });
};

// Get link type from link value
export const link_value_to_type = (self: LinkValue): LinkType => {
    return match_link_value<LinkType>(self, {
        text: (_) => 'text',
        bool: (_) => 'bool',
        integer: (_) => 'integer',
        number: (_) => 'number',
        array: (array) => ({
            array: clone_link_type(array.ty),
        }),
        object: (object) => ({
            object: object.map((item) => ({ key: item.key, ty: link_value_to_type(item.value) })),
        }),
    });
};

// Default link value for link type
export const link_type_to_default_link_value = (type: LinkType): LinkValue => {
    return match_link_type<LinkValue>(type, {
        text: () => ({ text: '' }),
        bool: () => ({ bool: false }),
        integer: () => ({ integer: 0 }),
        number: () => ({ number: 0 }),
        array: (array) => ({ array: { ty: JSON.parse(JSON.stringify(array)), values: [] } }),
        object: (object) => ({
            object: object.map(({ key, ty }) => ({
                key,
                value: link_type_to_default_link_value(ty),
            })),
        }),
    });
};
