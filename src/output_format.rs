use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Failed to format output: {0}")]
    FormattingError(String),
    #[error("Template error: {0}")]
    TemplateError(String),
}

#[derive(Debug, Serialize)]
pub struct TokenAnalysis {
    pub file_path: PathBuf,
    pub token_count: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisReport {
    pub files: Vec<TokenAnalysis>,
    pub total_tokens: usize,
    pub budget: Option<usize>,
    pub budget_remaining: Option<usize>,
    pub model: String,
    pub timestamp: String,
}

pub trait OutputFormatter {
    fn format(&self, report: &AnalysisReport) -> Result<String, FormatError>;
}

pub struct JsonFormatter;
pub struct CsvFormatter;
pub struct MarkdownFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for JsonFormatter {
    fn format(&self, report: &AnalysisReport) -> Result<String, FormatError> {
        serde_json::to_string_pretty(report)
            .map_err(|e| FormatError::FormattingError(e.to_string()))
    }
}

impl CsvFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for CsvFormatter {
    fn format(&self, report: &AnalysisReport) -> Result<String, FormatError> {
        let mut wtr = csv::WriterBuilder::new()
            .from_writer(vec![]);
        
        // Write header
        wtr.write_record(&["File Path", "Token Count", "Metadata"])
            .map_err(|e| FormatError::FormattingError(e.to_string()))?;
        
        // Write data
        for analysis in &report.files {
            let metadata_str = serde_json::to_string(&analysis.metadata)
                .unwrap_or_default();
            wtr.write_record(&[
                analysis.file_path.to_string_lossy().to_string(),
                analysis.token_count.to_string(),
                metadata_str,
            ]).map_err(|e| FormatError::FormattingError(e.to_string()))?;
        }
        
        // Write summary
        wtr.write_record(&["", "", ""])
            .map_err(|e| FormatError::FormattingError(e.to_string()))?;
        wtr.write_record(&["Total Tokens", &report.total_tokens.to_string(), ""])
            .map_err(|e| FormatError::FormattingError(e.to_string()))?;
        
        String::from_utf8(wtr.into_inner()
            .map_err(|e| FormatError::FormattingError(e.to_string()))?)
            .map_err(|e| FormatError::FormattingError(e.to_string()))
    }
}

impl MarkdownFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for MarkdownFormatter {
    fn format(&self, report: &AnalysisReport) -> Result<String, FormatError> {
        let mut output = String::new();
        
        // Add header
        output.push_str("# Token Analysis Report\n\n");
        output.push_str(&format!("Model: {}\n", report.model));
        output.push_str(&format!("Time: {}\n\n", report.timestamp));
        
        // Add table header
        output.push_str("## File Analysis\n\n");
        output.push_str("| File | Token Count | Metadata |\n");
        output.push_str("|------|-------------|----------|\n");
        
        // Add file data
        for analysis in &report.files {
            let metadata_str = serde_json::to_string(&analysis.metadata)
                .unwrap_or_default();
            output.push_str(&format!(
                "| {} | {} | {} |\n",
                analysis.file_path.to_string_lossy(),
                analysis.token_count,
                metadata_str
            ));
        }
        
        // Add summary
        output.push_str("\n## Summary\n\n");
        output.push_str(&format!("Total Tokens: {}\n", report.total_tokens));
        if let Some(budget) = report.budget {
            output.push_str(&format!("Budget: {}\n", budget));
        }
        if let Some(remaining) = report.budget_remaining {
            output.push_str(&format!("Remaining: {}\n", remaining));
        }
        
        Ok(output)
    }
}

pub fn create_formatter(format: &str) -> Result<Box<dyn OutputFormatter>, FormatError> {
    match format.to_lowercase().as_str() {
        "json" => Ok(Box::new(JsonFormatter::new())),
        "csv" => Ok(Box::new(CsvFormatter::new())),
        "markdown" | "md" => Ok(Box::new(MarkdownFormatter::new())),
        _ => Err(FormatError::UnsupportedFormat(format.to_string())),
    }
} 