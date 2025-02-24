import { InputValue } from '../../common/refer';

export type ConditionArrayCompare =
    | 'null'
    | 'not_null'
    | { equal: InputValue }
    | { not_equal: InputValue }
    | { contains: InputValue } // subtype of array
    | { not_contains: InputValue } // subtype of array
    | { length_equal: InputValue }
    | { length_not_equal: InputValue }
    | { length_greater: InputValue }
    | { length_greater_equal: InputValue }
    | { length_less: InputValue }
    | { length_less_equal: InputValue };

export const match_condition_array_compare = <T>(
    self: ConditionArrayCompare,
    {
        none,
        not_null,
        equal,
        not_equal,
        contains,
        not_contains,
        length_equal,
        length_not_equal,
        length_greater,
        length_greater_equal,
        length_less,
        length_less_equal,
    }: {
        none: () => T;
        not_null: () => T;
        equal: (equal: InputValue) => T;
        not_equal: (not_equal: InputValue) => T;
        contains: (contains: InputValue) => T;
        not_contains: (not_contains: InputValue) => T;
        length_equal: (length_equal: InputValue) => T;
        length_not_equal: (length_not_equal: InputValue) => T;
        length_greater: (length_greater: InputValue) => T;
        length_greater_equal: (length_greater_equal: InputValue) => T;
        length_less: (length_less: InputValue) => T;
        length_less_equal: (length_less_equal: InputValue) => T;
    },
): T => {
    if (self === 'null') return none();
    if (self === 'not_null') return not_null();
    if (typeof self === 'object') {
        if ('equal' in self) return equal(self.equal);
        if ('not_equal' in self) return not_equal(self.not_equal);
        if ('contains' in self) return contains(self.contains);
        if ('not_contains' in self) return not_contains(self.not_contains);
        if ('length_equal' in self) return length_equal(self.length_equal);
        if ('length_not_equal' in self) return length_not_equal(self.length_not_equal);
        if ('length_greater' in self) return length_greater(self.length_greater);
        if ('length_greater_equal' in self) return length_greater_equal(self.length_greater_equal);
        if ('length_less' in self) return length_less(self.length_less);
        if ('length_less_equal' in self) return length_less_equal(self.length_less_equal);
    }
    throw new Error('Invalid condition array compare');
};
