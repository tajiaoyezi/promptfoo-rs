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
    unimplemented!("task-12.1 RED skeleton: fixture manifest loader is not implemented")
}

pub fn validate_p0_fixture_corpus(
    _root: &Path,
    _matrix: &CapabilityMatrix,
) -> FixtureCorpusReport {
    unimplemented!("task-12.1 RED skeleton: P0 fixture corpus validator is not implemented")
}

pub fn fixture_count_by_priority(_report: &FixtureCorpusReport, _priority: Priority) -> usize {
    unimplemented!("task-12.1 RED skeleton: fixture priority counter is not implemented")
}

fn _read_fixture_yaml(path: &Path) -> Result<String, FixtureError> {
    fs::read_to_string(path).map_err(FixtureError::Read)
}
