import { link_type_is_match_js_value, LinkType, ObjectSubitem } from '@jellypack/types/lib/types';

import { same } from '../../../common/same';
import { RuntimeValues } from '../../../runtime/value';
import { ComponentId } from '../../common/identity';
import { all_endpoints_find_output_type, AllEndpoints, Endpoint } from '../../common/lets';
import { input_value_get_used_component, InputValue, ReferValue } from '../../common/refer';
import { ConditionArrayCompare, match_condition_array_compare } from './array';
import { ConditionBoolCompare, match_condition_bool_compare } from './bool';
import { ConditionNumberCompare, match_condition_number_compare } from './number';
import { ConditionObjectCompare, match_condition_object_compare } from './object';
import { ConditionTextCompare, match_condition_text_compare } from './text';

export interface ComponentCondition {
    id: ComponentId;
    inlets?: Endpoint[];
    metadata: ConditionMetadata;
}

export interface ConditionMetadata {
    conditions: Condition[];
}

export type Condition =
    | 'none'
    | { required: ConditionItem }
    | { deny: ConditionItem }
    | { and: Condition[] }
    | { or: Condition[] }
    | { not: Condition[] };

export const match_condition = <T>(
    self: Condition,
    {
        none,
        required,
        deny,
        and,
        or,
        not,
    }: {
        none: () => T;
        required: (required: ConditionItem) => T;
        deny: (deny: ConditionItem) => T;
        and: (and: Condition[]) => T;
        or: (or: Condition[]) => T;
        not: (not: Condition[]) => T;
    },
): T => {
    if (self === 'none') return none();
    if (typeof self === 'object') {
        if ('required' in self) return required(self.required);
        if ('deny' in self) return deny(self.deny);
        if ('and' in self) return and(self.and);
        if ('or' in self) return or(self.or);
        if ('not' in self) return not(self.not);
    }
    throw new Error('Invalid condition');
};

export interface ConditionItem {
    value: ReferValue;
    matches: ConditionMatches;
}

export type ConditionMatches =
    | { text: ConditionTextCompare }
    | { bool: ConditionBoolCompare }
    | { integer: ConditionNumberCompare }
    | { number: ConditionNumberCompare }
    | { array: ConditionArrayCompare }
    | { object: ConditionObjectCompare };

export const match_condition_matches = <T>(
    self: ConditionMatches,
    {
        text,
        bool,
        integer,
        number,
        array,
        object,
    }: {
        text: (text: ConditionTextCompare) => T;
        bool: (bool: ConditionBoolCompare) => T;
        integer: (integer: ConditionNumberCompare) => T;
        number: (number: ConditionNumberCompare) => T;
        array: (array: ConditionArrayCompare) => T;
        object: (object: ConditionObjectCompare) => T;
    },
): T => {
    if ('text' in self) return text(self.text);
    if ('bool' in self) return bool(self.bool);
    if ('integer' in self) return integer(self.integer);
    if ('number' in self) return number(self.number);
    if ('array' in self) return array(self.array);
    if ('object' in self) return object(self.object);
    throw new Error('Invalid condition matches');
};

export const component_condition_count_outputs = (self: ComponentCondition): number => {
    return 1 + self.metadata.conditions.length;
};

export const component_condition_get_used_component = (self: ComponentCondition): ComponentId[] => {
    const used = [];
    for (const c of self.metadata.conditions) used.push(...condition_get_used_component(c));
    return used;
};

const condition_get_used_component = (self: Condition): ComponentId[] => {
    const used: ComponentId[] = [];
    match_condition(self, {
        none: () => {
            /* do nothing */
        },
        required: (required) => {
            used.push(...condition_item_get_used_component(required));
        },
        deny: (deny) => {
            used.push(...condition_item_get_used_component(deny));
        },
        and: (and) => {
            for (const c of and) used.push(...condition_get_used_component(c));
        },
        or: (or) => {
            for (const c of or) used.push(...condition_get_used_component(c));
        },
        not: (not) => {
            for (const c of not) used.push(...condition_get_used_component(c));
        },
    });
    return used;
};

