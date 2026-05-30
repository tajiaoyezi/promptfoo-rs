pub mod finding;

use std::fmt;
use std::fs;
use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub use finding::{Finding, FindingLevel, FindingLocation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanInput {
    pub path: String,
    pub content: String,
    pub command: &'static str,
}

impl ScanInput {
    pub fn source(path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            command: "code-scans",
        }
    }

    pub fn from_path(path: impl AsRef<Path>, command: &'static str) -> Result<Self, ScanError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|err| ScanError::new(err.to_string()))?;
        Ok(Self {
            path: normalize_path(path),
            content,
            command,
        })
    }
}

pub fn run_scan(input: ScanInput) -> Result<Vec<Finding>, ScanError> {
    let eval_pattern = Regex::new(r"\beval\s*\(").map_err(|err| ScanError::new(err.to_string()))?;
    let mut findings = Vec::new();
    for (line_index, line) in input.content.lines().enumerate() {
        if let Some(match_) = eval_pattern.find(line) {
            findings.push(Finding {
                rule_id: "promptfoo.scan.eval".to_string(),
                level: FindingLevel::Warning,
                message: "Use of eval(...) can execute untrusted code".to_string(),
                locations: vec![FindingLocation {
                    path: input.path.clone(),
                    line: line_index as u64 + 1,
                    column: match_.start() as u64 + 1,
                }],
                metadata: json!({
                    "scanner": "promptfoo-rs",
                    "command": input.command,
                    "source": "scan-engine",
                }),
            });
        }
    }
    Ok(findings)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownLimitation {
    pub id: &'static str,
    pub gate_level: &'static str,
    pub applies_to: Vec<&'static str>,
    pub reason: &'static str,
}

pub fn known_limitations() -> Vec<KnownLimitation> {
    vec![KnownLimitation {
        id: "scan.false-positive-rate",
        gate_level: "not-1.0-gate",
        applies_to: vec!["code-scans", "scan-model", "model-audit"],
        reason: "PRD §Technical Risks R5 scopes scan false-positive rate as a known limitation, not a 1.0 release gate.",
    }]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanError {
    message: String,
}

impl ScanError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ScanError {}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
