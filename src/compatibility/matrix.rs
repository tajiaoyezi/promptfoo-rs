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
        let value: serde_json::Value =
            serde_json::from_str(&json).map_err(|error| MatrixError::Parse(error.to_string()))?;
        if value.get("rows").is_some() {
            return serde_json::from_value(value)
                .map_err(|error| MatrixError::Parse(error.to_string()));
        }
        if let Some(source) = value.get("source_inventory").and_then(serde_json::Value::as_str) {
            let inventory =
                super::inventory::CapabilityInventory::from_json_file(Path::new(source))
                    .map_err(|error| MatrixError::Parse(format!("{error:?}")))?;
            return Ok(expand_matrix_from_inventory(
                &inventory,
                &MatrixPolicy::default(),
            ));
        }
        Err(MatrixError::Parse(
            "matrix json must contain rows or source_inventory".to_string(),
        ))
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
    inventory: &CapabilityInventory,
    policy: &MatrixPolicy,
) -> CapabilityMatrix {
    let rows = inventory
        .items
        .iter()
        .map(|item| {
            let target_status = if item.status == "unresolved" {
                policy.p2_target_status.clone()
            } else if item.owner_hint == "script-bridge"
                || item.owner_hint == "node-api-wrapper"
                || item.name.contains("npm")
            {
                "bridge".to_string()
            } else if item.level_hint == "P2" {
                "later".to_string()
            } else {
                "native".to_string()
            };

            let verification = match item.level_hint.as_str() {
                "P0" if item.status == "unresolved" => {
                    format!("blocker:{}", item.stable_id)
                }
                "P0" => format!("{}{}", policy.p0_verification_prefix, item.stable_id),
                "P1" => format!("{}{}", policy.p1_verification_prefix, item.stable_id),
                "P2" => format!("registration:{}", item.stable_id),
                _ => "blocked:invalid-level".to_string(),
            };

            let notes = if item.level_hint == "P2" || item.status == "unresolved" {
                format!(
                    "reason: {}; later: {} classification task; source: {}",
                    item.unresolved_reason
                        .as_deref()
                        .unwrap_or("P2 item requires known-gap registration"),
                    item.owner_hint,
                    item.source_reference
                )
            } else {
                format!("source: {}", item.source_reference)
            };

            CapabilityRow {
                capability: item.stable_id.clone(),
                level: item.level_hint.clone(),
                target_status,
                verification,
                owner: item.owner_hint.clone(),
                notes,
            }
        })
        .collect();
    CapabilityMatrix { rows }
}

pub fn validate_no_silent_omissions(
    inventory: &CapabilityInventory,
    matrix: &CapabilityMatrix,
) -> MatrixCompletenessReport {
    let mut report = MatrixCompletenessReport::default();

    for item in &inventory.items {
        if !matrix
            .rows
            .iter()
            .any(|row| row.capability == item.stable_id)
        {
            report.missing_matrix_rows.push(item.stable_id.clone());
        }
    }

    for row in &matrix.rows {
        if !has_compatibility_level(&row.level) {
            report.rows_missing_level.push(row.capability.clone());
        }
        if !has_target_status(&row.target_status) {
            report.rows_missing_status.push(row.capability.clone());
        }
        if row.verification.trim().is_empty() {
            report.rows_missing_verification.push(row.capability.clone());
        }
        if row.owner.trim().is_empty() {
            report.rows_missing_owner.push(row.capability.clone());
        }

        if row.level == "P0"
            && !row.verification.contains("fixture:")
            && !row.verification.contains("blocker:")
        {
            report
                .p0_rows_missing_fixture_or_blocker
                .push(row.capability.clone());
        }
        if row.level == "P1" && !row.verification.contains("snapshot:") {
            report
                .p1_rows_missing_snapshot_plan
                .push(row.capability.clone());
        }
        if row.level == "P2" {
            let notes = normalize(&row.notes);
            let status = normalize(&row.target_status);
            if !notes.contains("reason:")
                || !(notes.contains("later:") || notes.contains("unsupported:"))
                || !(status.contains("later") || status.contains("unsupported"))
            {
                report
                    .p2_rows_missing_reason_or_target
                    .push(row.capability.clone());
            }
        }
    }

    report
}

pub fn matrix_release_blockers(report: &MatrixCompletenessReport) -> Vec<MatrixBlocker> {
    let mut blockers = Vec::new();
    push_blockers(
        &mut blockers,
        &report.missing_matrix_rows,
        "silent omission: inventory item has no item-level matrix row",
    );
    push_blockers(
        &mut blockers,
        &report.rows_missing_level,
        "matrix row missing P0/P1/P2 level",
    );
    push_blockers(
        &mut blockers,
        &report.rows_missing_status,
        "matrix row missing target status",
    );
    push_blockers(
        &mut blockers,
        &report.rows_missing_verification,
        "matrix row missing verification",
    );
    push_blockers(
        &mut blockers,
        &report.rows_missing_owner,
        "matrix row missing owner",
    );
    push_blockers(
        &mut blockers,
        &report.p0_rows_missing_fixture_or_blocker,
        "P0 row missing fixture or blocker reference",
    );
    push_blockers(
        &mut blockers,
        &report.p1_rows_missing_snapshot_plan,
        "P1 row missing snapshot plan",
    );
    push_blockers(
        &mut blockers,
        &report.p2_rows_missing_reason_or_target,
        "P2 row missing reason or later/unsupported target",
    );
    blockers
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

fn push_blockers(blockers: &mut Vec<MatrixBlocker>, item_ids: &[String], reason: &str) {
    blockers.extend(item_ids.iter().map(|item_id| MatrixBlocker {
        item_id: item_id.clone(),
        reason: reason.to_string(),
    }));
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
