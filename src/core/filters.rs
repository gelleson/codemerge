use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Static set of filter patterns that are always active
pub static STATIC_FILTERS: Lazy<HashSet<String>> = Lazy::new(|| {
    let mut filters = HashSet::new();

    // Version Control
    filters.insert("**/.git/**".to_string());
    filters.insert("**/.svn/**".to_string());
    filters.insert("**/.hg/**".to_string());
    filters.insert("**/.gitignore".to_string());
    filters.insert("**/.gitattributes".to_string());

    // Dependencies
    filters.insert("**/node_modules/**".to_string());
    filters.insert("**/target/**".to_string());
    filters.insert("**/vendor/**".to_string());
    filters.insert("**/*.lock".to_string());
    filters.insert("**/package-lock.json".to_string());
    filters.insert("**/yarn.lock".to_string());
    filters.insert("**/Cargo.lock".to_string());
    filters.insert("**/bun.lockb".to_string());

    // IDE and Editor
    filters.insert("**/.idea/**".to_string());
    filters.insert("**/.vscode/**".to_string());
    filters.insert("**/.vs/**".to_string());
    filters.insert("**/*.swp".to_string());
    filters.insert("**/*.swo".to_string());
    filters.insert("**/*.swn".to_string());
    filters.insert("**/*.bak".to_string());

    // Build and Output
    filters.insert("**/dist/**".to_string());
    filters.insert("**/build/**".to_string());
    filters.insert("**/out/**".to_string());
    filters.insert("**/bin/**".to_string());
    filters.insert("**/*.o".to_string());
    filters.insert("**/*.pyc".to_string());
    filters.insert("**/__pycache__/**".to_string());

    // System Files
    filters.insert("**/.DS_Store".to_string());
    filters.insert("**/.Spotlight-V100".to_string());
    filters.insert("**/.Trashes".to_string());
    filters.insert("**/Thumbs.db".to_string());
    filters.insert("**/desktop.ini".to_string());

    // Logs and Temporary
    filters.insert("**/logs/**".to_string());
    filters.insert("**/tmp/**".to_string());
    filters.insert("**/temp/**".to_string());
    filters.insert("**/*.log".to_string());
    filters.insert("**/npm-debug.log*".to_string());
    filters.insert("**/yarn-debug.log*".to_string());
    filters.insert("**/yarn-error.log*".to_string());

    // Environment and Configuration
    filters.insert("**/.env".to_string());
    filters.insert("**/.env.*".to_string());
    filters.insert("**/.env.local".to_string());
    filters.insert("**/.env.development.local".to_string());
    filters.insert("**/.env.test.local".to_string());
    filters.insert("**/.env.production.local".to_string());

    filters
});

/// Combine user-provided ignore patterns with static filters
pub fn combine_ignores(user_ignores: &[String]) -> Vec<String> {
    let mut all_ignores = STATIC_FILTERS.iter().cloned().collect::<Vec<_>>();
    all_ignores.extend(user_ignores.iter().cloned());
    all_ignores
}
