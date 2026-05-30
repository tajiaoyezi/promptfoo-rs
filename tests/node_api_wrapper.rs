use std::fs;

use promptfoo_rs::node_api::rpc::{
    handle_node_rpc, node_wrapper_release_gate, wrapper_contract, NodeRpcRequest,
};
use serde_json::json;

#[test]
fn test_9_2_1_node_wrapper_delegates_to_rust_core_rpc_boundary() {
    let contract = wrapper_contract();

    assert_eq!(contract.transport, "json-rpc-stdio");
    assert_eq!(contract.business_logic_owner, "rust-core");
    assert_eq!(contract.wrapper_role, "thin-client");
    assert!(contract.methods.contains(&"evaluate"));

    let index = fs::read_to_string("npm/src/index.ts").expect("wrapper entrypoint exists");
    assert!(index.contains("createPromptfooClient"), "{index}");
    assert!(index.contains("callRustCore"), "{index}");
    assert!(!index.contains("run_eval("), "{index}");
}

#[test]
fn test_9_2_2_node_rpc_params_errors_and_result_schema_are_snapshotted() {
    let request = NodeRpcRequest::new(
        "req-9.2",
        "evaluate",
        json!({
            "config": {
                "providers": [{ "id": "echo" }],
                "prompts": ["Hello {{name}}"],
                "tests": [{ "vars": { "name": "Ada" } }]
            },
            "options": {}
        }),
    );

    let response = handle_node_rpc(request).expect("evaluate RPC succeeds");
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, "req-9.2");
    assert_eq!(
        response.result["schema_version"],
        "promptfoo-rs.node-api.v1"
    );
    assert_eq!(response.result["method"], "evaluate");
    assert_eq!(response.result["result"]["status"], "ok");
    assert_eq!(response.result["result"]["summary"]["total_cases"], 1);
    assert_eq!(
        response.result["result"]["results"][0]["output"],
        "Hello Ada"
    );

    let error = handle_node_rpc(NodeRpcRequest::new("req-bad", "unknown.method", json!({})))
        .expect_err("unknown methods are stable errors");
    assert_eq!(error.code, "method_not_found");
    assert!(error.message.contains("unknown.method"));
}

#[test]
fn test_9_2_3_wrapper_core_drift_gate_is_release_blocking() {
    let gate = node_wrapper_release_gate();

    assert_eq!(gate.name, "node-api-wrapper-drift");
    assert!(gate.release_blocking);
    assert!(gate.protects.contains(&"wrapper/core drift"));
    assert!(gate.test_ids.contains(&"TEST-9.2.1"));
    assert!(gate.test_ids.contains(&"TEST-9.2.2"));
    assert!(gate.test_ids.contains(&"TEST-9.2.3"));
}
