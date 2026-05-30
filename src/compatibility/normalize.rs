use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::compatibility::harness::Artifact;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizationRules {
    pub timestamp: bool,
    pub path: bool,
    pub random_id: bool,
    pub latency: bool,
}

impl NormalizationRules {
    pub fn default_promptfoo_0_121_13() -> Self {
        Self {
            timestamp: true,
            path: true,
            random_id: true,
            latency: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalizedArtifact {
    pub fixture_name: String,
    pub payload: Value,
    pub applied_rules: Vec<String>,
}

pub fn normalize_artifact(artifact: &Artifact, rules: &NormalizationRules) -> NormalizedArtifact {
    let mut applied = BTreeSet::new();
    let payload = normalize_value(None, &artifact.payload, rules, &mut applied);
    NormalizedArtifact {
        fixture_name: artifact.fixture_name.clone(),
        payload,
        applied_rules: applied.into_iter().collect(),
    }
}

fn normalize_value(
    key: Option<&str>,
    value: &Value,
    rules: &NormalizationRules,
    applied: &mut BTreeSet<String>,
) -> Value {
    if rules.latency && key.map_or(false, is_latency_key) {
        applied.insert("latency".to_string());
        return Value::String("<normalized-latency>".to_string());
    }

    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(child_key, child_value)| {
                    (
                        child_key.clone(),
                        normalize_value(Some(child_key), child_value, rules, applied),
                    )
                })
                .collect::<Map<_, _>>(),
        ),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| normalize_value(key, item, rules, applied))
                .collect(),
        ),
        Value::String(text) => normalize_string(key, text, rules, applied),
        _ => value.clone(),
    }
}

fn normalize_string(
    key: Option<&str>,
    text: &str,
    rules: &NormalizationRules,
    applied: &mut BTreeSet<String>,
) -> Value {
    if rules.timestamp && (key.map_or(false, is_timestamp_key) || looks_like_timestamp(text)) {
        applied.insert("timestamp".to_string());
        return Value::String("<normalized-timestamp>".to_string());
    }
    if rules.path && (key.map_or(false, is_path_key) || looks_like_path(text)) {
        applied.insert("path".to_string());
        return Value::String("<normalized-path>".to_string());
    }
    if rules.random_id && key.map_or(false, is_random_id_key) && looks_like_random_id(text) {
        applied.insert("random-id".to_string());
        return Value::String("<normalized-random-id>".to_string());
    }
    Value::String(text.to_string())
}

fn is_timestamp_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key == "timestamp" || key.ends_with("_at") || key.ends_with("time")
}

fn looks_like_timestamp(value: &str) -> bool {
    value.len() >= 20 && value.contains('T') && value.ends_with('Z')
}

fn is_path_key(key: &str) -> bool {
    key.to_ascii_lowercase().contains("path")
}

fn looks_like_path(value: &str) -> bool {
    value.contains(":\\") || value.contains("\\\\") || value.starts_with("/tmp/")
}

fn is_random_id_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key == "id" || key.ends_with("_id") || key.ends_with("id")
}

fn looks_like_random_id(value: &str) -> bool {
    value.starts_with("run_") || value.len() >= 20
}

fn is_latency_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key.contains("latency") || key.contains("duration")
}
