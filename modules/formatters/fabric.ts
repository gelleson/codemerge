import { TextFormatter, type TextFormatterOptions } from "@modules/formatters/text.ts";
import type { Formatter } from "@modules/formatters/api.ts";

type FormatterOptions = TextFormatterOptions;


export function formatter(options: FormatterOptions): Formatter {
    switch (options.type) {
        case 'text':
            return new TextFormatter(options)
        default:
            throw new FormatterNotFoundError(options.type)
    }
}


export class FormatterNotFoundError extends Error {
    constructor(type: string) {
        super(`Formatter not found: ${type}`)
    }
}
