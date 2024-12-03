import type { LoadResult } from "@modules/loader";
import type { Formatter } from "@modules/formatters/api.ts";

export type TextFormatterOptions = {
    type: 'text'
}


export class TextFormatter implements Formatter {
    private options: TextFormatterOptions

    constructor(options: TextFormatterOptions) {
        this.options = options
    }

    async format(writer: WritableStreamDefaultWriter, results: LoadResult[]) {
        await writer.write("=== Result ===")
        await writer.write("\n")

        for (const result of results) {
            if (result.content) {
                await writer.write(`File: ${result.path}\n`)
                await writer.write(result.content)
            }
        }
    }
}