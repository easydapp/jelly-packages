export type ParsedWay = 'blob' | 'json' | 'text';

export const match_parsed_way = <T>(
    self: ParsedWay,
    {
        blob,
        json,
        text,
    }: {
        blob: () => T;
        json: () => T;
        text: () => T;
    },
): T => {
    if ('blob' === self) return blob();
    if ('json' === self) return json();
    if ('text' === self) return text();
    throw new Error('invalid http parsed way');
};

export const parse_response_by_way = async (self: ParsedWay, response: Response) => {
    let result: any = undefined;
    switch (self) {
        case 'blob': {
            const value = new Uint8Array(await (await response.blob()).arrayBuffer());
            result = [];
            for (let i = 0; i < value.byteLength; i++) result[i] = value[i];
            break;
        }
        case 'json': {
            result = await response.json();
            break;
        }
        case 'text': {
            result = await response.text();
            break;
        }
    }
    return result;
};
