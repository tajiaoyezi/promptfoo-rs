use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::config::{load_promptfoo_config, EnvOverlay};
use crate::eval::{run_eval, EvalOptions};

#[derive(Debug, Parser)]
#[command(
    name = "promptfoo-rs",
    version,
    about = "Promptfoo-compatible Rust CLI skeleton"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    pub fn parse_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::parse_from(args)
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run an eval from a promptfoo config file.
    Eval(EvalArgs),
    /// Open the local result viewer.
    View,
    /// Manage local eval cache state.
    Cache,
    /// Run redteam workflows.
    Redteam,
    /// Run MCP compatibility workflows.
    Mcp,
    /// Run code scan workflows.
    CodeScans,
    /// Run model scan workflows.
    ScanModel,
    /// Import promptfoo artifacts.
    Import,
    /// Export promptfoo artifacts.
    Export,
}

#[derive(Debug, Args)]
pub struct EvalArgs {
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,
}

pub fn run_cli(cli: Cli) -> Result<ExitCode, CliError> {
    match cli.command {
        Some(Command::Eval(args)) => handle_eval_command(args),
        Some(
            Command::View
            | Command::Cache
            | Command::Redteam
            | Command::Mcp
            | Command::CodeScans
            | Command::ScanModel
            | Command::Import
            | Command::Export,
        )
        | None => Ok(ExitCode::SUCCESS),
    }
}

pub fn handle_eval_command(args: EvalArgs) -> Result<ExitCode, CliError> {
    let config_path = args
        .config
        .ok_or_else(|| CliError::new("config path is required for eval (-c, --config)"))?;
    let config = load_promptfoo_config(&config_path, &EnvOverlay::default())
        .map_err(|err| CliError::new(format!("config {}: {err}", config_path.display())))?;
    let envelope = run_eval(config, EvalOptions::default()).map_err(CliError::new)?;
    let json = serde_json::to_string(&envelope)
        .map_err(|err| CliError::new(format!("result envelope serialization failed: {err}")))?;
    println!("{json}");
    Ok(ExitCode::SUCCESS)
}

pub fn main() -> ExitCode {
    match run_cli(Cli::parse()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug)]
pub struct CliError {
    message: String,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

impl CliError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
