import { InputValue } from '../../common/refer';

export type ConditionObjectCompare =
    | 'null'
    | 'not_null'
    | { equal: InputValue }
    | { not_equal: InputValue }
    | { contains_key: InputValue }
    | { not_contains_key: InputValue }
    | { contains_value: InputValue }
    | { not_contains_value: InputValue };

export const match_condition_object_compare = <T>(
    self: ConditionObjectCompare,
    {
        none,
        not_null,
        equal,
        not_equal,
        contains_key,
        not_contains_key,
        contains_value,
        not_contains_value,
    }: {
        none: () => T;
        not_null: () => T;
        equal: (equal: InputValue) => T;
        not_equal: (not_equal: InputValue) => T;
        contains_key: (contains_key: InputValue) => T;
        not_contains_key: (not_contains_key: InputValue) => T;
        contains_value: (contains_value: InputValue) => T;
        not_contains_value: (not_contains_value: InputValue) => T;
    },
): T => {
    if (self === 'null') return none();
    if (self === 'not_null') return not_null();
    if (typeof self === 'object') {
        if ('equal' in self) return equal(self.equal);
        if ('not_equal' in self) return not_equal(self.not_equal);
        if ('contains_key' in self) return contains_key(self.contains_key);
        if ('not_contains_key' in self) return not_contains_key(self.not_contains_key);
        if ('contains_value' in self) return contains_value(self.contains_value);
        if ('not_contains_value' in self) return not_contains_value(self.not_contains_value);
    }
    throw new Error('Invalid condition object compare');
};
