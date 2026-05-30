use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::mcp::protocol::{McpRequest, McpResponse};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct McpProvider {
    name: String,
}

impl McpProvider {
    pub fn in_memory(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn call(&self, request: McpRequest) -> Result<McpResponse, McpError> {
        if request.jsonrpc != "2.0" {
            return Err(McpError::new("MCP request jsonrpc must be 2.0"));
        }
        Ok(McpResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: json!({
                "provider": self.name,
                "method": request.method,
                "status": "ok",
                "params": request.params,
            }),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpTargetConfig {
    pub transport: McpTransport,
    pub command: Option<String>,
    pub url: Option<String>,
}

impl McpTargetConfig {
    pub fn stdio(command: impl Into<String>) -> Self {
        Self {
            transport: McpTransport::Stdio,
            command: Some(command.into()),
            url: None,
        }
    }

    pub fn http(url: impl Into<String>) -> Self {
        Self {
            transport: McpTransport::Http,
            command: None,
            url: Some(url.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum McpTransport {
    Stdio,
    Http,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpTarget {
    pub transport: McpTransport,
    pub endpoint: String,
}

pub fn materialize_mcp_target(config: McpTargetConfig) -> Result<McpTarget, McpError> {
    match config.transport {
        McpTransport::Stdio => {
            let command = config.command.unwrap_or_default();
            if command.trim().is_empty() {
                return Err(McpError::new(
                    "MCP target command is required for stdio transport",
                ));
            }
            Ok(McpTarget {
                transport: McpTransport::Stdio,
                endpoint: command,
            })
        }
        McpTransport::Http => {
            let url = config.url.unwrap_or_default();
            if !(url.starts_with("http://") || url.starts_with("https://")) {
                return Err(McpError::new(
                    "MCP target URL must start with http:// or https://",
                ));
            }
            Ok(McpTarget {
                transport: McpTransport::Http,
                endpoint: url,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct McpError {
    message: String,
}

impl McpError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for McpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for McpError {}
