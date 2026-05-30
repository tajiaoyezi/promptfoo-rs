use std::collections::BTreeMap;
use std::path::PathBuf;

use promptfoo_rs::assertions::custom::{
    custom_assertion_contracts, custom_assertion_schema_snapshot, evaluate_custom_assertion,
    BridgeStatus, CustomAssertionRequest, CustomAssertionResponse, Priority,
};
use promptfoo_rs::script_bridge::{
    reject_unauthorized_script, ScriptAuthorization, ScriptBridgeErrorKind, ScriptKind,
};
use serde_json::json;

#[test]
fn test_4_3_1_custom_contract_matrix_marks_priority_and_bridge_status() {
    let contracts = custom_assertion_contracts();

    assert!(contracts.iter().any(|contract| {
        contract.runtime == ScriptKind::JavaScript
            && contract.priority == Priority::P0
            && contract.bridge_status == BridgeStatus::Bridge
            && contract.capability == "custom assertion"
    }));
    assert!(contracts.iter().any(|contract| {
        contract.runtime == ScriptKind::Python
            && contract.priority == Priority::P0
            && contract.bridge_status == BridgeStatus::Bridge
            && contract.capability == "custom assertion"
    }));
    assert!(contracts.iter().any(|contract| {
        contract.runtime == ScriptKind::Shell
            && contract.priority == Priority::P1
            && contract.bridge_status == BridgeStatus::Bridge
            && contract.capability == "custom assertion"
    }));
}

#[test]
fn test_4_3_2_allow_scripts_disabled_returns_stable_rejection_error() {
    let request = CustomAssertionRequest {
        script_kind: ScriptKind::JavaScript,
        script_path: PathBuf::from("assertions/check.js"),
        input: json!({"output": "hello"}),
        timeout_ms: 1_000,
        env: BTreeMap::new(),
    };

    let error = evaluate_custom_assertion(request, ScriptAuthorization::Deny)
        .expect_err("TEST-4.3.2 default should reject custom script");

    assert_eq!(error.kind, ScriptBridgeErrorKind::Unauthorized);
    assert_eq!(error.code, "script_not_authorized");
    assert_eq!(error.script_kind, ScriptKind::JavaScript);
    assert_eq!(error.path, PathBuf::from("assertions/check.js"));
    assert!(error.message.contains("--allow-scripts"), "{error:?}");

    let direct = reject_unauthorized_script(ScriptKind::Python, "assertions/check.py".as_ref());
    assert_eq!(direct.kind, ScriptBridgeErrorKind::Unauthorized);
    assert_eq!(direct.code, "script_not_authorized");
}

#[test]
fn test_4_3_3_custom_assertion_request_response_schema_has_snapshot() {
    let snapshot = custom_assertion_schema_snapshot();

    assert_eq!(snapshot["request"]["required"], json!([
        "script_kind",
        "script_path",
        "input",
        "timeout_ms"
    ]));
    assert_eq!(snapshot["response"]["required"], json!(["pass", "score", "reason"]));

    let response = CustomAssertionResponse::passed(0.91, "matches policy", json!({
        "script_kind": "javascript"
    }));
    assert_eq!(
        serde_json::to_value(response).expect("response should serialize"),
        json!({
            "pass": true,
            "score": 0.91,
            "reason": "matches policy",
            "metadata": {
                "script_kind": "javascript"
            }
        })
    );
}
