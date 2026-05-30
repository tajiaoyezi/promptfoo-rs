use std::process::Command;

use promptfoo_rs::mcp::protocol::McpRequest;
use promptfoo_rs::mcp::provider::{materialize_mcp_target, McpProvider, McpTargetConfig};
use serde_json::{json, Value};

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_8_1_1_promptfoo_mcp_command_skeleton_runs() {
    let output = promptfoo_rs()
        .args(["mcp", "--mode", "list-tools"])
        .output()
        .expect("mcp command executes");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("stdout is JSON");
    assert_eq!(stdout["protocol"], "mcp");
    assert_eq!(stdout["status"], "ok");
    assert!(stdout["tools"]
        .as_array()
        .expect("tools array")
        .iter()
        .any(|tool| tool == "promptfoo.eval"));
}

#[test]
fn test_8_1_2_mcp_provider_request_response_protocol_snapshot() {
    let request = McpRequest::new(
        "req-001",
        "promptfoo.eval",
        json!({
            "config": "promptfooconfig.yaml",
            "vars": { "name": "Ada" }
        }),
    );

    let response = McpProvider::in_memory("promptfoo-rs")
        .call(request)
        .expect("provider call succeeds");
    let snapshot = serde_json::to_value(&response).expect("response serializes");

    assert_eq!(snapshot["jsonrpc"], "2.0");
    assert_eq!(snapshot["id"], "req-001");
    assert_eq!(snapshot["result"]["provider"], "promptfoo-rs");
    assert_eq!(snapshot["result"]["method"], "promptfoo.eval");
    assert_eq!(snapshot["result"]["status"], "ok");
}

#[test]
fn test_8_1_3_mcp_target_materialization_error_path_is_stable() {
    let err =
        materialize_mcp_target(McpTargetConfig::stdio("")).expect_err("empty command is rejected");
    assert!(err.to_string().contains("MCP target command"));

    let err = materialize_mcp_target(McpTargetConfig::http("ftp://example.invalid/mcp"))
        .expect_err("unsupported URL is rejected");
    assert!(err.to_string().contains("MCP target URL"));
    assert!(err.to_string().contains("http"));
}
