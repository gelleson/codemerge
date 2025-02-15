use crate::core::file::FileData;

pub fn apply_budget_filters(
    files: Vec<FileData>,
    min: usize,
    max: usize,
    limit_high: bool,
    limit_low: bool,
) -> Vec<FileData> {
    files
        .into_iter()
        .filter(|fd| {
            // Always skip files with 0 tokens
            if fd.tokens == 0 {
                return false;
            }

            let passes_low = if limit_low { fd.tokens >= min } else { true };
            let passes_high = if limit_high { fd.tokens <= max } else { true };
            passes_low && passes_high
        })
        .collect()
}
