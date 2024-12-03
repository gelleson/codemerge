import { compose, enabled } from "@modules/filters/index.ts";
import { removeLowBudget, removeHighBudget } from "@modules/filters/budget.ts";
import { emptyContentFilter } from "@modules/filters/content.ts";

export type Options = {
    maxBudget: number;
    minBudget: number;
    limitByHighBudget: boolean;
    limitByLowBudget: boolean;
}

export const staticFilters = (options: Options) => {
    return compose(
        enabled(options.limitByHighBudget, removeHighBudget(options.maxBudget)),
        enabled(options.limitByLowBudget, removeLowBudget(options.minBudget)),
        emptyContentFilter(),
    )
}