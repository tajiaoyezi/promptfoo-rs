use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

use crate::cache::resume::ResumeStore;
use crate::compatibility::matrix::CapabilityMatrix;
use crate::config::{
    load_promptfoo_config, EnvOverlay, NormalizedConfig, NormalizedPrompt, NormalizedProvider,
    NormalizedTestCase,
};
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
    /// Initialize a promptfoo project.
    Init(GapCommandArgs),
    /// Share eval results with promptfoo cloud.
    Share(GapCommandArgs),
    /// Manage promptfoo cloud authentication.
    Auth(GapCommandArgs),
    /// Manage promptfoo configuration.
    Config(GapCommandArgs),
    /// Run promptfoo debug workflows.
    Debug(GapCommandArgs),
    /// Delete promptfoo cloud resources.
    Delete(GapCommandArgs),
    /// Generate promptfoo assets.
    Generate(GapCommandArgs),
    /// Send promptfoo feedback.
    Feedback(GapCommandArgs),
    /// List promptfoo cloud resources.
    List(GapCommandArgs),
    /// Read promptfoo cloud logs.
    Logs(GapCommandArgs),
    /// Optimize prompts.
    Optimize(GapCommandArgs),
    /// Retry an eval.
    Retry(GapCommandArgs),
    /// Validate promptfoo configuration.
    Validate(GapCommandArgs),
    /// Show promptfoo resources.
    Show(GapCommandArgs),
}

