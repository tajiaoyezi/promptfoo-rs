use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnvOverlay {
    values: BTreeMap<String, String>,
}

impl EnvOverlay {
    pub fn from_dotenv(path: &Path) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path).map_err(ConfigError::Read)?;
        let mut values = BTreeMap::new();
        for line in contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                values.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(Self { values })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedConfig {
    pub description: Option<String>,
    pub providers: Vec<NormalizedProvider>,
    pub prompts: Vec<NormalizedPrompt>,
    pub tests: Vec<NormalizedTestCase>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NormalizedProvider {
    pub id: String,
    pub config: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalizedPrompt {
    pub source: Option<String>,
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalizedTestCase {
    pub vars: BTreeMap<String, String>,
    pub assertions: Vec<NormalizedAssertion>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalizedAssertion {
    pub assertion_type: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityFinding {
    pub path: String,
    pub class: DiffClass,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiffClass {
    Unsupported,
    Later,
    UpstreamAmbiguous,
    Bug,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigDiff {
    path: String,
    class: DiffClass,
    message: String,
}

impl ConfigDiff {
    pub fn unsupported(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            class: DiffClass::Unsupported,
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Read(std::io::Error),
    Parse(serde_yaml::Error),
    UnsupportedPrompt(Value),
}

pub fn load_promptfoo_config(
    path: &Path,
    env: &EnvOverlay,
) -> Result<NormalizedConfig, ConfigError> {
    let contents = fs::read_to_string(path).map_err(ConfigError::Read)?;
    let raw: RawConfig = serde_yaml::from_str(&contents).map_err(ConfigError::Parse)?;
    normalize_config(raw, path.parent().unwrap_or_else(|| Path::new(".")), env)
}

pub fn record_config_diff(diff: ConfigDiff) -> CompatibilityFinding {
    CompatibilityFinding {
        path: diff.path,
        class: diff.class,
        message: diff.message,
    }
}

fn normalize_config(
    raw: RawConfig,
    base_dir: &Path,
    env: &EnvOverlay,
) -> Result<NormalizedConfig, ConfigError> {
    Ok(NormalizedConfig {
        description: raw.description,
        providers: raw
            .providers
            .into_iter()
            .map(|provider| NormalizedProvider {
                id: provider.id,
                config: provider.config.unwrap_or(Value::Null),
            })
            .collect(),
        prompts: raw
            .prompts
            .into_iter()
            .map(|prompt| normalize_prompt(prompt, base_dir, env))
            .collect::<Result<Vec<_>, _>>()?,
        tests: raw
            .tests
            .into_iter()
            .map(|test| NormalizedTestCase {
                vars: stringify_map(test.vars),
                assertions: test
                    .assertions
                    .into_iter()
                    .map(|assertion| NormalizedAssertion {
                        assertion_type: assertion.assertion_type,
                        value: assertion.value.map(stringify_value),
                    })
                    .collect(),
            })
            .collect(),
    })
}

fn normalize_prompt(
    raw: RawPrompt,
    base_dir: &Path,
    env: &EnvOverlay,
) -> Result<NormalizedPrompt, ConfigError> {
    match raw.value {
        Value::String(value) if value.starts_with("file://") => {
            let relative = value.trim_start_matches("file://");
            let body = fs::read_to_string(resolve_fixture_path(base_dir, relative))
                .map_err(ConfigError::Read)?;
            Ok(NormalizedPrompt {
                source: Some(value),
                body: substitute_env(&body, env),
            })
        }
        Value::String(value) => Ok(NormalizedPrompt {
            source: None,
            body: substitute_env(&value, env),
        }),
        value => Err(ConfigError::UnsupportedPrompt(value)),
    }
}

fn resolve_fixture_path(base_dir: &Path, relative: &str) -> PathBuf {
    let path = Path::new(relative);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}

fn substitute_env(input: &str, env: &EnvOverlay) -> String {
    let mut output = input.to_string();
    for (key, value) in &env.values {
        output = output.replace(&format!("${{{key}}}"), value);
    }
    output
}

fn stringify_map(values: BTreeMap<String, Value>) -> BTreeMap<String, String> {
    values
        .into_iter()
        .map(|(key, value)| (key, stringify_value(value)))
        .collect()
}

fn stringify_value(value: Value) -> String {
    match value {
        Value::String(value) => value,
        other => other.to_string(),
    }
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    providers: Vec<RawProvider>,
    #[serde(default)]
    prompts: Vec<RawPrompt>,
    #[serde(default)]
    tests: Vec<RawTestCase>,
}

#[derive(Debug, Deserialize)]
struct RawProvider {
    id: String,
    #[serde(default)]
    config: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct RawPrompt {
    value: Value,
}

#[derive(Debug, Deserialize)]
struct RawTestCase {
    #[serde(default)]
    vars: BTreeMap<String, Value>,
    #[serde(default, rename = "assert")]
    assertions: Vec<RawAssertion>,
}

#[derive(Debug, Deserialize)]
struct RawAssertion {
    #[serde(rename = "type")]
    assertion_type: String,
    #[serde(default)]
    value: Option<Value>,
}
