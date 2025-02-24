import { InputValue } from '../../common/refer';

export type ConditionNumberCompare =
    | 'null'
    | 'not_null'
    | { equal: InputValue }
    | { not_equal: InputValue }
    | { greater: InputValue }
    | { greater_equal: InputValue }
    | { less: InputValue }
    | { less_equal: InputValue };

export const match_condition_number_compare = <T>(
    self: ConditionNumberCompare,
    {
        none,
        not_null,
        equal,
        not_equal,
        greater,
        greater_equal,
        less,
        less_equal,
    }: {
        none: () => T;
        not_null: () => T;
        equal: (equal: InputValue) => T;
        not_equal: (not_equal: InputValue) => T;
        greater: (greater: InputValue) => T;
        greater_equal: (greater_equal: InputValue) => T;
        less: (less: InputValue) => T;
        less_equal: (less_equal: InputValue) => T;
    },
): T => {
    if (self === 'null') return none();
    if (self === 'not_null') return not_null();
    if (typeof self === 'object') {
        if ('equal' in self) return equal(self.equal);
        if ('not_equal' in self) return not_equal(self.not_equal);
        if ('greater' in self) return greater(self.greater);
        if ('greater_equal' in self) return greater_equal(self.greater_equal);
        if ('less' in self) return less(self.less);
        if ('less_equal' in self) return less_equal(self.less_equal);
    }
    throw new Error('Invalid condition number compare');
};