#[derive(Debug, Args)]
pub struct EvalArgs {
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(long = "prompts", value_name = "PROMPT", action = ArgAction::Append)]
    pub prompts: Vec<String>,
    #[arg(long = "providers", value_name = "PROVIDER", action = ArgAction::Append)]
    pub providers: Vec<String>,
    #[arg(long = "tests", value_name = "TEST", action = ArgAction::Append)]
    pub tests: Vec<String>,
    #[arg(long = "vars", value_name = "KEY=VALUE", action = ArgAction::Append)]
    pub vars: Vec<String>,
    #[arg(long = "output", value_name = "FILE", action = ArgAction::Append)]
    pub output: Vec<PathBuf>,
    #[arg(long = "max-concurrency", value_name = "N")]
    pub max_concurrency: Option<usize>,
    #[arg(long = "repeat", value_name = "N")]
    pub repeat: Option<usize>,
    #[arg(long = "delay", value_name = "MILLISECONDS")]
    pub delay_ms: Option<u64>,
    #[arg(long = "no-cache", action = ArgAction::SetTrue)]
    pub no_cache: bool,
    #[arg(long = "resume", action = ArgAction::SetTrue)]
    pub resume: bool,
    #[arg(long = "retry-errors", action = ArgAction::SetTrue)]
    pub retry_errors: bool,
    #[arg(long = "filter-sample", value_name = "CASE_ID", action = ArgAction::Append)]
    pub filter_sample: Vec<String>,
    #[arg(long = "env-file", value_name = "FILE")]
    pub env_file: Option<PathBuf>,
    #[arg(long = "no-write", action = ArgAction::SetTrue)]
    pub no_write: bool,
    #[arg(long = "table", action = ArgAction::SetTrue)]
    pub table: bool,
    #[arg(long = "no-table", action = ArgAction::SetTrue)]
    pub no_table: bool,
    #[arg(long = "share", action = ArgAction::SetTrue)]
    pub share: bool,
    #[arg(long = "no-share", action = ArgAction::SetTrue)]
    pub no_share: bool,
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
    #[command(subcommand)]
    pub command: Option<RedteamCommand>,
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(long = "stage", value_enum, default_value_t = RedteamStageArg::Run)]
    pub stage: RedteamStageArg,
    #[arg(long = "report", value_name = "FILE")]
    pub report: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum RedteamCommand {
    /// Initialize redteam configuration.
    Init,
    /// Evaluate redteam tests.
    Eval,
    /// Generate redteam tests.
    Generate,
    /// Run redteam tests.
    Run,
    /// Write a redteam report.
    Report,
    /// List redteam plugin and strategy registry evidence.
    Plugins,
    /// Discover attack surfaces.
    Discover,
    /// Run poisoning workflows.
    Poison,
    /// Setup redteam resources.
    Setup,
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

#[derive(Debug, Args)]
pub struct GapCommandArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
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
        Some(Command::Share(_)) => Err(handle_explicit_gap_command(
            "share",
            "command:share",
            GapClass::Unsupported,
            "promptfoo cloud/share is out of scope for local promptfoo-rs; no-upload",
        )),
        Some(Command::Auth(_)) => Err(handle_explicit_gap_command(
            "auth",
            "command:auth",
            GapClass::Unsupported,
            "promptfoo cloud authentication requires an external account; no-upload",
        )),
        Some(Command::Config(_)) => Err(handle_explicit_gap_command(
            "config",
            "command:config",
            GapClass::Unsupported,
            "cloud/global config writes are disabled by default; no-upload",
        )),
        Some(Command::Init(_)) => Err(handle_explicit_gap_command(
            "init",
            "command:init",
            GapClass::Later,
            "interactive project scaffolding is registered for compatibility but not executed silently",
        )),
        Some(Command::Debug(_)) => Err(handle_explicit_gap_command(
            "debug",
            "command:debug",
            GapClass::Later,
            "debug diagnostics require a dedicated compatibility fixture",
        )),
        Some(Command::Delete(_)) => Err(handle_explicit_gap_command(
            "delete",
            "command:delete",
            GapClass::Unsupported,
            "remote deletion is a cloud operation and will not upload or mutate external state",
        )),
        Some(Command::Generate(_)) => Err(handle_explicit_gap_command(
            "generate",
            "command:generate",
            GapClass::Later,
            "generation workflows require task-specific fixtures before native execution",
        )),
        Some(Command::Feedback(_)) => Err(handle_explicit_gap_command(
            "feedback",
            "command:feedback",
            GapClass::Unsupported,
            "feedback submission is a cloud upload path and is disabled",
        )),
        Some(Command::List(_)) => Err(handle_explicit_gap_command(
            "list",
            "command:list",
            GapClass::Unsupported,
            "remote listing is a cloud account operation and is disabled",
        )),
        Some(Command::Logs(_)) => Err(handle_explicit_gap_command(
            "logs",
            "command:logs",
            GapClass::Unsupported,
            "remote log access is a cloud account operation and is disabled",
        )),
        Some(Command::Optimize(_)) => Err(handle_explicit_gap_command(
            "optimize",
            "command:optimize",
            GapClass::Later,
            "prompt optimization requires model-backed fixtures and is not run silently",
        )),
        Some(Command::Retry(_)) => Err(handle_explicit_gap_command(
            "retry",
            "command:retry",
            GapClass::Later,
            "standalone retry is tracked separately from local eval retry flags",
        )),
        Some(Command::Validate(_)) => Err(handle_explicit_gap_command(
            "validate",
            "command:validate",
            GapClass::Later,
            "standalone validation is tracked; eval config loading already fails closed",
        )),
        Some(Command::Show(_)) => Err(handle_explicit_gap_command(
            "show",
            "command:show",
            GapClass::Later,
            "show is registered as a compatibility gap until source-backed behavior is implemented",
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
    if args.share {
        return Err(handle_explicit_gap_command(
            "eval --share",
            "flag:share",
            GapClass::Unsupported,
            "sharing eval results uploads data and is disabled by default; use --no-share for local-only runs",
        ));
    }
    let config_path = args
        .config
        .clone()
        .ok_or_else(|| CliError::new("config path is required for eval (-c, --config)"))?;
    let env = if let Some(env_file) = args.env_file.as_ref() {
        EnvOverlay::from_dotenv(env_file)
            .map_err(|err| CliError::new(format!("env-file {}: {err}", env_file.display())))?
    } else {
        EnvOverlay::default()
    };
    let config = load_promptfoo_config(&config_path, &env)
        .map_err(|err| CliError::new(format!("config {}: {err}", config_path.display())))?;
    let config = apply_eval_flag_overrides(&args, config)?;
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
    let outputs = if args.no_write {
        Vec::new()
    } else {
        write_requested_outputs(&envelope, &output_targets)
            .map_err(|err| CliError::new(err.to_string()))?
    };
    Ok(CliRunArtifacts { envelope, outputs })
}

pub fn apply_eval_flag_overrides(
    args: &EvalArgs,
    mut config: NormalizedConfig,
) -> Result<NormalizedConfig, CliError> {
    if !args.prompts.is_empty() {
        config.prompts = args
            .prompts
            .iter()
            .map(|prompt| NormalizedPrompt {
                source: None,
                body: prompt.clone(),
            })
            .collect();
    }
    if !args.providers.is_empty() {
        config.providers = args
            .providers
            .iter()
            .map(|provider| NormalizedProvider {
                id: provider.clone(),
                config: serde_json::Value::Null,
            })
            .collect();
    }
    if !args.tests.is_empty() {
        config.tests = args
            .tests
            .iter()
            .map(|test| {
                let mut vars = std::collections::BTreeMap::new();
                vars.insert("test".to_string(), test.clone());
                NormalizedTestCase {
                    vars,
                    assertions: Vec::new(),
                }
            })
            .collect();
    }
    if !args.vars.is_empty() {
        let vars = parse_cli_vars(&args.vars)?;
        if config.tests.is_empty() {
            config.tests.push(NormalizedTestCase {
                vars,
                assertions: Vec::new(),
            });
        } else {
            for test in &mut config.tests {
                for (key, value) in &vars {
                    test.vars.insert(key.clone(), value.clone());
                }
            }
        }
    }
    if let Some(repeat) = args.repeat {
        let repeat = repeat.max(1);
        let original = if config.tests.is_empty() {
            vec![NormalizedTestCase {
                vars: Default::default(),
                assertions: Vec::new(),
            }]
        } else {
            config.tests.clone()
        };
        let mut repeated = Vec::new();
        for _ in 0..repeat {
            repeated.extend(original.clone());
        }
        config.tests = repeated;
    }
    if !args.filter_sample.is_empty() {
        let allowed = args
            .filter_sample
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        config.tests = config
            .tests
            .into_iter()
            .enumerate()
            .filter(|(index, _test)| allowed.contains(format!("case-{index}").as_str()))
            .map(|(_index, test)| test)
            .collect();
    }
    Ok(config)
}

fn parse_cli_vars(
    values: &[String],
) -> Result<std::collections::BTreeMap<String, String>, CliError> {
    let mut vars = std::collections::BTreeMap::new();
    for value in values {
        let Some((key, val)) = value.split_once('=') else {
            return Err(CliError::new(format!(
                "--vars value must use KEY=VALUE syntax: {value}"
            )));
        };
        vars.insert(key.to_string(), val.to_string());
    }
    Ok(vars)
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
    if let Some(command) = args.command {
        match command {
            RedteamCommand::Plugins => return handle_redteam_plugins_command(),
            RedteamCommand::Discover => {
                return Err(handle_explicit_gap_command(
                    "redteam discover",
                    "redteam:discover",
                    GapClass::Later,
                    "redteam discover is tracked but not run silently without dedicated fixtures",
                ));
            }
            RedteamCommand::Poison => {
                return Err(handle_explicit_gap_command(
                    "redteam poison",
                    "redteam:poison",
                    GapClass::Later,
                    "redteam poison requires explicit safety review and fixtures",
                ));
            }
            RedteamCommand::Setup => {
                return Err(handle_explicit_gap_command(
                    "redteam setup",
                    "redteam:setup",
                    GapClass::Later,
                    "redteam setup may materialize external resources and is disabled",
                ));
            }
            RedteamCommand::Init
            | RedteamCommand::Eval
            | RedteamCommand::Generate
            | RedteamCommand::Run
            | RedteamCommand::Report => {}
        }
    }
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

fn handle_redteam_plugins_command() -> Result<ExitCode, CliError> {
    let registry = crate::redteam::registry::RedteamRegistry::core_defaults();
    let json = serde_json::json!({
        "schema_version": "promptfoo-rs.redteam.plugins.v1",
        "plugins": registry.plugins,
        "strategies": registry.strategies,
        "matrix_item_id": "command:redteam",
    });
    println!(
        "{}",
        serde_json::to_string(&json).map_err(|err| {
            CliError::new(format!("redteam plugins serialization failed: {err}"))
        })?
    );
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapClass {
    Unsupported,
    Later,
    Blocked,
}

impl GapClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::Later => "later",
            Self::Blocked => "blocked",
        }
    }
}

pub fn handle_explicit_gap_command(
    command: &str,
    item_id: &str,
    class: GapClass,
    reason: &str,
) -> CliError {
    CliError::new(format!(
        "{command}: {} compatibility gap for {item_id}; reason: {reason}; no-upload; docs: docs/compatibility/matrix.md",
        class.as_str()
    ))
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
                CliSurfaceItem::implemented("command:model-audit"),
                CliSurfaceItem::implemented("command:import-file"),
                CliSurfaceItem::implemented("command:export"),
                CliSurfaceItem::later("command:init"),
                CliSurfaceItem::unsupported("command:share"),
                CliSurfaceItem::unsupported("command:auth"),
                CliSurfaceItem::unsupported("command:config"),
                CliSurfaceItem::later("command:debug"),
                CliSurfaceItem::unsupported("command:delete"),
                CliSurfaceItem::later("command:generate"),
                CliSurfaceItem::unsupported("command:feedback"),
                CliSurfaceItem::unsupported("command:list"),
                CliSurfaceItem::unsupported("command:logs"),
                CliSurfaceItem::later("command:optimize"),
                CliSurfaceItem::later("command:retry"),
                CliSurfaceItem::later("command:validate"),
                CliSurfaceItem::later("command:show"),
                CliSurfaceItem::implemented("flag:config"),
                CliSurfaceItem::implemented("flag:prompts"),
                CliSurfaceItem::implemented("flag:providers"),
                CliSurfaceItem::implemented("flag:tests"),
                CliSurfaceItem::implemented("flag:vars"),
                CliSurfaceItem::implemented("flag:output"),
                CliSurfaceItem::implemented("flag:max-concurrency"),
                CliSurfaceItem::implemented("flag:repeat"),
                CliSurfaceItem::implemented("flag:delay"),
                CliSurfaceItem::implemented("flag:no-cache"),
                CliSurfaceItem::implemented("flag:resume"),
                CliSurfaceItem::implemented("flag:retry-errors"),
                CliSurfaceItem::implemented("flag:filter-sample"),
                CliSurfaceItem::implemented("flag:env-file"),
                CliSurfaceItem::implemented("flag:no-write"),
                CliSurfaceItem::implemented("flag:table"),
                CliSurfaceItem::implemented("flag:no-table"),
                CliSurfaceItem::unsupported("flag:share"),
                CliSurfaceItem::implemented("flag:no-share"),
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

    fn unsupported(stable_id: &str) -> Self {
        Self::new(stable_id, CliItemStatus::Unsupported)
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
