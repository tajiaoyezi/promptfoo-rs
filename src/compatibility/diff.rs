use serde::{Deserialize, Serialize};

use crate::compatibility::normalize::NormalizedArtifact;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffClass {
    Matching,
    IntentionalDifference,
    Unsupported,
    Later,
    UpstreamAmbiguous,
    Bug,
    Unclassified,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFinding {
    pub capability: String,
    pub path: String,
    pub class: DiffClass,
    pub message: String,
}

impl DiffFinding {
    pub fn bug(
        capability: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(capability, path, DiffClass::Bug, message)
    }

    pub fn unclassified(
        capability: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(capability, path, DiffClass::Unclassified, message)
    }

    pub fn intentional_difference(
        capability: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(capability, path, DiffClass::IntentionalDifference, message)
    }

    pub fn unsupported(
        capability: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(capability, path, DiffClass::Unsupported, message)
    }

    pub fn new(
        capability: impl Into<String>,
        path: impl Into<String>,
        class: DiffClass,
        message: impl Into<String>,
    ) -> Self {
        Self {
            capability: capability.into(),
            path: path.into(),
            class,
            message: message.into(),
        }
    }
}

pub fn classify_diff(upstream: &NormalizedArtifact, rs: &NormalizedArtifact) -> Vec<DiffFinding> {
    if upstream.payload == rs.payload {
        return vec![DiffFinding::new(
            upstream.fixture_name.clone(),
            "$",
            DiffClass::Matching,
            "normalized artifacts match",
        )];
    }

    let class = rs
        .payload
        .pointer("/compatibility/classification")
        .and_then(|value| value.as_str())
        .and_then(parse_classification)
        .unwrap_or(DiffClass::Bug);
    let message = rs
        .payload
        .pointer("/compatibility/reason")
        .and_then(|value| value.as_str())
        .unwrap_or("normalized artifacts differ");

    vec![DiffFinding::new(
        upstream.fixture_name.clone(),
        "$",
        class,
        message,
    )]
}

fn parse_classification(value: &str) -> Option<DiffClass> {
    match value {
        "matching" => Some(DiffClass::Matching),
        "intentional-difference" => Some(DiffClass::IntentionalDifference),
        "unsupported" => Some(DiffClass::Unsupported),
        "later" => Some(DiffClass::Later),
        "upstream-ambiguous" => Some(DiffClass::UpstreamAmbiguous),
        "bug" => Some(DiffClass::Bug),
        "unclassified" => Some(DiffClass::Unclassified),
        _ => None,
    }
}
