use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;
use serde_json::{json, Value};

use crate::script_bridge::{
    reject_unauthorized_script, ScriptAuthorization, ScriptBridgeError, ScriptBridgeErrorKind,
    ScriptKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Priority {
    P0,
    P1,
    P2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BridgeStatus {
    Native,
    Bridge,
    Unsupported,
    Later,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CustomContract {
    pub runtime: ScriptKind,
    pub priority: Priority,
    pub bridge_status: BridgeStatus,
    pub capability: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomAssertionRequest {
    pub script_kind: ScriptKind,
    pub script_path: PathBuf,
    pub input: Value,
    pub timeout_ms: u64,
    pub env: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomAssertionResponse {
    #[serde(rename = "pass")]
    pub passed: bool,
    pub score: f64,
    pub reason: String,
    pub metadata: Value,
}

impl CustomAssertionResponse {
    pub fn passed(score: f64, reason: impl Into<String>, metadata: Value) -> Self {
        Self {
            passed: true,
            score,
            reason: reason.into(),
            metadata,
        }
    }
}

pub fn custom_assertion_contracts() -> Vec<CustomContract> {
    vec![
        CustomContract {
            runtime: ScriptKind::JavaScript,
            priority: Priority::P0,
            bridge_status: BridgeStatus::Bridge,
            capability: "custom assertion".to_string(),
        },
        CustomContract {
            runtime: ScriptKind::Python,
            priority: Priority::P0,
            bridge_status: BridgeStatus::Bridge,
            capability: "custom assertion".to_string(),
        },
        CustomContract {
            runtime: ScriptKind::Shell,
            priority: Priority::P1,
            bridge_status: BridgeStatus::Bridge,
            capability: "custom assertion".to_string(),
        },
    ]
}

pub fn evaluate_custom_assertion(
    request: CustomAssertionRequest,
    auth: ScriptAuthorization,
) -> Result<CustomAssertionResponse, ScriptBridgeError> {
    match auth {
        ScriptAuthorization::Deny => Err(reject_unauthorized_script(
            request.script_kind,
            &request.script_path,
        )),
        ScriptAuthorization::Allow => Err(ScriptBridgeError::new(
            ScriptBridgeErrorKind::ExecutionDeferred,
            "script_execution_deferred",
            request.script_kind,
            request.script_path,
            "custom assertion execution is delegated to the script-bridge phase",
        )),
    }
}

pub fn custom_assertion_schema_snapshot() -> Value {
    json!({
        "request": {
            "type": "object",
            "required": ["script_kind", "script_path", "input", "timeout_ms"],
            "properties": {
                "script_kind": {
                    "enum": ["javascript", "python", "shell", "ruby"]
                },
                "script_path": {
                    "type": "string"
                },
                "input": {
                    "type": "object"
                },
                "timeout_ms": {
                    "type": "integer",
                    "minimum": 1
                },
                "env": {
                    "type": "object",
                    "additionalProperties": { "type": "string" }
                }
            }
        },
        "response": {
            "type": "object",
            "required": ["pass", "score", "reason"],
            "properties": {
                "pass": { "type": "boolean" },
                "score": { "type": "number" },
                "reason": { "type": "string" },
                "metadata": { "type": "object" }
            }
        }
    })
}
