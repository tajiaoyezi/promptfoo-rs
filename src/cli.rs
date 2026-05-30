use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

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

pub fn run_cli(_cli: Cli) -> Result<ExitCode, CliError> {
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
