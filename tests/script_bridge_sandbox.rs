use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use promptfoo_rs::script_bridge::{
    redact_secrets, RedactionPolicy, ScriptAuthorization, ScriptBridge, ScriptBridgeErrorKind,
    ScriptKind, ScriptRequest, ScriptSandboxOptions,
};
use serde_json::json;

#[test]
fn test_9_1_1_allow_scripts_disabled_returns_stable_error() {
    let request = shell_request(echo_script(), "should not run");

    let error = ScriptBridge::execute(request, ScriptAuthorization::Deny)
        .expect_err("default authorization rejects script bridge execution");

    assert_eq!(error.kind, ScriptBridgeErrorKind::Unauthorized);
    assert_eq!(error.code, "script_not_authorized");
    assert_eq!(error.script_kind, ScriptKind::Shell);
    assert_eq!(error.path, PathBuf::from("test/fixtures/script-bridge/inline"));
    assert!(error.message.contains("--allow-scripts"), "{error:?}");
}

#[test]
fn test_9_1_2_authorized_subprocess_io_and_timeout_are_stable() {
    let response = ScriptBridge::execute(shell_request(echo_script(), "hello from stdin"), ScriptAuthorization::Allow)
        .expect("authorized shell script executes");

    assert_eq!(response.exit_code, Some(0));
    assert!(response.stdout.contains("stdin:hello from stdin"), "{response:?}");
    assert!(response.stdout.contains("allowed:visible"), "{response:?}");
    assert!(response.stderr.contains("stderr:bridge"), "{response:?}");

    let mut timeout_request = shell_request(timeout_script(), "");
    timeout_request.options.timeout = Duration::from_millis(25);
    let error = ScriptBridge::execute(timeout_request, ScriptAuthorization::Allow)
        .expect_err("timeout fixture is killed");

    assert_eq!(error.kind, ScriptBridgeErrorKind::Timeout);
    assert_eq!(error.code, "script_timeout");
}

#[test]
fn test_9_1_3_env_allowlist_and_secret_redaction_are_stable() {
    let response = ScriptBridge::execute(shell_request(env_script(), ""), ScriptAuthorization::Allow)
        .expect("authorized shell script executes");

    assert_eq!(response.exit_code, Some(0));
    assert!(response.stdout.contains("allowed:visible"), "{response:?}");
    assert!(response.stdout.contains("secret:"), "{response:?}");
    assert!(!response.stdout.contains("hidden"), "{response:?}");

    let mut payload = json!({
        "safe": "visible",
        "apiKey": "sk-test-secret",
        "nested": {
            "Authorization": "Bearer very-secret",
            "token": "token-secret"
        },
        "items": [
            { "providerHeader": "x-secret" }
        ]
    });
    redact_secrets(&mut payload, &RedactionPolicy::default());

    assert_eq!(payload["safe"], "visible");
    assert_eq!(payload["apiKey"], "[REDACTED]");
    assert_eq!(payload["nested"]["Authorization"], "[REDACTED]");
    assert_eq!(payload["nested"]["token"], "[REDACTED]");
    assert_eq!(payload["items"][0]["providerHeader"], "[REDACTED]");
}

fn shell_request(script: &str, stdin: &str) -> ScriptRequest {
    let (program, args) = shell_command(script);
    ScriptRequest {
        script_kind: ScriptKind::Shell,
        script_path: PathBuf::from("test/fixtures/script-bridge/inline"),
        program: PathBuf::from(program),
        args,
        stdin: stdin.to_string(),
        env: BTreeMap::from([
            ("PROMPTFOO_ALLOWED".to_string(), "visible".to_string()),
            ("PROMPTFOO_SECRET".to_string(), "hidden".to_string()),
        ]),
        options: ScriptSandboxOptions {
            timeout: Duration::from_secs(2),
            env_allowlist: vec!["PROMPTFOO_ALLOWED".to_string()],
            cwd: None,
            stdin_limit: 1024,
        },
    }
}

#[cfg(windows)]
fn shell_command(script: &str) -> (&'static str, Vec<String>) {
    (
        "powershell.exe",
        vec![
            "-NoProfile".to_string(),
            "-NonInteractive".to_string(),
            "-Command".to_string(),
            script.to_string(),
        ],
    )
}

#[cfg(not(windows))]
fn shell_command(script: &str) -> (&'static str, Vec<String>) {
    ("sh", vec!["-c".to_string(), script.to_string()])
}

#[cfg(windows)]
fn echo_script() -> &'static str {
    "$stdinText = [Console]::In.ReadToEnd(); [Console]::Out.WriteLine(\"stdin:$stdinText\"); [Console]::Error.WriteLine(\"stderr:bridge\"); [Console]::Out.WriteLine(\"allowed:$env:PROMPTFOO_ALLOWED\")"
}

#[cfg(not(windows))]
fn echo_script() -> &'static str {
    "stdin_text=$(cat); echo \"stdin:$stdin_text\"; echo \"stderr:bridge\" >&2; echo \"allowed:$PROMPTFOO_ALLOWED\""
}

#[cfg(windows)]
fn env_script() -> &'static str {
    "[Console]::Out.WriteLine(\"allowed:$env:PROMPTFOO_ALLOWED\"); [Console]::Out.WriteLine(\"secret:$env:PROMPTFOO_SECRET\")"
}

#[cfg(not(windows))]
fn env_script() -> &'static str {
    "echo \"allowed:$PROMPTFOO_ALLOWED\"; echo \"secret:$PROMPTFOO_SECRET\""
}

#[cfg(windows)]
fn timeout_script() -> &'static str {
    "Start-Sleep -Milliseconds 500; [Console]::Out.WriteLine(\"done\")"
}

#[cfg(not(windows))]
fn timeout_script() -> &'static str {
    "sleep 0.5; echo done"
}
