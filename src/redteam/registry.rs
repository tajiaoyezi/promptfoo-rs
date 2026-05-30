use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    P0,
    P1,
    P2,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryItem {
    pub id: String,
    pub level: CompatibilityLevel,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamRegistry {
    pub plugins: Vec<RegistryItem>,
    pub strategies: Vec<RegistryItem>,
}

impl RedteamRegistry {
    pub fn core_defaults() -> Self {
        Self {
            plugins: vec![
                item(
                    "prompt-injection",
                    CompatibilityLevel::P0,
                    "core jailbreak coverage",
                ),
                item(
                    "harmful-content",
                    CompatibilityLevel::P1,
                    "snapshot-only policy coverage",
                ),
                item(
                    "custom-policy",
                    CompatibilityLevel::P2,
                    "P2 reason: custom policy registry is project-specific",
                ),
            ],
            strategies: vec![
                item(
                    "jailbreak",
                    CompatibilityLevel::P0,
                    "single-turn jailbreak strategy",
                ),
                item(
                    "multi-turn",
                    CompatibilityLevel::P1,
                    "recorded multi-turn snapshot",
                ),
                item(
                    "agentic-chain",
                    CompatibilityLevel::P2,
                    "P2 reason: long-running agentic chains are deferred",
                ),
            ],
        }
    }

    pub fn plugins_by_level(&self, level: CompatibilityLevel) -> Vec<&RegistryItem> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.level == level)
            .collect()
    }

    pub fn strategies_by_level(&self, level: CompatibilityLevel) -> Vec<&RegistryItem> {
        self.strategies
            .iter()
            .filter(|strategy| strategy.level == level)
            .collect()
    }
}

fn item(id: &str, level: CompatibilityLevel, notes: &str) -> RegistryItem {
    RegistryItem {
        id: id.to_string(),
        level,
        notes: notes.to_string(),
    }
}
