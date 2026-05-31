use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::matrix::CapabilityMatrix;

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
    file.starts_with("src/") && file.to_ascii_lowercase().contains("config")
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

fn is_ts_or_js_file(file: &str) -> bool {
    matches!(
        file.rsplit_once('.').map(|(_, extension)| extension),
        Some("ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs")
    )
}

fn current_unix_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| format!("unix:{}", duration.as_secs()))
        .unwrap_or_else(|_| "unix:0".to_string())
}
