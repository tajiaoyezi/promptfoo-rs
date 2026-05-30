pub mod protocol;
pub mod provider;

use serde_json::{json, Value};

pub use provider::{materialize_mcp_target, McpProvider, McpTarget, McpTargetConfig};

pub fn tool_listing() -> Value {
    json!({
        "protocol": "mcp",
        "status": "ok",
        "tools": [
            "promptfoo.eval",
            "promptfoo.redteam",
            "promptfoo.scan"
        ]
    })
}
