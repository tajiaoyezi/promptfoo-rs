use crate::compatibility::release_gate::{ReleaseGateStatus, ReleaseGateSummary};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseChecklist {
    pub compatibility_gate: CompatibilityGateChecklist,
    pub artifacts: Vec<ReleaseArtifact>,
    pub install_channels: Vec<InstallChannel>,
    pub docs: DocsChecklist,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityGateChecklist {
    pub required_for_stable: bool,
    pub evidence_paths: Vec<&'static str>,
    pub policy: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseArtifact {
    pub name: &'static str,
    pub kind: ReleaseArtifactKind,
    pub source_path: &'static str,
    pub required_for_stable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReleaseArtifactKind {
    Binary,
    HomebrewFormula,
    CargoPackage,
    Container,
    NpmWrapper,
    GitHubAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstallChannel {
    GitHubReleases,
    Homebrew,
    Cargo,
    Docker,
    NpmWrapper,
    GitHubAction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocsChecklist {
    pub readme: bool,
    pub architecture: bool,
    pub release: bool,
    pub contributing: bool,
    pub compatibility_matrix: bool,
    pub github_action_example: bool,
}

impl DocsChecklist {
    pub fn is_complete(&self) -> bool {
        self.readme
            && self.architecture
            && self.release
            && self.contributing
            && self.compatibility_matrix
            && self.github_action_example
    }

    pub fn required_paths(&self) -> Vec<&'static str> {
        vec![
            "README.md",
            "docs/architecture.md",
            "docs/release.md",
            "docs/contributing.md",
            "docs/compatibility/matrix.md",
            ".github/workflows/release.yml",
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseDecision {
    pub channel: ReleaseChannel,
    pub stable_allowed: bool,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReleaseChannel {
    Stable,
    Prerelease,
    Nightly,
}

pub fn default_release_checklist() -> ReleaseChecklist {
    ReleaseChecklist {
        compatibility_gate: CompatibilityGateChecklist {
            required_for_stable: true,
            evidence_paths: vec![
                "docs/compatibility/baseline.lock.md",
                "docs/compatibility/matrix.md",
                "docs/specs/tasks/task-6.2-golden-diff-release-gate.md",
            ],
            policy: "P0 bug or unclassified compatibility diff blocks stable release",
        },
        artifacts: vec![
            ReleaseArtifact {
                name: "promptfoo-rs binary",
                kind: ReleaseArtifactKind::Binary,
                source_path: "target/release/promptfoo-rs",
                required_for_stable: true,
            },
            ReleaseArtifact {
                name: "Homebrew formula",
                kind: ReleaseArtifactKind::HomebrewFormula,
                source_path: "docs/release.md#homebrew",
                required_for_stable: true,
            },
            ReleaseArtifact {
                name: "Cargo package",
                kind: ReleaseArtifactKind::CargoPackage,
                source_path: "Cargo.toml",
                required_for_stable: true,
            },
            ReleaseArtifact {
                name: "Docker image",
                kind: ReleaseArtifactKind::Container,
                source_path: "Dockerfile",
                required_for_stable: true,
            },
            ReleaseArtifact {
                name: "npm wrapper package",
                kind: ReleaseArtifactKind::NpmWrapper,
                source_path: "npm/src/index.ts",
                required_for_stable: false,
            },
            ReleaseArtifact {
                name: "GitHub Action example",
                kind: ReleaseArtifactKind::GitHubAction,
                source_path: ".github/workflows/release.yml",
                required_for_stable: true,
            },
        ],
        install_channels: vec![
            InstallChannel::GitHubReleases,
            InstallChannel::Homebrew,
            InstallChannel::Cargo,
            InstallChannel::Docker,
            InstallChannel::NpmWrapper,
            InstallChannel::GitHubAction,
        ],
        docs: DocsChecklist {
            readme: true,
            architecture: true,
            release: true,
            contributing: true,
            compatibility_matrix: true,
            github_action_example: true,
        },
    }
}

pub fn evaluate_release_readiness(
    summary: &ReleaseGateSummary,
    checklist: &ReleaseChecklist,
) -> ReleaseDecision {
    let gate_blocked =
        summary.status == ReleaseGateStatus::Blocked || !summary.blocking_findings.is_empty();
    if gate_blocked {
        return ReleaseDecision {
            channel: ReleaseChannel::Prerelease,
            stable_allowed: false,
            reasons: vec![
                "compatibility release gate blocked by P0 findings".to_string(),
                "stable release is disabled; prerelease or nightly only".to_string(),
            ],
        };
    }

    if !checklist.docs.is_complete() || !required_stable_artifacts_present(checklist) {
        return ReleaseDecision {
            channel: ReleaseChannel::Nightly,
            stable_allowed: false,
            reasons: vec![
                "release checklist is incomplete".to_string(),
                "stable release is disabled; prerelease or nightly only".to_string(),
            ],
        };
    }

    ReleaseDecision {
        channel: ReleaseChannel::Stable,
        stable_allowed: true,
        reasons: vec![
            "compatibility release gate ready".to_string(),
            "release checklist complete".to_string(),
        ],
    }
}

fn required_stable_artifacts_present(checklist: &ReleaseChecklist) -> bool {
    [
        ReleaseArtifactKind::Binary,
        ReleaseArtifactKind::HomebrewFormula,
        ReleaseArtifactKind::CargoPackage,
        ReleaseArtifactKind::Container,
        ReleaseArtifactKind::GitHubAction,
    ]
    .iter()
    .all(|kind| {
        checklist
            .artifacts
            .iter()
            .any(|artifact| artifact.required_for_stable && &artifact.kind == kind)
    })
}
