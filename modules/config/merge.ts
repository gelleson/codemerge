export function mergeWithDefaults<T extends object, R extends Partial<T>>(target: T, source: R): T & R {
    const result = { ...target } as T & R;

    for (const key in source) {
        if (Object.prototype.hasOwnProperty.call(source, key)) {
            if (result[key] === undefined || result[key] === null) {
                result[key] = source[key] as any;
            }
        }
    }

    return result;
}