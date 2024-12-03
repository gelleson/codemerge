import type { LoadResult } from "@modules/loader";
import type { LoadResultFilter } from "@modules/filters/index.ts";


export function removeHighBudget(highBudget: number = 10_000): LoadResultFilter {
    return (result: LoadResult) => {
        return result.tokens < highBudget
    }
}

export function removeLowBudget(lowBudget: number = 0): LoadResultFilter {
    return (result: LoadResult) => {
        return result.tokens >= lowBudget
    }
}