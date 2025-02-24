export const transform_result = (value: any): any => {
    if (
        value instanceof Uint8Array ||
        value instanceof Uint16Array ||
        value instanceof Uint32Array ||
        value instanceof BigUint64Array ||
        value instanceof Int8Array ||
        value instanceof Int16Array ||
        value instanceof Int32Array ||
        value instanceof BigInt64Array
    ) {
        return Array.from(value as any);
    }

    return value;
};
