use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::config::{load_promptfoo_config, EnvOverlay};
use crate::eval::{run_eval, EvalOptions};
use crate::mcp::tool_listing;
use crate::output::write_sarif;
use crate::redteam::{
    load_redteam_config, run_redteam_flow, write_redteam_report_file, MockTarget,
};
use crate::scan::{known_limitations, run_scan, ScanInput};

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
    Redteam(RedteamArgs),
    /// Run MCP compatibility workflows.
    Mcp(McpArgs),
    /// Run code scan workflows.
    CodeScans(ScanArgs),
    /// Run model scan workflows.
    ScanModel(ScanArgs),
    /// Run model audit workflows.
    ModelAudit(ScanArgs),
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

#[derive(Debug, Args)]
pub struct RedteamArgs {
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(long = "stage", value_enum, default_value_t = RedteamStageArg::Run)]
    pub stage: RedteamStageArg,
    #[arg(long = "report", value_name = "FILE")]
    pub report: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum RedteamStageArg {
    Init,
    Generate,
    Eval,
    Run,
    Report,
}

#[derive(Debug, Args)]
pub struct McpArgs {
    #[arg(long = "mode", value_enum, default_value_t = McpModeArg::ListTools)]
    pub mode: McpModeArg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum McpModeArg {
    ListTools,
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    #[arg(long = "input", value_name = "FILE")]
    pub input: PathBuf,
    #[arg(long = "format", value_enum, default_value_t = ScanFormatArg::Json)]
    pub format: ScanFormatArg,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ScanFormatArg {
    Json,
    Sarif,
}

pub fn run_cli(cli: Cli) -> Result<ExitCode, CliError> {
    match cli.command {
        Some(Command::Eval(args)) => handle_eval_command(args),
        Some(Command::Redteam(args)) => handle_redteam_command(args),
        Some(Command::Mcp(args)) => handle_mcp_command(args),
        Some(Command::CodeScans(args)) => handle_scan_command("code-scans", args),
        Some(Command::ScanModel(args)) => handle_scan_command("scan-model", args),
        Some(Command::ModelAudit(args)) => handle_scan_command("model-audit", args),
        Some(Command::View | Command::Cache | Command::Import | Command::Export) | None => {
            Ok(ExitCode::SUCCESS)
        }
    }
}

pub fn handle_mcp_command(args: McpArgs) -> Result<ExitCode, CliError> {
    match args.mode {
        McpModeArg::ListTools => {
            let json = serde_json::to_string(&tool_listing()).map_err(|err| {
                CliError::new(format!("mcp tool listing serialization failed: {err}"))
            })?;
            println!("{json}");
            Ok(ExitCode::SUCCESS)
        }
    }
}

pub fn handle_scan_command(command: &'static str, args: ScanArgs) -> Result<ExitCode, CliError> {
    let input = ScanInput::from_path(&args.input, command)
        .map_err(|err| CliError::new(format!("scan input {}: {err}", args.input.display())))?;
    let findings = run_scan(input).map_err(|err| CliError::new(err.to_string()))?;
    match args.format {
        ScanFormatArg::Json => {
            let json = serde_json::json!({
                "schema_version": "promptfoo-rs.scan.v1",
                "command": command,
                "findings": findings,
                "known_limitations": known_limitations(),
            });
            println!(
                "{}",
                serde_json::to_string(&json).map_err(|err| {
                    CliError::new(format!("scan result serialization failed: {err}"))
                })?
            );
        }
        ScanFormatArg::Sarif => {
            let mut output = Vec::new();
            write_sarif(&findings, &mut output)
                .map_err(|err| CliError::new(format!("SARIF serialization failed: {err}")))?;
            println!(
                "{}",
                String::from_utf8(output).map_err(|err| {
                    CliError::new(format!("SARIF output was not valid UTF-8: {err}"))
                })?
            );
        }
    }
    Ok(ExitCode::SUCCESS)
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

pub fn handle_redteam_command(args: RedteamArgs) -> Result<ExitCode, CliError> {
    let Some(config_path) = args.config else {
        return Ok(ExitCode::SUCCESS);
    };
    let config = load_redteam_config(&config_path)
        .map_err(|err| CliError::new(format!("redteam config {}: {err}", config_path.display())))?;
    let target = MockTarget::new(config.target.id.clone()).with_blocked_keyword("secret");
    let report =
        run_redteam_flow(config.clone(), target).map_err(|err| CliError::new(err.to_string()))?;

    let report_path = args.report.or_else(|| {
        config
            .report
            .as_ref()
            .map(|report| PathBuf::from(&report.path))
    });
    if let Some(report_path) = report_path {
        write_redteam_report_file(&report, &report_path)
            .map_err(|err| CliError::new(err.to_string()))?;
    } else {
        let json = serde_json::to_string(&report)
            .map_err(|err| CliError::new(format!("redteam report serialization failed: {err}")))?;
        println!("{json}");
    }
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
