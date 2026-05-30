pub mod scheduler;

use serde::Serialize;

use crate::config::NormalizedConfig;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EvalOptions {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalResultEnvelope {
    pub status: String,
    pub summary: EvalSummary,
    pub results: Vec<EvalCaseResult>,
    pub errors: Vec<String>,
    pub metadata: EvalMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalSummary {
    pub total_cases: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalCaseResult {
    pub provider_id: String,
    pub prompt: String,
    pub output: String,
    pub vars: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalMetadata {
    pub runner: String,
}

pub type EvalError = String;

pub fn run_eval(
    config: NormalizedConfig,
    _options: EvalOptions,
) -> Result<EvalResultEnvelope, EvalError> {
    let provider_id = config
        .providers
        .first()
        .map(|provider| provider.id.clone())
        .unwrap_or_else(|| "echo".to_string());
    let prompt = config
        .prompts
        .first()
        .map(|prompt| prompt.body.clone())
        .unwrap_or_default();

    let results: Vec<EvalCaseResult> = if config.tests.is_empty() {
        vec![EvalCaseResult {
            provider_id,
            prompt: prompt.clone(),
            output: prompt,
            vars: Default::default(),
        }]
    } else {
        config
            .tests
            .into_iter()
            .map(|test| {
                let rendered = render_prompt(&prompt, &test.vars);
                EvalCaseResult {
                    provider_id: provider_id.clone(),
                    prompt: rendered.clone(),
                    output: rendered,
                    vars: test.vars,
                }
            })
            .collect()
    };

    Ok(EvalResultEnvelope {
        status: "ok".to_string(),
        summary: EvalSummary {
            total_cases: results.len(),
            passed: results.len(),
            failed: 0,
        },
        results,
        errors: Vec::new(),
        metadata: EvalMetadata {
            runner: "promptfoo-rs".to_string(),
        },
    })
}

fn render_prompt(prompt: &str, vars: &std::collections::BTreeMap<String, String>) -> String {
    let mut rendered = prompt.to_string();
    for (key, value) in vars {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}
