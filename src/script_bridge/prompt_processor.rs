use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{json, Value};

use crate::script_bridge::{
    ScriptAuthorization, ScriptBridge, ScriptBridgeError, ScriptBridgeErrorKind, ScriptKind,
    ScriptRequest, ScriptSandboxOptions,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PromptProcessorRequest {
    pub script_kind: ScriptKind,
    pub script_path: PathBuf,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub prompt: String,
    pub vars: Value,
    pub env: BTreeMap<String, String>,
    pub options: ScriptSandboxOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromptProcessorResponse {
    pub prompt: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

pub struct ScriptPromptProcessor;

impl PromptProcessorRequest {
    pub fn new(
        script_kind: ScriptKind,
        script_path: impl Into<PathBuf>,
        program: impl Into<PathBuf>,
        args: Vec<String>,
        prompt: impl Into<String>,
        vars: Value,
        options: ScriptSandboxOptions,
    ) -> Self {
        Self {
            script_kind,
            script_path: script_path.into(),
            program: program.into(),
            args,
            prompt: prompt.into(),
            vars,
            env: BTreeMap::new(),
            options,
        }
    }
}

impl ScriptPromptProcessor {
    pub fn process(
        request: PromptProcessorRequest,
        auth: ScriptAuthorization,
    ) -> Result<PromptProcessorResponse, ScriptBridgeError> {
        let script_kind = request.script_kind;
        let script_path = request.script_path.clone();
        let stdin = json!({
            "prompt": request.prompt,
            "vars": request.vars,
        })
        .to_string();
        let response = ScriptBridge::execute(
            ScriptRequest {
                script_kind,
                script_path: script_path.clone(),
                program: request.program,
                args: request.args,
                stdin,
                env: request.env,
                options: request.options,
            },
            auth,
        )?;

        if response.exit_code != Some(0) {
            return Err(ScriptBridgeError::new(
                ScriptBridgeErrorKind::Io,
                "script_prompt_processor_failed",
                script_kind,
                script_path,
                format!(
                    "prompt processor exited with {:?}: {}",
                    response.exit_code, response.stderr
                ),
            ));
        }

        let prompt = parse_prompt_processor_stdout(&response.stdout).map_err(|message| {
            ScriptBridgeError::new(
                ScriptBridgeErrorKind::Io,
                "script_prompt_processor_invalid_output",
                script_kind,
                script_path,
                message,
            )
        })?;

        Ok(PromptProcessorResponse {
            prompt,
            stdout: response.stdout,
            stderr: response.stderr,
            exit_code: response.exit_code,
        })
    }
}

fn parse_prompt_processor_stdout(stdout: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(stdout.trim()).map_err(|err| {
        format!("prompt processor stdout must be JSON with a prompt string: {err}")
    })?;
    if let Some(prompt) = value.as_str() {
        return Ok(prompt.to_string());
    }
    value
        .get("prompt")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| "prompt processor stdout JSON must contain a prompt string".to_string())
}
