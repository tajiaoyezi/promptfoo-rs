use std::io::Write;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

pub struct SarifLocation<'a> {
    pub uri: &'a str,
    pub line: u64,
    pub column: Option<u64>,
}

pub trait SarifFinding {
    fn rule_id(&self) -> &str;
    fn level(&self) -> &str;
    fn message(&self) -> &str;
    fn locations(&self) -> Vec<SarifLocation<'_>>;

    fn metadata(&self) -> Option<&Value> {
        None
    }
}

impl SarifFinding for Finding {
    fn rule_id(&self) -> &str {
        &self.rule_id
    }

    fn level(&self) -> &str {
        self.level.as_str()
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn locations(&self) -> Vec<SarifLocation<'_>> {
        vec![SarifLocation {
            uri: &self.file_path,
            line: self.line,
            column: None,
        }]
    }
}

pub fn write_sarif<F: SarifFinding>(findings: &[F], writer: impl Write) -> Result<(), OutputError> {
    let results = findings
        .iter()
        .map(|finding| {
            let locations = finding
                .locations()
                .into_iter()
                .map(|location| {
                    let mut region = json!({ "startLine": location.line });
                    if let Some(column) = location.column {
                        region["startColumn"] = json!(column);
                    }
                    json!({
                        "physicalLocation": {
                            "artifactLocation": { "uri": location.uri },
                            "region": region
                        }
                    })
                })
                .collect::<Vec<_>>();
            let mut result = json!({
                "ruleId": finding.rule_id(),
                "level": finding.level(),
                "message": { "text": finding.message() },
                "locations": locations
            });
            if let Some(metadata) = finding.metadata() {
                result["properties"] = json!({ "metadata": metadata });
            }
            result
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
