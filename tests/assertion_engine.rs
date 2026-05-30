use promptfoo_rs::assertions::{
    aggregate_assertions, build_model_graded_prompt, evaluate_assertion, parse_model_graded_score,
    Assertion, AssertionContext, AssertionResult, AssertionStatus, ModelGradedAssertion,
};
use serde_json::json;

#[test]
fn test_4_2_1_deterministic_assertions_have_stable_result_snapshots() {
    let context = AssertionContext::new(json!({
        "answer": "Hello Ada",
        "score": 5
    }));
    let assertions = [
        Assertion::equals(json!({"answer": "Hello Ada", "score": 5})),
        Assertion::contains("Ada"),
        Assertion::regex(r"Hello\s+Ada"),
        Assertion::json_pointer("/score", json!(5)),
        Assertion::json_schema(json!({
            "type": "object",
            "required": ["answer", "score"],
            "properties": {
                "answer": { "type": "string" },
                "score": { "type": "number" }
            }
        })),
    ];

    let snapshot = assertions
        .iter()
        .map(|assertion| evaluate_assertion(assertion, &context))
        .collect::<Vec<_>>();

    assert_eq!(
        snapshot
            .iter()
            .map(|result| (&result.assertion_type, result.status))
            .collect::<Vec<_>>(),
        vec![
            (&"equals".to_string(), AssertionStatus::Passed),
            (&"contains".to_string(), AssertionStatus::Passed),
            (&"regex".to_string(), AssertionStatus::Passed),
            (&"json".to_string(), AssertionStatus::Passed),
            (&"schema".to_string(), AssertionStatus::Passed),
        ]
    );
    assert!(snapshot.iter().all(|result| result.error.is_none()));
}

#[test]
fn test_4_2_2_model_graded_assertion_records_prompt_threshold_score_and_metadata() {
    let context = AssertionContext::new(json!("The answer explains the safety tradeoff."));
    let assertion = ModelGradedAssertion {
        rubric: "Rate whether the answer explains the safety tradeoff.".to_string(),
        threshold: 0.7,
    };

    let request = build_model_graded_prompt(&assertion, &context);
    assert!(request.prompt.contains(&assertion.rubric));
    assert!(request
        .prompt
        .contains("The answer explains the safety tradeoff."));
    assert_eq!(request.threshold, 0.7);
    assert_eq!(request.metadata["grading_kind"], "model-graded");
    assert_eq!(request.metadata["compare_raw_llm_text"], false);

    let passed =
        parse_model_graded_score(&json!({"score": 0.82, "reason": "clear enough"}), &request);
    assert_eq!(passed.status, AssertionStatus::Passed);
    assert_eq!(passed.metadata["score"], 0.82);
    assert_eq!(passed.metadata["threshold"], 0.7);

    let failed = parse_model_graded_score(&json!({"score": 0.42, "reason": "thin"}), &request);
    assert_eq!(failed.status, AssertionStatus::Failed);
    assert_eq!(failed.metadata["reason"], "thin");
}

#[test]
fn test_4_2_3_assertion_aggregation_has_stable_pass_fail_error_shape() {
    let summary = aggregate_assertions(vec![
        AssertionResult::passed("equals"),
        AssertionResult::failed("contains", "expected output to contain Ada"),
        AssertionResult::error("schema", "invalid schema"),
    ]);

    assert_eq!(summary.status, AssertionStatus::Failed);
    assert_eq!(summary.total, 3);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.errors, 1);
    assert_eq!(
        serde_json::to_value(&summary).expect("summary should serialize"),
        json!({
            "status": "failed",
            "total": 3,
            "passed": 1,
            "failed": 1,
            "errors": 1
        })
    );
}
