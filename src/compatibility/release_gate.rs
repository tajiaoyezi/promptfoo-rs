use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::compatibility::diff::{DiffClass, DiffFinding};
use crate::compatibility::matrix::{CapabilityMatrix, CapabilityRow};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseChannel {
    Stable,
    Prerelease,
    Nightly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseGateStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseGateSummary {
    pub status: ReleaseGateStatus,
    pub release_channel: ReleaseChannel,
    pub stable_allowed: bool,
    pub blocking_findings: Vec<DiffFinding>,
    pub required_p0_fixture_count: usize,
    pub observed_p0_fixture_count: usize,
    pub artifact_paths: Vec<String>,
    pub missing_artifact_paths: Vec<String>,
    pub p1_snapshot_total: usize,
    pub p1_snapshot_covered: usize,
    pub p2_registration_total: usize,
    pub p2_registered: usize,
    pub notes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateConfig {
    pub matrix: CapabilityMatrix,
    pub findings: Vec<DiffFinding>,
    pub required_p0_fixture_count: usize,
    pub observed_p0_fixture_count: usize,
    pub artifact_paths: Vec<PathBuf>,
    pub release_channel: ReleaseChannel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateError {
    message: String,
    exit_code: i32,
}

impl GateError {
    pub fn new(message: impl Into<String>, exit_code: i32) -> Self {
        Self {
            message: message.into(),
            exit_code,
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl std::fmt::Display for GateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for GateError {}

pub fn evaluate_release_gate(
    matrix: &CapabilityMatrix,
    findings: &[DiffFinding],
) -> ReleaseGateSummary {
    let blocking_findings = findings
        .iter()
        .filter(|finding| {
            is_p0_capability(matrix, &finding.capability)
                && matches!(finding.class, DiffClass::Bug | DiffClass::Unclassified)
        })
        .cloned()
        .collect::<Vec<_>>();

    let p1_rows = rows_with_level(matrix, "P1");
    let p2_rows = rows_with_level(matrix, "P2");
    let p1_snapshot_covered = p1_rows
        .iter()
        .filter(|row| has_non_blocking_finding(findings, &row.capability))
        .count();
    let p2_registered = p2_rows
        .iter()
        .filter(|row| has_non_blocking_finding(findings, &row.capability))
        .count();

    let status = if blocking_findings.is_empty() {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    };

    ReleaseGateSummary {
        status,
        release_channel: ReleaseChannel::Stable,
        stable_allowed: status == ReleaseGateStatus::Ready,
        blocking_findings,
        required_p0_fixture_count: 0,
        observed_p0_fixture_count: 0,
        artifact_paths: Vec::new(),
        missing_artifact_paths: Vec::new(),
        p1_snapshot_total: p1_rows.len(),
        p1_snapshot_covered,
        p2_registration_total: p2_rows.len(),
        p2_registered,
        notes: vec![
            format!(
                "P1 snapshot coverage: {p1_snapshot_covered}/{}",
                p1_rows.len()
            ),
            format!(
                "P2 registration coverage: {p2_registered}/{}",
                p2_rows.len()
            ),
        ],
    }
}

pub fn run_full_compatibility_gate(config: &GateConfig) -> Result<ReleaseGateSummary, GateError> {
    let mut summary = evaluate_release_gate(&config.matrix, &config.findings);
    summary.release_channel = config.release_channel;
    summary.required_p0_fixture_count = config.required_p0_fixture_count;
    summary.observed_p0_fixture_count = config.observed_p0_fixture_count;
    summary.artifact_paths = config
        .artifact_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect();
    summary.missing_artifact_paths = config
        .artifact_paths
        .iter()
        .filter(|path| !path.exists())
        .map(|path| path.display().to_string())
        .collect();

    if config.observed_p0_fixture_count < config.required_p0_fixture_count {
        summary.status = ReleaseGateStatus::Blocked;
        summary.notes.push(format!(
            "P0 fixture coverage missing: {}/{}",
            config.observed_p0_fixture_count, config.required_p0_fixture_count
        ));
    }
    if !summary.missing_artifact_paths.is_empty() {
        summary.status = ReleaseGateStatus::Blocked;
        summary.notes.push(format!(
            "release gate artifacts missing: {}",
            summary.missing_artifact_paths.join(", ")
        ));
    }
    let incomplete_rows = incomplete_matrix_rows(&config.matrix);
    if !incomplete_rows.is_empty() {
        summary.status = ReleaseGateStatus::Blocked;
        summary.notes.push(format!(
            "matrix has incomplete release gate rows: {}",
            incomplete_rows.join(", ")
        ));
    }

    summary.stable_allowed = summary.status == ReleaseGateStatus::Ready
        || config.release_channel != ReleaseChannel::Stable;
    Ok(summary)
}

pub fn write_release_gate_summary(
    summary: &ReleaseGateSummary,
    path: &Path,
) -> Result<(), GateError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            GateError::new(
                format!("failed to create summary dir {}: {error}", parent.display()),
                1,
            )
        })?;
    }
    let json = serde_json::to_string_pretty(summary)
        .map_err(|error| GateError::new(format!("failed to serialize gate summary: {error}"), 1))?;
    fs::write(path, json)
        .map_err(|error| GateError::new(format!("failed to write {}: {error}", path.display()), 1))
}

pub fn assert_stable_allowed(summary: &ReleaseGateSummary) -> Result<(), GateError> {
    if summary.release_channel == ReleaseChannel::Stable && !summary.stable_allowed {
        return Err(GateError::new(
            "stable release blocked by compatibility gate",
            1,
        ));
    }
    Ok(())
}

fn is_p0_capability(matrix: &CapabilityMatrix, capability: &str) -> bool {
    matrix
        .rows
        .iter()
        .any(|row| same_capability(&row.capability, capability) && row.level.contains("P0"))
}

fn rows_with_level<'a>(matrix: &'a CapabilityMatrix, level: &str) -> Vec<&'a CapabilityRow> {
    matrix
        .rows
        .iter()
        .filter(|row| row.level.contains(level))
        .collect()
}

fn has_non_blocking_finding(findings: &[DiffFinding], capability: &str) -> bool {
    findings.iter().any(|finding| {
        same_capability(&finding.capability, capability)
            && !matches!(finding.class, DiffClass::Bug | DiffClass::Unclassified)
    })
}

fn same_capability(left: &str, right: &str) -> bool {
    left.trim().eq_ignore_ascii_case(right.trim())
}

fn incomplete_matrix_rows(matrix: &CapabilityMatrix) -> Vec<String> {
    matrix
        .rows
        .iter()
        .filter(|row| {
            row.capability.trim().is_empty()
                || row.level.trim().is_empty()
                || row.target_status.trim().is_empty()
                || row.verification.trim().is_empty()
                || row.owner.trim().is_empty()
        })
        .map(|row| row.capability.clone())
        .collect()
}
