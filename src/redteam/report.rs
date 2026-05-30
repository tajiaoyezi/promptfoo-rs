use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Serialize;

use crate::redteam::flow::{RedteamError, RedteamFinding, RedteamReport, RedteamStageRecord};
use crate::redteam::risk::{score_risk, RiskSummary};

const REDTEAM_REPORT_SCHEMA_VERSION: &str = "promptfoo-rs.redteam.report.v1";

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RedteamReportArtifact<'a> {
    pub schema_version: &'static str,
    pub risk: RiskSummary,
    pub status: &'a str,
    pub target_id: &'a str,
    pub stages: &'a [RedteamStageRecord],
    pub findings: &'a [RedteamFinding],
    pub errors: &'a [String],
    pub report: &'a RedteamReport,
}

pub fn write_redteam_report(
    report: &RedteamReport,
    mut writer: impl Write,
) -> Result<(), RedteamError> {
    let artifact = RedteamReportArtifact {
        schema_version: REDTEAM_REPORT_SCHEMA_VERSION,
        risk: score_risk(&report.findings),
        status: &report.status,
        target_id: &report.target_id,
        stages: &report.stages,
        findings: &report.findings,
        errors: &report.errors,
        report,
    };
    serde_json::to_writer(&mut writer, &artifact)
        .map_err(|err| RedteamError::new(format!("redteam report serialization failed: {err}")))?;
    writer
        .write_all(b"\n")
        .map_err(|err| RedteamError::new(format!("redteam report write failed: {err}")))?;
    Ok(())
}

pub fn write_redteam_report_file(report: &RedteamReport, path: &Path) -> Result<(), RedteamError> {
    let file = File::create(path)
        .map_err(|err| RedteamError::new(format!("redteam report {}: {err}", path.display())))?;
    write_redteam_report(report, file)
}
