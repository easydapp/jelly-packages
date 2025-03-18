import { get_cached_call_result, get_identity_value_by_id } from '..';
import { deepClone } from '../../../../common/clones';
import { CallingData, HttpActionData } from '../../../../runtime/calling';
import { RuntimeValues } from '../../../../runtime/value';
import { CodeData, CodeDataAnchor } from '../../../../store/code';
import { CodeExecutor } from '../../../../wasm';
import {
    call_trigger_get_used_component,
    ComponentCallTrigger,
    match_component_call_trigger,
} from '../../../common/call_trigger';
import { CodeContent } from '../../../common/code';
import { ComponentId } from '../../../common/identity';
import { AllEndpoints } from '../../../common/lets';
import {
    input_value_get_used_component,
    InputValue,
    named_value_get_used_component,
    NamedValue,
} from '../../../common/refer';
import { ComponentIdentityValue, PlainComponentIdentityValue } from '../../identity';
import { identity_http_metadata_get_anonymous_value, IdentityHttpOutput } from '../../identity/http';
import { get_http_body_value, http_body_get_used_component, HttpBody } from './body';
import { get_http_headers_value } from './headers';
import { HttpMethod } from './method';
import { parse_by_post } from './post';
import { parse_response_by_way, ParsedWay } from './way';

export type ExecuteHttpCall = (param: {
    identity: IdentityHttpOutput;
    url: string;
    method: HttpMethod;
    headers: [string, string][] | undefined;
    body: any;
    parsed: ParsedWay;
    alive: number;
}) => Promise<[any, [string, string][]]>;

export interface CallHttpMetadata {
    trigger: ComponentCallTrigger;
    identity?: ComponentId;
    url: InputValue;
    method: HttpMethod;
    headers?: NamedValue[];
    body?: HttpBody;
    parsed: ParsedWay;
    post?: CodeContent;
}

export const call_http_metadata_get_used_component = (self: CallHttpMetadata): ComponentId[] => {
    const used: ComponentId[] = [];
    used.push(...call_trigger_get_used_component(self.trigger));
    if (self.identity) used.push(self.identity);
    used.push(...input_value_get_used_component(self.url));
    for (const header of self.headers ?? []) used.push(...named_value_get_used_component(header));
    if (self.body) used.push(...http_body_get_used_component(self.body));
    return used;
};

export const get_call_http_value = async (
    self: CallHttpMetadata,
    id: ComponentId,
    endpoints: AllEndpoints | undefined,
    trigger: ComponentId | undefined,
    identity_triggered: Record<ComponentId, boolean>,
    identity: Record<ComponentId, ComponentIdentityValue>,
    identity_fetched: Record<ComponentId, PlainComponentIdentityValue>,
    runtime_values: RuntimeValues,
    codes: Record<CodeDataAnchor, CodeData>,
    calling: CallingData,
    code_executor: CodeExecutor | undefined,
    execute_http_call: ExecuteHttpCall | undefined,
): Promise<any> => {
    // 0. cache
    const alive = match_component_call_trigger(self.trigger, {
        loading: (loading) => loading.alive ?? 120000,
        clock: (clock) => (id !== trigger ? undefined : clock.sleep),
        click: () => (id !== trigger ? undefined : 0),
    });
    if (alive === undefined) return undefined;

    // 1. identity
    if (self.identity !== undefined) {
        identity_triggered[self.identity] = true;
    }

    calling.set_connecting(true); // ! Identity link
    let identity_metadata: IdentityHttpOutput | undefined;
    try {
        identity_metadata =
            self.identity === undefined
                ? identity_http_metadata_get_anonymous_value()
                : (
                      await get_identity_value_by_id<{
                          http: IdentityHttpOutput;
                      }>(self.identity, identity, identity_fetched)
                  )?.http;
    } finally {
        calling.set_connecting(false); // ! Identity link
    }

    if (identity_metadata === undefined) return undefined;

    calling.set_identity_value({ http: identity_metadata }); // ! Save identity

    // 2. Target address
    const url = runtime_values.find_input_value<string>(self.url, 'text');
    if (url === undefined) return undefined;

    // 3. Request
    const headers = get_http_headers_value(self.headers, runtime_values);

    // 4. Request
    let body: any = undefined;
    let data_of_request_body: any | undefined = undefined;
    if (self.body) {
        body = await get_http_body_value(
            self.body,
            endpoints,
            runtime_values,
            codes,
            (data) => (data_of_request_body = data),
            code_executor,
        );
        if (body === undefined) return undefined;
    }

    // 5. Request
    const proxy = identity_metadata.proxy ?? 'https://p.easydapp.ai';
    const proxy_body: HttpActionData = {
        url,
        method: self.method,
        headers,
        body,
        cached: alive === 0 ? undefined : alive / 1000,
    };
    const key = { proxy, proxy_body };
    const [result, response_headers] = await get_cached_call_result(
        id,
        key,
        alive,
        () => calling.start({ http: deepClone(proxy_body) }), // ! Start call
        (call_index: number, [result, response_headers]: [any, string[][]]) =>
            calling.result(call_index, {
                result: deepClone(result),
                response_headers: deepClone(response_headers),
            }), // ! Save the call result
        (call_index: number) => calling.over(call_index), // ! End call
        async () => {
            console.debug(`🚀 ~ call http key:`, key);

            // ! Agent intercept execution, if necessary
            if (execute_http_call) {
                return execute_http_call({
                    identity: identity_metadata,
                    url,
                    method: self.method,
                    headers,
                    body,
                    parsed: self.parsed,
                    alive,
                });
            }

            const response = await fetch(proxy, {
                method: 'POST',
                body: JSON.stringify(proxy_body),
            });
            // 6. Treat the response header
            const response_headers: [string, string][] = [];
            for (const key of response.headers.keys()) {
                response_headers.push([key, response.headers.get(key) ?? '']);
            }
            // 7. Processing response body
            const result: any = await parse_response_by_way(self.parsed, response);
            return [result, response_headers];
        },
    );

    if (result == undefined) throw new Error(`result of http call can not be undefined`);

    let value = result;

    // 8. Post -treatment
    if (self.post) {
        value = await parse_by_post(
            self.post,
            codes,
            {
                value,
                response_headers,
                url,
                method: self.method,
                headers,
                body,
                data_of_request_body,
            },
            code_executor,
        );
    }

    return value;
};
