use glob::Pattern;
use ignore::{Walk, WalkBuilder};
use std::path::PathBuf;

pub fn create_walk_builder(ignores: &[PathBuf], filters: &[String]) -> Walk {
    let mut builder = WalkBuilder::new(".");
    let ignore_patterns: Vec<Pattern> = ignores
        .iter()
        .filter_map(|path| path.to_str().and_then(|s| Pattern::new(s).ok()))
        .collect();

    let filter_patterns: Vec<glob::Pattern> = filters
        .iter()
        .filter_map(|f| glob::Pattern::new(f).ok())
        .collect();

    builder.filter_entry(move |entry| {
        let path = entry.path();

        // Always include directories to traverse into them
        if entry.file_type().map_or(false, |ft| ft.is_dir()) {
            return true;
        }

        // For files, apply ignore and filter patterns
        let not_ignored = !ignore_patterns
            .iter()
            .any(|pattern| pattern.matches_path(path));
        let passes_filter = filter_patterns.is_empty()
            || filter_patterns
                .iter()
                .any(|pattern| pattern.matches_path(path));

        not_ignored && passes_filter
    });

    builder.build()
} 