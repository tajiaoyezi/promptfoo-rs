use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

use crate::cache::resume::ResumeStore;
use crate::compatibility::matrix::CapabilityMatrix;
use crate::config::{load_promptfoo_config, EnvOverlay};
use crate::eval::{run_eval, EvalOptions, EvalResultEnvelope};
use crate::mcp::tool_listing;
use crate::output::{write_output, write_sarif, OutputError, OutputFormat, RunSummary};
use crate::redteam::{
    load_redteam_config, run_redteam_flow, write_redteam_report_file, MockTarget,
};
use crate::results::{AssertionResultRecord, ResultRecord, ResultStatus};
use crate::scan::{known_limitations, run_scan, ScanInput};
use crate::viewer_server::{
    build_results_table, export_viewer_records, load_viewer_records, ExportFormat, ResultSource,
    ViewerFilter,
};

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
    #[arg(long = "output", value_name = "FILE", action = ArgAction::Append)]
    pub output: Vec<PathBuf>,
    #[arg(long = "max-concurrency", value_name = "N")]
    pub max_concurrency: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ViewArgs {
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct CacheArgs {
    #[arg(
        long = "path",
        value_name = "FILE",
        default_value = ".promptfoo-rs-cache.jsonl"
    )]
    pub path: PathBuf,
    #[arg(long = "expected-case", value_name = "CASE_ID", action = ArgAction::Append)]
    pub expected_cases: Vec<String>,
    #[arg(long = "clear", action = ArgAction::SetTrue)]
    pub clear: bool,
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

#[derive(Debug, Args)]
pub struct ImportArgs {
    pub file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    #[arg(long = "input", value_name = "FILE")]
    pub input: Option<PathBuf>,
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
        Some(Command::View(args)) => handle_view_command(args),
        Some(Command::Cache(args)) => handle_cache_command(args),
        Some(Command::Import(args)) => handle_import_command(args),
        Some(Command::Export(args)) => handle_export_command(args),
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

pub fn handle_view_command(args: ViewArgs) -> Result<ExitCode, CliError> {
    let source = resolve_viewer_source(args.directory)?;
    let records =
        load_viewer_records(source.clone()).map_err(|err| CliError::new(format!("view: {err}")))?;
    let table = build_results_table(&records, ViewerFilter::default());
    let (source_kind, source_path) = source_descriptor(&source);
    let payload = serde_json::json!({
        "schema_version": "promptfoo-rs.viewer.cli.v1",
        "source": {
            "kind": source_kind,
            "path": source_path.to_string_lossy(),
        },
        "record_count": records.len(),
        "columns": table.columns,
        "rows": table.rows,
    });
    println!(
        "{}",
        serde_json::to_string(&payload)
            .map_err(|err| CliError::new(format!("view result serialization failed: {err}")))?
    );
    Ok(ExitCode::SUCCESS)
}

pub fn handle_cache_command(args: CacheArgs) -> Result<ExitCode, CliError> {
    if args.clear {
        if args.path.exists() {
            std::fs::remove_file(&args.path).map_err(|err| {
                CliError::new(format!(
                    "cache: failed to clear {}: {err}",
                    args.path.display()
                ))
            })?;
        }
        let payload = serde_json::json!({
            "schema_version": "promptfoo-rs.cache.cli.v1",
            "status": "cleared",
            "path": args.path.to_string_lossy(),
            "upload_attempts": 0,
        });
        println!(
            "{}",
            serde_json::to_string(&payload).map_err(|err| CliError::new(format!(
                "cache result serialization failed: {err}"
            )))?
        );
        return Ok(ExitCode::SUCCESS);
    }

    let state = if args.path.exists() {
        ResumeStore::load(&args.path).map_err(|err| {
            CliError::new(format!(
                "cache: failed to load {}: {err}",
                args.path.display()
            ))
        })?
    } else {
        Default::default()
    };
    let completed_cases = state.completed_case_ids();
    let remaining_cases = state.remaining_cases(&args.expected_cases);
    let corrupt_records = state
        .corrupt_records
        .iter()
        .map(|record| {
            serde_json::json!({
                "line": record.line,
                "message": record.message,
            })
        })
        .collect::<Vec<_>>();
    let payload = serde_json::json!({
        "schema_version": "promptfoo-rs.cache.cli.v1",
        "path": args.path.to_string_lossy(),
        "completed_count": completed_cases.len(),
        "completed_cases": completed_cases,
        "corrupt_count": state.corrupt_records.len(),
        "corrupt_records": corrupt_records,
        "remaining_cases": remaining_cases,
        "upload_attempts": 0,
    });
    println!(
        "{}",
        serde_json::to_string(&payload)
            .map_err(|err| CliError::new(format!("cache result serialization failed: {err}")))?
    );
    Ok(ExitCode::SUCCESS)
}

pub fn handle_import_command(args: ImportArgs) -> Result<ExitCode, CliError> {
    let Some(file) = args.file else {
        return Err(CliError::new("import: file path is required"));
    };
    let source = resolve_viewer_source(Some(file))?;
    let records = load_viewer_records(source.clone())
        .map_err(|err| CliError::new(format!("import: {err}")))?;
    let (source_kind, source_path) = source_descriptor(&source);
    let payload = serde_json::json!({
        "schema_version": "promptfoo-rs.import.cli.v1",
        "source": {
            "kind": source_kind,
            "path": source_path.to_string_lossy(),
        },
        "record_count": records.len(),
        "status_counts": status_counts(&records),
        "upload_attempts": 0,
    });
    println!(
        "{}",
        serde_json::to_string(&payload)
            .map_err(|err| CliError::new(format!("import result serialization failed: {err}")))?
    );
    Ok(ExitCode::SUCCESS)
}

pub fn handle_export_command(args: ExportArgs) -> Result<ExitCode, CliError> {
    let Some(input) = args.input else {
        return Err(CliError::new("export: --input is required"));
    };
    let Some(output) = args.output else {
        return Err(CliError::new("export: --output is required"));
    };
    let source = resolve_viewer_source(Some(input))?;
    let records = load_viewer_records(source.clone())
        .map_err(|err| CliError::new(format!("export: {err}")))?;
    let table = build_results_table(&records, ViewerFilter::default());
    let format = export_format_from_path(&output)?;
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            CliError::new(format!(
                "export: failed to create {}: {err}",
                parent.display()
            ))
        })?;
    }
    let body = export_viewer_records(&table, format.clone())
        .map_err(|err| CliError::new(format!("export: {err}")))?;
    std::fs::write(&output, body).map_err(|err| {
        CliError::new(format!(
            "export: failed to write {}: {err}",
            output.display()
        ))
    })?;
    let (source_kind, source_path) = source_descriptor(&source);
    let payload = serde_json::json!({
        "schema_version": "promptfoo-rs.export.cli.v1",
        "input": source_path.to_string_lossy(),
        "input_kind": source_kind,
        "output": output.to_string_lossy(),
        "format": export_format_name(&format),
        "record_count": records.len(),
        "upload_attempts": 0,
    });
    println!(
        "{}",
        serde_json::to_string(&payload)
            .map_err(|err| CliError::new(format!("export result serialization failed: {err}")))?
    );
    Ok(ExitCode::SUCCESS)
}

