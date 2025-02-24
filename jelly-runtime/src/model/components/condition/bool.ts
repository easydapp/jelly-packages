import { InputValue } from '../../common/refer';

export type ConditionBoolCompare =
    | 'null'
    | 'not_null'
    | { equal: InputValue }
    | { not_equal: InputValue }
    | 'is_true'
    | 'is_false';

export const match_condition_bool_compare = <T>(
    self: ConditionBoolCompare,
    {
        none,
        not_null,
        equal,
        not_equal,
        is_true,
        is_false,
    }: {
        none: () => T;
        not_null: () => T;
        equal: (equal: InputValue) => T;
        not_equal: (not_equal: InputValue) => T;
        is_true: () => T;
        is_false: () => T;
    },
): T => {
    if (self === 'null') return none();
    if (self === 'not_null') return not_null();
    if (typeof self === 'object') {
        if ('equal' in self) return equal(self.equal);
        if ('not_equal' in self) return not_equal(self.not_equal);
    }
    if (self === 'is_true') return is_true();
    if (self === 'is_false') return is_false();
    throw new Error('Invalid condition bool compare');
};