const condition_item_get_used_component = (self: ConditionItem): ComponentId[] => {
    const used = [];
    used.push(self.value.endpoint.id);
    const parse_input_value = (value: InputValue) => {
        used.push(...input_value_get_used_component(value));
    };
    match_condition_matches(self.matches, {
        text: (text) =>
            match_condition_text_compare(text, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                contains: parse_input_value,
                not_contains: parse_input_value,
                starts_with: parse_input_value,
                not_starts_with: parse_input_value,
                ends_with: parse_input_value,
                not_ends_with: parse_input_value,
                length_equal: parse_input_value,
                length_not_equal: parse_input_value,
                length_greater: parse_input_value,
                length_greater_equal: parse_input_value,
                length_less: parse_input_value,
                length_less_equal: parse_input_value,
                regex: parse_input_value,
                not_regex: parse_input_value,
            }),
        bool: (bool) =>
            match_condition_bool_compare(bool, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                is_true: () => {
                    /* do nothing */
                },
                is_false: () => {
                    /* do nothing */
                },
            }),
        integer: (integer) =>
            match_condition_number_compare(integer, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                greater: parse_input_value,
                greater_equal: parse_input_value,
                less: parse_input_value,
                less_equal: parse_input_value,
            }),
        number: (number) =>
            match_condition_number_compare(number, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                greater: parse_input_value,
                greater_equal: parse_input_value,
                less: parse_input_value,
                less_equal: parse_input_value,
            }),
        array: (array) =>
            match_condition_array_compare(array, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                contains: parse_input_value,
                not_contains: parse_input_value,
                length_equal: parse_input_value,
                length_not_equal: parse_input_value,
                length_greater: parse_input_value,
                length_greater_equal: parse_input_value,
                length_less: parse_input_value,
                length_less_equal: parse_input_value,
            }),
        object: (object) =>
            match_condition_object_compare(object, {
                none: () => {
                    /* do nothing */
                },
                not_null: () => {
                    /* do nothing */
                },
                equal: parse_input_value,
                not_equal: parse_input_value,
                contains_key: parse_input_value,
                not_contains_key: parse_input_value,
                contains_value: parse_input_value,
                not_contains_value: parse_input_value,
            }),
    });
    return used;
};

export const get_condition_index = (
    self: ComponentCondition,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
): number | undefined => {
    // console.error('get_condition_index', self.id);
    const results: (boolean | undefined)[] = [];
    let index = 0;
    for (const condition of self.metadata.conditions) {
        results[index] = assert_condition(condition, endpoints, runtime_values);
        if (results[index] !== undefined && results[index]) return index;
        index += 1;
    }
    if (0 <= results.findIndex((r) => r === undefined)) return undefined;
    return index;
};

const assert_condition = (
    self: Condition,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
): boolean | undefined => {
    if (self === 'none') return true;
    if (endpoints === undefined) {
        console.error('condition endpoints is undefined');
        throw new Error(`invalid endpoint`);
    }
    // console.error('assert_condition', self, endpoints, runtime_values);
    return match_condition(self, {
        none: () => true,
        required: (required) => assert_condition_item(required, endpoints, runtime_values),
        deny: (deny) => {
            const r = assert_condition_item(deny, endpoints, runtime_values);
            if (r !== undefined) return !r;
            return undefined;
        },
        and: (and) => {
            for (const condition of and) {
                const r = assert_condition(condition, endpoints, runtime_values);
                if (r === undefined) return undefined;
                if (!r) return false;
            }
            return true;
        },
        or: (or) => {
            for (const condition of or) {
                const r = assert_condition(condition, endpoints, runtime_values);
                if (r === undefined) return undefined;
                if (r) return true;
            }
            return false;
        },
        not: (not) => {
            for (const condition of not) {
                const r = assert_condition(condition, endpoints, runtime_values);
                if (r === undefined) return undefined;
                if (r) return false;
            }
            return true;
        },
    });
};

