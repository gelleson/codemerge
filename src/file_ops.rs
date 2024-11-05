use glob::Pattern;
use ignore::{Walk, WalkBuilder};

pub fn create_walk_builder(ignores: &[String], filters: &[String]) -> Walk {
    let mut builder = WalkBuilder::new(".");
    
    // Enable .gitignore functionality
    builder.git_ignore(true);
    builder.hidden(false); // Don't ignore hidden files by default
    
    // Add custom ignore patterns
    let ignore_patterns: Vec<Pattern> = ignores
        .iter()
        .filter_map(|s| Pattern::new(s).ok())
        .collect();

    let filter_patterns: Vec<Pattern> = filters
        .iter()
        .filter_map(|f| Pattern::new(f).ok())
        .collect();

    builder.filter_entry(move |entry| {
        let path = entry.path();

        // Explicitly ignore .git directories
        if path.components().any(|c| c.as_os_str() == ".git") {
            return false;
        }

        // For directories other than .git, include them to traverse
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
