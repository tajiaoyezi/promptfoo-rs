use std::path::Path;

use promptfoo_rs::compatibility::release_gate::ReleaseGateStatus;
use promptfoo_rs::release::{evaluate_security_defaults, SecurityRun};
use promptfoo_rs::script_bridge::{
    redact_secrets, reject_unauthorized_script, RedactionPolicy, ScriptKind,
};
use serde_json::json;

#[test]
fn test_15_2_3_security_gate_requires_default_deny_redaction_and_no_upload() {
    /* TEST-15.2.3 */
    let unauthorized = reject_unauthorized_script(
        ScriptKind::JavaScript,
        Path::new("fixtures/custom-provider.js"),
    );
    assert_eq!(unauthorized.code, "script_not_authorized");

    let mut payload = json!({
        "apiKey": "sk-live-secret",
        "providerHeaders": {
            "Authorization": "Bearer token-123"
        },
        "prompt": "local prompt remains local"
    });
    redact_secrets(&mut payload, &RedactionPolicy::default());
    let redacted = serde_json::to_string(&payload).expect("redacted payload serializes");

    let passing = evaluate_security_defaults(&SecurityRun {
        custom_scripts_default_denied: true,
        unauthorized_error_code: unauthorized.code,
        log_sample: redacted.clone(),
        artifact_sample: redacted,
        known_secret_values: vec!["sk-live-secret".to_string(), "token-123".to_string()],
        upload_attempts: 0,
        no_upload_evidence: vec!["local-only runtime smoke".to_string()],
        artifact_path: "target/release-gates/security.json".to_string(),
    });

    assert_eq!(passing.status, ReleaseGateStatus::Ready, "{passing:#?}");
    assert!(passing.default_deny_passed, "{passing:#?}");
    assert!(passing.redaction_passed, "{passing:#?}");
    assert!(passing.no_upload_passed, "{passing:#?}");

    let blocked = evaluate_security_defaults(&SecurityRun {
        custom_scripts_default_denied: false,
        unauthorized_error_code: "missing".to_string(),
        log_sample: "apiKey=sk-live-secret".to_string(),
        artifact_sample: "token=token-123".to_string(),
        known_secret_values: vec!["sk-live-secret".to_string(), "token-123".to_string()],
        upload_attempts: 1,
        no_upload_evidence: Vec::new(),
        artifact_path: "target/release-gates/security.json".to_string(),
    });

    assert_eq!(blocked.status, ReleaseGateStatus::Blocked, "{blocked:#?}");
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("default deny")),
        "{blocked:#?}"
    );
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("redaction")),
        "{blocked:#?}"
    );
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("upload")),
        "{blocked:#?}"
    );
}
