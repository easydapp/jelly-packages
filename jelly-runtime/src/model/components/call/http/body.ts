import { deepClone } from '../../../../common/clones';
import { RuntimeValues } from '../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../store/code';
import { CodeExecutor } from '../../../../wasm';
import { CodeContent } from '../../../common/code';
import { ComponentId } from '../../../common/identity';
import { AllEndpoints } from '../../../common/lets';
import {
    code_value_get_used_component,
    CodeValue,
    named_value_get_used_component,
    NamedValue,
} from '../../../common/refer';
import { doFunctionTransformByCodeContent } from '../../code';

export interface HttpBodyPlain {
    data?: NamedValue[];
}

export interface HttpBodyCode {
    data?: CodeValue[];
    code: CodeContent;
}

export type HttpBody = { plain: HttpBodyPlain } | { code: HttpBodyCode };

export const match_http_body = <T>(
    self: HttpBody,
    {
        plain,
        code,
    }: {
        plain: (plain: HttpBodyPlain) => T;
        code: (code: HttpBodyCode) => T;
    },
): T => {
    if ('plain' in self) return plain(self.plain);
    if ('code' in self) return code(self.code);
    throw new Error('invalid http body');
};
export const match_http_body_async = async <T>(
    self: HttpBody,
    {
        plain,
        code,
    }: {
        plain: (plain: HttpBodyPlain) => Promise<T>;
        code: (code: HttpBodyCode) => Promise<T>;
    },
): Promise<T> => {
    if ('plain' in self) return plain(self.plain);
    if ('code' in self) return code(self.code);
    throw new Error('invalid http body');
};

export const http_body_get_used_component = (self: HttpBody): ComponentId[] => {
    const used: ComponentId[] = [];
    match_http_body(self, {
        plain: (plain) => {
            for (const named_value of plain.data ?? []) {
                used.push(...named_value_get_used_component(named_value));
            }
        },
        code: (code) => {
            for (const code_value of code.data ?? []) {
                used.push(...code_value_get_used_component(code_value));
            }
        },
    });
    return used;
};

export const get_http_body_value = async (
    self: HttpBody,
    endpoints: AllEndpoints | undefined,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    handle: (data_of_request_body: any) => void,
    code_executor: CodeExecutor | undefined,
) => {
    return await match_http_body_async(self, {
        plain: async (plain) => {
            const body: any = {};
            for (const named_value of plain.data ?? []) {
                const value = runtime_values.find_input_value(named_value.value);
                if (value === undefined) return undefined;
                body[named_value.name] = value;
            }
            return body;
        },
        code: async (code) => {
            const data = runtime_values.find_data(endpoints, code.data ?? []);
            if (data === undefined) return undefined;
            handle(deepClone(data));
            return await doFunctionTransformByCodeContent(
                code.code,
                codes,
                [['data', data]],
                code_executor,
            );
        },
    });
};
