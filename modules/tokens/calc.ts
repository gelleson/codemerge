import {
    countTokens,
    isWithinTokenLimit,
} from 'gpt-tokenizer'
import type { LoadResult } from "@modules/loader";

export function calcTokens(text: string) {
    return countTokens(text)
}

export function calcTokensWithLimit(text: string, limit: number): false | number {
    return isWithinTokenLimit(text, limit)
}

export function sort(array: LoadResult[]) {
    return array.sort((a, b) => {
        return b.tokens - a.tokens
    })
}
