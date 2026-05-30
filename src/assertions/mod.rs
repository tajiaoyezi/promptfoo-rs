pub mod deterministic;
pub mod model_graded;

use serde::Serialize;
use serde_json::{json, Value};

pub use deterministic::evaluate_deterministic;
pub use model_graded::{
    build_model_graded_prompt, parse_model_graded_score, GraderRequest, ModelGradedAssertion,
};

pub type DeterministicAssertion = Assertion;

#[derive(Clone, Debug, PartialEq)]
pub enum Assertion {
    Equals(Value),
    Contains(String),
    Regex(String),
    JsonPointer { pointer: String, expected: Value },
    JsonSchema(Value),
}

impl Assertion {
    pub fn equals(expected: Value) -> Self {
        Self::Equals(expected)
    }

    pub fn contains(expected: impl Into<String>) -> Self {
        Self::Contains(expected.into())
    }

    pub fn regex(pattern: impl Into<String>) -> Self {
        Self::Regex(pattern.into())
    }

    pub fn json_pointer(pointer: impl Into<String>, expected: Value) -> Self {
        Self::JsonPointer {
            pointer: pointer.into(),
            expected,
        }
    }

    pub fn json_schema(schema: Value) -> Self {
        Self::JsonSchema(schema)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssertionContext {
    pub output: Value,
    pub output_text: String,
}

impl AssertionContext {
    pub fn new(output: Value) -> Self {
        let output_text = output
            .as_str()
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| output.to_string());
        Self {
            output,
            output_text,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssertionStatus {
    Passed,
    Failed,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AssertionResult {
    pub assertion_type: String,
    pub status: AssertionStatus,
    pub message: Option<String>,
    pub error: Option<String>,
    pub metadata: Value,
}

impl AssertionResult {
    pub fn passed(assertion_type: impl Into<String>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            status: AssertionStatus::Passed,
            message: None,
            error: None,
            metadata: json!({}),
        }
    }

    pub fn failed(assertion_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            status: AssertionStatus::Failed,
            message: Some(message.into()),
            error: None,
            metadata: json!({}),
        }
    }

    pub fn error(assertion_type: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            status: AssertionStatus::Error,
            message: None,
            error: Some(error.into()),
            metadata: json!({}),
        }
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AssertionSummary {
    pub status: AssertionStatus,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
}

pub fn evaluate_assertion(assertion: &Assertion, context: &AssertionContext) -> AssertionResult {
    deterministic::evaluate_deterministic(assertion, context)
}

pub fn aggregate_assertions(results: Vec<AssertionResult>) -> AssertionSummary {
    let passed = results
        .iter()
        .filter(|result| result.status == AssertionStatus::Passed)
        .count();
    let failed = results
        .iter()
        .filter(|result| result.status == AssertionStatus::Failed)
        .count();
    let errors = results
        .iter()
        .filter(|result| result.status == AssertionStatus::Error)
        .count();
    let status = if failed > 0 {
        AssertionStatus::Failed
    } else if errors > 0 {
        AssertionStatus::Error
    } else {
        AssertionStatus::Passed
    };

    AssertionSummary {
        status,
        total: results.len(),
        passed,
        failed,
        errors,
    }
}
