use serde_json::{json, Value};

use super::{AssertionContext, AssertionResult, AssertionStatus};

#[derive(Clone, Debug, PartialEq)]
pub struct ModelGradedAssertion {
    pub rubric: String,
    pub threshold: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraderRequest {
    pub prompt: String,
    pub threshold: f64,
    pub metadata: Value,
}

pub fn build_model_graded_prompt(
    assertion: &ModelGradedAssertion,
    context: &AssertionContext,
) -> GraderRequest {
    let prompt = format!(
        "You are grading a promptfoo assertion.\nRubric:\n{}\n\nOutput:\n{}\n\nReturn JSON with numeric score and reason.",
        assertion.rubric, context.output_text
    );

    GraderRequest {
        prompt,
        threshold: assertion.threshold,
        metadata: json!({
            "grading_kind": "model-graded",
            "threshold": assertion.threshold,
            "compare_raw_llm_text": false,
            "score_schema": {
                "type": "object",
                "required": ["score", "reason"],
                "properties": {
                    "score": { "type": "number" },
                    "reason": { "type": "string" }
                }
            }
        }),
    }
}

pub fn parse_model_graded_score(raw: &Value, request: &GraderRequest) -> AssertionResult {
    let score = match raw.get("score").and_then(Value::as_f64) {
        Some(score) => score,
        None => return AssertionResult::error("model-graded", "missing numeric score"),
    };
    let reason = raw
        .get("reason")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let status = if score >= request.threshold {
        AssertionStatus::Passed
    } else {
        AssertionStatus::Failed
    };

    AssertionResult {
        assertion_type: "model-graded".to_string(),
        status,
        message: Some(reason.clone()),
        error: None,
        metadata: json!({
            "score": score,
            "threshold": request.threshold,
            "reason": reason,
            "grading_kind": request.metadata["grading_kind"],
            "compare_raw_llm_text": request.metadata["compare_raw_llm_text"],
        }),
    }
}
