use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::output::OutputError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingLevel {
    Error,
    Warning,
    Note,
}

impl FindingLevel {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub level: FindingLevel,
    pub message: String,
    pub file_path: String,
    pub line: u64,
}

pub fn write_sarif(findings: &[Finding], writer: impl Write) -> Result<(), OutputError> {
    let results = findings
        .iter()
        .map(|finding| {
            json!({
                "ruleId": finding.rule_id,
                "level": finding.level.as_str(),
                "message": { "text": finding.message },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": finding.file_path },
                        "region": { "startLine": finding.line }
                    }
                }]
            })
        })
        .collect::<Vec<_>>();

    let payload = json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "promptfoo-rs",
                    "informationUri": "https://github.com/promptfoo/promptfoo"
                }
            },
            "results": results
        }]
    });
    serde_json::to_writer(writer, &payload)?;
    Ok(())
}