pub fn resolve_viewer_source(path: Option<PathBuf>) -> Result<ResultSource, CliError> {
    let path = match path {
        Some(path) => path,
        None => std::env::current_dir()
            .map_err(|err| CliError::new(format!("view: current directory unavailable: {err}")))?,
    };
    if path.is_dir() {
        for candidate in [
            "results.jsonl",
            "results.sqlite",
            "results.sqlite3",
            "results.db",
        ] {
            let candidate_path = path.join(candidate);
            if candidate_path.is_file() {
                return source_from_file(candidate_path);
            }
        }
        return Err(CliError::new(format!(
            "view: no results.jsonl, results.sqlite, results.sqlite3, or results.db found in {}",
            path.display()
        )));
    }
    if path.is_file() {
        return source_from_file(path);
    }
    Err(CliError::new(format!(
        "view: result source not found: {}",
        path.display()
    )))
}

fn source_from_file(path: PathBuf) -> Result<ResultSource, CliError> {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "sqlite" | "sqlite3" | "db" => Ok(ResultSource::sqlite(path)),
        "jsonl" => Ok(ResultSource::jsonl(path)),
        _ => Err(CliError::new(format!(
            "view: unsupported result source format: {}",
            path.display()
        ))),
    }
}

fn source_descriptor(source: &ResultSource) -> (&'static str, &Path) {
    match source {
        ResultSource::Jsonl(path) => ("jsonl", path.as_path()),
        ResultSource::Sqlite(path) => ("sqlite", path.as_path()),
    }
}

fn status_counts(records: &[ResultRecord]) -> BTreeMap<&'static str, usize> {
    let mut counts = BTreeMap::new();
    for record in records {
        *counts.entry(record.status.as_str()).or_insert(0) += 1;
    }
    counts
}

fn export_format_from_path(path: &Path) -> Result<ExportFormat, CliError> {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "json" => Ok(ExportFormat::Json),
        "csv" => Ok(ExportFormat::Csv),
        _ => Err(CliError::new(format!(
            "export: unsupported output format for {}; use .json or .csv",
            path.display()
        ))),
    }
}

