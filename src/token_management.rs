use std::collections::HashMap;
use thiserror::Error;
use tiktoken_rs::{o200k_base, CoreBPE};
use claude_tokenizer::get_tokenizer;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token budget exceeded: {used} tokens used, budget is {budget}")]
    BudgetExceeded { used: usize, budget: usize },
    #[error("Tokenizer not found: {0}")]
    TokenizerNotFound(String),
    #[error("Failed to tokenize: {0}")]
    TokenizationFailed(String),
}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub model: String,
    pub budget: Option<usize>,
    pub warning_threshold: Option<f32>, // percentage of budget
}

pub struct TokenManager {
    config: TokenConfig,
    tokenizers: HashMap<String, Box<dyn Tokenizer>>,
    current_usage: usize,
}

pub trait Tokenizer: Send + Sync {
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError>;
    fn name(&self) -> &str;
}

impl TokenManager {
    pub fn new(config: TokenConfig) -> Self {
        let mut tokenizers: HashMap<String, Box<dyn Tokenizer>> = HashMap::new();
        // Initialize with default tokenizers
        tokenizers.insert("gpt-3.5".to_string(), Box::new(GPT35Tokenizer::new()) as Box<dyn Tokenizer>);
        tokenizers.insert("gpt-4".to_string(), Box::new(GPT4Tokenizer::new()) as Box<dyn Tokenizer>);
        tokenizers.insert("claude".to_string(), Box::new(ClaudeTokenizer::new()) as Box<dyn Tokenizer>);

        Self {
            config,
            tokenizers,
            current_usage: 0,
        }
    }

    pub fn register_tokenizer(&mut self, name: String, tokenizer: Box<dyn Tokenizer>) {
        self.tokenizers.insert(name, tokenizer);
    }

    pub fn count_tokens(&mut self, text: &str) -> Result<usize, TokenError> {
        let tokenizer = self.tokenizers.get(&self.config.model)
            .ok_or_else(|| TokenError::TokenizerNotFound(self.config.model.clone()))?;
        
        let count = tokenizer.count_tokens(text)?;
        self.current_usage += count;

        // Check budget if set
        if let Some(budget) = self.config.budget {
            if self.current_usage > budget {
                return Err(TokenError::BudgetExceeded {
                    used: self.current_usage,
                    budget,
                });
            }

            // Check warning threshold
            if let Some(threshold) = self.config.warning_threshold {
                let threshold_tokens = (budget as f32 * threshold) as usize;
                if self.current_usage >= threshold_tokens {
                    eprintln!(
                        "Warning: Token usage ({}) is approaching budget ({})",
                        self.current_usage, budget
                    );
                }
            }
        }

        Ok(count)
    }

    pub fn get_usage(&self) -> usize {
        self.current_usage
    }

    pub fn reset_usage(&mut self) {
        self.current_usage = 0;
    }
}

// Implementation of specific tokenizers
struct GPT35Tokenizer {
    bpe: CoreBPE,
}
struct GPT4Tokenizer {
    bpe: CoreBPE,
}
struct ClaudeTokenizer {
}

impl GPT35Tokenizer {

    fn new() -> Self {
        Self {
            bpe: o200k_base().expect("Failed to load BPE model"),
        }
    }
}

impl GPT4Tokenizer {
    fn new() -> Self {
        Self {
            bpe: o200k_base().expect("Failed to load BPE model"),
        }
    }
}

impl ClaudeTokenizer {
    fn new() -> Self {
        Self {}
    }
}

// Implement the Tokenizer trait for each specific tokenizer
impl Tokenizer for GPT35Tokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        Ok(self.bpe.encode(text, Default::default()).len())
    }

    fn name(&self) -> &str {
        "gpt-3.5"
    }
}

impl Tokenizer for GPT4Tokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        Ok(self.bpe.encode(text, Default::default()).len())
    }

    fn name(&self) -> &str {
        "gpt-4"
    }
}

impl Tokenizer for ClaudeTokenizer {
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        Ok(get_tokenizer().encode(text, Default::default()).unwrap().len())
    }

    fn name(&self) -> &str {
        "claude"
    }
} 