const check_value_and_execute = (
    left: any,
    type: LinkType,
    right: any,
    compare: (left: any, right: any) => boolean | undefined,
    normal = true,
): boolean | undefined => {
    if (left === undefined) return undefined;
    if (!link_type_is_match_js_value(type, left)) return undefined;
    if (right === undefined) return undefined;
    const r = compare(left, right);
    if (r === undefined) return undefined;
    return normal ? r : !r;
};
const assert_condition_item = (
    self: ConditionItem,
    endpoints: AllEndpoints,
    runtime_values: RuntimeValues,
): boolean | undefined => {
    const output = all_endpoints_find_output_type(endpoints, self.value.endpoint, self.value.refer);
    const value = runtime_values.find_input_value<any | undefined>({ refer: self.value });
    if (value !== undefined && !link_type_is_match_js_value(output, value)) {
        console.error('type is mismatch', output, value);
        throw new Error(`type is mismatch`);
    }
    return match_condition_matches(self.matches, {
        text: (text) =>
            match_condition_text_compare(text, {
                none: () => value === undefined,
                not_null: () => value !== undefined,
                equal: (equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(equal, 'text'),
                        same,
                    ),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(not_equal, 'text'),
                        same,
                        false,
                    ),
                contains: (contains) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(contains, 'text'),
                        (left, right) => left.indexOf(right) >= 0,
                    ),
                not_contains: (not_contains) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(not_contains, 'text'),
                        (left, right) => left.indexOf(right) >= 0,
                        false,
                    ),
                starts_with: (starts_with) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(starts_with, 'text'),
                        (left, right) => left.startsWith(right),
                    ),
                not_starts_with: (not_starts_with) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(not_starts_with, 'text'),
                        (left, right) => left.startsWith(right),
                        false,
                    ),
                ends_with: (ends_with) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(ends_with, 'text'),
                        (left, right) => left.endsWith(right),
                    ),
                not_ends_with: (not_ends_with) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(not_ends_with, 'text'),
                        (left, right) => left.endsWith(right),
                        false,
                    ),
                length_equal: (length_equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_equal, 'integer'),
                        (left, right) => left.length === right,
                    ),
                length_not_equal: (length_not_equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_not_equal, 'integer'),
                        (left, right) => left.length !== right,
                    ),
                length_greater: (length_greater) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_greater, 'integer'),
                        (left, right) => left.length > right,
                    ),
                length_greater_equal: (length_greater_equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_greater_equal, 'integer'),
                        (left, right) => left.length >= right,
                    ),
                length_less: (length_less) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_less, 'integer'),
                        (left, right) => left.length < right,
                    ),
                length_less_equal: (length_less_equal) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<number>(length_less_equal, 'integer'),
                        (left, right) => left.length <= right,
                    ),
                regex: (regex) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(regex, 'text'),
                        (left, right) => new RegExp(right).test(left),
                    ),
                not_regex: (not_regex) =>
                    check_value_and_execute(
                        value,
                        'text',
                        runtime_values.find_input_value<string>(not_regex, 'text'),
                        (left, right) => new RegExp(right).test(left),
                        false,
                    ),
            }),
        bool: (bool) =>
            match_condition_bool_compare(bool, {
                none: () => value === undefined,
                not_null: () => value !== undefined,
                equal: (equal) =>
                    check_value_and_execute(
                        value,
                        'bool',
                        runtime_values.find_input_value<boolean>(equal, 'bool'),
                        same,
                    ),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        'bool',
                        runtime_values.find_input_value<boolean>(not_equal, 'bool'),
                        same,
                        false,
                    ),
                is_true: () => check_value_and_execute(value, 'bool', true, same),
                is_false: () => check_value_and_execute(value, 'bool', false, same),
            }),
        integer: (integer) =>
            match_condition_number_compare(integer, {
                none: () => value === undefined,
                not_null: () => value !== undefined,
                equal: (equal) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(equal, 'integer'),
                        same,
                    ),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(not_equal, 'integer'),
                        same,
                        false,
                    ),
                greater: (greater) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(greater, 'integer'),
                        (left, right) => left > right,
                    ),
                greater_equal: (greater_equal) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(greater_equal, 'integer'),
                        (left, right) => left >= right,
                    ),
                less: (less) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(less, 'integer'),
                        (left, right) => left < right,
                    ),
                less_equal: (less_equal) =>
                    check_value_and_execute(
                        value,
                        'integer',
                        runtime_values.find_input_value<number>(less_equal, 'integer'),
                        (left, right) => left <= right,
                    ),
            }),
        number: (number) =>
            match_condition_number_compare(number, {
                none: () => value === undefined,
                not_null: () => value !== undefined,
                equal: (equal) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(equal, 'number'),
                        same,
                    ),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(not_equal, 'number'),
                        same,
                        false,
                    ),
                greater: (greater) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(greater, 'number'),
                        (left, right) => left > right,
                    ),
                greater_equal: (greater_equal) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(greater_equal, 'number'),
                        (left, right) => left >= right,
                    ),
                less: (less) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(less, 'number'),
                        (left, right) => left < right,
                    ),
                less_equal: (less_equal) =>
                    check_value_and_execute(
                        value,
                        'number',
                        runtime_values.find_input_value<number>(less_equal, 'number'),
                        (left, right) => left <= right,
                    ),
            }),
        array: (array) =>
            match_condition_array_compare(array, {
                none: () => value === undefined,
                not_null: () => value === undefined,
                equal: (equal) =>
                    check_value_and_execute(value, output, runtime_values.find_input_value<any>(equal, output), same),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(not_equal, output),
                        same,
                        false,
                    ),
                contains: (contains) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(contains, (output as { array: LinkType }).array),
                        (left, right) => 0 <= left.findIndex((s: any) => same(s, right)),
                    ),
                not_contains: (not_contains) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(not_contains, (output as { array: LinkType }).array),
                        (left, right) => 0 <= left.findIndex((s: any) => same(s, right)),
                        false,
                    ),
                length_equal: (length_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_equal, 'integer'),
                        (left, right) => left.length === right,
                    ),
                length_not_equal: (length_not_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_not_equal, 'integer'),
                        (left, right) => left.length !== right,
                    ),
                length_greater: (length_greater) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_greater, 'integer'),
                        (left, right) => left.length > right,
                    ),
                length_greater_equal: (length_greater_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_greater_equal, 'integer'),
                        (left, right) => left.length >= right,
                    ),
                length_less: (length_less) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_less, 'integer'),
                        (left, right) => left.length < right,
                    ),
                length_less_equal: (length_less_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<number>(length_less_equal, 'integer'),
                        (left, right) => left.length <= right,
                    ),
            }),
        object: (object) =>
            match_condition_object_compare(object, {
                none: () => value === undefined,
                not_null: () => value !== undefined,
                equal: (equal) =>
                    check_value_and_execute(value, output, runtime_values.find_input_value<any>(equal, output), same),
                not_equal: (not_equal) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(not_equal, output),
                        same,
                        false,
                    ),
                contains_key: (contains_key) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(contains_key, 'text'),
                        (left, right) => left[right] !== undefined,
                    ),
                not_contains_key: (not_contains_key) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(not_contains_key, 'text'),
                        (left, right) => left[right] === undefined,
                    ),
                contains_value: (contains_value) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(contains_value),
                        (left, right) => {
                            for (const item of (output as unknown as { output: ObjectSubitem[] }).output) {
                                if (!link_type_is_match_js_value(item.ty, right)) continue;
                                if (same(left[item.key], right)) {
                                    return true;
                                }
                            }
                            return false;
                        },
                    ),
                not_contains_value: (not_contains_value) =>
                    check_value_and_execute(
                        value,
                        output,
                        runtime_values.find_input_value<any>(not_contains_value),
                        (left, right) => {
                            return !(() => {
                                for (const item of (output as unknown as { output: ObjectSubitem[] }).output) {
                                    if (link_type_is_match_js_value(item.ty, right) && same(left[item.key], right)) {
                                        return true;
                                    }
                                }
                                return false;
                            })();
                        },
                    ),
            }),
    });
};

