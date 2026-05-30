pub mod anthropic;
pub mod http;
pub mod ollama;
pub mod openai;

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde_json::{json, Map, Value};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderKind {
    OpenAiCompatible,
    Http,
    Ollama,
    Anthropic,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderConfig {
    pub id: String,
    pub kind: ProviderKind,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub headers: BTreeMap<String, String>,
    pub env: BTreeMap<String, String>,
    pub options: Map<String, Value>,
}

impl ProviderConfig {
    pub fn openai_compatible(id: impl Into<String>) -> Self {
        Self::new(id, ProviderKind::OpenAiCompatible)
    }

    pub fn http(id: impl Into<String>) -> Self {
        Self::new(id, ProviderKind::Http)
    }

    pub fn ollama(id: impl Into<String>) -> Self {
        Self::new(id, ProviderKind::Ollama)
    }

    pub fn anthropic(id: impl Into<String>) -> Self {
        Self::new(id, ProviderKind::Anthropic)
    }

    fn new(id: impl Into<String>, kind: ProviderKind) -> Self {
        Self {
            id: id.into(),
            kind,
            base_url: None,
            model: None,
            headers: BTreeMap::new(),
            env: BTreeMap::new(),
            options: Map::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    pub fn with_env(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(name.into(), value.into());
        self
    }

    pub fn with_option(mut self, name: impl Into<String>, value: Value) -> Self {
        self.options.insert(name.into(), value);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderInput {
    pub prompt: String,
}

impl ProviderInput {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderRequestSnapshot {
    pub provider_id: String,
    pub kind: ProviderKind,
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub body: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderSnapshot {
    pub request: ProviderRequestSnapshot,
    pub response_output_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderResponseSnapshot {
    pub provider_id: String,
    pub output: String,
    pub raw: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderError {
    message: String,
}

impl ProviderError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ProviderError {}

impl From<reqwest::Error> for ProviderError {
    fn from(value: reqwest::Error) -> Self {
        Self::new(value.to_string())
    }
}

pub trait Provider: Send + Sync {
    fn id(&self) -> &str;
    fn kind(&self) -> ProviderKind;
}

struct ProviderDescriptor {
    id: String,
    kind: ProviderKind,
}

impl Provider for ProviderDescriptor {
    fn id(&self) -> &str {
        &self.id
    }

    fn kind(&self) -> ProviderKind {
        self.kind
    }
}

pub struct ProviderRegistry {
    providers: BTreeMap<String, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn register_p0_defaults() -> Self {
        let mut registry = Self {
            providers: BTreeMap::new(),
        };
        registry.register("anthropic", ProviderKind::Anthropic);
        registry.register("http", ProviderKind::Http);
        registry.register("ollama", ProviderKind::Ollama);
        registry.register("openai-compatible", ProviderKind::OpenAiCompatible);
        registry
    }

    fn register(&mut self, id: impl Into<String>, kind: ProviderKind) {
        let id = id.into();
        self.providers.insert(
            id.clone(),
            Arc::new(ProviderDescriptor { id, kind }) as Arc<dyn Provider>,
        );
    }

    pub fn provider_ids(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }

    pub fn resolve(&self, id: &ProviderId) -> Result<Arc<dyn Provider>, ProviderError> {
        self.providers
            .get(id.as_str())
            .cloned()
            .ok_or_else(|| ProviderError::new(format!("unknown provider: {}", id.as_str())))
    }

    pub fn snapshot(
        &self,
        config: &ProviderConfig,
        input: ProviderInput,
    ) -> Result<ProviderSnapshot, ProviderError> {
        self.resolve(&ProviderId::new(config.id.clone()))?;
        let request = normalize_provider_request(config.clone(), input)?;
        Ok(ProviderSnapshot {
            response_output_path: response_output_path(config.kind).to_string(),
            request,
        })
    }

    pub async fn call(
        &self,
        config: &ProviderConfig,
        input: ProviderInput,
    ) -> Result<ProviderResponseSnapshot, ProviderError> {
        self.resolve(&ProviderId::new(config.id.clone()))?;
        let request = normalize_provider_request(config.clone(), input)?;
        tracing::debug!(
            provider_id = %request.provider_id,
            url = %request.url,
            "calling provider endpoint"
        );

        let client = reqwest::Client::new();
        let mut builder = client.post(&request.url);
        for (name, value) in &request.headers {
            builder = builder.header(name, value);
        }
        let raw = builder
            .json(&request.body)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        Ok(ProviderResponseSnapshot {
            provider_id: request.provider_id,
            output: extract_output(request.kind, &raw)?,
            raw,
        })
    }
}

pub fn normalize_provider_request(
    config: ProviderConfig,
    input: ProviderInput,
) -> Result<ProviderRequestSnapshot, ProviderError> {
    let mut headers = normalize_headers(&config);
    headers.insert("content-type".to_string(), "application/json".to_string());

    let (url, body) = match config.kind {
        ProviderKind::OpenAiCompatible => {
            if let Some(api_key) = config.env.get("OPENAI_API_KEY") {
                headers.insert("authorization".to_string(), format!("Bearer {api_key}"));
            }
            let mut body = Map::new();
            body.insert(
                "model".to_string(),
                Value::String(resolve_model(
                    &config,
                    "OPENAI_MODEL",
                    openai::DEFAULT_MODEL,
                )),
            );
            body.insert(
                "messages".to_string(),
                json!([{ "role": "user", "content": input.prompt }]),
            );
            merge_options(&mut body, &config.options);
            (
                config
                    .base_url
                    .unwrap_or_else(|| openai::DEFAULT_URL.to_string()),
                Value::Object(body),
            )
        }
        ProviderKind::Http => {
            let url = config
                .base_url
                .ok_or_else(|| ProviderError::new("HTTP provider requires base_url"))?;
            let mut body = Map::new();
            body.insert("prompt".to_string(), Value::String(input.prompt));
            merge_options(&mut body, &config.options);
            (url, Value::Object(body))
        }
        ProviderKind::Ollama => {
            let mut body = Map::new();
            body.insert(
                "model".to_string(),
                Value::String(resolve_model(
                    &config,
                    "OLLAMA_MODEL",
                    ollama::DEFAULT_MODEL,
                )),
            );
            body.insert("prompt".to_string(), Value::String(input.prompt));
            body.insert("stream".to_string(), Value::Bool(false));
            merge_options(&mut body, &config.options);
            (
                config
                    .base_url
                    .unwrap_or_else(|| ollama::DEFAULT_URL.to_string()),
                Value::Object(body),
            )
        }
        ProviderKind::Anthropic => {
            if let Some(api_key) = config.env.get("ANTHROPIC_API_KEY") {
                headers.insert("x-api-key".to_string(), api_key.clone());
            }
            headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());
            let mut body = Map::new();
            body.insert(
                "model".to_string(),
                Value::String(resolve_model(
                    &config,
                    "ANTHROPIC_MODEL",
                    anthropic::DEFAULT_MODEL,
                )),
            );
            body.insert("max_tokens".to_string(), json!(1024));
            body.insert(
                "messages".to_string(),
                json!([{ "role": "user", "content": input.prompt }]),
            );
            merge_options(&mut body, &config.options);
            (
                config
                    .base_url
                    .unwrap_or_else(|| anthropic::DEFAULT_URL.to_string()),
                Value::Object(body),
            )
        }
    };

    Ok(ProviderRequestSnapshot {
        provider_id: config.id,
        kind: config.kind,
        method: "POST".to_string(),
        url,
        headers,
        body,
    })
}

fn normalize_headers(config: &ProviderConfig) -> BTreeMap<String, String> {
    config
        .headers
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), expand_env(value, &config.env)))
        .collect()
}

fn resolve_model(config: &ProviderConfig, env_key: &str, default: &str) -> String {
    config
        .model
        .as_deref()
        .map(|model| expand_env(model, &config.env))
        .or_else(|| config.env.get(env_key).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn expand_env(value: &str, env: &BTreeMap<String, String>) -> String {
    let mut expanded = value.to_string();
    for (name, replacement) in env {
        expanded = expanded.replace(&format!("${{{name}}}"), replacement);
        expanded = expanded.replace(&format!("${name}"), replacement);
    }
    expanded
}

fn merge_options(body: &mut Map<String, Value>, options: &Map<String, Value>) {
    for (name, value) in options {
        body.insert(name.clone(), value.clone());
    }
}

fn response_output_path(kind: ProviderKind) -> &'static str {
    match kind {
        ProviderKind::OpenAiCompatible => openai::RESPONSE_OUTPUT_PATH,
        ProviderKind::Http => http::RESPONSE_OUTPUT_PATH,
        ProviderKind::Ollama => ollama::RESPONSE_OUTPUT_PATH,
        ProviderKind::Anthropic => anthropic::RESPONSE_OUTPUT_PATH,
    }
}

fn extract_output(kind: ProviderKind, raw: &Value) -> Result<String, ProviderError> {
    let value = match kind {
        ProviderKind::OpenAiCompatible => raw.pointer("/choices/0/message/content"),
        ProviderKind::Http => raw.pointer("/output"),
        ProviderKind::Ollama => raw.pointer("/response"),
        ProviderKind::Anthropic => raw.pointer("/content/0/text"),
    };

    value
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            ProviderError::new(format!(
                "missing provider response output path: {}",
                response_output_path(kind)
            ))
        })
}
