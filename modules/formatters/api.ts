import type { LoadResult } from "@modules/loader";

export interface Formatter {
     format(stream: WritableStreamDefaultWriter, results: LoadResult[]): Promise<void>
}
