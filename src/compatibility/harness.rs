use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::compatibility::normalize::NormalizationRules;

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
}

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HarnessError {}
