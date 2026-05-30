use serde::{Deserialize, Serialize};

use crate::compatibility::diff::{DiffClass, DiffFinding};
use crate::compatibility::matrix::{CapabilityMatrix, CapabilityRow};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseGateStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseGateSummary {
    pub status: ReleaseGateStatus,
    pub blocking_findings: Vec<DiffFinding>,
    pub p1_snapshot_total: usize,
    pub p1_snapshot_covered: usize,
    pub p2_registration_total: usize,
    pub p2_registered: usize,
    pub notes: Vec<String>,
}

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
        blocking_findings,
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
