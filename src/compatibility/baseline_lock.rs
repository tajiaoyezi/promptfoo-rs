use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaselineLock {
    pub git_tag: GitTagArtifact,
    pub git_commit: GitCommitArtifact,
    pub npm_artifact: NpmArtifact,
    pub container_artifact: ContainerArtifact,
}

impl BaselineLock {
    pub fn from_markdown(path: &Path) -> Result<Self, BaselineLockError> {
        let markdown = fs::read_to_string(path).map_err(BaselineLockError::Read)?;
        parse_baseline_lock(&markdown)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitTagArtifact {
    pub reference: String,
    pub evidence: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitCommitArtifact {
    pub sha: String,
    pub evidence: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpmArtifact {
    pub package: String,
    pub integrity: String,
    pub shasum: String,
    pub evidence: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerArtifact {
    pub reference: String,
    pub digest: String,
    pub evidence: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaselineLockReport {
    pub missing_artifacts: Vec<String>,
    pub floating_references: Vec<String>,
}

impl BaselineLockReport {
    pub fn is_complete(&self) -> bool {
        self.missing_artifacts.is_empty() && self.floating_references.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseGateStatus {
    Ready,
    Blocked,
}

#[derive(Debug)]
pub enum BaselineLockError {
    Read(std::io::Error),
    MissingRow(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityTargetPolicy {
    pub stable_targets: Vec<StableTarget>,
    pub moving_upstream_observations: Vec<UpstreamObservation>,
}

impl CompatibilityTargetPolicy {
    pub fn load(_path: &Path) -> Result<CompatibilityTargetPolicy, TargetPolicyError> {
        let markdown = fs::read_to_string(_path).map_err(TargetPolicyError::Read)?;
        let json = extract_json_fence(&markdown).ok_or_else(|| {
            TargetPolicyError::Parse("target policy must contain a fenced json block".to_string())
        })?;
        serde_json::from_str(json).map_err(|error| TargetPolicyError::Parse(error.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableTarget {
    pub id: String,
    pub kind: StableTargetKind,
    pub package_version: String,
    pub git_ref: String,
    pub git_commit: String,
    pub npm_integrity: String,
    pub container_digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StableTargetKind {
    FrozenBaseline,
    Rebaselined,
    Floating,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamObservation {
    pub head: String,
    pub package_version: String,
    pub collected_at: String,
    pub source: String,
    pub modifies_frozen_baseline: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetPolicyReport {
    pub stable_target_count: usize,
    pub rejected_reasons: Vec<String>,
    pub moving_upstream_is_tracking_only: bool,
}

impl TargetPolicyReport {
    pub fn is_release_ready(&self) -> bool {
        self.rejected_reasons.is_empty()
            && self.stable_target_count == 1
            && self.moving_upstream_is_tracking_only
    }
}

#[derive(Debug)]
pub enum TargetPolicyError {
    Read(std::io::Error),
    Parse(String),
}

pub fn validate_single_stable_target(_policy: &CompatibilityTargetPolicy) -> TargetPolicyReport {
    let mut rejected_reasons = Vec::new();
    let stable_target_count = _policy.stable_targets.len();

    if stable_target_count != 1 {
        rejected_reasons.push(format!(
            "multiple stable targets are not allowed: found {stable_target_count}"
        ));
    }

    for target in &_policy.stable_targets {
        if target.kind == StableTargetKind::Floating
            || contains_floating_reference(&target.id)
            || contains_floating_reference(&target.package_version)
            || contains_floating_reference(&target.git_ref)
            || contains_floating_reference(&target.git_commit)
            || contains_floating_reference(&target.container_digest)
        {
            rejected_reasons.push(format!("floating stable target: {}", target.id));
        }
        if target.git_commit.len() != 40
            || !target.git_commit.chars().all(|c| c.is_ascii_hexdigit())
        {
            rejected_reasons.push(format!(
                "stable target commit is not immutable: {}",
                target.id
            ));
        }
        if target.npm_integrity.trim().is_empty() || target.container_digest.trim().is_empty() {
            rejected_reasons.push(format!(
                "stable target lacks artifact evidence: {}",
                target.id
            ));
        }
    }

    let moving_upstream_is_tracking_only = !_policy.moving_upstream_observations.is_empty()
        && _policy
            .moving_upstream_observations
            .iter()
            .all(|observation| !observation.modifies_frozen_baseline);

    if !moving_upstream_is_tracking_only {
        rejected_reasons.push("moving upstream observations must be tracking-only".to_string());
    }

    TargetPolicyReport {
        stable_target_count,
        rejected_reasons,
        moving_upstream_is_tracking_only,
    }
}

pub fn record_moving_upstream_observation(
    head: &str,
    package_version: &str,
) -> UpstreamObservation {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    UpstreamObservation {
        head: head.to_string(),
        package_version: package_version.to_string(),
        collected_at: format!("unix:{seconds}"),
        source: "upstream origin/main tracking".to_string(),
        modifies_frozen_baseline: false,
    }
}

pub fn validate_baseline_lock(lock: &BaselineLock) -> BaselineLockReport {
    let mut missing_artifacts = Vec::new();
    let mut floating_references = Vec::new();

    require(
        &mut missing_artifacts,
        "git tag",
        !lock.git_tag.reference.trim().is_empty()
            && !lock.git_tag.evidence.trim().is_empty()
            && lock.git_tag.status == "Verified",
    );
    require(
        &mut missing_artifacts,
        "git commit",
        is_full_sha(&lock.git_commit.sha)
            && !lock.git_commit.evidence.trim().is_empty()
            && lock.git_commit.status == "Verified",
    );
    require(
        &mut missing_artifacts,
        "npm artifact",
        !lock.npm_artifact.package.trim().is_empty()
            && !lock.npm_artifact.integrity.trim().is_empty()
            && !lock.npm_artifact.shasum.trim().is_empty()
            && lock.npm_artifact.status == "Verified",
    );
    require(
        &mut missing_artifacts,
        "container artifact",
        !lock.container_artifact.reference.trim().is_empty()
            && !lock.container_artifact.digest.trim().is_empty()
            && lock.container_artifact.status == "Verified",
    );

    for (label, value) in [
        ("git tag", lock.git_tag.reference.as_str()),
        ("git tag evidence", lock.git_tag.evidence.as_str()),
        ("git commit", lock.git_commit.sha.as_str()),
        ("npm artifact", lock.npm_artifact.package.as_str()),
        ("npm evidence", lock.npm_artifact.evidence.as_str()),
        (
            "container artifact",
            lock.container_artifact.reference.as_str(),
        ),
        (
            "container evidence",
            lock.container_artifact.evidence.as_str(),
        ),
    ] {
        if contains_floating_reference(value) {
            floating_references.push(label.to_string());
        }
    }

    BaselineLockReport {
        missing_artifacts,
        floating_references,
    }
}

pub fn baseline_lock_release_status(report: &BaselineLockReport) -> ReleaseGateStatus {
    if report.is_complete() {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    }
}

fn parse_baseline_lock(markdown: &str) -> Result<BaselineLock, BaselineLockError> {
    let git_tag = find_row(markdown, "Git tag")
        .map(|row| GitTagArtifact {
            reference: row.expected,
            evidence: row.evidence,
            status: row.status,
        })
        .ok_or(BaselineLockError::MissingRow("Git tag"))?;
    let git_commit = find_row(markdown, "Git commit")
        .map(|row| GitCommitArtifact {
            sha: row.expected,
            evidence: row.evidence,
            status: row.status,
        })
        .ok_or(BaselineLockError::MissingRow("Git commit"))?;
    let npm_artifact = find_row(markdown, "npm artifact")
        .map(|row| NpmArtifact {
            package: row.expected,
            integrity: extract_after(&row.evidence, "integrity `"),
            shasum: extract_after(&row.evidence, "shasum `"),
            evidence: row.evidence,
            status: row.status,
        })
        .ok_or(BaselineLockError::MissingRow("npm artifact"))?;
    let container_artifact = find_row(markdown, "container artifact")
        .map(|row| ContainerArtifact {
            reference: row.expected,
            digest: extract_sha256(&row.evidence),
            evidence: row.evidence,
            status: row.status,
        })
        .ok_or(BaselineLockError::MissingRow("container artifact"))?;

    Ok(BaselineLock {
        git_tag,
        git_commit,
        npm_artifact,
        container_artifact,
    })
}

struct ArtifactRow {
    expected: String,
    evidence: String,
    status: String,
}

fn find_row(markdown: &str, artifact: &str) -> Option<ArtifactRow> {
    markdown.lines().find_map(|line| {
        let cells = markdown_row_cells(line)?;
        if cells.first().map(String::as_str) != Some(artifact) {
            return None;
        }
        Some(ArtifactRow {
            expected: cells.get(1)?.to_string(),
            evidence: cells.get(2)?.to_string(),
            status: cells.get(3)?.to_string(),
        })
    })
}

fn markdown_row_cells(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let cells: Vec<String> = trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect();
    if cells.len() == 4 {
        Some(cells)
    } else {
        None
    }
}

fn extract_after(text: &str, marker: &str) -> String {
    text.split_once(marker)
        .and_then(|(_, rest)| rest.split_once('`').map(|(value, _)| value.to_string()))
        .unwrap_or_default()
}

fn extract_sha256(text: &str) -> String {
    text.split_whitespace()
        .map(|part| part.trim_matches(|c: char| c == '`' || c == ';' || c == ',' || c == ':'))
        .find(|part| part.starts_with("sha256:") && part.len() == 71)
        .unwrap_or_default()
        .to_string()
}

fn extract_json_fence(markdown: &str) -> Option<&str> {
    let marker = "```json";
    let (_, rest) = markdown.split_once(marker)?;
    let rest = rest
        .strip_prefix('\r')
        .or_else(|| rest.strip_prefix('\n'))
        .unwrap_or(rest);
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let (json, _) = rest.split_once("```")?;
    Some(json.trim())
}

fn contains_floating_reference(value: &str) -> bool {
    value
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.')
        .any(|token| matches!(token, "latest" | "main" | "master" | "HEAD"))
}

fn is_full_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|c| c.is_ascii_hexdigit())
}

fn require(missing: &mut Vec<String>, label: &str, ok: bool) {
    if !ok {
        missing.push(label.to_string());
    }
}