fn export_format_name(format: &ExportFormat) -> &'static str {
    match format {
        ExportFormat::Json => "json",
        ExportFormat::Csv => "csv",
    }
}

pub fn handle_eval_command(args: EvalArgs) -> Result<ExitCode, CliError> {
    let artifacts = run_eval_cli(args)?;
    let json = serde_json::to_string(&artifacts.envelope)
        .map_err(|err| CliError::new(format!("result envelope serialization failed: {err}")))?;
    println!("{json}");
    if artifacts.envelope.status == "ok" {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

pub type EvalCliArgs = EvalArgs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CliRunArtifacts {
    pub envelope: EvalResultEnvelope,
    pub outputs: Vec<OutputArtifact>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputTarget {
    pub path: PathBuf,
    pub format: OutputFormat,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputArtifact {
    pub path: PathBuf,
    pub format: OutputFormat,
}

pub fn run_eval_cli(args: EvalCliArgs) -> Result<CliRunArtifacts, CliError> {
    let config_path = args
        .config
        .clone()
        .ok_or_else(|| CliError::new("config path is required for eval (-c, --config)"))?;
    let config = load_promptfoo_config(&config_path, &EnvOverlay::default())
        .map_err(|err| CliError::new(format!("config {}: {err}", config_path.display())))?;
    let envelope = run_eval(
        config,
        EvalOptions {
            max_concurrency: args.max_concurrency,
            ..EvalOptions::default()
        },
    )
    .map_err(CliError::new)?;
    let output_targets = args
        .output
        .iter()
        .map(|path| OutputTarget::from_path(path.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    let outputs = write_requested_outputs(&envelope, &output_targets)
        .map_err(|err| CliError::new(err.to_string()))?;
    Ok(CliRunArtifacts { envelope, outputs })
}

pub fn write_requested_outputs(
    envelope: &EvalResultEnvelope,
    outputs: &[OutputTarget],
) -> Result<Vec<OutputArtifact>, OutputError> {
    let summary = run_summary_from_envelope(envelope);
    let mut artifacts = Vec::new();
    for output in outputs {
        if let Some(parent) = output.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = std::fs::File::create(&output.path)?;
        write_output(output.format, &summary, file)?;
        artifacts.push(OutputArtifact {
            path: output.path.clone(),
            format: output.format,
        });
    }
    Ok(artifacts)
}

impl OutputTarget {
    fn from_path(path: PathBuf) -> Result<Self, CliError> {
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let format = match extension.as_str() {
            "json" => OutputFormat::Json,
            "jsonl" => OutputFormat::Jsonl,
            "csv" => OutputFormat::Csv,
            "yml" | "yaml" => OutputFormat::Yaml,
            "xml" => OutputFormat::Junit,
            "sarif" => OutputFormat::Sarif,
            "html" | "htm" => OutputFormat::Html,
            _ => {
                return Err(CliError::new(format!(
                    "unsupported output format for {}",
                    path.display()
                )))
            }
        };
        Ok(Self { path, format })
    }
}

fn run_summary_from_envelope(envelope: &EvalResultEnvelope) -> RunSummary {
    RunSummary {
        eval_id: "eval-cli".to_string(),
        records: envelope
            .results
            .iter()
            .map(|result| ResultRecord {
                eval_id: "eval-cli".to_string(),
                case_id: result.case_id.clone(),
                provider_id: result.provider_id.clone(),
                status: result_status(&result.status),
                result: Some(serde_json::json!({ "output": result.output })),
                assertion_results: result
                    .assertion_results
                    .iter()
                    .map(|assertion| AssertionResultRecord {
                        assertion_type: assertion.assertion_type.clone(),
                        status: result_status(&assertion.status),
                        message: assertion.message.clone(),
                    })
                    .collect(),
                latency_ms: 0,
                metadata: serde_json::json!({ "source": "eval-cli" }),
                error: result.error.clone(),
            })
            .collect(),
    }
}

fn result_status(status: &str) -> ResultStatus {
    match status {
        "passed" => ResultStatus::Passed,
        "failed" => ResultStatus::Failed,
        "error" => ResultStatus::Error,
        _ => ResultStatus::Skipped,
    }
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
                CliSurfaceItem::implemented("command:view-directory"),
                CliSurfaceItem::implemented("command:cache"),
                CliSurfaceItem::implemented("command:redteam"),
                CliSurfaceItem::implemented("command:mcp"),
                CliSurfaceItem::implemented("command:code-scans"),
                CliSurfaceItem::implemented("command:scan-model"),
                CliSurfaceItem::implemented("command:import-file"),
                CliSurfaceItem::implemented("command:export"),
                CliSurfaceItem::implemented("flag:config"),
                CliSurfaceItem::implemented("flag:output"),
                CliSurfaceItem::implemented("flag:max-concurrency"),
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
