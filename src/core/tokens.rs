use super::file::FileData;
use serde_json;
use std::sync::OnceLock;
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

// Initialize the tokenizer as a global singleton.
static TOKENIZER: OnceLock<CoreBPE> = OnceLock::new();

/// Return a reference to the global tokenizer.
fn get_tokenizer() -> &'static CoreBPE {
    TOKENIZER.get_or_init(|| get_bpe_from_model("gpt-4").expect("Failed to load tokenizer"))
}

/// Count the number of tokens in a given text.
pub fn count_tokens(text: &str) -> usize {
    get_tokenizer().encode_with_special_tokens(text).len()
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
