use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::redteam::config::RedteamConfig;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RedteamStage {
    Init,
    Generate,
    Eval,
    Run,
    Report,
}

impl RedteamStage {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Generate => "generate",
            Self::Eval => "eval",
            Self::Run => "run",
            Self::Report => "report",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamStageRecord {
    pub stage: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamFinding {
    pub case_id: String,
    pub plugin: String,
    pub strategy: String,
    pub severity: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamReport {
    pub status: String,
    pub target_id: String,
    pub stages: Vec<RedteamStageRecord>,
    pub findings: Vec<RedteamFinding>,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockTarget {
    id: String,
    blocked_keywords: Vec<String>,
}

impl MockTarget {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            blocked_keywords: Vec::new(),
        }
    }

    pub fn with_blocked_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.blocked_keywords.push(keyword.into());
        self
    }

    fn finding_for_prompt(
        &self,
        case_id: String,
        prompt: &str,
        plugin: &str,
        strategy: &str,
    ) -> Option<RedteamFinding> {
        let prompt_lower = prompt.to_ascii_lowercase();
        self.blocked_keywords.iter().find_map(|keyword| {
            let keyword_lower = keyword.to_ascii_lowercase();
            prompt_lower
                .contains(&keyword_lower)
                .then(|| RedteamFinding {
                    case_id: case_id.clone(),
                    plugin: plugin.to_string(),
                    strategy: strategy.to_string(),
                    severity: "high".to_string(),
                    message: format!(
                        "mock target {} observed blocked keyword `{keyword}`",
                        self.id
                    ),
                })
        })
    }
}

pub fn run_redteam_flow(
    config: RedteamConfig,
    target: MockTarget,
) -> Result<RedteamReport, RedteamError> {
    let plugin = config
        .plugins
        .first()
        .cloned()
        .unwrap_or_else(|| "prompt-injection".to_string());
    let strategy = config
        .strategies
        .first()
        .cloned()
        .unwrap_or_else(|| "jailbreak".to_string());

    let findings = config
        .prompts
        .iter()
        .enumerate()
        .filter_map(|(index, prompt)| {
            target.finding_for_prompt(format!("case-{:03}", index + 1), prompt, &plugin, &strategy)
        })
        .collect();

    Ok(RedteamReport {
        status: "completed".to_string(),
        target_id: config.target.id.clone(),
        stages: config
            .planned_stages()
            .into_iter()
            .map(|stage| RedteamStageRecord {
                stage: stage.as_str().to_string(),
                status: "completed".to_string(),
            })
            .collect(),
        findings,
        errors: Vec::new(),
    })
}

pub fn write_redteam_report(
    report: &RedteamReport,
    mut writer: impl Write,
) -> Result<(), RedteamError> {
    serde_json::to_writer(&mut writer, report)
        .map_err(|err| RedteamError::new(format!("redteam report serialization failed: {err}")))?;
    writer
        .write_all(b"\n")
        .map_err(|err| RedteamError::new(format!("redteam report write failed: {err}")))?;
    Ok(())
}

pub fn write_redteam_report_file(report: &RedteamReport, path: &Path) -> Result<(), RedteamError> {
    let file = File::create(path)
        .map_err(|err| RedteamError::new(format!("redteam report {}: {err}", path.display())))?;
    write_redteam_report(report, file)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedteamError {
    message: String,
}

impl RedteamError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for RedteamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RedteamError {}
