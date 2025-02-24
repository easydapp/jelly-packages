// Hex text to array
export const hex2array = (hex: string): number[] => {
    if (hex.startsWith('0x')) hex = hex.slice(2);
    if (hex.length === 0) return [];
    if (hex.length % 2 !== 0) throw new Error(`Invalid hex text: ${hex}`);
    const value: number[] = [];
    for (let i = 0; i < hex.length; i += 2) value.push(parseInt(hex.slice(i, i + 2), 16));
    return value;
};

// Array to hex text. No 0x prefix
export const array2hex = (value: number[] | Uint8Array): string => {
    let s = '';
    // eslint-disable-next-line @typescript-eslint/prefer-for-of
    for (let i = 0; i < value.length; i++) {
        s += value[i].toString(16).padStart(2, '0');
    }
    return s;
};
