use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::inventory::CapabilityInventory;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityMatrix {
    pub rows: Vec<CapabilityRow>,
}

impl CapabilityMatrix {
    pub fn from_markdown(path: &Path) -> Result<Self, MatrixError> {
        let markdown = fs::read_to_string(path).map_err(MatrixError::Read)?;
        Ok(parse_matrix(&markdown))
    }

    pub fn from_json_file(path: &Path) -> Result<Self, MatrixError> {
        let json = fs::read_to_string(path).map_err(MatrixError::Read)?;
        serde_json::from_str(&json).map_err(|error| MatrixError::Parse(error.to_string()))
    }

    pub fn covers_domain(&self, domain: &str) -> bool {
        let needle = normalize(domain);
        self.rows.iter().any(|row| {
            let haystack = normalize(&row.capability);
            match needle.as_str() {
                "cli" => haystack.contains("cli"),
                "config" => haystack.contains("config") || haystack.contains("promptfooconfig"),
                "provider" => haystack.contains("provider"),
                "assertion" => haystack.contains("assertion"),
                "redteam" => haystack.contains("redteam"),
                "mcp" => haystack.contains("mcp"),
                "scan" => haystack.contains("scan") || haystack.contains("audit"),
                "output" => haystack.contains("output") || haystack.contains("json"),
                "node api" => haystack.contains("node api") || haystack.contains("wrapper"),
                "cloud/share" => haystack.contains("cloud") && haystack.contains("share"),
                _ => haystack.contains(&needle),
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRow {
    pub capability: String,
    pub level: String,
    pub target_status: String,
    pub verification: String,
    pub owner: String,
    pub notes: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MatrixReport {
    pub missing_domains: Vec<String>,
    pub rows_missing_level: Vec<String>,
    pub rows_missing_target_status: Vec<String>,
    pub rows_missing_verification: Vec<String>,
    pub rows_missing_owner: Vec<String>,
    pub p2_rows_missing_reason: Vec<String>,
}

#[derive(Debug)]
pub enum MatrixError {
    Read(std::io::Error),
    Parse(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixPolicy {
    pub p0_verification_prefix: String,
    pub p1_verification_prefix: String,
    pub p2_target_status: String,
}

impl Default for MatrixPolicy {
    fn default() -> Self {
        Self {
            p0_verification_prefix: "fixture:".to_string(),
            p1_verification_prefix: "snapshot:".to_string(),
            p2_target_status: "later".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MatrixCompletenessReport {
    pub missing_matrix_rows: Vec<String>,
    pub rows_missing_level: Vec<String>,
    pub rows_missing_status: Vec<String>,
    pub rows_missing_verification: Vec<String>,
    pub rows_missing_owner: Vec<String>,
    pub p0_rows_missing_fixture_or_blocker: Vec<String>,
    pub p1_rows_missing_snapshot_plan: Vec<String>,
    pub p2_rows_missing_reason_or_target: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixBlocker {
    pub item_id: String,
    pub reason: String,
}

pub fn expand_matrix_from_inventory(
    _inventory: &CapabilityInventory,
    _policy: &MatrixPolicy,
) -> CapabilityMatrix {
    unimplemented!("task-11.3 RED skeleton: matrix expansion is not implemented")
}

pub fn validate_no_silent_omissions(
    _inventory: &CapabilityInventory,
    _matrix: &CapabilityMatrix,
) -> MatrixCompletenessReport {
    unimplemented!("task-11.3 RED skeleton: matrix completeness validator is not implemented")
}

pub fn matrix_release_blockers(_report: &MatrixCompletenessReport) -> Vec<MatrixBlocker> {
    unimplemented!("task-11.3 RED skeleton: matrix blocker conversion is not implemented")
}

pub fn validate_matrix_completeness(matrix: &CapabilityMatrix) -> MatrixReport {
    let mut report = MatrixReport::default();

    for domain in [
        "CLI",
        "config",
        "provider",
        "assertion",
        "redteam",
        "MCP",
        "scan",
        "output",
        "Node API",
        "cloud/share",
    ] {
        if !matrix.covers_domain(domain) {
            report.missing_domains.push(domain.to_string());
        }
    }

    for row in &matrix.rows {
        if !has_compatibility_level(&row.level) {
            report.rows_missing_level.push(row.capability.clone());
        }
        if !has_target_status(&row.target_status) {
            report
                .rows_missing_target_status
                .push(row.capability.clone());
        }
        if row.verification.trim().is_empty() {
            report
                .rows_missing_verification
                .push(row.capability.clone());
        }
        if row.owner.trim().is_empty() {
            report.rows_missing_owner.push(row.capability.clone());
        }
        if row.level.contains("P2") && !normalize(&row.notes).contains("p2 reason") {
            report.p2_rows_missing_reason.push(row.capability.clone());
        }
    }

    report
}

fn parse_matrix(markdown: &str) -> CapabilityMatrix {
    let rows = markdown
        .lines()
        .filter_map(markdown_row_cells)
        .filter(|cells| cells.len() == 6)
        .filter(|cells| cells[0] != "Capability" && !cells[0].starts_with("---"))
        .map(|cells| CapabilityRow {
            capability: cells[0].clone(),
            level: cells[1].clone(),
            target_status: cells[2].clone(),
            verification: cells[3].clone(),
            owner: cells[4].clone(),
            notes: cells[5].clone(),
        })
        .collect();

    CapabilityMatrix { rows }
}

fn markdown_row_cells(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    Some(
        trimmed
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect(),
    )
}

fn has_compatibility_level(level: &str) -> bool {
    level
        .split('/')
        .any(|part| matches!(part, "P0" | "P1" | "P2"))
}

fn has_target_status(status: &str) -> bool {
    let normalized = normalize(status);
    ["native", "bridge", "unsupported", "later"]
        .iter()
        .any(|allowed| normalized.contains(allowed))
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
