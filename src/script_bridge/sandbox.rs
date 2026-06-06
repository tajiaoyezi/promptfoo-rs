use std::collections::BTreeMap;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::script_bridge::{
    reject_unauthorized_script, ScriptAuthorization, ScriptBridgeError, ScriptBridgeErrorKind,
    ScriptKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptRequest {
    pub script_kind: ScriptKind,
    pub script_path: PathBuf,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub stdin: String,
    pub env: BTreeMap<String, String>,
    pub options: ScriptSandboxOptions,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptSandboxOptions {
    pub timeout: Duration,
    pub env_allowlist: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub stdin_limit: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptResponse {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub struct ScriptBridge;

impl ScriptBridge {
    pub fn execute(
        request: ScriptRequest,
        auth: ScriptAuthorization,
    ) -> Result<ScriptResponse, ScriptBridgeError> {
        if auth == ScriptAuthorization::Deny {
            return Err(reject_unauthorized_script(
                request.script_kind,
                &request.script_path,
            ));
        }
        if request.stdin.len() > request.options.stdin_limit {
            return Err(ScriptBridgeError::new(
                ScriptBridgeErrorKind::StdinLimitExceeded,
                "script_stdin_limit_exceeded",
                request.script_kind,
                request.script_path,
                format!(
                    "script stdin exceeded configured limit of {} bytes",
                    request.options.stdin_limit
                ),
            ));
        }

        let mut command = Command::new(&request.program);
        command
            .args(&request.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env_clear();
        if let Some(cwd) = &request.options.cwd {
            command.current_dir(cwd);
        }
        preserve_platform_process_env(&mut command);
        for key in &request.options.env_allowlist {
            if let Some(value) = request.env.get(key) {
                command.env(key, value);
            }
        }

        let mut child = command.spawn().map_err(|err| {
            script_io_error(
                request.script_kind,
                request.script_path.clone(),
                format!("failed to spawn script process: {err}"),
            )
        })?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(request.stdin.as_bytes()).map_err(|err| {
                script_io_error(
                    request.script_kind,
                    request.script_path.clone(),
                    format!("failed to write script stdin: {err}"),
                )
            })?;
        }

        let started_at = Instant::now();
        loop {
            if child
                .try_wait()
                .map_err(|err| {
                    script_io_error(
                        request.script_kind,
                        request.script_path.clone(),
                        format!("failed to poll script process: {err}"),
                    )
                })?
                .is_some()
            {
                let output = child.wait_with_output().map_err(|err| {
                    script_io_error(
                        request.script_kind,
                        request.script_path.clone(),
                        format!("failed to collect script output: {err}"),
                    )
                })?;
                return Ok(ScriptResponse {
                    exit_code: output.status.code(),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                });
            }
            if started_at.elapsed() >= request.options.timeout {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ScriptBridgeError::new(
                    ScriptBridgeErrorKind::Timeout,
                    "script_timeout",
                    request.script_kind,
                    request.script_path,
                    format!(
                        "script exceeded timeout of {}ms",
                        request.options.timeout.as_millis()
                    ),
                ));
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
}

fn script_io_error(
    script_kind: ScriptKind,
    path: PathBuf,
    message: impl Into<String>,
) -> ScriptBridgeError {
    ScriptBridgeError::new(
        ScriptBridgeErrorKind::Io,
        "script_io_error",
        script_kind,
        path,
        message,
    )
}

#[cfg(windows)]
fn preserve_platform_process_env(command: &mut Command) {
    for key in ["SystemRoot", "WINDIR"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
}

#[cfg(not(windows))]
fn preserve_platform_process_env(command: &mut Command) {
    for key in ["PATH", "HOME", "TMPDIR", "LANG"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
}
