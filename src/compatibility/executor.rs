use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::compatibility::harness::HarnessError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub env_clear: bool,
    pub timeout_ms: u64,
}

impl CommandSpec {
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            env: BTreeMap::new(),
            env_clear: true,
            timeout_ms: 120_000,
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    pub fn env_clear(mut self, env_clear: bool) -> Self {
        self.env_clear = env_clear;
        self
    }

    pub fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandExecution {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub duration_ms: u128,
}

pub fn execute_command(
    spec: &CommandSpec,
    working_dir: &Path,
) -> Result<CommandExecution, HarnessError> {
    let mut command = Command::new(&spec.program);
    command
        .args(&spec.args)
        .current_dir(working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if spec.env_clear {
        command.env_clear();
    }
    command.envs(&spec.env);

    let start = Instant::now();
    let mut child = command.spawn().map_err(|error| {
        HarnessError::new(format!(
            "failed to spawn command {}: {error}",
            spec.program.display()
        ))
    })?;

    let timeout = Duration::from_millis(spec.timeout_ms.max(1));
    let mut timed_out = false;
    loop {
        if child
            .try_wait()
            .map_err(|error| HarnessError::new(format!("failed to poll command: {error}")))?
            .is_some()
        {
            break;
        }
        if start.elapsed() >= timeout {
            timed_out = true;
            let _ = child.kill();
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

    let output = child
        .wait_with_output()
        .map_err(|error| HarnessError::new(format!("failed to collect command output: {error}")))?;

    Ok(CommandExecution {
        program: spec.program.display().to_string(),
        args: spec.args.clone(),
        env: spec.env.clone(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code(),
        timed_out,
        duration_ms: start.elapsed().as_millis(),
    })
}
