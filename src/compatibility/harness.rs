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

impl CorpusRunSummary {
    pub fn new(fixtures: Vec<CorpusFixtureArtifacts>) -> Self {
        Self {
            schema: "promptfoo-rs.real-upstream-corpus.v1".to_string(),
            fixtures,
        }
    }
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
