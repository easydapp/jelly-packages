// Add a comma every 3 number
export const format_number = (text_number: string, count = 3): string => {
    const splits = text_number.split('.');
    const res1: string[] = [];
    const res2: string[] = [];
    splits[0]
        .split('')
        .reverse()
        .map((item, i) => {
            if (i % count == 0 && i != 0) res1.push(',');
            res1.push(item);
        });
    if (splits.length > 1) {
        splits[1].split('').map((item, i) => {
            if (i % count == 0 && i != 0) res2.push(',');
            res2.push(item);
        });
    }
    return res1.reverse().join('') + (splits.length > 1 ? '.' + res2.join('') : '');
};

// There is only an integer part for each three digits
export const format_integer = (text_number: string, count = 3): string => {
    const splits = text_number.split('.');
    const res1: string[] = [];
    splits[0]
        .split('')
        .reverse()
        .map((item, i) => {
            if (i % count == 0 && i != 0) res1.push(',');
            res1.push(item);
        });
    return res1.reverse().join('') + (splits.length > 1 ? '.' + splits[1] : '');
};

/// Back to a unit number with specified accuracy
export const unit = (decimal: number): string => {
    let unit = 1n;
    while (0 < decimal) {
        unit *= 10n;
        decimal -= 1;
    }
    return `${unit}`;
};
