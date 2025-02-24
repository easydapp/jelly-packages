// Support types
export type LinkType =
    | 'text' // text, Corresponding to JS string
    | 'bool' // boolean, Corresponding to JS boolean
    | 'integer' // integer, Corresponding to JS number // ! Attention to the scope of safety. Number.MIN_SAFE_INTEGER <= x <= Number.MAX_SAFE_INTEGER
    | 'number' // float, Corresponding to JS number
    | { array: LinkType } // array, corresponding to JS Array
    | { object: ObjectSubitem[] }; // object, Corresponding to JS object

// Subtype of object
export interface ObjectSubitem {
    key: string; // key
    ty: LinkType; // subtype
}

// match types
export const match_link_type = <T>(
    self: LinkType,
    {
        text,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: () => T;
        bool: () => T;
        integer: () => T;
        number: () => T;
        array: (array: LinkType) => T;
        object: (object: ObjectSubitem[]) => T;
    },
): T => {
    if (self === 'text') return text();
    if (self === 'bool') return bool();
    if (self === 'integer') return integer();
    if (self === 'number') return number();
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('unknown link type');
};

// match types
export const match_link_type_async = async <T>(
    self: LinkType,
    {
        text,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: () => Promise<T>;
        bool: () => Promise<T>;
        integer: () => Promise<T>;
        number: () => Promise<T>;
        array: (array: LinkType) => Promise<T>;
        object: (object: ObjectSubitem[]) => Promise<T>;
    },
): Promise<T> => {
    if (self === 'text') return text();
    if (self === 'bool') return bool();
    if (self === 'integer') return integer();
    if (self === 'number') return number();
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('unknown link type');
};

// Verify whether the JS value matches the link type
export const link_type_is_match_js_value = (self: LinkType, value: any): boolean => {
    const result = match_link_type(self, {
        text: () => typeof value === 'string',
        bool: () => typeof value === 'boolean',
        integer: () => Number.isInteger(value),
        number: () => typeof value === 'number',
        array: (array) => {
            if (!Array.isArray(value)) return false;
            // eslint-disable-next-line @typescript-eslint/prefer-for-of
            for (let i = 0; i < value.length; i++) {
                if (!link_type_is_match_js_value(array, value[i])) return false;
            }
            return true;
        },
        object: (object) => {
            if (typeof value !== 'object') return false;
            for (const sub of object) {
                if (!link_type_is_match_js_value(sub.ty, value[sub.key])) return false;
            }
            return true;
        },
    });

    if (!result) {
        console.error('link type is not match js value', self, value);
    }

    return result;
};

// clone
export const clone_link_type = (type: LinkType): LinkType => {
    return match_link_type<LinkType>(type, {
        text: () => 'text',
        bool: () => 'bool',
        integer: () => 'integer',
        number: () => 'number',
        array: (array) => ({ array: clone_link_type(array) }),
        object: (object) => ({
            object: object.map((item) => ({ key: item.key, ty: clone_link_type(item.ty) })),
        }),
    });
};
