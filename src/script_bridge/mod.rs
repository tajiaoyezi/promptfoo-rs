use std::fmt;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptAuthorization {
    Allow,
    Deny,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScriptKind {
    JavaScript,
    Python,
    Shell,
    Ruby,
}

impl fmt::Display for ScriptKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ScriptKind::JavaScript => "javascript",
            ScriptKind::Python => "python",
            ScriptKind::Shell => "shell",
            ScriptKind::Ruby => "ruby",
        };
        f.write_str(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptBridgeErrorKind {
    Unauthorized,
    ExecutionDeferred,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptBridgeError {
    pub kind: ScriptBridgeErrorKind,
    pub code: String,
    pub script_kind: ScriptKind,
    pub path: PathBuf,
    pub message: String,
}

impl ScriptBridgeError {
    pub fn new(
        kind: ScriptBridgeErrorKind,
        code: impl Into<String>,
        script_kind: ScriptKind,
        path: impl Into<PathBuf>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: code.into(),
            script_kind,
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ScriptBridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ScriptBridgeError {}

pub fn reject_unauthorized_script(script_kind: ScriptKind, path: &Path) -> ScriptBridgeError {
    ScriptBridgeError::new(
        ScriptBridgeErrorKind::Unauthorized,
        "script_not_authorized",
        script_kind,
        path,
        format!(
            "{script_kind} script '{}' requires explicit --allow-scripts authorization",
            path.display()
        ),
    )
}
