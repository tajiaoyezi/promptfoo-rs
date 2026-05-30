pub mod csv;
pub mod html;
pub mod json;
pub mod junit;
pub mod sarif;

use std::fmt;
use std::io::Write;

use serde::Serialize;

use crate::results::{ResultRecord, ResultStatus};

pub use sarif::{write_sarif, Finding, FindingLevel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Jsonl,
    Csv,
    Yaml,
    Junit,
    Sarif,
    Html,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RunSummary {
    pub eval_id: String,
    pub records: Vec<ResultRecord>,
}

impl RunSummary {
    pub fn counts(&self) -> SummaryCounts {
        self.records
            .iter()
            .fold(SummaryCounts::default(), |mut counts, record| {
                counts.total += 1;
                match record.status {
                    ResultStatus::Passed => counts.passed += 1,
                    ResultStatus::Failed => counts.failed += 1,
                    ResultStatus::Error => counts.errors += 1,
                    ResultStatus::Skipped => counts.skipped += 1,
                }
                counts
            })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SummaryCounts {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
    pub skipped: usize,
}

pub fn write_output(
    format: OutputFormat,
    summary: &RunSummary,
    writer: impl Write,
) -> Result<(), OutputError> {
    match format {
        OutputFormat::Json => json::write_json(summary, writer),
        OutputFormat::Jsonl => json::write_jsonl(summary, writer),
        OutputFormat::Csv => csv::write_csv(summary, writer),
        OutputFormat::Yaml => json::write_yaml(summary, writer),
        OutputFormat::Junit => junit::write_junit(summary, writer),
        OutputFormat::Sarif => {
            let findings: [sarif::Finding; 0] = [];
            sarif::write_sarif(&findings, writer)
        }
        OutputFormat::Html => html::write_html(summary, writer),
    }
}

pub fn write_junit(summary: &RunSummary, writer: impl Write) -> Result<(), OutputError> {
    junit::write_junit(summary, writer)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputError {
    message: String,
}

impl OutputError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OutputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for OutputError {}

impl From<std::io::Error> for OutputError {
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<serde_json::Error> for OutputError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<serde_yaml::Error> for OutputError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::new(value.to_string())
    }
}

impl From<::csv::Error> for OutputError {
    fn from(value: ::csv::Error) -> Self {
        Self::new(value.to_string())
    }
}
