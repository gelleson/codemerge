import type { LoadResultFilter } from "@modules/filters/index.ts";

export function emptyContentFilter(): LoadResultFilter {
  return (result) => {
    if (result.error) {
      return false;
    }
    return result.content !== "";
  };
}
