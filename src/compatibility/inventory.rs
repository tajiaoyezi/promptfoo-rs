use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::matrix::CapabilityMatrix;
use super::provider_assertion::{
    ProviderModuleBurndownReport, ProviderModuleResolution, ProviderModuleResolutionKind,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityInventory {
    pub items: Vec<InventoryItem>,
}

impl CapabilityInventory {
    pub fn from_json_file(path: &Path) -> Result<Self, InventoryError> {
        let json = fs::read_to_string(path).map_err(InventoryError::Read)?;
        serde_json::from_str(&json).map_err(|error| InventoryError::Parse(error.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub stable_id: String,
    pub category: String,
    pub name: String,
    pub source_reference: String,
    pub level_hint: String,
    pub status: String,
    pub owner_hint: String,
    pub unresolved_reason: Option<String>,
}

impl InventoryItem {
    pub fn stable_id(category: &str, name: &str) -> String {
        let category = slug(category);
        let name = slug(name);
        format!("{category}:{name}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamSnapshot {
    pub source_ref: String,
    pub items: Vec<SnapshotItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotItem {
    pub category: String,
    pub name: String,
    pub source_reference: String,
    pub level_hint: String,
    pub status: String,
    pub owner_hint: String,
    pub unresolved_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryReport {
    pub missing_categories: Vec<String>,
    pub items_missing_metadata: Vec<String>,
    pub unresolved_items: Vec<String>,
    pub release_blocking_unresolved: usize,
}

impl InventoryReport {
    pub fn is_complete(&self) -> bool {
        self.missing_categories.is_empty()
            && self.items_missing_metadata.is_empty()
            && self.release_blocking_unresolved == 0
    }
}

#[derive(Debug)]
pub enum InventoryError {
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(String),
    Validation(String),
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(formatter, "failed to read inventory input: {error}"),
            Self::Write(error) => write!(formatter, "failed to write inventory output: {error}"),
            Self::Parse(error) => write!(formatter, "failed to parse inventory input: {error}"),
            Self::Validation(error) => write!(formatter, "inventory validation failed: {error}"),
        }
    }
}

impl std::error::Error for InventoryError {}

pub type CompatibilityEvidenceError = InventoryError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenSourceReference {
    pub package_version: String,
    pub git_ref: String,
    pub git_commit: String,
    pub npm_integrity: String,
    pub acquisition_command: String,
    #[serde(default)]
    pub source_files: Vec<String>,
}

impl FrozenSourceReference {
    pub fn new(
        package_version: &str,
        git_ref: &str,
        git_commit: &str,
        npm_integrity: &str,
        acquisition_command: &str,
    ) -> Self {
        Self {
            package_version: package_version.to_string(),
            git_ref: git_ref.to_string(),
            git_commit: git_commit.to_string(),
            npm_integrity: npm_integrity.to_string(),
            acquisition_command: acquisition_command.to_string(),
            source_files: Vec::new(),
        }
    }

    pub fn from_baseline_lock(path: &Path) -> Result<Self, InventoryError> {
        let markdown = fs::read_to_string(path).map_err(InventoryError::Read)?;
        let package_version = extract_promptfoo_version(&markdown).ok_or_else(|| {
            InventoryError::Parse("missing promptfoo package version".to_string())
        })?;
        let git_commit = extract_git_commit(&markdown)
            .ok_or_else(|| InventoryError::Parse("missing frozen git commit".to_string()))?;
        let npm_integrity = extract_npm_integrity(&markdown)
            .ok_or_else(|| InventoryError::Parse("missing npm integrity".to_string()))?;
        let git_ref = format!("refs/tags/{package_version}");
        Ok(Self::new(
            &package_version,
            &git_ref,
            &git_commit,
            &npm_integrity,
            &format!("git ls-tree -r --name-only {git_ref}"),
        ))
    }

    pub fn with_source_files<I, S>(mut self, source_files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.source_files = source_files.into_iter().map(Into::into).collect();
        self
    }

    pub fn validate_non_floating(&self) -> Result<(), InventoryError> {
        let floating = ["latest", "main", "master", "head", "HEAD", "*"];
        for value in [
            self.package_version.as_str(),
            self.git_ref.as_str(),
            self.git_commit.as_str(),
        ] {
            if floating
                .iter()
                .any(|needle| value.eq_ignore_ascii_case(needle) || value.ends_with("/main"))
            {
                return Err(InventoryError::Validation(format!(
                    "floating source reference is not allowed: {value}"
                )));
            }
        }
        if self.package_version.trim().is_empty() {
            return Err(InventoryError::Validation(
                "package version must be frozen".to_string(),
            ));
        }
        if self.git_commit.len() != 40 || !self.git_commit.chars().all(|ch| ch.is_ascii_hexdigit())
        {
            return Err(InventoryError::Validation(
                "git commit must be a full 40 character SHA".to_string(),
            ));
        }
        if !self.npm_integrity.starts_with("sha512-") {
            return Err(InventoryError::Validation(
                "npm integrity must be a frozen sha512 value".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetMode {
    Frozen,
    Current,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUpstreamObservation {
    pub current_head: String,
    pub frozen_tag_ref: String,
    pub frozen_tag_commit: String,
    pub observed_release_ref: Option<String>,
    pub observed_release_commit: Option<String>,
    pub observed_at: String,
    pub source: String,
    #[serde(default)]
    pub evidence_refs: BTreeMap<String, String>,
}

impl CurrentUpstreamObservation {
    pub fn from_ls_remote(output: &str) -> Result<Self, TargetPolicyError> {
        let mut current_head = None;
        let mut frozen_tag_commit = None;
        let mut observed_release_ref = None;
        let mut observed_release_commit = None;

        for line in output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            let mut parts = line.split_whitespace();
            let sha = parts
                .next()
                .ok_or_else(|| TargetPolicyError::Parse("missing ls-remote sha".to_string()))?;
            let reference = parts
                .next()
                .ok_or_else(|| TargetPolicyError::Parse("missing ls-remote ref".to_string()))?;
            if !is_full_sha(sha) {
                return Err(TargetPolicyError::Parse(format!(
                    "ls-remote ref {reference} did not contain a full sha"
                )));
            }
            match reference {
                "HEAD" => current_head = Some(sha.to_string()),
                "refs/tags/0.121.13" => frozen_tag_commit = Some(sha.to_string()),
                other if other.starts_with("refs/tags/") => {
                    observed_release_ref = Some(other.to_string());
                    observed_release_commit = Some(sha.to_string());
                }
                _ => {}
            }
        }

        Ok(Self {
            current_head: current_head.ok_or_else(|| {
                TargetPolicyError::Parse("ls-remote output missing HEAD".to_string())
            })?,
            frozen_tag_ref: "refs/tags/0.121.13".to_string(),
            frozen_tag_commit: frozen_tag_commit.ok_or_else(|| {
                TargetPolicyError::Parse("ls-remote output missing refs/tags/0.121.13".to_string())
            })?,
            observed_release_ref,
            observed_release_commit,
            observed_at: current_unix_timestamp(),
            source: "git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7".to_string(),
            evidence_refs: BTreeMap::new(),
        })
    }

    pub fn with_current_evidence_refs<I, K, V>(mut self, refs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.evidence_refs = refs
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentClaimPolicy {
    pub schema: String,
    pub status: String,
    pub target_mode: TargetMode,
    pub stable_claim: String,
    pub current_perfect_claim_allowed: bool,
    pub reason: String,
    pub frozen: FrozenSourceReference,
    pub current: CurrentUpstreamObservation,
    pub required_current_evidence: Vec<String>,
    pub missing_current_evidence: Vec<String>,
    pub mismatched_current_evidence: Vec<String>,
}

#[derive(Debug)]
pub enum TargetPolicyError {
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(String),
}

pub fn evaluate_current_claim_policy(
    frozen: &FrozenSourceReference,
    current: &CurrentUpstreamObservation,
    mode: TargetMode,
) -> CurrentClaimPolicy {
    let required_current_evidence = required_current_evidence();
    let mut missing_current_evidence = Vec::new();
    let mut mismatched_current_evidence = Vec::new();

    if mode == TargetMode::Current {
        for evidence_key in &required_current_evidence {
            match current.evidence_refs.get(evidence_key.as_str()) {
                Some(ref_sha) if ref_sha == &current.current_head => {}
                Some(_) => mismatched_current_evidence.push(evidence_key.clone()),
                None => missing_current_evidence.push(evidence_key.clone()),
            }
        }
    }

    let head_differs = current.current_head != frozen.git_commit;
    let current_mode_evidence_ready =
        missing_current_evidence.is_empty() && mismatched_current_evidence.is_empty();
    let current_perfect_claim_allowed = match mode {
        TargetMode::Frozen => false,
        TargetMode::Current => current_mode_evidence_ready,
    };
    let reason = match mode {
        TargetMode::Frozen if head_differs => format!(
            "target mode is frozen; current HEAD {} differs from frozen baseline {}",
            current.current_head, frozen.git_commit
        ),
        TargetMode::Frozen => {
            "target mode is frozen; current-perfect claims require current mode evidence"
                .to_string()
        }
        TargetMode::Current if current_mode_evidence_ready => format!(
            "all current mode evidence shares observed ref {}",
            current.current_head
        ),
        TargetMode::Current => format!(
            "current mode evidence is incomplete or mismatched for observed ref {}",
            current.current_head
        ),
    };
    let status = if mode == TargetMode::Frozen || current_perfect_claim_allowed {
        "ready"
    } else {
        "blocked"
    };
    let stable_claim = match mode {
        TargetMode::Frozen => "frozen-baseline compatibility",
        TargetMode::Current if current_perfect_claim_allowed => "current-upstream perfect refactor",
        TargetMode::Current => "current-upstream blocked",
    };

    CurrentClaimPolicy {
        schema: "promptfoo-rs.current-upstream-policy.v1".to_string(),
        status: status.to_string(),
        target_mode: mode,
        stable_claim: stable_claim.to_string(),
        current_perfect_claim_allowed,
        reason,
        frozen: frozen.clone(),
        current: current.clone(),
        required_current_evidence,
        missing_current_evidence,
        mismatched_current_evidence,
    }
}

pub fn write_current_upstream_policy(
    policy: &CurrentClaimPolicy,
    path: &Path,
) -> Result<(), TargetPolicyError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(TargetPolicyError::Write)?;
    }
    let json = serde_json::to_string_pretty(policy)
        .map_err(|error| TargetPolicyError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(TargetPolicyError::Write)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpmPackageObservation {
    pub package_name: String,
    pub package_version: String,
    pub git_head: String,
    pub tarball: String,
    pub integrity: String,
    pub modified: String,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamDistributionTarget {
    pub schema: String,
    pub status: String,
    pub frozen: FrozenSourceReference,
    pub npm_core: NpmPackageObservation,
    pub github: CurrentUpstreamObservation,
    pub npm_core_matches_frozen_baseline: bool,
    pub repository_head_matches_npm_core: bool,
    pub github_latest_release_is_core_package: bool,
    pub github_latest_release_channel: String,
    pub current_repository_perfect_claim_allowed: bool,
    pub reason: String,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestGithubEvidence {
    pub default_branch_head: String,
    pub npm_tag_ref: String,
    pub npm_tag_commit: String,
    pub latest_release_ref: String,
    pub latest_release_commit: String,
    pub latest_release_name: String,
    pub latest_release_url: String,
    pub latest_release_published_at: String,
    pub latest_release_channel: String,
    pub latest_release_is_core_package: bool,
    pub source: String,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestTargetLock {
    pub schema: String,
    pub status: String,
    pub npm_latest: NpmPackageObservation,
    pub github: CurrentLatestGithubEvidence,
    pub target_selection_blocker_resolved: bool,
    pub current_latest_claim_allowed: bool,
    pub downstream_required_evidence: Vec<String>,
    pub reason: String,
    pub observed_at: String,
}

#[derive(Debug)]
pub enum CurrentLatestTargetError {
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for CurrentLatestTargetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(
                formatter,
                "failed to read current latest target input: {error}"
            ),
            Self::Write(error) => write!(
                formatter,
                "failed to write current latest target output: {error}"
            ),
            Self::Parse(error) => write!(
                formatter,
                "failed to parse current latest target input: {error}"
            ),
        }
    }
}

impl std::error::Error for CurrentLatestTargetError {}

impl CurrentLatestTargetLock {
    pub fn from_observations(
        npm_json: &str,
        latest_release_json: &str,
        ls_remote_output: &str,
    ) -> Result<Self, CurrentLatestTargetError> {
        let npm_latest = parse_npm_package_observation(npm_json)
            .map_err(|error| CurrentLatestTargetError::Parse(error.to_string()))?;
        reject_floating_completion_value(&npm_latest.package_version)?;
        reject_floating_completion_value(&npm_latest.git_head)?;

        let release: serde_json::Value = serde_json::from_str(latest_release_json)
            .map_err(|error| CurrentLatestTargetError::Parse(error.to_string()))?;
        let latest_tag = nested_or_flat_string(&release, &["tagName"], "tag_name")
            .or_else(|| json_string(&release, "tagName"))
            .ok_or_else(|| {
                CurrentLatestTargetError::Parse(
                    "GitHub latest release metadata missing tagName/tag_name".to_string(),
                )
            })?;
        validate_safe_tag(&latest_tag)?;
        let latest_release_ref = format!("refs/tags/{latest_tag}");
        let release_commit =
            nested_or_flat_string(&release, &["targetCommitish"], "target_commitish")
                .or_else(|| json_string(&release, "targetCommitish"))
                .ok_or_else(|| {
                    CurrentLatestTargetError::Parse(
                        "GitHub latest release metadata missing targetCommitish/target_commitish"
                            .to_string(),
                    )
                })?;
        if !is_full_sha(&release_commit) {
            return Err(CurrentLatestTargetError::Parse(
                "GitHub latest release target commit must be a full 40 character SHA".to_string(),
            ));
        }
        let release_name = nested_or_flat_string(&release, &["name"], "name")
            .unwrap_or_else(|| latest_tag.clone());
        let release_url = nested_or_flat_string(&release, &["htmlUrl"], "html_url")
            .or_else(|| json_string(&release, "htmlUrl"))
            .unwrap_or_else(|| {
                format!("https://github.com/promptfoo/promptfoo/releases/tag/{latest_tag}")
            });
        let published_at = nested_or_flat_string(&release, &["publishedAt"], "published_at")
            .or_else(|| json_string(&release, "publishedAt"))
            .unwrap_or_else(|| "unknown".to_string());

        let npm_tag_ref = format!("refs/tags/{}", npm_latest.package_version);
        let mut default_branch_head = None;
        let mut npm_tag_commit = None;
        let mut latest_release_commit = None;
        for line in ls_remote_output
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            let mut parts = line.split_whitespace();
            let sha = parts.next().ok_or_else(|| {
                CurrentLatestTargetError::Parse("missing ls-remote sha".to_string())
            })?;
            let reference = parts.next().ok_or_else(|| {
                CurrentLatestTargetError::Parse("missing ls-remote ref".to_string())
            })?;
            if !is_full_sha(sha) {
                return Err(CurrentLatestTargetError::Parse(format!(
                    "ls-remote ref {reference} did not contain a full sha"
                )));
            }
            if reference == "HEAD" {
                default_branch_head = Some(sha.to_string());
            } else if reference == npm_tag_ref || reference == format!("{npm_tag_ref}^{{}}") {
                npm_tag_commit = Some(sha.to_string());
            } else if reference == latest_release_ref
                || reference == format!("{latest_release_ref}^{{}}")
            {
                latest_release_commit = Some(sha.to_string());
            }
        }
        let default_branch_head = default_branch_head.ok_or_else(|| {
            CurrentLatestTargetError::Parse("ls-remote output missing HEAD".to_string())
        })?;
        let npm_tag_commit = npm_tag_commit.ok_or_else(|| {
            CurrentLatestTargetError::Parse(format!("ls-remote output missing {npm_tag_ref}"))
        })?;
        let latest_release_commit = latest_release_commit.ok_or_else(|| {
            CurrentLatestTargetError::Parse(format!(
                "ls-remote output missing {latest_release_ref}"
            ))
        })?;
        if latest_release_commit != release_commit {
            return Err(CurrentLatestTargetError::Parse(format!(
                "GitHub release metadata commit {release_commit} differs from ls-remote {latest_release_commit}"
            )));
        }

        let latest_release_channel =
            classify_github_release_channel(Some(latest_release_ref.as_str()));
        let latest_release_is_core_package = latest_release_channel == "core-package"
            && latest_release_commit == npm_latest.git_head;
        let downstream_required_evidence = [
            "current_latest_source_inventory",
            "current_latest_matrix",
            "current_latest_golden_corpus",
            "current_latest_quality_gate",
            "external_authority_or_waivers",
            "publication_authority_or_waivers",
        ]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
        let default_branch_matches_npm = default_branch_head == npm_latest.git_head;
        let status = if default_branch_matches_npm && latest_release_is_core_package {
            "locked"
        } else {
            "locked-with-drift"
        };
        let reason = current_latest_target_reason(
            &npm_latest,
            &default_branch_head,
            &latest_release_ref,
            &latest_release_channel,
        );

        Ok(Self {
            schema: "promptfoo-rs.current-latest-target.v1".to_string(),
            status: status.to_string(),
            npm_latest,
            github: CurrentLatestGithubEvidence {
                default_branch_head,
                npm_tag_ref,
                npm_tag_commit,
                latest_release_ref,
                latest_release_commit,
                latest_release_name: release_name,
                latest_release_url: release_url,
                latest_release_published_at: published_at,
                latest_release_channel,
                latest_release_is_core_package,
                source: "git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/<npm-version> refs/tags/<latest-release>".to_string(),
                observed_at: current_unix_timestamp(),
            },
            target_selection_blocker_resolved: true,
            current_latest_claim_allowed: false,
            downstream_required_evidence,
            reason,
            observed_at: current_unix_timestamp(),
        })
    }
}

pub fn write_current_latest_target_lock(
    lock: &CurrentLatestTargetLock,
    json_path: &Path,
    markdown_path: &Path,
) -> Result<(), CurrentLatestTargetError> {
    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent).map_err(CurrentLatestTargetError::Write)?;
    }
    if let Some(parent) = markdown_path.parent() {
        fs::create_dir_all(parent).map_err(CurrentLatestTargetError::Write)?;
    }
    let json = serde_json::to_string_pretty(lock)
        .map_err(|error| CurrentLatestTargetError::Parse(error.to_string()))?;
    fs::write(json_path, format!("{json}\n")).map_err(CurrentLatestTargetError::Write)?;
    fs::write(markdown_path, current_latest_lock_markdown(lock))
        .map_err(CurrentLatestTargetError::Write)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestInventoryRow {
    pub stable_id: String,
    pub category: String,
    pub name: String,
    pub source_reference: String,
    pub source_file: String,
    pub level: String,
    pub implementation_status: String,
    pub verification_owner: String,
    pub evidence_kind: String,
    pub evidence_reference: String,
    pub blocker_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestInventoryReport {
    pub schema: String,
    pub status: String,
    pub target: CurrentLatestTargetLock,
    pub extraction_mode: String,
    pub source_root: String,
    pub extraction_timestamp: String,
    pub source_counts: SourceInventoryCounts,
    pub rows: Vec<CurrentLatestInventoryRow>,
    pub categories: Vec<String>,
    pub unclassified_rows: Vec<String>,
    pub rows_missing_evidence: Vec<String>,
    pub perfect_refactor_claim_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestMatrixRow {
    pub item_id: String,
    pub category: String,
    pub source_reference: String,
    pub level: String,
    pub implementation_status: String,
    pub verification_owner: String,
    pub evidence_kind: String,
    pub evidence_reference: String,
    pub blocker_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentLatestMatrixReport {
    pub schema: String,
    pub status: String,
    pub target_ref: String,
    pub rows: Vec<CurrentLatestMatrixRow>,
    pub unclassified_rows: Vec<String>,
    pub rows_missing_evidence: Vec<String>,
    pub perfect_refactor_claim_allowed: bool,
}

pub fn extract_current_latest_inventory(
    lock: &CurrentLatestTargetLock,
    source_root: &Path,
) -> Result<CurrentLatestInventoryReport, InventoryError> {
    validate_current_latest_lock_for_source_inventory(lock)?;
    if !source_root.is_dir() {
        return Err(InventoryError::Validation(format!(
            "current latest source root does not exist or is not a directory: {}",
            source_root.display()
        )));
    }

    let mut files = Vec::new();
    collect_current_latest_files(source_root, source_root, &mut files)?;
    files.sort();

    let mut counts = SourceInventoryCounts::default();
    let mut rows = BTreeMap::new();

    for file in files {
        for category in current_latest_file_categories(&file) {
            increment_current_latest_count(&mut counts, category);
            insert_current_latest_file_row(&mut rows, lock, category, &file);
        }

        let content_path = source_root.join(&file);
        let content = fs::read_to_string(&content_path).unwrap_or_default();
        for flag in extract_flag_tokens(&content) {
            insert_current_latest_flag_row(&mut rows, lock, &file, &flag);
        }
    }

    let rows = rows.into_values().collect::<Vec<_>>();
    let categories = current_latest_categories(&rows);
    let unclassified_rows = current_latest_unclassified_rows(&rows);
    let rows_missing_evidence = current_latest_rows_missing_evidence(&rows);
    let status = if !unclassified_rows.is_empty() || !rows_missing_evidence.is_empty() {
        "ready-with-blockers"
    } else {
        "ready"
    };

    Ok(CurrentLatestInventoryReport {
        schema: "promptfoo-rs.current-latest-source-inventory.v1".to_string(),
        status: status.to_string(),
        target: lock.clone(),
        extraction_mode: "current-latest-locked-source-tree".to_string(),
        source_root: source_root.display().to_string(),
        extraction_timestamp: current_unix_timestamp(),
        source_counts: counts,
        rows,
        categories,
        unclassified_rows,
        rows_missing_evidence,
        perfect_refactor_claim_allowed: false,
    })
}

pub fn reconcile_current_latest_matrix(
    inventory: &CurrentLatestInventoryReport,
    existing_matrix: &CapabilityMatrix,
) -> CurrentLatestMatrixReport {
    let explicit_rows = existing_matrix
        .rows
        .iter()
        .map(|row| (row.capability.as_str(), row))
        .collect::<BTreeMap<_, _>>();
    let rows = inventory
        .rows
        .iter()
        .map(|row| {
            explicit_rows
                .get(row.stable_id.as_str())
                .map(|explicit| current_latest_matrix_row_from_explicit(row, explicit))
                .unwrap_or_else(|| current_latest_matrix_row_from_inventory(row))
        })
        .collect::<Vec<_>>();
    let unclassified_rows = rows
        .iter()
        .filter(|row| row.category == "unclassified")
        .map(|row| row.item_id.clone())
        .collect::<Vec<_>>();
    let rows_missing_evidence = rows
        .iter()
        .filter(|row| {
            row.evidence_kind.trim().is_empty() || row.evidence_reference.trim().is_empty()
        })
        .map(|row| row.item_id.clone())
        .collect::<Vec<_>>();
    let has_non_native_or_blocked = rows.iter().any(|row| {
        row.implementation_status != "native"
            || row.evidence_kind == "blocker"
            || row.blocker_reason.is_some()
    });
    let perfect_refactor_claim_allowed = unclassified_rows.is_empty()
        && rows_missing_evidence.is_empty()
        && !has_non_native_or_blocked;
    let status = if unclassified_rows.is_empty() && rows_missing_evidence.is_empty() {
        "ready"
    } else {
        "ready-with-blockers"
    };

    CurrentLatestMatrixReport {
        schema: "promptfoo-rs.current-latest-matrix.v1".to_string(),
        status: status.to_string(),
        target_ref: inventory.target.github.default_branch_head.clone(),
        rows,
        unclassified_rows,
        rows_missing_evidence,
        perfect_refactor_claim_allowed,
    }
}

pub fn write_current_latest_inventory_artifacts(
    inventory: &CurrentLatestInventoryReport,
    matrix: &CurrentLatestMatrixReport,
    inventory_path: &Path,
    matrix_path: &Path,
) -> Result<(), InventoryError> {
    if let Some(parent) = inventory_path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    if let Some(parent) = matrix_path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let inventory_json = serde_json::to_string_pretty(inventory)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    let matrix_json = serde_json::to_string_pretty(matrix)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(inventory_path, format!("{inventory_json}\n")).map_err(InventoryError::Write)?;
    fs::write(matrix_path, format!("{matrix_json}\n")).map_err(InventoryError::Write)
}

#[derive(Debug)]
pub enum DistributionTargetError {
    Read(std::io::Error),
    Write(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for DistributionTargetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(error) => write!(
                formatter,
                "failed to read distribution target input: {error}"
            ),
            Self::Write(error) => write!(
                formatter,
                "failed to write distribution target output: {error}"
            ),
            Self::Parse(error) => write!(
                formatter,
                "failed to parse distribution target input: {error}"
            ),
        }
    }
}

impl std::error::Error for DistributionTargetError {}

pub fn parse_npm_package_observation(
    json: &str,
) -> Result<NpmPackageObservation, DistributionTargetError> {
    let value: serde_json::Value = serde_json::from_str(json)
        .map_err(|error| DistributionTargetError::Parse(error.to_string()))?;
    let package_version = json_string(&value, "version").ok_or_else(|| {
        DistributionTargetError::Parse("npm metadata missing version".to_string())
    })?;
    let git_head = json_string(&value, "gitHead").ok_or_else(|| {
        DistributionTargetError::Parse("npm metadata missing gitHead".to_string())
    })?;
    let tarball =
        nested_or_flat_string(&value, &["dist", "tarball"], "dist.tarball").ok_or_else(|| {
            DistributionTargetError::Parse("npm metadata missing dist.tarball".to_string())
        })?;
    let integrity = nested_or_flat_string(&value, &["dist", "integrity"], "dist.integrity")
        .ok_or_else(|| {
            DistributionTargetError::Parse("npm metadata missing dist.integrity".to_string())
        })?;
    let modified = nested_or_flat_string(&value, &["time", "modified"], "time.modified")
        .ok_or_else(|| {
            DistributionTargetError::Parse("npm metadata missing time.modified".to_string())
        })?;

    if !is_full_sha(&git_head) {
        return Err(DistributionTargetError::Parse(
            "npm gitHead must be a full 40 character SHA".to_string(),
        ));
    }
    if !integrity.starts_with("sha512-") {
        return Err(DistributionTargetError::Parse(
            "npm integrity must be a sha512 value".to_string(),
        ));
    }
    if !tarball.starts_with("https://registry.npmjs.org/") {
        return Err(DistributionTargetError::Parse(
            "npm tarball must come from registry.npmjs.org".to_string(),
        ));
    }

    Ok(NpmPackageObservation {
        package_name: "promptfoo".to_string(),
        package_version,
        git_head,
        tarball,
        integrity,
        modified,
        source:
            "npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json"
                .to_string(),
    })
}

pub fn build_upstream_distribution_target(
    npm: NpmPackageObservation,
    github: CurrentUpstreamObservation,
    frozen: FrozenSourceReference,
) -> UpstreamDistributionTarget {
    let npm_core_matches_frozen_baseline = npm.package_version == frozen.package_version
        && npm.git_head == frozen.git_commit
        && npm.integrity == frozen.npm_integrity;
    let repository_head_matches_npm_core = github.current_head == npm.git_head;
    let github_latest_release_channel =
        classify_github_release_channel(github.observed_release_ref.as_deref());
    let github_latest_release_is_core_package = github_latest_release_channel == "core-package"
        && github.observed_release_commit.as_deref() == Some(npm.git_head.as_str());
    let current_repository_perfect_claim_allowed =
        repository_head_matches_npm_core && github_latest_release_is_core_package;
    let status = if current_repository_perfect_claim_allowed {
        "ready"
    } else if npm_core_matches_frozen_baseline {
        "ready-with-drift"
    } else {
        "blocked"
    };
    let reason = distribution_target_reason(
        &npm,
        &github,
        &frozen,
        npm_core_matches_frozen_baseline,
        repository_head_matches_npm_core,
        github_latest_release_is_core_package,
        &github_latest_release_channel,
    );

    UpstreamDistributionTarget {
        schema: "promptfoo-rs.upstream-distribution-target.v1".to_string(),
        status: status.to_string(),
        frozen,
        npm_core: npm,
        github,
        npm_core_matches_frozen_baseline,
        repository_head_matches_npm_core,
        github_latest_release_is_core_package,
        github_latest_release_channel,
        current_repository_perfect_claim_allowed,
        reason,
        observed_at: current_unix_timestamp(),
    }
}

pub fn write_upstream_distribution_target(
    target: &UpstreamDistributionTarget,
    path: &Path,
) -> Result<(), DistributionTargetError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(DistributionTargetError::Write)?;
    }
    let json = serde_json::to_string_pretty(target)
        .map_err(|error| DistributionTargetError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(DistributionTargetError::Write)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInventoryCounts {
    pub command_related_files: usize,
    pub provider_files: usize,
    pub assertion_files: usize,
    pub redteam_plugin_files: usize,
    pub redteam_strategy_files: usize,
    pub viewer_app_files: usize,
    pub example_files: usize,
    pub output_files: usize,
    pub config_files: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceExtractedInventory {
    pub baseline: FrozenSourceReference,
    pub extraction_timestamp: String,
    pub source_counts: SourceInventoryCounts,
    pub items: Vec<InventoryItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceInventoryStatus {
    Ready,
    ReadyWithBlockers,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInventoryBlocker {
    pub item_id: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceInventoryReport {
    pub schema: String,
    pub status: SourceInventoryStatus,
    pub baseline: FrozenSourceReference,
    pub extraction_timestamp: String,
    pub source_counts: SourceInventoryCounts,
    pub inventory_item_count: usize,
    pub source_extracted_item_count: usize,
    pub items_missing_metadata: Vec<String>,
    pub missing_matrix_rows: Vec<String>,
    pub silent_omissions: Vec<String>,
    pub release_blockers: Vec<SourceInventoryBlocker>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAccountingLedger {
    pub schema: String,
    pub source_extracted_item_count: usize,
    pub ledger_item_count: usize,
    pub unrepresented_item_count: usize,
    pub p0_blocker_count: usize,
    pub rows: Vec<SourceAccountingRow>,
    pub unrepresented_items: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAccountingBurndownSummary {
    pub schema: String,
    pub viewer_config_reclassified_count: usize,
    pub p0_accounting_blocker_count: usize,
    pub remaining_p0_blockers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreConfigSourceDecision {
    pub item_id: String,
    pub source_reference: String,
    pub classification: String,
    pub level: String,
    pub target_status: String,
    pub owner: String,
    pub verification: String,
    pub reason: String,
    pub fixture_path: Option<String>,
    pub local_runtime_parity: bool,
    pub external_authority_required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreConfigSourceBurndownReport {
    pub schema: String,
    pub non_app_config_total: usize,
    pub non_app_config_fixture_covered_count: usize,
    pub non_app_config_external_blocker_count: usize,
    pub non_app_config_auxiliary_registration_count: usize,
    pub non_app_config_generic_blocker_count: usize,
    pub decisions: Vec<CoreConfigSourceDecision>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSourceAccountingDecision {
    pub item_id: String,
    pub source_reference: String,
    pub classification: String,
    pub level: String,
    pub target_status: String,
    pub owner: String,
    pub verification: String,
    pub reason: String,
    pub fixture_ids: Vec<String>,
    pub local_fixture_covered: bool,
    pub external_authority_required: bool,
    pub release_blocking: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderSourceAccountingReconciliationReport {
    pub schema: String,
    pub provider_source_total: usize,
    pub resolved_provider_fixture_count: usize,
    pub provider_external_authority_count: usize,
    pub provider_source_generic_blocker_count: usize,
    pub source_p0_accounting_blocker_count: usize,
    pub remaining_source_p0_blockers: Vec<String>,
    pub resolved_provider_source_rows: Vec<ProviderSourceAccountingDecision>,
    pub remaining_provider_source_blockers: Vec<ProviderSourceAccountingDecision>,
    pub decisions: Vec<ProviderSourceAccountingDecision>,
}

impl SourceAccountingLedger {
    pub fn unrepresented_items(&self) -> Vec<String> {
        self.unrepresented_items.clone()
    }

    pub fn p0_blockers(&self) -> Vec<String> {
        self.rows
            .iter()
            .filter(|row| row.level == "P0" && row.verification.starts_with("blocker:"))
            .map(|row| row.item_id.clone())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAccountingRow {
    pub item_id: String,
    pub category: String,
    pub source_reference: String,
    pub level: String,
    pub target_status: String,
    pub owner: String,
    pub verification: String,
    pub reason: String,
    pub generated: bool,
}

pub struct SourceInventoryExtractor;

impl SourceInventoryExtractor {
    pub fn extract(
        source: &FrozenSourceReference,
    ) -> Result<SourceExtractedInventory, InventoryError> {
        source.validate_non_floating()?;
        if source.source_files.is_empty() {
            return Err(InventoryError::Validation(
                "source extraction requires a frozen source file list".to_string(),
            ));
        }

        let mut counts = SourceInventoryCounts::default();
        let mut items = BTreeMap::new();

        for file in &source.source_files {
            let file = normalize_source_path(file);
            if is_command_related_file(&file) {
                counts.command_related_files += 1;
                insert_source_item(&mut items, "command", &file);
            }
            if is_provider_file(&file) {
                counts.provider_files += 1;
                insert_source_item(&mut items, "provider", &file);
            }
            if is_assertion_file(&file) {
                counts.assertion_files += 1;
                insert_source_item(&mut items, "assertion", &file);
            }
            if is_redteam_plugin_file(&file) {
                counts.redteam_plugin_files += 1;
                insert_source_item(&mut items, "redteam-plugin", &file);
            }
            if is_redteam_strategy_file(&file) {
                counts.redteam_strategy_files += 1;
                insert_source_item(&mut items, "redteam-strategy", &file);
            }
            if is_viewer_file(&file) {
                counts.viewer_app_files += 1;
                insert_source_item(&mut items, "viewer", &file);
            }
            if is_example_file(&file) {
                counts.example_files += 1;
                insert_source_item(&mut items, "example", &file);
            }
            if is_output_file(&file) {
                counts.output_files += 1;
                insert_source_item(&mut items, "output", &file);
            }
            if is_config_file(&file) {
                counts.config_files += 1;
                insert_source_item(&mut items, "config", &file);
            }
        }

        Ok(SourceExtractedInventory {
            baseline: source.clone(),
            extraction_timestamp: current_unix_timestamp(),
            source_counts: counts,
            items: items.into_values().collect(),
        })
    }
}

pub fn validate_source_extracted_inventory(
    inventory: &SourceExtractedInventory,
    matrix: &CapabilityMatrix,
) -> SourceInventoryReport {
    let matrix_rows: BTreeSet<_> = matrix
        .rows
        .iter()
        .map(|row| row.capability.as_str())
        .collect();
    let mut items_missing_metadata = Vec::new();
    let mut missing_matrix_rows = Vec::new();
    let mut release_blockers = Vec::new();

    for item in &inventory.items {
        let expected_id = InventoryItem::stable_id(&item.category, &item.name);
        if item.stable_id != expected_id
            || item.category.trim().is_empty()
            || item.name.trim().is_empty()
            || item.source_reference.trim().is_empty()
            || !matches!(item.level_hint.as_str(), "P0" | "P1" | "P2")
            || item.owner_hint.trim().is_empty()
        {
            items_missing_metadata.push(item.stable_id.clone());
            release_blockers.push(SourceInventoryBlocker {
                item_id: item.stable_id.clone(),
                reason: "source-extracted item missing required metadata".to_string(),
            });
        }
        if !matrix_rows.contains(item.stable_id.as_str()) {
            missing_matrix_rows.push(item.stable_id.clone());
            release_blockers.push(SourceInventoryBlocker {
                item_id: item.stable_id.clone(),
                reason: "missing matrix row for source-extracted item".to_string(),
            });
        }
        if item.status == "unresolved" && item.unresolved_reason.as_deref().unwrap_or("").is_empty()
        {
            release_blockers.push(SourceInventoryBlocker {
                item_id: item.stable_id.clone(),
                reason: "unresolved source-extracted item missing reason".to_string(),
            });
        }
    }

    let status = if !items_missing_metadata.is_empty() {
        SourceInventoryStatus::Blocked
    } else if !release_blockers.is_empty() {
        SourceInventoryStatus::ReadyWithBlockers
    } else {
        SourceInventoryStatus::Ready
    };

    SourceInventoryReport {
        schema: "promptfoo-rs.source-inventory-evidence.v2".to_string(),
        status,
        baseline: inventory.baseline.clone(),
        extraction_timestamp: inventory.extraction_timestamp.clone(),
        source_counts: inventory.source_counts.clone(),
        inventory_item_count: inventory.items.len(),
        source_extracted_item_count: inventory.items.len(),
        items_missing_metadata,
        missing_matrix_rows,
        silent_omissions: Vec::new(),
        release_blockers,
    }
}

pub fn build_source_accounting_ledger(
    inventory: &SourceExtractedInventory,
    matrix: &CapabilityMatrix,
) -> SourceAccountingLedger {
    let explicit_rows: BTreeMap<&str, &super::matrix::CapabilityRow> = matrix
        .rows
        .iter()
        .map(|row| (row.capability.as_str(), row))
        .collect();
    let mut rows = Vec::new();
    let mut represented = BTreeSet::new();

    for item in &inventory.items {
        if let Some(row) = explicit_rows.get(item.stable_id.as_str()) {
            represented.insert(item.stable_id.clone());
            rows.push(SourceAccountingRow {
                item_id: item.stable_id.clone(),
                category: item.category.clone(),
                source_reference: item.source_reference.clone(),
                level: row.level.clone(),
                target_status: row.target_status.clone(),
                owner: row.owner.clone(),
                verification: row.verification.clone(),
                reason: row.notes.clone(),
                generated: false,
            });
            continue;
        }

        represented.insert(item.stable_id.clone());
        rows.push(classify_generated_source_accounting_row(item));
    }

    let unrepresented_items = inventory
        .items
        .iter()
        .filter(|item| !represented.contains(&item.stable_id))
        .map(|item| item.stable_id.clone())
        .collect::<Vec<_>>();
    let p0_blocker_count = rows
        .iter()
        .filter(|row| row.level == "P0" && row.verification.starts_with("blocker:"))
        .count();

    SourceAccountingLedger {
        schema: "promptfoo-rs.source-inventory-ledger.v1".to_string(),
        source_extracted_item_count: inventory.items.len(),
        ledger_item_count: rows.len(),
        unrepresented_item_count: unrepresented_items.len(),
        p0_blocker_count,
        rows,
        unrepresented_items,
    }
}

pub fn write_source_inventory_evidence(
    report: &SourceInventoryReport,
    path: &Path,
) -> Result<(), InventoryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(InventoryError::Write)
}

pub fn write_source_accounting_ledger(
    ledger: &SourceAccountingLedger,
    path: &Path,
) -> Result<(), InventoryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let json = serde_json::to_string_pretty(ledger)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(InventoryError::Write)
}

pub fn extract_upstream_inventory(
    snapshot: &UpstreamSnapshot,
) -> Result<CapabilityInventory, InventoryError> {
    let items = snapshot
        .items
        .iter()
        .map(|item| InventoryItem {
            stable_id: InventoryItem::stable_id(&item.category, &item.name),
            category: item.category.clone(),
            name: item.name.clone(),
            source_reference: if item.source_reference.trim().is_empty() {
                snapshot.source_ref.clone()
            } else {
                item.source_reference.clone()
            },
            level_hint: item.level_hint.clone(),
            status: item.status.clone(),
            owner_hint: item.owner_hint.clone(),
            unresolved_reason: item.unresolved_reason.clone(),
        })
        .collect();
    Ok(CapabilityInventory { items })
}

pub fn validate_inventory_completeness(inventory: &CapabilityInventory) -> InventoryReport {
    let required_categories = [
        "command",
        "flag",
        "provider",
        "assertion",
        "redteam-plugin",
        "redteam-strategy",
        "output",
        "config",
        "node-api",
        "viewer",
        "release",
    ];

    let mut missing_categories = Vec::new();
    for category in required_categories {
        if !inventory.items.iter().any(|item| item.category == category) {
            missing_categories.push(category.to_string());
        }
    }

    let mut items_missing_metadata = Vec::new();
    let mut unresolved_items = Vec::new();

    for item in &inventory.items {
        let expected_id = InventoryItem::stable_id(&item.category, &item.name);
        let missing_metadata = item.stable_id != expected_id
            || item.source_reference.trim().is_empty()
            || !matches!(item.level_hint.as_str(), "P0" | "P1" | "P2")
            || item.status.trim().is_empty()
            || item.owner_hint.trim().is_empty();
        if missing_metadata {
            items_missing_metadata.push(item.stable_id.clone());
        }
        if item.status == "unresolved" || item.unresolved_reason.is_some() {
            unresolved_items.push(item.stable_id.clone());
        }
    }

    InventoryReport {
        missing_categories,
        items_missing_metadata,
        release_blocking_unresolved: unresolved_items.len(),
        unresolved_items,
    }
}

fn slug(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn extract_promptfoo_version(markdown: &str) -> Option<String> {
    for line in markdown.lines().filter(|line| line.contains("promptfoo")) {
        for token in
            line.split(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '|' | ',' | '(' | ')'))
        {
            let candidate = token
                .trim_matches(|ch: char| !ch.is_ascii_alphanumeric() && ch != '.' && ch != '@');
            if let Some(version) = candidate.strip_prefix("promptfoo@") {
                if is_version(version) {
                    return Some(version.to_string());
                }
            }
            if is_version(candidate) {
                return Some(candidate.to_string());
            }
        }
    }
    None
}

fn extract_git_commit(markdown: &str) -> Option<String> {
    markdown
        .split(|ch: char| !ch.is_ascii_hexdigit())
        .find(|token| token.len() == 40 && token.chars().all(|ch| ch.is_ascii_hexdigit()))
        .map(str::to_string)
}

fn extract_npm_integrity(markdown: &str) -> Option<String> {
    let start = markdown.find("sha512-")?;
    let tail = &markdown[start..];
    let end = tail
        .find(|ch: char| ch.is_whitespace() || matches!(ch, '`' | '|' | ',' | '"'))
        .unwrap_or(tail.len());
    Some(tail[..end].trim_end_matches('.').to_string())
}

fn is_version(value: &str) -> bool {
    let mut parts = value.split('.');
    let Some(major) = parts.next() else {
        return false;
    };
    let Some(minor) = parts.next() else {
        return false;
    };
    let Some(patch) = parts.next() else {
        return false;
    };
    parts.next().is_none()
        && [major, minor, patch]
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
}

fn normalize_source_path(path: &str) -> String {
    path.trim()
        .trim_start_matches("package/")
        .replace('\\', "/")
}

fn insert_source_item(items: &mut BTreeMap<String, InventoryItem>, category: &str, file: &str) {
    let name = file_without_extension(file);
    let stable_id = InventoryItem::stable_id(category, &name);
    let (level_hint, status, owner_hint, unresolved_reason) = classify_source_item(category, file);
    items
        .entry(stable_id.clone())
        .or_insert_with(|| InventoryItem {
            stable_id,
            category: category.to_string(),
            name,
            source_reference: format!("promptfoo@0.121.13:{file}"),
            level_hint,
            status,
            owner_hint,
            unresolved_reason,
        });
}

pub fn is_viewer_config_source_reference(source_reference: &str) -> bool {
    source_reference.replace('\\', "/").contains(":src/app/")
}

pub fn classify_generated_source_accounting_row(item: &InventoryItem) -> SourceAccountingRow {
    if item.category == "config" && is_viewer_config_source_reference(&item.source_reference) {
        return SourceAccountingRow {
            item_id: item.stable_id.clone(),
            category: item.category.clone(),
            source_reference: item.source_reference.clone(),
            level: "P1".to_string(),
            target_status: "later".to_string(),
            owner: "web-viewer".to_string(),
            verification: format!("viewer:{}", item.stable_id),
            reason: format!(
                "Local Web viewer P1 scope correction; src/app config/editor source is accounted as viewer UI evidence, not P0 promptfooconfig runtime parity; source: {}",
                item.source_reference
            ),
            generated: true,
        };
    }
    if item.category == "config" {
        let generic = default_generated_accounting_row(item);
        return core_config_decision_to_source_accounting_row(&classify_non_app_config_source_row(
            &generic,
        ));
    }
    default_generated_accounting_row(item)
}

pub fn classify_non_app_config_source_row(row: &SourceAccountingRow) -> CoreConfigSourceDecision {
    if row.category != "config" || is_viewer_config_source_reference(&row.source_reference) {
        return CoreConfigSourceDecision {
            item_id: row.item_id.clone(),
            source_reference: row.source_reference.clone(),
            classification: "not-non-app-config".to_string(),
            level: row.level.clone(),
            target_status: row.target_status.clone(),
            owner: row.owner.clone(),
            verification: row.verification.clone(),
            reason: row.reason.clone(),
            fixture_path: None,
            local_runtime_parity: false,
            external_authority_required: false,
        };
    }

    let normalized = normalized_source_reference(&row.source_reference);
    if is_runtime_config_source_reference(&normalized) {
        return CoreConfigSourceDecision {
            item_id: row.item_id.clone(),
            source_reference: row.source_reference.clone(),
            classification: "native-fixture".to_string(),
            level: "P0".to_string(),
            target_status: "native".to_string(),
            owner: "config-loader".to_string(),
            verification: "fixture:config:promptfooconfig-yaml-json".to_string(),
            reason: format!(
                "runtime promptfooconfig/env/file config source covered by existing P0 native config fixtures; source: {}",
                row.source_reference
            ),
            fixture_path: Some("compatibility/fixtures/config/yaml-prompts/fixture.yaml".to_string()),
            local_runtime_parity: true,
            external_authority_required: false,
        };
    }

    if is_redteam_config_source_reference(&normalized) {
        return CoreConfigSourceDecision {
            item_id: row.item_id.clone(),
            source_reference: row.source_reference.clone(),
            classification: "bridge-fixture".to_string(),
            level: "P0".to_string(),
            target_status: "bridge".to_string(),
            owner: "redteam-engine".to_string(),
            verification: "fixture:config:redteam-yaml".to_string(),
            reason: format!(
                "redteam promptfooconfig source covered by existing bridge fixture for redteam.yaml compatibility; source: {}",
                row.source_reference
            ),
            fixture_path: Some("compatibility/fixtures/config/redteam-yaml/fixture.yaml".to_string()),
            local_runtime_parity: true,
            external_authority_required: false,
        };
    }

    if is_auxiliary_config_source_reference(&normalized) {
        return CoreConfigSourceDecision {
            item_id: row.item_id.clone(),
            source_reference: row.source_reference.clone(),
            classification: "auxiliary-registration".to_string(),
            level: "P1".to_string(),
            target_status: "later".to_string(),
            owner: auxiliary_config_owner(&normalized).to_string(),
            verification: format!("snapshot:{}", row.item_id),
            reason: format!(
                "non-core auxiliary config source is registered under its P1 command domain and is not counted as P0 promptfooconfig runtime parity; source: {}",
                row.source_reference
            ),
            fixture_path: None,
            local_runtime_parity: false,
            external_authority_required: false,
        };
    }

    CoreConfigSourceDecision {
        item_id: row.item_id.clone(),
        source_reference: row.source_reference.clone(),
        classification: "external-blocker".to_string(),
        level: "P0".to_string(),
        target_status: "blocked".to_string(),
        owner: "external-authority".to_string(),
        verification: format!("blocker:{}", row.item_id),
        reason: format!(
            "explicit external cloud/server/telemetry config blocker; not counted as local runtime parity without product authority, credentials, or service contract evidence; source: {}",
            row.source_reference
        ),
        fixture_path: None,
        local_runtime_parity: false,
        external_authority_required: true,
    }
}

pub fn validate_core_config_source_burndown(
    ledger: &SourceAccountingLedger,
) -> CoreConfigSourceBurndownReport {
    let decisions = ledger
        .rows
        .iter()
        .filter(|row| {
            row.category == "config" && !is_viewer_config_source_reference(&row.source_reference)
        })
        .map(classify_non_app_config_source_row)
        .collect::<Vec<_>>();
    let non_app_config_fixture_covered_count = decisions
        .iter()
        .filter(|decision| {
            decision.classification == "native-fixture"
                || decision.classification == "bridge-fixture"
        })
        .count();
    let non_app_config_external_blocker_count = decisions
        .iter()
        .filter(|decision| decision.classification == "external-blocker")
        .count();
    let non_app_config_auxiliary_registration_count = decisions
        .iter()
        .filter(|decision| decision.classification == "auxiliary-registration")
        .count();
    let non_app_config_generic_blocker_count = decisions
        .iter()
        .filter(|decision| {
            decision
                .reason
                .contains("generated P0 accounting row requires")
        })
        .count();

    CoreConfigSourceBurndownReport {
        schema: "promptfoo-rs.core-config-source-burndown.v1".to_string(),
        non_app_config_total: decisions.len(),
        non_app_config_fixture_covered_count,
        non_app_config_external_blocker_count,
        non_app_config_auxiliary_registration_count,
        non_app_config_generic_blocker_count,
        decisions,
    }
}

pub fn write_core_config_source_burndown(
    report: &CoreConfigSourceBurndownReport,
    path: &Path,
) -> Result<(), CompatibilityEvidenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(InventoryError::Write)
}

pub fn source_accounting_burndown_summary(
    ledger: &SourceAccountingLedger,
) -> SourceAccountingBurndownSummary {
    let viewer_config_reclassified_count = ledger
        .rows
        .iter()
        .filter(|row| {
            row.category == "config"
                && is_viewer_config_source_reference(&row.source_reference)
                && row.level == "P1"
                && row.owner == "web-viewer"
        })
        .count();
    let remaining_p0_blockers = ledger.p0_blockers();
    SourceAccountingBurndownSummary {
        schema: "promptfoo-rs.source-accounting-burndown.v1".to_string(),
        viewer_config_reclassified_count,
        p0_accounting_blocker_count: remaining_p0_blockers.len(),
        remaining_p0_blockers,
    }
}

pub fn classify_provider_source_accounting_row(
    row: &SourceAccountingRow,
    provider_report: &ProviderModuleBurndownReport,
) -> ProviderSourceAccountingDecision {
    if row.category != "provider" {
        return ProviderSourceAccountingDecision {
            item_id: row.item_id.clone(),
            source_reference: row.source_reference.clone(),
            classification: "non-provider".to_string(),
            level: row.level.clone(),
            target_status: row.target_status.clone(),
            owner: row.owner.clone(),
            verification: row.verification.clone(),
            reason: row.reason.clone(),
            fixture_ids: vec![],
            local_fixture_covered: false,
            external_authority_required: false,
            release_blocking: row.level == "P0" && row.verification.starts_with("blocker:"),
        };
    }

    if let Some(resolution) = provider_report
        .resolved_by_fixture
        .iter()
        .find(|resolution| resolution.item_id == row.item_id)
    {
        return provider_fixture_source_decision(row, resolution);
    }

    if let Some(resolution) = provider_report
        .remaining_blockers
        .iter()
        .find(|resolution| resolution.item_id == row.item_id)
    {
        return provider_blocker_source_decision(row, resolution);
    }

    let release_blocking = row.level == "P0" && row.verification.starts_with("blocker:");
    ProviderSourceAccountingDecision {
        item_id: row.item_id.clone(),
        source_reference: row.source_reference.clone(),
        classification: if release_blocking {
            "provider-generic-blocker".to_string()
        } else {
            "already-accounted-provider".to_string()
        },
        level: row.level.clone(),
        target_status: row.target_status.clone(),
        owner: row.owner.clone(),
        verification: row.verification.clone(),
        reason: row.reason.clone(),
        fixture_ids: vec![],
        local_fixture_covered: false,
        external_authority_required: false,
        release_blocking,
    }
}

pub fn validate_provider_source_accounting_reconciliation(
    ledger: &SourceAccountingLedger,
    provider_report: &ProviderModuleBurndownReport,
) -> ProviderSourceAccountingReconciliationReport {
    let decisions = ledger
        .rows
        .iter()
        .filter(|row| row.category == "provider")
        .map(|row| classify_provider_source_accounting_row(row, provider_report))
        .collect::<Vec<_>>();
    let resolved_provider_source_rows = decisions
        .iter()
        .filter(|decision| decision.local_fixture_covered && !decision.release_blocking)
        .cloned()
        .collect::<Vec<_>>();
    let remaining_provider_source_blockers = decisions
        .iter()
        .filter(|decision| decision.release_blocking)
        .cloned()
        .collect::<Vec<_>>();
    let mut remaining_source_p0_blockers = ledger
        .rows
        .iter()
        .filter(|row| row.level == "P0")
        .filter_map(|row| {
            if row.category == "provider" {
                decisions
                    .iter()
                    .find(|decision| decision.item_id == row.item_id)
                    .filter(|decision| decision.release_blocking)
                    .map(|decision| decision.item_id.clone())
            } else if row.verification.starts_with("blocker:") {
                Some(row.item_id.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    remaining_source_p0_blockers.sort();

    ProviderSourceAccountingReconciliationReport {
        schema: "promptfoo-rs.provider-source-accounting-reconciliation.v1".to_string(),
        provider_source_total: decisions.len(),
        resolved_provider_fixture_count: resolved_provider_source_rows.len(),
        provider_external_authority_count: remaining_provider_source_blockers
            .iter()
            .filter(|decision| decision.external_authority_required)
            .count(),
        provider_source_generic_blocker_count: remaining_provider_source_blockers
            .iter()
            .filter(|decision| !decision.external_authority_required)
            .count(),
        source_p0_accounting_blocker_count: remaining_source_p0_blockers.len(),
        remaining_source_p0_blockers,
        resolved_provider_source_rows,
        remaining_provider_source_blockers,
        decisions,
    }
}

pub fn write_provider_source_accounting_reconciliation(
    report: &ProviderSourceAccountingReconciliationReport,
    path: &Path,
) -> Result<(), InventoryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(InventoryError::Write)
}

fn core_config_decision_to_source_accounting_row(
    decision: &CoreConfigSourceDecision,
) -> SourceAccountingRow {
    SourceAccountingRow {
        item_id: decision.item_id.clone(),
        category: "config".to_string(),
        source_reference: decision.source_reference.clone(),
        level: decision.level.clone(),
        target_status: decision.target_status.clone(),
        owner: decision.owner.clone(),
        verification: decision.verification.clone(),
        reason: decision.reason.clone(),
        generated: true,
    }
}

fn provider_fixture_source_decision(
    row: &SourceAccountingRow,
    resolution: &ProviderModuleResolution,
) -> ProviderSourceAccountingDecision {
    ProviderSourceAccountingDecision {
        item_id: row.item_id.clone(),
        source_reference: row.source_reference.clone(),
        classification: "fixture-covered-provider".to_string(),
        level: "P0".to_string(),
        target_status: "native".to_string(),
        owner: "provider-runtime".to_string(),
        verification: resolution.verification.clone(),
        reason: format!(
            "provider source row reconciled from provider burndown fixture evidence: {}",
            resolution.reason
        ),
        fixture_ids: resolution.fixture_ids.clone(),
        local_fixture_covered: true,
        external_authority_required: false,
        release_blocking: false,
    }
}

fn provider_blocker_source_decision(
    row: &SourceAccountingRow,
    resolution: &ProviderModuleResolution,
) -> ProviderSourceAccountingDecision {
    let external_authority_required = resolution.requires_external_authority
        || matches!(
            resolution.kind,
            ProviderModuleResolutionKind::ExternalBlocker
        );
    ProviderSourceAccountingDecision {
        item_id: row.item_id.clone(),
        source_reference: row.source_reference.clone(),
        classification: if external_authority_required {
            "external-authority-provider".to_string()
        } else {
            "provider-generic-blocker".to_string()
        },
        level: "P0".to_string(),
        target_status: "blocked".to_string(),
        owner: if external_authority_required {
            "external-authority".to_string()
        } else {
            "provider-runtime".to_string()
        },
        verification: resolution.verification.clone(),
        reason: format!(
            "provider source row remains release-blocking from provider burndown: {}",
            resolution.reason
        ),
        fixture_ids: resolution.fixture_ids.clone(),
        local_fixture_covered: false,
        external_authority_required,
        release_blocking: true,
    }
}

fn normalized_source_reference(source_reference: &str) -> String {
    source_reference.replace('\\', "/").to_ascii_lowercase()
}

fn is_runtime_config_source_reference(normalized_source_reference: &str) -> bool {
    normalized_source_reference.contains(":src/commands/config.ts")
        || normalized_source_reference.contains(":src/configtypes.ts")
        || normalized_source_reference.contains(":src/util/config/")
}

fn is_redteam_config_source_reference(normalized_source_reference: &str) -> bool {
    normalized_source_reference.contains(":src/redteam/plugins/policy/evals/promptfooconfig.yaml")
}

fn is_auxiliary_config_source_reference(normalized_source_reference: &str) -> bool {
    normalized_source_reference.contains(":src/codescan/config/")
        || normalized_source_reference
            .contains(":src/commands/mcp/tools/validatepromptfooconfig.ts")
}

fn auxiliary_config_owner(normalized_source_reference: &str) -> &'static str {
    if normalized_source_reference.contains(":src/codescan/config/") {
        "scan-engine"
    } else {
        "mcp-runtime"
    }
}

fn default_generated_accounting_row(item: &InventoryItem) -> SourceAccountingRow {
    let level = item.level_hint.clone();
    let (target_status, verification, reason_prefix) = match level.as_str() {
        "P0" => (
            "blocked".to_string(),
            format!("blocker:{}", item.stable_id),
            "generated P0 accounting row requires native fixture, bridge fixture, or explicit waiver",
        ),
        "P1" => (
            "later".to_string(),
            format!("snapshot:{}", item.stable_id),
            "generated P1 accounting row requires snapshot verification before parity claim",
        ),
        "P2" => (
            "later".to_string(),
            format!("registration:{}", item.stable_id),
            "generated P2 accounting row records known gap or follow-up target",
        ),
        _ => (
            "blocked".to_string(),
            format!("blocker:{}", item.stable_id),
            "generated accounting row has invalid level and requires manual classification",
        ),
    };
    let source_reason = item
        .unresolved_reason
        .as_deref()
        .unwrap_or("source-extracted item was not present in explicit item-level matrix");

    SourceAccountingRow {
        item_id: item.stable_id.clone(),
        category: item.category.clone(),
        source_reference: item.source_reference.clone(),
        level,
        target_status,
        owner: item.owner_hint.clone(),
        verification,
        reason: format!(
            "{reason_prefix}; reason: {source_reason}; source: {}",
            item.source_reference
        ),
        generated: true,
    }
}

fn classify_source_item(category: &str, file: &str) -> (String, String, String, Option<String>) {
    match category {
        "command" => (
            "P1".to_string(),
            "discovered".to_string(),
            "cli".to_string(),
            None,
        ),
        "provider" if is_p0_provider_file(file) => (
            "P0".to_string(),
            "discovered".to_string(),
            "provider-runtime".to_string(),
            None,
        ),
        "provider" => (
            "P2".to_string(),
            "unresolved".to_string(),
            "provider-runtime".to_string(),
            Some(
                "source-extracted long-tail provider requires task-17.4 classification".to_string(),
            ),
        ),
        "assertion" => (
            "P1".to_string(),
            "discovered".to_string(),
            "assertion-engine".to_string(),
            None,
        ),
        "redteam-plugin" | "redteam-strategy" => (
            "P1".to_string(),
            "discovered".to_string(),
            "redteam-engine".to_string(),
            None,
        ),
        "viewer" => (
            "P1".to_string(),
            "discovered".to_string(),
            "viewer".to_string(),
            None,
        ),
        "example" => (
            "P2".to_string(),
            "discovered".to_string(),
            "compatibility".to_string(),
            Some(
                "example inventory is evidence-only unless referenced by a P0/P1 fixture"
                    .to_string(),
            ),
        ),
        "output" => (
            "P1".to_string(),
            "discovered".to_string(),
            "reporting".to_string(),
            None,
        ),
        "config" => (
            "P0".to_string(),
            "discovered".to_string(),
            "config".to_string(),
            None,
        ),
        _ => (
            "P2".to_string(),
            "unresolved".to_string(),
            "compatibility".to_string(),
            Some("source-extracted item requires manual classification".to_string()),
        ),
    }
}

fn file_without_extension(file: &str) -> String {
    let Some((without_extension, _extension)) = file.rsplit_once('.') else {
        return file.to_string();
    };
    without_extension.to_string()
}

fn is_command_related_file(file: &str) -> bool {
    (file == "src/main.ts"
        || file.starts_with("src/commands/")
        || file.starts_with("src/redteam/commands/")
        || file.starts_with("src/codeScan/"))
        && is_ts_or_js_file(file)
}

fn is_provider_file(file: &str) -> bool {
    file.starts_with("src/providers/") && is_ts_or_js_file(file)
}

fn is_assertion_file(file: &str) -> bool {
    file.starts_with("src/assertions/") && is_ts_or_js_file(file)
}

fn is_redteam_plugin_file(file: &str) -> bool {
    file.starts_with("src/redteam/plugins/") && is_ts_or_js_file(file)
}

fn is_redteam_strategy_file(file: &str) -> bool {
    file.starts_with("src/redteam/strategies/") && is_ts_or_js_file(file)
}

fn is_viewer_file(file: &str) -> bool {
    file.starts_with("src/app/")
        || file.starts_with("src/server/")
        || file.starts_with("src/openapi/")
}

fn is_example_file(file: &str) -> bool {
    file.starts_with("examples/")
}

fn is_output_file(file: &str) -> bool {
    let lower = file.to_ascii_lowercase();
    file.starts_with("src/")
        && ["output", "report", "csv", "junit", "sarif", "yaml", "jsonl"]
            .iter()
            .any(|needle| lower.contains(needle))
}

fn is_config_file(file: &str) -> bool {
    file.starts_with("src/")
        && file.to_ascii_lowercase().contains("config")
        && !is_current_latest_viewer_config_file(file)
}

fn is_current_latest_viewer_config_file(file: &str) -> bool {
    file.starts_with("src/app/") && file.to_ascii_lowercase().contains("config")
}

fn is_p0_provider_file(file: &str) -> bool {
    [
        "src/providers/openai",
        "src/providers/http",
        "src/providers/ollama",
        "src/providers/anthropic",
    ]
    .iter()
    .any(|prefix| file.starts_with(prefix))
}

fn is_eval_runtime_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (matches!(
            file,
            "src/evaluate.ts" | "src/evaluator.ts" | "src/evaluatorHelpers.ts" | "src/testCase.ts"
        ) || file.starts_with("src/scheduler/")
            || file.starts_with("src/testCase/")
            || file.starts_with("src/optimizer/"))
}

fn is_cache_store_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file == "src/cache.ts"
            || file.starts_with("src/database/")
            || file.starts_with("src/storage/"))
}

fn is_prompt_processing_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file.starts_with("src/prompts/")
            || file.starts_with("src/external/prompts/")
            || file.starts_with("src/optimizer/"))
}

fn is_assertion_support_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file.starts_with("src/matchers/")
            || file.starts_with("src/external/matchers/")
            || file.starts_with("src/external/assertions/")
            || matches!(
                file,
                "src/remoteGrading.ts" | "src/remoteScoring.ts" | "src/guardrails.ts"
            ))
}

fn is_redteam_support_file(file: &str) -> bool {
    file.starts_with("src/redteam/") && is_ts_or_js_file(file)
}

fn is_schema_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file == "src/contracts.ts"
            || file.starts_with("src/types/")
            || file.starts_with("src/contracts/")
            || file.starts_with("src/models/")
            || file.starts_with("src/validators/"))
}

fn is_script_bridge_file(file: &str) -> bool {
    is_ts_or_js_file(file) && (file.starts_with("src/python/") || file.starts_with("src/ruby/"))
}

fn is_import_export_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file.starts_with("src/importers/") || file.starts_with("src/util/exportToFile/"))
}

fn is_integration_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file.starts_with("src/integrations/")
            || matches!(file, "src/googleSheets.ts" | "src/microsoftSharepoint.ts"))
}

fn is_cloud_share_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (matches!(
            file,
            "src/share.ts"
                | "src/feedback.ts"
                | "src/onboarding.ts"
                | "src/suggestions.ts"
                | "src/telemetry.ts"
                | "src/telemetryEvents.ts"
                | "src/updates.ts"
        ) || file.starts_with("src/updates/"))
}

fn is_blob_store_file(file: &str) -> bool {
    file.starts_with("src/blobs/") && is_ts_or_js_file(file)
}

fn is_observability_file(file: &str) -> bool {
    file.starts_with("src/tracing/") && is_ts_or_js_file(file)
}

fn is_runtime_support_file(file: &str) -> bool {
    is_ts_or_js_file(file)
        && (file.starts_with("src/util/")
            || file.starts_with("src/constants/")
            || file.starts_with("src/__mocks__/")
            || matches!(
                file,
                "src/cliState.ts"
                    | "src/constants.ts"
                    | "src/entrypoint.ts"
                    | "src/envars.ts"
                    | "src/envOverrides.ts"
                    | "src/esm.ts"
                    | "src/logger.ts"
                    | "src/logger.browser.ts"
                    | "src/mainUtils.ts"
                    | "src/migrate.ts"
                    | "src/table.ts"
                    | "src/version.ts"
            ))
}

fn is_ts_or_js_file(file: &str) -> bool {
    matches!(
        file.rsplit_once('.').map(|(_, extension)| extension),
        Some("ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs")
    )
}

fn json_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key)?.as_str().map(ToString::to_string)
}

fn nested_or_flat_string(
    value: &serde_json::Value,
    nested_path: &[&str],
    flat_key: &str,
) -> Option<String> {
    let mut nested = Some(value);
    for key in nested_path {
        nested = nested.and_then(|node| node.get(*key));
    }
    nested
        .and_then(|node| node.as_str())
        .map(ToString::to_string)
        .or_else(|| json_string(value, flat_key))
}

fn classify_github_release_channel(ref_name: Option<&str>) -> String {
    let Some(ref_name) = ref_name else {
        return "none".to_string();
    };
    let tag = ref_name.strip_prefix("refs/tags/").unwrap_or(ref_name);
    if tag.starts_with("code-scan-action-") {
        "github-action".to_string()
    } else if tag
        .chars()
        .next()
        .map(|ch| ch.is_ascii_digit())
        .unwrap_or(false)
    {
        "core-package".to_string()
    } else {
        "other".to_string()
    }
}

fn distribution_target_reason(
    npm: &NpmPackageObservation,
    github: &CurrentUpstreamObservation,
    frozen: &FrozenSourceReference,
    npm_matches_frozen: bool,
    repository_head_matches_npm_core: bool,
    latest_release_is_core_package: bool,
    latest_release_channel: &str,
) -> String {
    if repository_head_matches_npm_core && latest_release_is_core_package {
        return format!(
            "npm core package {}, repository HEAD, and GitHub latest core release share {}",
            npm.package_version, npm.git_head
        );
    }
    if npm_matches_frozen {
        let mut reason = format!(
            "npm core package {} matches frozen baseline {}, preserving frozen-baseline evidence for the published core package",
            npm.package_version, frozen.git_commit
        );
        if !repository_head_matches_npm_core {
            reason.push_str(&format!(
                "; repository HEAD {} differs from npm core gitHead {}",
                github.current_head, npm.git_head
            ));
        }
        if !latest_release_is_core_package {
            reason.push_str(&format!(
                "; GitHub latest observed release {:?} is classified as {}, not npm core package evidence",
                github.observed_release_ref, latest_release_channel
            ));
        }
        return reason;
    }
    format!(
        "npm core package {} ({}) differs from frozen baseline {}",
        npm.package_version, npm.git_head, frozen.git_commit
    )
}

fn reject_floating_completion_value(value: &str) -> Result<(), CurrentLatestTargetError> {
    let lower = value.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "latest" | "main" | "master" | "head" | "refs/heads/main" | "refs/heads/master"
    ) {
        return Err(CurrentLatestTargetError::Parse(format!(
            "floating current-latest completion proof is not allowed: {value}"
        )));
    }
    Ok(())
}

fn validate_safe_tag(tag: &str) -> Result<(), CurrentLatestTargetError> {
    if tag.trim() != tag
        || tag.is_empty()
        || tag.contains("..")
        || tag.chars().any(|ch| {
            ch.is_whitespace() || matches!(ch, '\0' | '^' | '~' | ':' | '?' | '*' | '[' | '\\')
        })
    {
        return Err(CurrentLatestTargetError::Parse(format!(
            "GitHub latest release tag is not a safe ref name: {tag}"
        )));
    }
    Ok(())
}

fn current_latest_target_reason(
    npm: &NpmPackageObservation,
    default_branch_head: &str,
    latest_release_ref: &str,
    latest_release_channel: &str,
) -> String {
    let mut parts = vec![format!(
        "npm latest package {} records gitHead {}",
        npm.package_version, npm.git_head
    )];
    if default_branch_head != npm.git_head {
        parts.push(format!(
            "GitHub default branch HEAD {default_branch_head} differs from npm latest gitHead {}",
            npm.git_head
        ));
    }
    if latest_release_channel != "core-package" {
        parts.push(format!(
            "GitHub latest release {latest_release_ref} is classified as {latest_release_channel}, not core package release evidence",
        ));
    }
    parts.push(
        "downstream source inventory, golden corpus, quality, external authority, and publication evidence are still required"
            .to_string(),
    );
    parts.join("; ")
}

fn current_latest_lock_markdown(lock: &CurrentLatestTargetLock) -> String {
    format!(
        "# Current Latest Target Lock\n\n\
        - **Schema**: `{}`\n\
        - **Status**: `{}`\n\
        - **Observed At**: `{}`\n\
        - **npm latest**: `promptfoo@{}` / `{}`\n\
        - **npm tarball**: `{}`\n\
        - **npm integrity**: `{}`\n\
        - **GitHub default branch HEAD**: `{}`\n\
        - **GitHub latest release**: `{}` / `{}` / channel `{}`\n\
        - **Target selection blocker resolved**: `{}`\n\
        - **Current latest claim allowed**: `{}`\n\n\
        ## Reason\n\n{}\n\n\
        ## Downstream Required Evidence\n\n{}\n",
        lock.schema,
        lock.status,
        lock.observed_at,
        lock.npm_latest.package_version,
        lock.npm_latest.git_head,
        lock.npm_latest.tarball,
        lock.npm_latest.integrity,
        lock.github.default_branch_head,
        lock.github.latest_release_ref,
        lock.github.latest_release_commit,
        lock.github.latest_release_channel,
        lock.target_selection_blocker_resolved,
        lock.current_latest_claim_allowed,
        lock.reason,
        lock.downstream_required_evidence
            .iter()
            .map(|item| format!("- `{item}`"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn validate_current_latest_lock_for_source_inventory(
    lock: &CurrentLatestTargetLock,
) -> Result<(), InventoryError> {
    if lock.schema != "promptfoo-rs.current-latest-target.v1" {
        return Err(InventoryError::Validation(format!(
            "unexpected current latest target schema: {}",
            lock.schema
        )));
    }
    for value in [
        lock.npm_latest.package_version.as_str(),
        lock.npm_latest.git_head.as_str(),
        lock.github.default_branch_head.as_str(),
        lock.github.npm_tag_commit.as_str(),
        lock.github.latest_release_commit.as_str(),
    ] {
        let lower = value.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "latest" | "main" | "master" | "head" | "refs/heads/main" | "refs/heads/master"
        ) {
            return Err(InventoryError::Validation(format!(
                "floating current latest source reference is not allowed: {value}"
            )));
        }
    }
    if !is_full_sha(&lock.npm_latest.git_head)
        || !is_full_sha(&lock.github.default_branch_head)
        || !is_full_sha(&lock.github.npm_tag_commit)
        || !is_full_sha(&lock.github.latest_release_commit)
    {
        return Err(InventoryError::Validation(
            "current latest lock requires full source SHAs".to_string(),
        ));
    }
    Ok(())
}

fn collect_current_latest_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<String>,
) -> Result<(), InventoryError> {
    for entry in fs::read_dir(current).map_err(InventoryError::Read)? {
        let entry = entry.map_err(InventoryError::Read)?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type().map_err(InventoryError::Read)?.is_dir() {
            if matches!(
                name.as_ref(),
                ".git" | "node_modules" | "target" | ".turbo" | ".next" | "dist" | "build"
            ) {
                continue;
            }
            collect_current_latest_files(root, &path, files)?;
            continue;
        }
        if !entry.file_type().map_err(InventoryError::Read)?.is_file() {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|error| InventoryError::Parse(error.to_string()))?;
        files.push(normalize_source_path(&relative.to_string_lossy()));
    }
    Ok(())
}

fn current_latest_file_categories(file: &str) -> Vec<&'static str> {
    let mut categories = Vec::new();
    if is_command_related_file(file) {
        categories.push("command");
    }
    if is_provider_file(file) {
        categories.push("provider");
    }
    if is_assertion_file(file) {
        categories.push("assertion");
    }
    if is_redteam_plugin_file(file) {
        categories.push("redteam-plugin");
    }
    if is_redteam_strategy_file(file) {
        categories.push("redteam-strategy");
    }
    if is_output_file(file) {
        categories.push("output");
    }
    if is_config_file(file) {
        categories.push("config");
    }
    if is_viewer_file(file) {
        categories.push("viewer");
    }
    if is_node_api_file(file) {
        categories.push("node-api");
    }
    if is_example_file(file) {
        categories.push("example");
    }
    if is_docs_file(file) {
        categories.push("docs");
    }
    if categories.is_empty() && is_eval_runtime_file(file) {
        categories.push("eval-runner");
    }
    if categories.is_empty() && is_cache_store_file(file) {
        categories.push("cache-store");
    }
    if categories.is_empty() && is_prompt_processing_file(file) {
        categories.push("prompt-processing");
    }
    if categories.is_empty() && is_assertion_support_file(file) {
        categories.push("assertion-support");
    }
    if categories.is_empty() && is_redteam_support_file(file) {
        categories.push("redteam-support");
    }
    if categories.is_empty() && is_schema_file(file) {
        categories.push("schema");
    }
    if categories.is_empty() && is_script_bridge_file(file) {
        categories.push("script-bridge");
    }
    if categories.is_empty() && is_import_export_file(file) {
        categories.push("import-export");
    }
    if categories.is_empty() && is_integration_file(file) {
        categories.push("integration");
    }
    if categories.is_empty() && is_cloud_share_file(file) {
        categories.push("cloud-share");
    }
    if categories.is_empty() && is_blob_store_file(file) {
        categories.push("blob-store");
    }
    if categories.is_empty() && is_observability_file(file) {
        categories.push("observability");
    }
    if categories.is_empty() && is_runtime_support_file(file) {
        categories.push("runtime-support");
    }
    if categories.is_empty() && file.starts_with("src/") && is_ts_or_js_file(file) {
        categories.push("unclassified");
    }
    categories
}

fn increment_current_latest_count(counts: &mut SourceInventoryCounts, category: &str) {
    match category {
        "command" => counts.command_related_files += 1,
        "provider" => counts.provider_files += 1,
        "assertion" => counts.assertion_files += 1,
        "redteam-plugin" => counts.redteam_plugin_files += 1,
        "redteam-strategy" => counts.redteam_strategy_files += 1,
        "output" => counts.output_files += 1,
        "config" => counts.config_files += 1,
        "viewer" => counts.viewer_app_files += 1,
        "example" => counts.example_files += 1,
        _ => {}
    }
}

fn insert_current_latest_file_row(
    rows: &mut BTreeMap<String, CurrentLatestInventoryRow>,
    lock: &CurrentLatestTargetLock,
    category: &str,
    file: &str,
) {
    let name = slug(&file_without_extension(file));
    let stable_id = InventoryItem::stable_id(category, &name);
    rows.entry(stable_id.clone()).or_insert_with(|| {
        let (level, implementation_status, verification_owner, evidence_kind, reason) =
            current_latest_default_metadata(category, &stable_id, file);
        CurrentLatestInventoryRow {
            stable_id: stable_id.clone(),
            category: category.to_string(),
            name,
            source_reference: current_latest_source_reference(lock, file, None),
            source_file: file.to_string(),
            level,
            implementation_status,
            verification_owner,
            evidence_kind,
            evidence_reference: default_evidence_reference(category, &stable_id),
            blocker_reason: reason,
        }
    });
}

fn insert_current_latest_flag_row(
    rows: &mut BTreeMap<String, CurrentLatestInventoryRow>,
    lock: &CurrentLatestTargetLock,
    file: &str,
    flag: &str,
) {
    let name = slug(flag);
    let stable_id = InventoryItem::stable_id("flag", &name);
    rows.entry(stable_id.clone())
        .or_insert_with(|| CurrentLatestInventoryRow {
            stable_id: stable_id.clone(),
            category: "flag".to_string(),
            name,
            source_reference: current_latest_source_reference(
                lock,
                file,
                Some(&format!("--{flag}")),
            ),
            source_file: file.to_string(),
            level: "P1".to_string(),
            implementation_status: "later".to_string(),
            verification_owner: "cli".to_string(),
            evidence_kind: "snapshot".to_string(),
            evidence_reference: format!("snapshot:{stable_id}"),
            blocker_reason: Some(format!(
                "current-latest flag --{flag} requires CLI parity snapshot or fixture evidence"
            )),
        });
}

fn current_latest_default_metadata(
    category: &str,
    stable_id: &str,
    file: &str,
) -> (String, String, String, String, Option<String>) {
    match category {
        "command" => current_latest_metadata(
            "P1",
            "later",
            "cli",
            "snapshot",
            stable_id,
            "current-latest command requires CLI behavior snapshot or fixture evidence",
        ),
        "provider" if is_p0_provider_file(file) => current_latest_metadata(
            "P0",
            "blocked",
            "provider-runtime",
            "blocker",
            stable_id,
            "current-latest P0 provider requires native or bridge fixture evidence",
        ),
        "provider" => current_latest_metadata(
            "P2",
            "later",
            "provider-runtime",
            "registration",
            stable_id,
            "current-latest long-tail provider is registered until fixture evidence promotes it",
        ),
        "assertion" => current_latest_metadata(
            "P1",
            "later",
            "assertion-engine",
            "snapshot",
            stable_id,
            "current-latest assertion requires snapshot evidence",
        ),
        "redteam-plugin" | "redteam-strategy" => current_latest_metadata(
            "P1",
            "later",
            "redteam-engine",
            "snapshot",
            stable_id,
            "current-latest redteam surface requires snapshot evidence",
        ),
        "output" => current_latest_metadata(
            "P1",
            "later",
            "reporting",
            "snapshot",
            stable_id,
            "current-latest output surface requires output contract snapshot",
        ),
        "config" => current_latest_metadata(
            "P0",
            "blocked",
            "config-loader",
            "blocker",
            stable_id,
            "current-latest config surface requires fixture evidence",
        ),
        "eval-runner" => current_latest_metadata(
            "P0",
            "blocked",
            "eval-runner",
            "blocker",
            stable_id,
            "current-latest eval runtime requires fixture evidence",
        ),
        "cache-store" => current_latest_metadata(
            "P0",
            "blocked",
            "cache-resume-store",
            "blocker",
            stable_id,
            "current-latest cache and result store surface requires fixture evidence",
        ),
        "prompt-processing" => current_latest_metadata(
            "P0",
            "blocked",
            "config-loader",
            "blocker",
            stable_id,
            "current-latest prompt processing surface requires fixture evidence",
        ),
        "script-bridge" => current_latest_metadata(
            "P0",
            "blocked",
            "script-bridge",
            "blocker",
            stable_id,
            "current-latest script bridge surface requires authorized subprocess fixture evidence",
        ),
        "viewer" => current_latest_metadata(
            "P1",
            "later",
            "web-viewer",
            "snapshot",
            stable_id,
            "current-latest viewer surface requires data-contract or browser snapshot",
        ),
        "assertion-support" => current_latest_metadata(
            "P1",
            "later",
            "assertion-engine",
            "snapshot",
            stable_id,
            "current-latest assertion support surface requires matcher or grading snapshot evidence",
        ),
        "redteam-support" => current_latest_metadata(
            "P1",
            "later",
            "redteam-engine",
            "snapshot",
            stable_id,
            "current-latest redteam support surface requires registry or behavior snapshot evidence",
        ),
        "schema" => current_latest_metadata(
            "P1",
            "later",
            "protocol",
            "snapshot",
            stable_id,
            "current-latest schema/model/contract surface requires protocol snapshot evidence",
        ),
        "import-export" => current_latest_metadata(
            "P1",
            "later",
            "output-writers",
            "snapshot",
            stable_id,
            "current-latest import/export surface requires conversion snapshot evidence",
        ),
        "blob-store" => current_latest_metadata(
            "P1",
            "later",
            "eval-runner",
            "snapshot",
            stable_id,
            "current-latest blob and media storage surface requires data-contract snapshot evidence",
        ),
        "runtime-support" => current_latest_metadata(
            "P1",
            "later",
            "runtime",
            "snapshot",
            stable_id,
            "current-latest runtime support surface requires deterministic snapshot evidence",
        ),
        "observability" => current_latest_metadata(
            "P1",
            "later",
            "observability",
            "snapshot",
            stable_id,
            "current-latest tracing and observability surface requires telemetry snapshot evidence",
        ),
        "node-api" => current_latest_metadata(
            "P1",
            "later",
            "node-api-wrapper",
            "snapshot",
            stable_id,
            "current-latest Node API surface requires wrapper contract snapshot",
        ),
        "example" => current_latest_metadata(
            "P2",
            "later",
            "compatibility",
            "registration",
            stable_id,
            "current-latest example is registered unless promoted into P0/P1 corpus",
        ),
        "docs" => current_latest_metadata(
            "P2",
            "later",
            "compatibility",
            "registration",
            stable_id,
            "current-latest documented workflow is registered until mapped to executable evidence",
        ),
        "integration" => current_latest_metadata(
            "P2",
            "later",
            "compatibility",
            "registration",
            stable_id,
            "current-latest external integration is registered until promoted with fixture or authority evidence",
        ),
        "cloud-share" => current_latest_metadata(
            "P2",
            "unsupported",
            "compatibility",
            "registration",
            stable_id,
            "current-latest cloud/share surface remains local-first unsupported unless legal brand and service authority are provided",
        ),
        _ => current_latest_metadata(
            "P0",
            "blocked",
            "compatibility",
            "blocker",
            stable_id,
            "current-latest source row is unclassified and must be mapped before any perfect-refactor claim",
        ),
    }
}

fn current_latest_metadata(
    level: &str,
    implementation_status: &str,
    owner: &str,
    evidence_kind: &str,
    stable_id: &str,
    reason: &str,
) -> (String, String, String, String, Option<String>) {
    (
        level.to_string(),
        implementation_status.to_string(),
        owner.to_string(),
        evidence_kind.to_string(),
        Some(format!("{reason}; item: {stable_id}")),
    )
}

fn default_evidence_reference(category: &str, stable_id: &str) -> String {
    match category {
        "provider" | "config" | "eval-runner" | "cache-store" | "prompt-processing"
        | "script-bridge" | "unclassified" => format!("blocker:{stable_id}"),
        "example" | "docs" | "integration" | "cloud-share" => {
            format!("registration:{stable_id}")
        }
        _ => format!("snapshot:{stable_id}"),
    }
}

fn current_latest_matrix_row_from_inventory(
    row: &CurrentLatestInventoryRow,
) -> CurrentLatestMatrixRow {
    CurrentLatestMatrixRow {
        item_id: row.stable_id.clone(),
        category: row.category.clone(),
        source_reference: row.source_reference.clone(),
        level: row.level.clone(),
        implementation_status: row.implementation_status.clone(),
        verification_owner: row.verification_owner.clone(),
        evidence_kind: row.evidence_kind.clone(),
        evidence_reference: row.evidence_reference.clone(),
        blocker_reason: row.blocker_reason.clone(),
    }
}

fn current_latest_matrix_row_from_explicit(
    inventory_row: &CurrentLatestInventoryRow,
    matrix_row: &super::matrix::CapabilityRow,
) -> CurrentLatestMatrixRow {
    CurrentLatestMatrixRow {
        item_id: inventory_row.stable_id.clone(),
        category: inventory_row.category.clone(),
        source_reference: inventory_row.source_reference.clone(),
        level: matrix_row.level.clone(),
        implementation_status: matrix_row.target_status.clone(),
        verification_owner: matrix_row.owner.clone(),
        evidence_kind: matrix_evidence_kind(&matrix_row.verification),
        evidence_reference: matrix_row.verification.clone(),
        blocker_reason: if matrix_row.target_status == "blocked"
            || matrix_row.verification.starts_with("blocker:")
        {
            Some(matrix_row.notes.clone())
        } else {
            None
        },
    }
}

fn matrix_evidence_kind(verification: &str) -> String {
    verification
        .split_once(':')
        .map(|(prefix, _)| prefix.to_string())
        .filter(|prefix| !prefix.is_empty())
        .unwrap_or_else(|| "manual".to_string())
}

fn current_latest_categories(rows: &[CurrentLatestInventoryRow]) -> Vec<String> {
    rows.iter()
        .map(|row| row.category.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn current_latest_unclassified_rows(rows: &[CurrentLatestInventoryRow]) -> Vec<String> {
    rows.iter()
        .filter(|row| row.category == "unclassified")
        .map(|row| row.stable_id.clone())
        .collect()
}

fn current_latest_rows_missing_evidence(rows: &[CurrentLatestInventoryRow]) -> Vec<String> {
    rows.iter()
        .filter(|row| {
            row.evidence_kind.trim().is_empty() || row.evidence_reference.trim().is_empty()
        })
        .map(|row| row.stable_id.clone())
        .collect()
}

fn current_latest_source_reference(
    lock: &CurrentLatestTargetLock,
    file: &str,
    fragment: Option<&str>,
) -> String {
    let mut reference = format!(
        "promptfoo@current-latest:{}:{file}",
        lock.github.default_branch_head
    );
    if let Some(fragment) = fragment {
        reference.push('#');
        reference.push_str(fragment);
    }
    reference
}

fn extract_flag_tokens(content: &str) -> BTreeSet<String> {
    let bytes = content.as_bytes();
    let mut flags = BTreeSet::new();
    let mut index = 0;
    while index + 2 <= bytes.len() {
        if bytes[index] != b'-' || bytes.get(index + 1) != Some(&b'-') {
            index += 1;
            continue;
        }
        let start = index + 2;
        let mut end = start;
        while end < bytes.len()
            && (bytes[end].is_ascii_alphanumeric() || matches!(bytes[end], b'-' | b'_'))
        {
            end += 1;
        }
        if end > start {
            flags.insert(String::from_utf8_lossy(&bytes[start..end]).to_string());
        }
        index = end.max(index + 1);
    }
    flags
}

fn is_node_api_file(file: &str) -> bool {
    (file == "src/index.ts"
        || file == "src/index.js"
        || file.starts_with("src/node/")
        || file.starts_with("npm/src/")
        || file.starts_with("packages/node/"))
        && is_ts_or_js_file(file)
}

fn is_docs_file(file: &str) -> bool {
    let lower = file.to_ascii_lowercase();
    file.starts_with("docs/") && (lower.ends_with(".md") || lower.ends_with(".mdx"))
}

fn required_current_evidence() -> Vec<String> {
    [
        "source_inventory",
        "matrix",
        "fixtures",
        "golden_corpus",
        "release_candidate",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn is_full_sha(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn current_unix_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| format!("unix:{}", duration.as_secs()))
        .unwrap_or_else(|_| "unix:0".to_string())
}
