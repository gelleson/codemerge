import type { LoadResult } from "@modules/loader";

export type LoadResultFilter = (result: LoadResult) => boolean

export function enabled(enabled: boolean, fn: LoadResultFilter): LoadResultFilter {
    return (result: LoadResult) => {
        if (!enabled) {
            return true
        }
        return enabled && fn(result)
    }
}

export function compose(...fns: LoadResultFilter[]): (results: LoadResult[]) => LoadResult[] {
    return (results: LoadResult[]) => results.filter(
        (result) => fns.every(fn => fn(result))
    )
}

export * from './budget.ts'
export * from './factory.ts'