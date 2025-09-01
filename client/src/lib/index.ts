export function random_bigint(max: bigint) {
    if (typeof max === 'number') {
        max = BigInt(max);
    }

    const byteLength = (max.toString(2).length + 7) >> 3;
    const maxValue = max + 1n;

    const array = new Uint8Array(byteLength);

    while (true) {
        crypto.getRandomValues(array);

        let result = 0n;
        for (let i = 0; i < array.length; i++) {
            result = (result << 8n) + BigInt(array[i]);
        }

        if (result < maxValue) {
            return result;
        }
    }
}

export type Metadata = {
    colors: string[];
    combinations: string;
    patterns: string[];
};

export type Patterns = {
    // hex
    base: string,
    // pattern id, hex
    patterns: [string, string][]
}

export function format_num_string(num: string): string {
    let result = "";
    let count = 0;

    for (let i = num.length - 1; i >= 0; i--) {
        result = num[i] + result;
        count++;
        if (count % 3 === 0 && i !== 0) {
            result = "," + result;
        }
    }

    return result;
}

export const PER_PAGE = BigInt(20);