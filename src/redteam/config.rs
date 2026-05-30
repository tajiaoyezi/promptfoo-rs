use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::redteam::flow::{RedteamError, RedteamStage};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamConfig {
    pub target: RedteamTargetConfig,
    #[serde(default)]
    pub prompts: Vec<String>,
    #[serde(default)]
    pub plugins: Vec<String>,
    #[serde(default)]
    pub strategies: Vec<String>,
    pub report: Option<RedteamReportConfig>,
}

impl RedteamConfig {
    pub fn planned_stages(&self) -> Vec<RedteamStage> {
        vec![
            RedteamStage::Init,
            RedteamStage::Generate,
            RedteamStage::Eval,
            RedteamStage::Run,
            RedteamStage::Report,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamTargetConfig {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamReportConfig {
    pub path: String,
}

pub fn load_redteam_config(path: &Path) -> Result<RedteamConfig, RedteamError> {
    let yaml = fs::read_to_string(path)
        .map_err(|err| RedteamError::new(format!("redteam config {}: {err}", path.display())))?;
    serde_yaml::from_str(&yaml)
        .map_err(|err| RedteamError::new(format!("redteam config {}: {err}", path.display())))
}
