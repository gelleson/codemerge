use super::file::FileData;
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{OnceLock, RwLock};
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

// Initialize the tokenizer and cache as global singletons.
static TOKENIZER: OnceLock<CoreBPE> = OnceLock::new();
static TOKEN_CACHE: OnceLock<TokenCache> = OnceLock::new();

/// A cache that stores token counts indexed by a 64-bit hash.
struct TokenCache {
    cache: RwLock<HashMap<u64, usize>>,
    max_entries: usize,
}

impl TokenCache {
    fn new(max_entries: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::with_capacity(max_entries)),
            max_entries,
        }
    }

    /// Look up the token count by hash key. If not present, compute it and insert into the cache.
    fn get_or_insert<F>(&self, key: u64, f: F) -> usize
    where
        F: FnOnce() -> usize,
    {
        if let Ok(cache) = self.cache.read() {
            if let Some(&count) = cache.get(&key) {
                return count;
            }
        }
        let count = f();
        if let Ok(mut cache) = self.cache.write() {
            if cache.len() >= self.max_entries {
                cache.clear();
            }
            cache.insert(key, count);
        }
        count
    }
}

/// Compute a 64-bit hash for the given text to use as a cache key.
fn compute_hash(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

/// Return a reference to the global tokenizer.
fn get_tokenizer() -> &'static CoreBPE {
    TOKENIZER.get_or_init(|| get_bpe_from_model("gpt-4").expect("Failed to load tokenizer"))
}

/// Return a reference to the global token cache.
fn get_cache() -> &'static TokenCache {
    TOKEN_CACHE.get_or_init(|| {
        TokenCache::new(10_000) // Cache up to 10k entries
    })
}

/// Count the number of tokens in a given text. For texts shorter than 100 characters,
/// we bypass the cache. For longer texts, we use the hash of the text as a cache key.
pub fn count_tokens(text: &str) -> usize {
    if text.len() < 100 {
        // Bypass the cache for very short texts.
        return get_tokenizer().encode_with_special_tokens(text).len();
    }
    let key = compute_hash(text);
    get_cache().get_or_insert(key, || {
        get_tokenizer().encode_with_special_tokens(text).len()
    })
}

pub fn format_token_board(files: &[FileData], max_display: usize) -> String {
    let mut result = String::new();
    let max_path_len = files.iter().map(|f| f.path.len()).max().unwrap_or(0);

    result.push_str("\nToken Statistics:\n");
    result.push_str(&"─".repeat(max_path_len + 20));
    result.push('\n');

    let mut sorted_files = files.to_vec();
    sorted_files.sort_by(|a, b| b.tokens.cmp(&a.tokens));

    for file in sorted_files.iter().take(max_display) {
        let padding = " ".repeat(max_path_len - file.path.len());
        result.push_str(&format!(
            "{}{} │ {:>8} tokens\n",
            file.path, padding, file.tokens
        ));
    }

    result.push_str(&"─".repeat(max_path_len + 20));
    result.push('\n');

    let total_tokens: usize = files.iter().map(|f| f.tokens).sum();
    result.push_str(&format!("Total tokens: {}\n", total_tokens));

    result
}

pub fn format_token_json(files: &[FileData], max_display: usize) -> String {
    let total: usize = files.iter().map(|f| f.tokens).sum();
    let mut sorted_files = files.to_vec();
    sorted_files.sort_by(|a, b| b.tokens.cmp(&a.tokens));

    let display_files: Vec<_> = sorted_files
        .iter()
        .take(max_display)
        .map(|f| {
            serde_json::json!({
                "path": f.path,
                "tokens": f.tokens,
            })
        })
        .collect();

    serde_json::json!({
        "total": total,
        "results": display_files,
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_counting() {
        let text = "Hello, world!";
        assert!(count_tokens(text) > 0);
    }

    #[test]
    fn test_token_caching() {
        let text = "This is a test of the token cache.".repeat(10);

        // First count should compute
        let count1 = count_tokens(&text);

        // Second count should use cache
        let count2 = count_tokens(&text);

        assert_eq!(count1, count2);
    }

    #[test]
    fn test_token_board_formatting() {
        let files = vec![
            FileData::new("test1.txt", "content1"),
            FileData::new("test2.txt", "content2"),
        ];

        let board = format_token_board(&files, 2);
        assert!(board.contains("test1.txt"));
        assert!(board.contains("test2.txt"));
        assert!(board.contains("tokens"));
    }

    #[test]
    fn test_token_json_formatting() {
        let files = vec![
            FileData::new("test1.txt", "content1"),
            FileData::new("test2.txt", "content2"),
        ];

        let json = format_token_json(&files, 2);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["total"].as_u64().is_some());
        assert!(parsed["results"].as_array().unwrap().len() == 2);
    }
}
