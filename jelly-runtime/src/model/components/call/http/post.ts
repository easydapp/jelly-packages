import { CodeData, CodeDataAnchor } from '../../../../store/code';
import { CodeExecutor } from '../../../../wasm';
import { CodeContent } from '../../../common/code';
import { doFunctionTransformByCodeContent } from '../../code';
import { HttpMethod } from './method';

export const parse_by_post = async (
    self: CodeContent,
    codes: Record<CodeDataAnchor, CodeData>,
    {
        value,
        response_headers,
        url,
        method,
        headers,
        body,
        data_of_request_body,
    }: {
        value: any;
        response_headers: [string, string][];
        url: string;
        method: HttpMethod;
        headers: [string, string][] | undefined;
        body: any | undefined;
        data_of_request_body: any | undefined;
    },
    code_executor: CodeExecutor | undefined,
) => {
    return await doFunctionTransformByCodeContent(
        self,
        codes,
        [
            ['data', value],
            ['response_headers', response_headers],
            ['request_url', url],
            ['request_method', method],
            ['request_headers', headers],
            ['request_body', body],
            ['data_of_request_body', data_of_request_body],
        ],
        code_executor,
    );
};