export const component_condition_get_expected_component = (self: ComponentCondition): ComponentId[] => {
    const used: ComponentId[] = [];
    for (const c of self.metadata.conditions) used.push(...condition_get_expected_component(c));
    return used;
};

const condition_get_expected_component = (self: Condition): ComponentId[] => {
    const used: ComponentId[] = [];
    match_condition(self, {
        none: () => {
            /* do nothing */
        },
        required: (required) => {
            used.push(...condition_item_get_expected_component(required));
        },
        deny: (deny) => {
            used.push(...condition_item_get_expected_component(deny));
        },
        and: (and) => {
            for (const c of and) used.push(...condition_get_expected_component(c));
        },
        or: (or) => {
            for (const c of or) used.push(...condition_get_expected_component(c));
        },
        not: (not) => {
            for (const c of not) used.push(...condition_get_expected_component(c));
        },
    });
    return used;
};

const condition_item_get_expected_component = (self: ConditionItem): ComponentId[] => {
    const used: ComponentId[] = [];
    const nullable = () => {
        used.push(self.value.endpoint.id);
    };

    match_condition_matches(self.matches, {
        text: (text) =>
            match_condition_text_compare(text, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                contains: () => {
                    /* do nothing */
                },
                not_contains: () => {
                    /* do nothing */
                },
                starts_with: () => {
                    /* do nothing */
                },
                not_starts_with: () => {
                    /* do nothing */
                },
                ends_with: () => {
                    /* do nothing */
                },
                not_ends_with: () => {
                    /* do nothing */
                },
                length_equal: () => {
                    /* do nothing */
                },
                length_not_equal: () => {
                    /* do nothing */
                },
                length_greater: () => {
                    /* do nothing */
                },
                length_greater_equal: () => {
                    /* do nothing */
                },
                length_less: () => {
                    /* do nothing */
                },
                length_less_equal: () => {
                    /* do nothing */
                },
                regex: () => {
                    /* do nothing */
                },
                not_regex: () => {
                    /* do nothing */
                },
            }),
        bool: (bool) =>
            match_condition_bool_compare(bool, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                is_true: () => {
                    /* do nothing */
                },
                is_false: () => {
                    /* do nothing */
                },
            }),
        integer: (integer) =>
            match_condition_number_compare(integer, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                greater: () => {
                    /* do nothing */
                },
                greater_equal: () => {
                    /* do nothing */
                },
                less: () => {
                    /* do nothing */
                },
                less_equal: () => {
                    /* do nothing */
                },
            }),
        number: (number) =>
            match_condition_number_compare(number, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                greater: () => {
                    /* do nothing */
                },
                greater_equal: () => {
                    /* do nothing */
                },
                less: () => {
                    /* do nothing */
                },
                less_equal: () => {
                    /* do nothing */
                },
            }),
        array: (array) =>
            match_condition_array_compare(array, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                contains: () => {
                    /* do nothing */
                },
                not_contains: () => {
                    /* do nothing */
                },
                length_equal: () => {
                    /* do nothing */
                },
                length_not_equal: () => {
                    /* do nothing */
                },
                length_greater: () => {
                    /* do nothing */
                },
                length_greater_equal: () => {
                    /* do nothing */
                },
                length_less: () => {
                    /* do nothing */
                },
                length_less_equal: () => {
                    /* do nothing */
                },
            }),
        object: (object) =>
            match_condition_object_compare(object, {
                none: nullable,
                not_null: () => {
                    /* do nothing */
                },
                equal: () => {
                    /* do nothing */
                },
                not_equal: () => {
                    /* do nothing */
                },
                contains_key: () => {
                    /* do nothing */
                },
                not_contains_key: () => {
                    /* do nothing */
                },
                contains_value: () => {
                    /* do nothing */
                },
                not_contains_value: () => {
                    /* do nothing */
                },
            }),
    });
    return used;
};
