use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::matrix::CapabilityMatrix;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureManifest {
    pub id: String,
    pub test_id: String,
    pub matrix_item_ids: Vec<String>,
    pub priority: Priority,
    pub provider_mocking: ProviderMocking,
    pub required_env: Vec<String>,
    pub expected_outputs: Vec<String>,
    pub normalization_rules: Vec<String>,
    pub blocks_stable_release: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    P0,
    P1,
    P2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderMocking {
    Mock,
    Recorded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureCorpusReport {
    pub tracked_fixture_count: usize,
    pub fixtures: Vec<FixtureRecord>,
    pub invalid_fixtures: Vec<String>,
    pub matrix_links_missing: Vec<String>,
    pub fixtures_requiring_real_secrets: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixtureRecord {
    pub path: PathBuf,
    pub manifest: FixtureManifest,
}

#[derive(Debug)]
pub enum FixtureError {
    Read(std::io::Error),
    Parse(String),
}

pub fn load_fixture_manifest(_path: &Path) -> Result<FixtureManifest, FixtureError> {
    let yaml = fs::read_to_string(_path).map_err(FixtureError::Read)?;
    serde_yaml::from_str(&yaml).map_err(|error| FixtureError::Parse(error.to_string()))
}

pub fn validate_p0_fixture_corpus(root: &Path, matrix: &CapabilityMatrix) -> FixtureCorpusReport {
    let matrix_ids: std::collections::BTreeSet<_> = matrix
        .rows
        .iter()
        .map(|row| row.capability.as_str())
        .collect();
    let mut report = FixtureCorpusReport {
        tracked_fixture_count: 0,
        fixtures: Vec::new(),
        invalid_fixtures: Vec::new(),
        matrix_links_missing: Vec::new(),
        fixtures_requiring_real_secrets: Vec::new(),
    };

    let mut paths = Vec::new();
    collect_fixture_paths(root, &mut paths);
    paths.sort();

    for path in paths {
        match load_fixture_manifest(&path) {
            Ok(manifest) => {
                report.tracked_fixture_count += 1;
                validate_manifest(&path, &manifest, &matrix_ids, &mut report);
                report.fixtures.push(FixtureRecord { path, manifest });
            }
            Err(error) => report
                .invalid_fixtures
                .push(format!("{}: {error:?}", path.display())),
        }
    }

    report
}

pub fn fixture_count_by_priority(report: &FixtureCorpusReport, priority: Priority) -> usize {
    report
        .fixtures
        .iter()
        .filter(|record| record.manifest.priority == priority)
        .count()
}

fn _read_fixture_yaml(path: &Path) -> Result<String, FixtureError> {
    fs::read_to_string(path).map_err(FixtureError::Read)
}

fn collect_fixture_paths(root: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_paths(&path, paths);
        } else if path.file_name().and_then(|name| name.to_str()) == Some("fixture.yaml") {
            paths.push(path);
        }
    }
}

fn validate_manifest(
    path: &Path,
    manifest: &FixtureManifest,
    matrix_ids: &std::collections::BTreeSet<&str>,
    report: &mut FixtureCorpusReport,
) {
    let display = path.display().to_string();
    if manifest.id.trim().is_empty()
        || manifest.test_id.trim().is_empty()
        || manifest.matrix_item_ids.is_empty()
        || manifest.expected_outputs.is_empty()
        || manifest.normalization_rules.is_empty()
        || !manifest.blocks_stable_release
    {
        report.invalid_fixtures.push(display.clone());
    }
    for matrix_id in &manifest.matrix_item_ids {
        if !matrix_ids.contains(matrix_id.as_str()) {
            report
                .matrix_links_missing
                .push(format!("{} -> {}", manifest.id, matrix_id));
        }
    }
    for env in &manifest.required_env {
        let upper = env.to_ascii_uppercase();
        if upper.contains("KEY") || upper.contains("TOKEN") || upper.contains("SECRET") {
            report
                .fixtures_requiring_real_secrets
                .push(format!("{} -> {}", manifest.id, env));
        }
    }
}
