use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::output::sarif::{SarifFinding, SarifLocation};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub level: FindingLevel,
    pub message: String,
    pub locations: Vec<FindingLocation>,
    pub metadata: Value,
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
        self.locations
            .iter()
            .map(|location| SarifLocation {
                uri: &location.path,
                line: location.line,
                column: Some(location.column),
            })
            .collect()
    }

    fn metadata(&self) -> Option<&Value> {
        Some(&self.metadata)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingLevel {
    Error,
    Warning,
    Note,
}

impl FindingLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingLocation {
    pub path: String,
    pub line: u64,
    pub column: u64,
}
