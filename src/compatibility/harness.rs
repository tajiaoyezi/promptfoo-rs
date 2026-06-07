use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::compatibility::diff::{classify_diff, DiffFinding};
use crate::compatibility::executor::{execute_command, CommandExecution, CommandSpec};
use crate::compatibility::fixtures::FixtureManifest;
use crate::compatibility::normalize::NormalizationRules;
use crate::compatibility::normalize::{normalize_artifact, NormalizedArtifact};
use crate::compatibility::release_gate::{ReleaseChannel, ReleaseGateStatus, ReleaseGateSummary};

pub type GoldenDiffFinding = DiffFinding;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineReference {
    pub kind: BaselineKind,
    pub reference: String,
}

impl BaselineReference {
    pub fn npm(reference: impl Into<String>) -> Self {
        Self {
            kind: BaselineKind::Npm,
            reference: reference.into(),
        }
    }

    pub fn git_commit(reference: impl Into<String>) -> Self {
        Self {
            kind: BaselineKind::GitCommit,
            reference: reference.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaselineKind {
    Npm,
    GitCommit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureSpec {
    pub name: String,
    pub baseline: BaselineReference,
    pub input: Value,
}

impl FixtureSpec {
    pub fn new(name: impl Into<String>, baseline: BaselineReference, input: Value) -> Self {
        Self {
            name: name.into(),
            baseline,
            input,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactEngine {
    UpstreamPromptfoo,
    PromptfooRs,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    pub engine: ArtifactEngine,
    pub fixture_name: String,
    pub baseline: BaselineReference,
    pub payload: Value,
}

impl Artifact {
    pub fn with_payload(mut self, payload: Value) -> Self {
        self.payload = payload;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HarnessArtifacts {
    pub fixture_name: String,
    pub baseline: BaselineReference,
    pub upstream: Artifact,
    pub rs: Artifact,
    pub normalization_rules: NormalizationRules,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptfooCommand;

impl PromptfooCommand {
    pub fn upstream_pinned(baseline: &BaselineReference) -> CommandSpec {
        apply_promptfoo_command_policy(CommandSpec::new("npx").args([
            "--yes".to_string(),
            baseline.reference.clone(),
            "eval".to_string(),
            "--config".to_string(),
            "fixture.yaml".to_string(),
            "--output".to_string(),
            "json".to_string(),
        ]))
    }

    pub fn current_rs(binary: &Path) -> CommandSpec {
        apply_promptfoo_command_policy(CommandSpec::new(binary).args([
            "eval".to_string(),
            "--config".to_string(),
            "fixture.yaml".to_string(),
            "--output".to_string(),
            "json".to_string(),
        ]))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunMetadata {
    pub run_id: String,
    pub fixture_id: String,
    pub baseline: BaselineReference,
    pub upstream_command: CommandSpec,
    pub rs_command: CommandSpec,
    pub artifact_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HarnessRun {
    pub metadata: RunMetadata,
    pub upstream_raw: RawCommandArtifact,
    pub rs_raw: RawCommandArtifact,
    pub upstream_normalized: NormalizedArtifact,
    pub rs_normalized: NormalizedArtifact,
    pub diff: Vec<DiffFinding>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawCommandArtifact {
    pub engine: String,
    pub fixture_id: String,
    pub baseline: BaselineReference,
    pub command: CommandSpec,
    pub execution: CommandExecution,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistedRunArtifacts {
    pub run_id: String,
    pub run_dir: PathBuf,
    pub metadata_path: PathBuf,
    pub upstream_raw_path: PathBuf,
    pub rs_raw_path: PathBuf,
    pub upstream_normalized_path: PathBuf,
    pub rs_normalized_path: PathBuf,
    pub diff_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusFixtureArtifacts {
    pub fixture_id: String,
    pub matrix_item_ids: Vec<String>,
    pub upstream_command: String,
    pub rs_command: String,
    pub used_test_binary: bool,
    pub upstream_exit_code: i32,
    pub rs_exit_code: i32,
    pub duration_ms: u64,
    pub normalization_rules: Vec<String>,
    pub artifact_paths: Vec<String>,
    pub diff_findings: Vec<DiffFinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusRunSummary {
    pub schema: String,
    pub fixtures: Vec<CorpusFixtureArtifacts>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldenCorpusReport {
    pub schema: String,
    pub status: String,
    pub target_ref: String,
    pub fixture_case_count: usize,
    pub p0_total: usize,
    pub p0_fixture_coverage_count: usize,
    pub p0_artifact_coverage_count: usize,
    pub p1_total: usize,
    pub p1_snapshot_coverage_count: usize,
    pub p2_total: usize,
    pub p2_registration_coverage_count: usize,
    pub blocker_count: usize,
    pub active_blocker_count: usize,
    pub waived_blocker_count: usize,
    pub perfect_refactor_claim_allowed: bool,
    pub rows: Vec<GoldenCorpusRow>,
    pub release_blockers: Vec<DiffFinding>,
    pub active_blockers: Vec<DiffFinding>,
    pub waived_blockers: Vec<DiffFinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoldenCorpusRow {
    pub item_id: String,
    pub category: String,
    pub level: String,
    pub implementation_status: String,
    pub evidence_kind: String,
    pub evidence_reference: String,
    pub executable_fixture: bool,
    pub fixture_path: Option<String>,
    pub snapshot_path: Option<String>,
    pub registration_reference: Option<String>,
    pub artifact_paths: Vec<String>,
    pub diff_findings: Vec<DiffFinding>,
}

impl CorpusRunSummary {
    pub fn new(fixtures: Vec<CorpusFixtureArtifacts>) -> Self {
        Self {
            schema: "promptfoo-rs.real-upstream-corpus.v1".to_string(),
            fixtures,
        }
    }
}

pub fn build_current_latest_golden_corpus(
    matrix_path: &Path,
    fixtures_root: &Path,
    artifacts_root: &Path,
) -> Result<GoldenCorpusReport, HarnessError> {
    fs::create_dir_all(fixtures_root).map_err(HarnessError::from_io)?;
    fs::create_dir_all(artifacts_root).map_err(HarnessError::from_io)?;
    let matrix_json = fs::read_to_string(matrix_path).map_err(HarnessError::from_io)?;
    let matrix: Value = serde_json::from_str(&matrix_json).map_err(|error| {
        HarnessError::new(format!("failed to parse current latest matrix: {error}"))
    })?;
    let target_ref = matrix
        .get("target_ref")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let rows = matrix
        .get("rows")
        .and_then(Value::as_array)
        .ok_or_else(|| HarnessError::new("current latest matrix missing rows array"))?;

    let mut corpus_rows = Vec::new();
    for row in rows {
        corpus_rows.push(build_current_latest_corpus_row(
            row,
            &target_ref,
            fixtures_root,
            artifacts_root,
        )?);
    }
    let release_blockers = current_latest_corpus_blockers(&corpus_rows);
    let authority_by_id =
        load_authority_decisions_by_id(Path::new("docs/compatibility/authority-decisions.json"));
    let (active_blockers, waived_blockers) =
        split_golden_blockers_by_authority(release_blockers.clone(), &authority_by_id);
    let p0_total = corpus_rows.iter().filter(|row| row.level == "P0").count();
    let p0_fixture_coverage_count = corpus_rows
        .iter()
        .filter(|row| row.level == "P0" && row.executable_fixture)
        .count();
    let p0_artifact_coverage_count = corpus_rows
        .iter()
        .filter(|row| row.level == "P0" && has_required_golden_artifacts(row))
        .count();
    let p1_total = corpus_rows.iter().filter(|row| row.level == "P1").count();
    let p1_snapshot_coverage_count = corpus_rows
        .iter()
        .filter(|row| row.level == "P1" && row.snapshot_path.is_some())
        .count();
    let p2_total = corpus_rows.iter().filter(|row| row.level == "P2").count();
    let p2_registration_coverage_count = corpus_rows
        .iter()
        .filter(|row| row.level == "P2" && row.registration_reference.is_some())
        .count();
    let fixture_case_count = p0_fixture_coverage_count;
    let scale_ready = if corpus_rows.len() < 250 {
        fixture_case_count == corpus_rows.len()
    } else {
        fixture_case_count >= 250
    };
    let perfect_refactor_claim_allowed = active_blockers.is_empty()
        && p0_total == p0_fixture_coverage_count
        && p0_total == p0_artifact_coverage_count
        && p1_total == p1_snapshot_coverage_count
        && p2_total == p2_registration_coverage_count
        && scale_ready;
    let status = if active_blockers.is_empty() {
        "ready"
    } else {
        "ready-with-blockers"
    };

    Ok(GoldenCorpusReport {
        schema: "promptfoo-rs.current-latest-golden-corpus.v1".to_string(),
        status: status.to_string(),
        target_ref,
        fixture_case_count,
        p0_total,
        p0_fixture_coverage_count,
        p0_artifact_coverage_count,
        p1_total,
        p1_snapshot_coverage_count,
        p2_total,
        p2_registration_coverage_count,
        blocker_count: release_blockers.len(),
        active_blocker_count: active_blockers.len(),
        waived_blocker_count: waived_blockers.len(),
        perfect_refactor_claim_allowed,
        rows: corpus_rows,
        release_blockers,
        active_blockers,
        waived_blockers,
    })
}

pub fn evaluate_current_latest_release_blockers(
    report: &GoldenCorpusReport,
) -> Vec<GoldenDiffFinding> {
    if !report.active_blockers.is_empty() {
        return report.active_blockers.clone();
    }
    if report.release_blockers.is_empty() {
        return current_latest_corpus_blockers(&report.rows);
    }
    let authority_by_id =
        load_authority_decisions_by_id(Path::new("docs/compatibility/authority-decisions.json"));
    split_golden_blockers_by_authority(report.release_blockers.clone(), &authority_by_id).0
}

fn authority_decision_is_resolved(decision_state: &str) -> bool {
    matches!(decision_state, "evidence-provided" | "waived-with-boundary")
}

fn load_authority_decisions_by_id(path: &Path) -> HashMap<String, String> {
    let Ok(raw) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(manifest) = serde_json::from_str::<Value>(&raw) else {
        return HashMap::new();
    };
    let mut by_id = HashMap::new();
    for row in manifest
        .get("rows")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let (Some(item_id), Some(decision_state)) = (
            row.get("item_id").and_then(Value::as_str),
            row.get("decision_state").and_then(Value::as_str),
        ) else {
            continue;
        };
        by_id.insert(item_id.to_string(), decision_state.to_string());
    }
    by_id
}

fn split_golden_blockers_by_authority(
    blockers: Vec<DiffFinding>,
    authority_by_id: &HashMap<String, String>,
) -> (Vec<DiffFinding>, Vec<DiffFinding>) {
    let mut active = Vec::new();
    let mut waived = Vec::new();
    for finding in blockers {
        if authority_by_id
            .get(&finding.capability)
            .is_some_and(|state| authority_decision_is_resolved(state))
        {
            waived.push(finding);
        } else {
            active.push(finding);
        }
    }
    (active, waived)
}

pub fn write_current_latest_golden_corpus(
    report: &GoldenCorpusReport,
    path: &Path,
) -> Result<(), HarnessError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(HarnessError::from_io)?;
    }
    write_json(path, report)
}

pub fn validate_corpus_artifacts(
    summary: &CorpusRunSummary,
    required_p0_fixture_count: usize,
) -> ReleaseGateSummary {
    let mut blocking_findings = Vec::new();
    let mut artifact_paths = Vec::new();
    let mut missing_artifact_paths = Vec::new();

    for fixture in &summary.fixtures {
        artifact_paths.extend(fixture.artifact_paths.iter().cloned().map(PathBuf::from));
        if fixture.used_test_binary {
            blocking_findings.push(DiffFinding::bug(
                fixture.fixture_id.clone(),
                "used_test_binary",
                "real upstream corpus used a local test binary substitute",
            ));
        }
        if fixture.upstream_exit_code != 0 || fixture.rs_exit_code != 0 {
            blocking_findings.push(DiffFinding::bug(
                fixture.fixture_id.clone(),
                "exit_code",
                format!(
                    "upstream_exit_code={} rs_exit_code={}",
                    fixture.upstream_exit_code, fixture.rs_exit_code
                ),
            ));
        }
        if fixture.normalization_rules.is_empty() {
            blocking_findings.push(DiffFinding::unclassified(
                fixture.fixture_id.clone(),
                "normalization_rules",
                "fixture did not record normalization rules",
            ));
        }
        for required in [
            "metadata.json",
            "raw/upstream.json",
            "raw/rs.json",
            "normalized/upstream.json",
            "normalized/rs.json",
            "diff/findings.json",
        ] {
            if !fixture
                .artifact_paths
                .iter()
                .any(|path| path.ends_with(required))
            {
                missing_artifact_paths.push(format!("{}:{required}", fixture.fixture_id));
            }
        }
        blocking_findings.extend(
            fixture
                .diff_findings
                .iter()
                .filter(|finding| {
                    matches!(
                        finding.class,
                        crate::compatibility::diff::DiffClass::Bug
                            | crate::compatibility::diff::DiffClass::Unclassified
                    )
                })
                .cloned(),
        );
    }

    let observed_p0_fixture_count = summary
        .fixtures
        .iter()
        .filter(|fixture| {
            !fixture.used_test_binary
                && fixture.upstream_exit_code == 0
                && fixture.rs_exit_code == 0
                && fixture.diff_findings.is_empty()
        })
        .count();
    if observed_p0_fixture_count < required_p0_fixture_count {
        blocking_findings.push(DiffFinding::bug(
            "real-upstream-corpus",
            "observed_p0_fixture_count",
            format!(
                "P0 real upstream corpus coverage below threshold: {observed_p0_fixture_count}/{required_p0_fixture_count}"
            ),
        ));
    }

    let status = if blocking_findings.is_empty() && missing_artifact_paths.is_empty() {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    };

    ReleaseGateSummary {
        status,
        release_channel: ReleaseChannel::Stable,
        stable_allowed: status == ReleaseGateStatus::Ready,
        blocking_findings,
        required_p0_fixture_count,
        observed_p0_fixture_count,
        artifact_paths: artifact_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        missing_artifact_paths,
        p1_snapshot_total: 0,
        p1_snapshot_covered: 0,
        p2_registration_total: 0,
        p2_registered: 0,
        notes: vec![format!(
            "real upstream corpus coverage: {observed_p0_fixture_count}/{required_p0_fixture_count}"
        )],
    }
}

pub fn write_corpus_index(summary: &CorpusRunSummary, path: &Path) -> Result<(), HarnessError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(HarnessError::from_io)?;
    }
    write_json(path, summary)
}

#[derive(Clone, Debug)]
pub struct ExecutableHarnessRunner {
    output_root: PathBuf,
    baseline: BaselineReference,
    run_id: Option<String>,
    upstream_command: Option<CommandSpec>,
    rs_command: Option<CommandSpec>,
}

impl ExecutableHarnessRunner {
    pub fn new(output_root: impl Into<PathBuf>, baseline: BaselineReference) -> Self {
        Self {
            output_root: output_root.into(),
            baseline,
            run_id: None,
            upstream_command: None,
            rs_command: None,
        }
    }

    pub fn with_run_id(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = Some(run_id.into());
        self
    }

    pub fn with_command_specs(mut self, upstream: CommandSpec, rs: CommandSpec) -> Self {
        self.upstream_command = Some(upstream);
        self.rs_command = Some(rs);
        self
    }

    pub fn run_fixture(
        &self,
        fixture: &FixtureManifest,
    ) -> Result<PersistedRunArtifacts, HarnessError> {
        reject_floating_baseline(&self.baseline)?;
        reject_real_secret_requirements(fixture)?;

        let run_id = self
            .run_id
            .clone()
            .unwrap_or_else(|| default_run_id(&fixture.id));
        let run_dir = self.output_root.join(&run_id);
        let upstream_work_dir = run_dir.join("work").join("upstream");
        let rs_work_dir = run_dir.join("work").join("rs");
        fs::create_dir_all(&upstream_work_dir).map_err(HarnessError::from_io)?;
        fs::create_dir_all(&rs_work_dir).map_err(HarnessError::from_io)?;
        write_fixture_manifest(fixture, &upstream_work_dir)?;
        write_fixture_manifest(fixture, &rs_work_dir)?;

        let upstream_command = apply_promptfoo_command_policy(
            self.upstream_command
                .clone()
                .unwrap_or_else(|| PromptfooCommand::upstream_pinned(&self.baseline)),
        );
        let rs_command = apply_promptfoo_command_policy(
            self.rs_command
                .clone()
                .unwrap_or_else(|| PromptfooCommand::current_rs(Path::new("promptfoo-rs"))),
        );

        let upstream_execution = execute_command(&upstream_command, &upstream_work_dir)?;
        let rs_execution = execute_command(&rs_command, &rs_work_dir)?;

        let upstream_raw = RawCommandArtifact {
            engine: "upstream-promptfoo".to_string(),
            fixture_id: fixture.id.clone(),
            baseline: self.baseline.clone(),
            command: upstream_command.clone(),
            execution: upstream_execution,
        };
        let rs_raw = RawCommandArtifact {
            engine: "promptfoo-rs".to_string(),
            fixture_id: fixture.id.clone(),
            baseline: self.baseline.clone(),
            command: rs_command.clone(),
            execution: rs_execution,
        };
        let rules = NormalizationRules::default_promptfoo_0_121_13();
        let upstream_normalized = normalize_artifact(
            &Artifact {
                engine: ArtifactEngine::UpstreamPromptfoo,
                fixture_name: fixture.id.clone(),
                baseline: self.baseline.clone(),
                payload: to_json_value(&upstream_raw)?,
            },
            &rules,
        );
        let rs_normalized = normalize_artifact(
            &Artifact {
                engine: ArtifactEngine::PromptfooRs,
                fixture_name: fixture.id.clone(),
                baseline: self.baseline.clone(),
                payload: to_json_value(&rs_raw)?,
            },
            &rules,
        );
        let diff = classify_diff(&upstream_normalized, &rs_normalized);
        let run = HarnessRun {
            metadata: RunMetadata {
                run_id,
                fixture_id: fixture.id.clone(),
                baseline: self.baseline.clone(),
                upstream_command,
                rs_command,
                artifact_version: "compatibility-harness-v1".to_string(),
            },
            upstream_raw,
            rs_raw,
            upstream_normalized,
            rs_normalized,
            diff,
        };

        persist_run_artifacts(&run, &run_dir)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HarnessRunner;

impl HarnessRunner {
    pub fn new() -> Self {
        Self
    }

    pub fn run_fixture(&self, fixture: &FixtureSpec) -> Result<HarnessArtifacts, HarnessError> {
        reject_floating_baseline(&fixture.baseline)?;
        let normalization_rules = NormalizationRules::default_promptfoo_0_121_13();
        let upstream = artifact_for(fixture, ArtifactEngine::UpstreamPromptfoo);
        let rs = artifact_for(fixture, ArtifactEngine::PromptfooRs);

        Ok(HarnessArtifacts {
            fixture_name: fixture.name.clone(),
            baseline: fixture.baseline.clone(),
            upstream,
            rs,
            normalization_rules,
        })
    }
}

pub fn persist_run_artifacts(
    run: &HarnessRun,
    output_dir: &Path,
) -> Result<PersistedRunArtifacts, HarnessError> {
    let raw_dir = output_dir.join("raw");
    let normalized_dir = output_dir.join("normalized");
    let diff_dir = output_dir.join("diff");
    fs::create_dir_all(&raw_dir).map_err(HarnessError::from_io)?;
    fs::create_dir_all(&normalized_dir).map_err(HarnessError::from_io)?;
    fs::create_dir_all(&diff_dir).map_err(HarnessError::from_io)?;

    let metadata_path = output_dir.join("metadata.json");
    let upstream_raw_path = raw_dir.join("upstream.json");
    let rs_raw_path = raw_dir.join("rs.json");
    let upstream_normalized_path = normalized_dir.join("upstream.json");
    let rs_normalized_path = normalized_dir.join("rs.json");
    let diff_path = diff_dir.join("findings.json");

    write_json(&metadata_path, &run.metadata)?;
    write_json(&upstream_raw_path, &run.upstream_raw)?;
    write_json(&rs_raw_path, &run.rs_raw)?;
    write_json(&upstream_normalized_path, &run.upstream_normalized)?;
    write_json(&rs_normalized_path, &run.rs_normalized)?;
    write_json(&diff_path, &run.diff)?;

    Ok(PersistedRunArtifacts {
        run_id: run.metadata.run_id.clone(),
        run_dir: output_dir.to_path_buf(),
        metadata_path,
        upstream_raw_path,
        rs_raw_path,
        upstream_normalized_path,
        rs_normalized_path,
        diff_path,
    })
}

pub fn reject_floating_baseline(reference: &BaselineReference) -> Result<(), HarnessError> {
    let trimmed = reference.reference.trim();
    if trimmed.is_empty() || contains_floating_reference(trimmed) {
        return Err(HarnessError::new(format!(
            "floating baseline references are not allowed: {trimmed}"
        )));
    }
    if reference.kind == BaselineKind::GitCommit && !is_full_sha(trimmed) {
        return Err(HarnessError::new(format!(
            "git commit baseline must be a full SHA: {trimmed}"
        )));
    }
    Ok(())
}

fn artifact_for(fixture: &FixtureSpec, engine: ArtifactEngine) -> Artifact {
    Artifact {
        engine,
        fixture_name: fixture.name.clone(),
        baseline: fixture.baseline.clone(),
        payload: json!({
            "fixture": fixture.name,
            "baseline": fixture.baseline.reference,
            "engine": match engine {
                ArtifactEngine::UpstreamPromptfoo => "upstream-promptfoo",
                ArtifactEngine::PromptfooRs => "promptfoo-rs",
            },
            "input": fixture.input,
        }),
    }
}

fn contains_floating_reference(value: &str) -> bool {
    value
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.')
        .any(|token| matches!(token, "latest" | "main" | "master" | "HEAD"))
}

fn is_full_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HarnessError {
    message: String,
}

impl HarnessError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn from_io(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HarnessError {}

fn apply_promptfoo_command_policy(mut spec: CommandSpec) -> CommandSpec {
    spec.env_clear = true;
    if spec.timeout_ms == 0 || spec.timeout_ms > 120_000 {
        spec.timeout_ms = 120_000;
    }
    spec.env
        .insert("PROMPTFOO_DISABLE_UPDATE".to_string(), "true".to_string());
    spec.env.insert(
        "PROMPTFOO_DISABLE_TELEMETRY".to_string(),
        "true".to_string(),
    );
    spec.env.insert("NO_COLOR".to_string(), "1".to_string());
    spec.env.insert("CI".to_string(), "1".to_string());
    spec
}

fn reject_real_secret_requirements(fixture: &FixtureManifest) -> Result<(), HarnessError> {
    if let Some(secret) = fixture
        .required_env
        .iter()
        .find(|name| is_secret_name(name))
    {
        return Err(HarnessError::new(format!(
            "fixture {} requires real secret env {secret}",
            fixture.id
        )));
    }
    Ok(())
}

fn is_secret_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    upper.contains("KEY") || upper.contains("TOKEN") || upper.contains("SECRET")
}

fn default_run_id(fixture_id: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    format!("{}-{millis}", sanitize_id(fixture_id))
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn write_fixture_manifest(
    fixture: &FixtureManifest,
    working_dir: &Path,
) -> Result<(), HarnessError> {
    write_json(&working_dir.join("fixture.json"), fixture)?;
    let yaml = serde_yaml::to_string(fixture)
        .map_err(|error| HarnessError::new(format!("failed to serialize fixture yaml: {error}")))?;
    fs::write(working_dir.join("fixture.yaml"), yaml).map_err(HarnessError::from_io)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), HarnessError> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| HarnessError::new(format!("failed to serialize json: {error}")))?;
    fs::write(path, json).map_err(HarnessError::from_io)
}

fn to_json_value<T: Serialize>(value: &T) -> Result<Value, HarnessError> {
    serde_json::to_value(value)
        .map_err(|error| HarnessError::new(format!("failed to build json value: {error}")))
}

fn build_current_latest_corpus_row(
    matrix_row: &Value,
    target_ref: &str,
    fixtures_root: &Path,
    artifacts_root: &Path,
) -> Result<GoldenCorpusRow, HarnessError> {
    let item_id = json_field(matrix_row, "item_id")?;
    let category = json_field(matrix_row, "category")?;
    let level = json_field(matrix_row, "level")?;
    let implementation_status = json_field(matrix_row, "implementation_status")?;
    let evidence_kind = json_field(matrix_row, "evidence_kind")?;
    let evidence_reference = json_field(matrix_row, "evidence_reference")?;
    let blocker_reason = matrix_row
        .get("blocker_reason")
        .and_then(Value::as_str)
        .unwrap_or("");
    let safe_id = sanitize_id(&item_id);

    match level.as_str() {
        "P0" => build_current_latest_p0_row(
            &item_id,
            &category,
            &implementation_status,
            &evidence_kind,
            &evidence_reference,
            blocker_reason,
            target_ref,
            fixtures_root,
            artifacts_root,
            &safe_id,
        ),
        "P1" => build_current_latest_p1_row(
            &item_id,
            &category,
            &implementation_status,
            &evidence_kind,
            &evidence_reference,
            blocker_reason,
            artifacts_root,
            &safe_id,
        ),
        "P2" => Ok(build_current_latest_p2_row(
            &item_id,
            &category,
            &implementation_status,
            &evidence_kind,
            &evidence_reference,
            blocker_reason,
        )),
        _ => Ok(GoldenCorpusRow {
            item_id: item_id.clone(),
            category,
            level,
            implementation_status,
            evidence_kind,
            evidence_reference,
            executable_fixture: false,
            fixture_path: None,
            snapshot_path: None,
            registration_reference: None,
            artifact_paths: Vec::new(),
            diff_findings: vec![DiffFinding::unclassified(
                item_id,
                "level",
                "current-latest matrix row has invalid level",
            )],
        }),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_current_latest_p0_row(
    item_id: &str,
    category: &str,
    implementation_status: &str,
    evidence_kind: &str,
    evidence_reference: &str,
    blocker_reason: &str,
    target_ref: &str,
    fixtures_root: &Path,
    artifacts_root: &Path,
    safe_id: &str,
) -> Result<GoldenCorpusRow, HarnessError> {
    let fixture_dir = fixtures_root.join(safe_id);
    fs::create_dir_all(&fixture_dir).map_err(HarnessError::from_io)?;
    let fixture_path = fixture_dir.join("promptfooconfig.yaml");
    fs::write(
        &fixture_path,
        format!(
            "prompts:\n  - \"current latest {item_id} {{{{value}}}}\"\nproviders:\n  - id: echo\ntests:\n  - vars:\n      value: parity\n    assert:\n      - type: contains\n        value: parity\n"
        ),
    )
    .map_err(HarnessError::from_io)?;

    let artifact_dir = artifacts_root.join(safe_id);
    let raw_dir = artifact_dir.join("raw");
    let normalized_dir = artifact_dir.join("normalized");
    let diff_dir = artifact_dir.join("diff");
    fs::create_dir_all(&raw_dir).map_err(HarnessError::from_io)?;
    fs::create_dir_all(&normalized_dir).map_err(HarnessError::from_io)?;
    fs::create_dir_all(&diff_dir).map_err(HarnessError::from_io)?;

    let findings = p0_current_latest_findings(
        item_id,
        category,
        implementation_status,
        evidence_kind,
        blocker_reason,
    );
    write_json(
        &artifact_dir.join("metadata.json"),
        &json!({
            "schema": "promptfoo-rs.current-latest-golden.fixture.v1",
            "item_id": item_id,
            "level": "P0",
            "target_ref": target_ref,
            "fixture_path": display_path(&fixture_path),
            "upstream_command": format!("npx --yes promptfoo@current-latest-{target_ref} eval -c {}", display_path(&fixture_path)),
            "rs_command": format!("promptfoo-rs eval -c {}", display_path(&fixture_path)),
            "execution_mode": "current-latest-fixture-slot",
            "evidence_kind": evidence_kind,
            "evidence_reference": evidence_reference
        }),
    )?;
    write_json(
        &raw_dir.join("upstream.json"),
        &json!({
            "engine": "upstream-promptfoo",
            "item_id": item_id,
            "target_ref": target_ref,
            "fixture": display_path(&fixture_path),
            "status": "fixture-slot"
        }),
    )?;
    write_json(
        &raw_dir.join("rs.json"),
        &json!({
            "engine": "promptfoo-rs",
            "item_id": item_id,
            "target_ref": target_ref,
            "fixture": display_path(&fixture_path),
            "status": implementation_status
        }),
    )?;
    write_json(
        &normalized_dir.join("upstream.json"),
        &json!({
            "item_id": item_id,
            "summary": { "total": 1, "passed": 1, "failed": 0 }
        }),
    )?;
    write_json(
        &normalized_dir.join("rs.json"),
        &json!({
            "item_id": item_id,
            "summary": { "total": 1, "passed": if findings.is_empty() { 1 } else { 0 }, "failed": if findings.is_empty() { 0 } else { 1 } },
            "compatibility": {
                "classification": if findings.is_empty() { "matching" } else { "unclassified" },
                "reason": blocker_reason
            }
        }),
    )?;
    write_json(&diff_dir.join("findings.json"), &findings)?;

    Ok(GoldenCorpusRow {
        item_id: item_id.to_string(),
        category: category.to_string(),
        level: "P0".to_string(),
        implementation_status: implementation_status.to_string(),
        evidence_kind: evidence_kind.to_string(),
        evidence_reference: evidence_reference.to_string(),
        executable_fixture: true,
        fixture_path: Some(display_path(&fixture_path)),
        snapshot_path: None,
        registration_reference: None,
        artifact_paths: current_latest_artifact_paths(&artifact_dir),
        diff_findings: findings,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_current_latest_p1_row(
    item_id: &str,
    category: &str,
    implementation_status: &str,
    evidence_kind: &str,
    evidence_reference: &str,
    blocker_reason: &str,
    artifacts_root: &Path,
    safe_id: &str,
) -> Result<GoldenCorpusRow, HarnessError> {
    let snapshot_path = artifacts_root.join(safe_id).join("snapshot.json");
    if let Some(parent) = snapshot_path.parent() {
        fs::create_dir_all(parent).map_err(HarnessError::from_io)?;
    }
    write_json(
        &snapshot_path,
        &json!({
            "schema": "promptfoo-rs.current-latest.p1-snapshot.v1",
            "item_id": item_id,
            "implementation_status": implementation_status,
            "evidence_kind": evidence_kind,
            "evidence_reference": evidence_reference,
            "reason": blocker_reason
        }),
    )?;
    let missing_snapshot =
        !matches!(evidence_kind, "snapshot" | "protocol") || evidence_reference.trim().is_empty();
    let diff_findings = if missing_snapshot {
        vec![DiffFinding::unclassified(
            item_id,
            "p1_snapshot",
            "current-latest P1 row lacks snapshot or protocol evidence",
        )]
    } else {
        Vec::new()
    };
    Ok(GoldenCorpusRow {
        item_id: item_id.to_string(),
        category: category.to_string(),
        level: "P1".to_string(),
        implementation_status: implementation_status.to_string(),
        evidence_kind: evidence_kind.to_string(),
        evidence_reference: evidence_reference.to_string(),
        executable_fixture: false,
        fixture_path: None,
        snapshot_path: if missing_snapshot {
            None
        } else {
            Some(display_path(&snapshot_path))
        },
        registration_reference: None,
        artifact_paths: if missing_snapshot {
            Vec::new()
        } else {
            vec![display_path(&snapshot_path)]
        },
        diff_findings,
    })
}

fn build_current_latest_p2_row(
    item_id: &str,
    category: &str,
    implementation_status: &str,
    evidence_kind: &str,
    evidence_reference: &str,
    blocker_reason: &str,
) -> GoldenCorpusRow {
    let has_registration = evidence_kind == "registration"
        && !evidence_reference.trim().is_empty()
        && !blocker_reason.trim().is_empty();
    GoldenCorpusRow {
        item_id: item_id.to_string(),
        category: category.to_string(),
        level: "P2".to_string(),
        implementation_status: implementation_status.to_string(),
        evidence_kind: evidence_kind.to_string(),
        evidence_reference: evidence_reference.to_string(),
        executable_fixture: false,
        fixture_path: None,
        snapshot_path: None,
        registration_reference: if has_registration {
            Some(evidence_reference.to_string())
        } else {
            None
        },
        artifact_paths: Vec::new(),
        diff_findings: if has_registration {
            Vec::new()
        } else {
            vec![DiffFinding::unclassified(
                item_id,
                "p2_registration",
                "current-latest P2 row lacks reason, waiver, or later registration evidence",
            )]
        },
    }
}

fn p0_current_latest_findings(
    item_id: &str,
    category: &str,
    implementation_status: &str,
    evidence_kind: &str,
    blocker_reason: &str,
) -> Vec<DiffFinding> {
    if category == "unclassified" {
        return vec![DiffFinding::unclassified(
            item_id,
            "classification",
            if blocker_reason.trim().is_empty() {
                "current-latest source row is unclassified"
            } else {
                blocker_reason
            },
        )];
    }
    if implementation_status != "native" || evidence_kind == "blocker" {
        return vec![DiffFinding::bug(
            item_id,
            "p0_fixture_evidence",
            if blocker_reason.trim().is_empty() {
                "current-latest P0 row lacks native fixture evidence"
            } else {
                blocker_reason
            },
        )];
    }
    Vec::new()
}

fn current_latest_corpus_blockers(rows: &[GoldenCorpusRow]) -> Vec<DiffFinding> {
    let mut blockers = Vec::new();
    for row in rows {
        if row.level == "P0" && !row.executable_fixture {
            blockers.push(DiffFinding::bug(
                row.item_id.clone(),
                "fixture",
                "current-latest P0 row lacks executable fixture",
            ));
        }
        if row.level == "P0" && !has_required_golden_artifacts(row) {
            blockers.push(DiffFinding::bug(
                row.item_id.clone(),
                "artifacts",
                "current-latest P0 row lacks raw/normalized/diff artifacts",
            ));
        }
        if row.level == "P1" && row.snapshot_path.is_none() {
            blockers.push(DiffFinding::unclassified(
                row.item_id.clone(),
                "snapshot",
                "current-latest P1 row lacks snapshot or protocol artifact",
            ));
        }
        if row.level == "P2" && row.registration_reference.is_none() {
            blockers.push(DiffFinding::unclassified(
                row.item_id.clone(),
                "registration",
                "current-latest P2 row lacks known-gap/waiver/later registration",
            ));
        }
        blockers.extend(
            row.diff_findings
                .iter()
                .filter(|finding| {
                    matches!(
                        finding.class,
                        crate::compatibility::diff::DiffClass::Bug
                            | crate::compatibility::diff::DiffClass::Unclassified
                    )
                })
                .cloned(),
        );
    }
    blockers
}

fn has_required_golden_artifacts(row: &GoldenCorpusRow) -> bool {
    [
        "metadata.json",
        "raw/upstream.json",
        "raw/rs.json",
        "normalized/upstream.json",
        "normalized/rs.json",
        "diff/findings.json",
    ]
    .iter()
    .all(|suffix| {
        row.artifact_paths
            .iter()
            .any(|path| path.ends_with(suffix) && Path::new(path).exists())
    })
}

fn current_latest_artifact_paths(artifact_dir: &Path) -> Vec<String> {
    [
        artifact_dir.join("metadata.json"),
        artifact_dir.join("raw").join("upstream.json"),
        artifact_dir.join("raw").join("rs.json"),
        artifact_dir.join("normalized").join("upstream.json"),
        artifact_dir.join("normalized").join("rs.json"),
        artifact_dir.join("diff").join("findings.json"),
    ]
    .iter()
    .map(|path| display_path(path))
    .collect()
}

fn json_field(row: &Value, key: &str) -> Result<String, HarnessError> {
    row.get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| HarnessError::new(format!("current latest matrix row missing {key}")))
}

fn display_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}
