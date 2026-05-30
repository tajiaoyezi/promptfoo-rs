use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::compatibility::matrix::CapabilityMatrix;
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
    View(ViewArgs),
    /// Manage local eval cache state.
    Cache(CacheArgs),
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
    Import(ImportArgs),
    /// Export promptfoo artifacts.
    Export(ExportArgs),
}

#[derive(Debug, Args)]
pub struct EvalArgs {
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,
    #[arg(long = "max-concurrency", value_name = "N")]
    pub max_concurrency: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ViewArgs {
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct CacheArgs {}

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

#[derive(Debug, Args)]
pub struct ImportArgs {
    pub file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    #[arg(long = "output", value_name = "FILE")]
    pub output: Option<PathBuf>,
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
        Some(Command::View(_)) => Err(unsupported_command_error(
            "view",
            "not yet implemented; local viewer CLI launch is tracked as command:view-directory",
        )),
        Some(Command::Cache(_)) => Err(unsupported_command_error(
            "cache",
            "not yet implemented; cache subcommands are tracked for task 13.2",
        )),
        Some(Command::Import(_)) => Err(unsupported_command_error(
            "import",
            "not yet implemented; promptfoo artifact import is tracked as command:import-file",
        )),
        Some(Command::Export(_)) => Err(unsupported_command_error(
            "export",
            "not yet implemented; promptfoo artifact export is tracked as command:export",
        )),
        None => Err(unsupported_command_error(
            "promptfoo-rs",
            "command is required; run promptfoo-rs --help",
        )),
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
    if args.output.is_some() {
        return Err(unsupported_command_error(
            "eval --output",
            "not yet implemented; output file parity is tracked for task 13.2",
        ));
    }
    if args.max_concurrency.is_some() {
        return Err(unsupported_command_error(
            "eval --max-concurrency",
            "not yet implemented; scheduler parity is tracked for task 13.2",
        ));
    }
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

pub fn unsupported_command_error(command: &str, reason: &str) -> CliError {
    CliError::new(format!("{command}: {reason}"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandInventory {
    pub items: Vec<CommandInventoryItem>,
}

impl CommandInventory {
    pub fn from_matrix(matrix: &CapabilityMatrix) -> Self {
        let mut items = matrix
            .rows
            .iter()
            .filter(|row| {
                row.capability.starts_with("command:") || row.capability.starts_with("flag:")
            })
            .map(|row| CommandInventoryItem {
                stable_id: row.capability.clone(),
                level: row.level.clone(),
            })
            .collect::<Vec<_>>();
        items.sort_by(|left, right| left.stable_id.cmp(&right.stable_id));
        Self { items }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandInventoryItem {
    pub stable_id: String,
    pub level: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliSurface {
    pub items: Vec<CliSurfaceItem>,
}

impl CliSurface {
    pub fn current() -> Self {
        Self {
            items: vec![
                CliSurfaceItem::implemented("command:eval"),
                CliSurfaceItem::later("command:view-directory"),
                CliSurfaceItem::later("command:cache"),
                CliSurfaceItem::implemented("command:redteam"),
                CliSurfaceItem::implemented("command:mcp"),
                CliSurfaceItem::implemented("command:code-scans"),
                CliSurfaceItem::implemented("command:scan-model"),
                CliSurfaceItem::later("command:import-file"),
                CliSurfaceItem::later("command:export"),
                CliSurfaceItem::implemented("flag:config"),
                CliSurfaceItem::later("flag:output"),
                CliSurfaceItem::later("flag:max-concurrency"),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliSurfaceItem {
    pub stable_id: String,
    pub status: CliItemStatus,
    pub empty_success: bool,
}

impl CliSurfaceItem {
    fn implemented(stable_id: &str) -> Self {
        Self::new(stable_id, CliItemStatus::Implemented)
    }

    fn later(stable_id: &str) -> Self {
        Self::new(stable_id, CliItemStatus::Later)
    }

    fn new(stable_id: &str, status: CliItemStatus) -> Self {
        Self {
            stable_id: stable_id.to_string(),
            status,
            empty_success: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CliItemStatus {
    Implemented,
    Unsupported,
    Later,
    Blocked,
}

impl CliItemStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::Unsupported => "unsupported",
            Self::Later => "later",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliParityReport {
    pub unmapped_items: Vec<String>,
    pub empty_success_commands: Vec<String>,
    pub status_by_item: Vec<(String, String)>,
}

pub fn validate_cli_surface(cli: &CliSurface, inventory: &CommandInventory) -> CliParityReport {
    let surface_by_id = cli
        .items
        .iter()
        .map(|item| (item.stable_id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let inventory_ids = inventory
        .items
        .iter()
        .map(|item| item.stable_id.as_str())
        .collect::<BTreeSet<_>>();

    let mut unmapped_items = Vec::new();
    let mut status_by_item = Vec::new();
    for item in &inventory.items {
        if let Some(surface_item) = surface_by_id.get(item.stable_id.as_str()) {
            status_by_item.push((
                item.stable_id.clone(),
                surface_item.status.as_str().to_string(),
            ));
        } else {
            unmapped_items.push(item.stable_id.clone());
        }
    }

    let empty_success_commands = cli
        .items
        .iter()
        .filter(|item| {
            item.stable_id.starts_with("command:")
                && inventory_ids.contains(item.stable_id.as_str())
                && item.empty_success
        })
        .map(|item| item.stable_id.clone())
        .collect();

    CliParityReport {
        unmapped_items,
        empty_success_commands,
        status_by_item,
    }
}
