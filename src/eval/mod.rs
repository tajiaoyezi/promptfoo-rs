pub mod retry;
pub mod scheduler;

use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value;

use crate::assertions::{evaluate_assertion, Assertion, AssertionContext, AssertionStatus};
use crate::cache::CacheStore;
use crate::config::{NormalizedAssertion, NormalizedConfig, NormalizedTestCase};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EvalOptions {
    pub max_concurrency: Option<usize>,
    pub resume_completed_cases: Vec<String>,
}

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
    pub errors: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalCaseResult {
    pub case_id: String,
    pub provider_id: String,
    pub prompt: String,
    pub output: String,
    pub vars: std::collections::BTreeMap<String, String>,
    pub status: String,
    pub assertion_results: Vec<EvalAssertionResult>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalAssertionResult {
    pub assertion_type: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvalMetadata {
    pub runner: String,
    pub max_concurrency: Option<usize>,
    pub resume: EvalResumeMetadata,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct EvalResumeMetadata {
    pub completed_cases: Vec<String>,
}

pub type EvalError = String;

pub fn run_eval(
    config: NormalizedConfig,
    options: EvalOptions,
) -> Result<EvalResultEnvelope, EvalError> {
    run_eval_internal(&config, &options, &BTreeSet::new())
}

pub fn resume_eval_from_cache(
    config: &NormalizedConfig,
    cache: &CacheStore,
) -> Result<EvalResultEnvelope, EvalError> {
    let completed = cache
        .state()
        .completed_case_ids()
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let completed_set = completed
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    run_eval_internal(
        config,
        &EvalOptions {
            resume_completed_cases: completed.clone(),
            ..EvalOptions::default()
        },
        &completed_set,
    )
}

fn run_eval_internal(
    config: &NormalizedConfig,
    options: &EvalOptions,
    skip_case_ids: &BTreeSet<&str>,
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

    let tests = if config.tests.is_empty() {
        vec![NormalizedTestCase {
            vars: Default::default(),
            assertions: Vec::new(),
        }]
    } else {
        config.tests.clone()
    };

    let mut results = Vec::new();
    for (index, test) in tests.into_iter().enumerate() {
        let case_id = format!("case-{index}");
        if skip_case_ids.contains(case_id.as_str()) {
            continue;
        }
        results.push(run_case(&provider_id, &prompt, case_id, test));
    }

    let passed = results
        .iter()
        .filter(|result| result.status == "passed")
        .count();
    let failed = results
        .iter()
        .filter(|result| result.status == "failed")
        .count();
    let errors = results
        .iter()
        .filter(|result| result.status == "error")
        .count();
    let status = if errors > 0 {
        "error"
    } else if failed > 0 {
        "failed"
    } else {
        "ok"
    };

    Ok(EvalResultEnvelope {
        status: status.to_string(),
        summary: EvalSummary {
            total_cases: results.len(),
            passed,
            failed,
            errors,
        },
        results,
        errors: Vec::new(),
        metadata: EvalMetadata {
            runner: "promptfoo-rs".to_string(),
            max_concurrency: options.max_concurrency,
            resume: EvalResumeMetadata {
                completed_cases: options.resume_completed_cases.clone(),
            },
        },
    })
}

fn run_case(
    provider_id: &str,
    prompt: &str,
    case_id: String,
    test: NormalizedTestCase,
) -> EvalCaseResult {
    let rendered = render_prompt(prompt, &test.vars);
    if provider_id == "fail" {
        return EvalCaseResult {
            case_id,
            provider_id: provider_id.to_string(),
            prompt: rendered,
            output: String::new(),
            vars: test.vars,
            status: "error".to_string(),
            assertion_results: Vec::new(),
            error: Some("provider failure".to_string()),
        };
    }

    let assertion_results = test
        .assertions
        .iter()
        .map(|assertion| evaluate_normalized_assertion(assertion, &rendered))
        .collect::<Vec<_>>();
    let failed = assertion_results
        .iter()
        .any(|result| result.status == "failed" || result.status == "error");
    EvalCaseResult {
        case_id,
        provider_id: provider_id.to_string(),
        prompt: rendered.clone(),
        output: rendered,
        vars: test.vars,
        status: if failed { "failed" } else { "passed" }.to_string(),
        assertion_results,
        error: None,
    }
}

fn evaluate_normalized_assertion(
    assertion: &NormalizedAssertion,
    output: &str,
) -> EvalAssertionResult {
    let Some(assertion_model) = assertion_model(assertion) else {
        return EvalAssertionResult {
            assertion_type: assertion.assertion_type.clone(),
            status: "error".to_string(),
            message: Some(format!(
                "unsupported assertion {}",
                assertion.assertion_type
            )),
        };
    };
    let result = evaluate_assertion(
        &assertion_model,
        &AssertionContext::new(Value::String(output.to_string())),
    );
    EvalAssertionResult {
        assertion_type: result.assertion_type,
        status: match result.status {
            AssertionStatus::Passed => "passed",
            AssertionStatus::Failed => "failed",
            AssertionStatus::Error => "error",
        }
        .to_string(),
        message: result.message.or(result.error),
    }
}

fn assertion_model(assertion: &NormalizedAssertion) -> Option<Assertion> {
    match assertion.assertion_type.as_str() {
        "equals" => Some(Assertion::equals(Value::String(
            assertion.value.clone().unwrap_or_default(),
        ))),
        "contains" => Some(Assertion::contains(
            assertion.value.clone().unwrap_or_default(),
        )),
        "regex" => Some(Assertion::regex(
            assertion.value.clone().unwrap_or_default(),
        )),
        _ => None,
    }
}

fn render_prompt(prompt: &str, vars: &std::collections::BTreeMap<String, String>) -> String {
    let mut rendered = prompt.to_string();
    for (key, value) in vars {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}
