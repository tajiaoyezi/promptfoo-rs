use std::cmp::max;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::thread;

use serde_json::Value;

use crate::script_bridge::{
    ScriptAuthorization, ScriptBridge, ScriptBridgeError, ScriptBridgeErrorKind, ScriptKind,
    ScriptRequest, ScriptSandboxOptions,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RubyBridgeRequest {
    pub script_path: PathBuf,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub payload: Value,
    pub env: BTreeMap<String, String>,
    pub options: ScriptSandboxOptions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RubyBridgeResponse {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub json: Value,
}

pub struct RubyBridge;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RubyWorkerPool {
    max_workers: usize,
}

impl RubyBridgeRequest {
    pub fn new(
        script_path: impl Into<PathBuf>,
        program: impl Into<PathBuf>,
        args: Vec<String>,
        payload: Value,
        options: ScriptSandboxOptions,
    ) -> Self {
        Self {
            script_path: script_path.into(),
            program: program.into(),
            args,
            payload,
            env: BTreeMap::new(),
            options,
        }
    }
}

impl RubyBridge {
    pub fn call(
        request: RubyBridgeRequest,
        auth: ScriptAuthorization,
    ) -> Result<RubyBridgeResponse, ScriptBridgeError> {
        let script_path = request.script_path.clone();
        let response = ScriptBridge::execute(
            ScriptRequest {
                script_kind: ScriptKind::Ruby,
                script_path: script_path.clone(),
                program: request.program,
                args: request.args,
                stdin: request.payload.to_string(),
                env: request.env,
                options: request.options,
            },
            auth,
        )?;

        if response.exit_code != Some(0) {
            return Err(ScriptBridgeError::new(
                ScriptBridgeErrorKind::Io,
                "ruby_bridge_failed",
                ScriptKind::Ruby,
                script_path,
                format!(
                    "ruby bridge exited with {:?}: {}",
                    response.exit_code, response.stderr
                ),
            ));
        }

        let json = serde_json::from_str(response.stdout.trim()).map_err(|err| {
            ScriptBridgeError::new(
                ScriptBridgeErrorKind::Io,
                "ruby_bridge_invalid_json",
                ScriptKind::Ruby,
                script_path,
                format!("ruby bridge stdout must be JSON: {err}"),
            )
        })?;

        Ok(RubyBridgeResponse {
            exit_code: response.exit_code,
            stdout: response.stdout,
            stderr: response.stderr,
            json,
        })
    }
}

impl RubyWorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers: max(1, max_workers),
        }
    }

    pub fn execute(
        &self,
        requests: Vec<RubyBridgeRequest>,
        auth: ScriptAuthorization,
    ) -> Vec<Result<RubyBridgeResponse, ScriptBridgeError>> {
        let mut results = Vec::with_capacity(requests.len());
        for chunk in requests.chunks(self.max_workers) {
            let handles = chunk
                .iter()
                .cloned()
                .map(|request| thread::spawn(move || RubyBridge::call(request, auth)))
                .collect::<Vec<_>>();
            for handle in handles {
                results.push(handle.join().expect("ruby worker thread should not panic"));
            }
        }
        results
    }
}
