use serde::{Deserialize, Serialize};

use crate::redteam::flow::RedteamFinding;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    #[default]
    None,
    Low,
    Medium,
    High,
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskSummary {
    pub total_findings: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub unknown: usize,
    pub max_severity: Severity,
    pub weighted_score: u32,
}

pub fn score_risk(findings: &[RedteamFinding]) -> RiskSummary {
    let mut summary = RiskSummary::default();
    for finding in findings {
        summary.total_findings += 1;
        match parse_severity(&finding.severity) {
            Severity::High => {
                summary.high += 1;
                summary.weighted_score += 100;
                summary.max_severity = Severity::High;
            }
            Severity::Medium => {
                summary.medium += 1;
                summary.weighted_score += 50;
                if !matches!(summary.max_severity, Severity::High) {
                    summary.max_severity = Severity::Medium;
                }
            }
            Severity::Low => {
                summary.low += 1;
                summary.weighted_score += 10;
                if matches!(summary.max_severity, Severity::None | Severity::Unknown) {
                    summary.max_severity = Severity::Low;
                }
            }
            Severity::Unknown => {
                summary.unknown += 1;
                if matches!(summary.max_severity, Severity::None) {
                    summary.max_severity = Severity::Unknown;
                }
            }
            Severity::None => {}
        }
    }
    summary
}

fn parse_severity(value: &str) -> Severity {
    match value.to_ascii_lowercase().as_str() {
        "high" => Severity::High,
        "medium" => Severity::Medium,
        "low" => Severity::Low,
        "" => Severity::None,
        _ => Severity::Unknown,
    }
}
