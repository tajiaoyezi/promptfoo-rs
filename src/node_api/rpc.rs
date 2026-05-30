use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::{NormalizedConfig, NormalizedPrompt, NormalizedProvider, NormalizedTestCase};
use crate::eval::{run_eval, EvalOptions};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Value,
}

impl NodeRpcRequest {
    pub fn new(id: impl Into<String>, method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: id.into(),
            method: method.into(),
            params,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRpcResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeRpcError {
    pub code: String,
    pub message: String,
}

impl NodeRpcError {
    fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for NodeRpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for NodeRpcError {}

pub fn handle_node_rpc(request: NodeRpcRequest) -> Result<NodeRpcResponse, NodeRpcError> {
    if request.jsonrpc != "2.0" {
        return Err(NodeRpcError::new(
            "invalid_request",
            "node API JSON-RPC version must be 2.0",
        ));
    }
    match request.method.as_str() {
        "evaluate" => {
            let config = parse_node_config(
                request
                    .params
                    .get("config")
                    .ok_or_else(|| NodeRpcError::new("invalid_params", "missing config param"))?,
            )?;
            let result = run_eval(config, EvalOptions::default()).map_err(|err| {
                NodeRpcError::new("internal_error", format!("eval core failed: {err}"))
            })?;
            Ok(NodeRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: json!({
                    "schema_version": "promptfoo-rs.node-api.v1",
                    "method": "evaluate",
                    "result": result,
                }),
            })
        }
        other => Err(NodeRpcError::new(
            "method_not_found",
            format!("unknown Node API method: {other}"),
        )),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NodeWrapperContract {
    pub transport: &'static str,
    pub business_logic_owner: &'static str,
    pub wrapper_role: &'static str,
    pub methods: Vec<&'static str>,
}

pub fn wrapper_contract() -> NodeWrapperContract {
    NodeWrapperContract {
        transport: "json-rpc-stdio",
        business_logic_owner: "rust-core",
        wrapper_role: "thin-client",
        methods: vec!["evaluate"],
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NodeWrapperReleaseGate {
    pub name: &'static str,
    pub release_blocking: bool,
    pub protects: Vec<&'static str>,
    pub test_ids: Vec<&'static str>,
}

pub fn node_wrapper_release_gate() -> NodeWrapperReleaseGate {
    NodeWrapperReleaseGate {
        name: "node-api-wrapper-drift",
        release_blocking: true,
        protects: vec!["wrapper/core drift"],
        test_ids: vec!["TEST-9.2.1", "TEST-9.2.2", "TEST-9.2.3"],
    }
}

fn parse_node_config(value: &Value) -> Result<NormalizedConfig, NodeRpcError> {
    let providers = value
        .get("providers")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|provider| NormalizedProvider {
            id: provider
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("echo")
                .to_string(),
            config: provider.get("config").cloned().unwrap_or(Value::Null),
        })
        .collect();

    let prompts = value
        .get("prompts")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|prompt| {
            prompt
                .as_str()
                .map(|body| NormalizedPrompt {
                    source: None,
                    body: body.to_string(),
                })
                .ok_or_else(|| NodeRpcError::new("invalid_params", "prompt must be a string"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let tests = value
        .get("tests")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|test| NormalizedTestCase {
            vars: stringify_object(test.get("vars")),
            assertions: Vec::new(),
        })
        .collect();

    Ok(NormalizedConfig {
        description: value
            .get("description")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        providers,
        prompts,
        tests,
    })
}

fn stringify_object(value: Option<&Value>) -> BTreeMap<String, String> {
    value
        .and_then(Value::as_object)
        .map(|object| {
            object
                .iter()
                .map(|(key, value)| (key.clone(), stringify_value(value)))
                .collect()
        })
        .unwrap_or_default()
}

fn stringify_value(value: &Value) -> String {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| value.to_string())
}
