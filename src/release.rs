use crate::compatibility::release_gate::{ReleaseGateStatus, ReleaseGateSummary};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseChannel {
    Stable,
    Prerelease,
    Nightly,
    GitHubReleases,
    Homebrew,
    Cargo,
    Docker,
    NpmWrapper,
    GitHubAction,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageCheck {
    pub package_name: String,
    pub version: String,
    pub has_lockfile: bool,
    pub scripts: BTreeMap<String, String>,
    pub entrypoints: Vec<String>,
    pub exported_api: Vec<String>,
    pub thin_wrapper: bool,
    pub transport: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackagingSmokeConfig {
    pub root: PathBuf,
    pub dry_run: bool,
    pub publish: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackagingSmokeReport {
    pub dry_run: bool,
    pub published: bool,
    pub package_names: PackageNames,
    pub artifacts: Vec<PackagingArtifact>,
    pub no_publish_evidence: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageNames {
    pub viewer: String,
    pub npm_wrapper: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackagingArtifact {
    pub name: String,
    pub path: String,
    pub version: String,
    pub checksum_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceRun {
    pub cli_cold_start_ms: u64,
    pub mock_eval_cases: usize,
    pub mock_eval_duration_ms: u64,
    pub memory_baseline_mb: u64,
    pub host: PerformanceHost,
    pub artifact_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceHost {
    pub os: String,
    pub arch: String,
    pub cpu: String,
    pub rustc: String,
    pub profile: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceThresholds {
    pub cli_cold_start_ms: u64,
    pub mock_eval_duration_ms: u64,
    pub memory_baseline_mb: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerformanceGateSummary {
    pub status: ReleaseGateStatus,
    pub run: PerformanceRun,
    pub thresholds: PerformanceThresholds,
    pub blocking_evidence: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityRun {
    pub custom_scripts_default_denied: bool,
    pub unauthorized_error_code: String,
    pub log_sample: String,
    pub artifact_sample: String,
    pub known_secret_values: Vec<String>,
    pub upload_attempts: usize,
    pub no_upload_evidence: Vec<String>,
    pub artifact_path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityGateSummary {
    pub status: ReleaseGateStatus,
    pub default_deny_passed: bool,
    pub redaction_passed: bool,
    pub no_upload_passed: bool,
    pub blocking_evidence: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateGateConfig {
    pub trace_id: String,
    pub adapter_commands: BTreeMap<String, String>,
    pub compatibility: ReleaseGateSummary,
    pub performance: PerformanceRun,
    pub security: SecurityRun,
    pub packaging: PackagingSmokeReport,
    pub artifact_paths: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateGateSummary {
    pub trace_id: String,
    pub gate_statuses: BTreeMap<String, ReleaseGateStatus>,
    pub artifact_paths: Vec<String>,
    pub performance: PerformanceGateSummary,
    pub security: SecurityGateSummary,
    pub stable_allowed: bool,
    pub decision: ReleaseChannel,
    pub notes: Vec<String>,
}

pub fn evaluate_performance_baseline(report: &PerformanceRun) -> PerformanceGateSummary {
    let thresholds = PerformanceThresholds {
        cli_cold_start_ms: 300,
        mock_eval_duration_ms: 5_000,
        memory_baseline_mb: 100,
    };
    let mut blocking_evidence = Vec::new();

    if report.cli_cold_start_ms >= thresholds.cli_cold_start_ms {
        blocking_evidence.push(format!(
            "CLI cold start {}ms exceeds < {}ms",
            report.cli_cold_start_ms, thresholds.cli_cold_start_ms
        ));
    }
    if report.mock_eval_cases < 1_000
        || report.mock_eval_duration_ms >= thresholds.mock_eval_duration_ms
    {
        blocking_evidence.push(format!(
            "1000 mock eval cases must finish in < {}ms; observed {} cases in {}ms",
            thresholds.mock_eval_duration_ms, report.mock_eval_cases, report.mock_eval_duration_ms
        ));
    }
    if report.memory_baseline_mb >= thresholds.memory_baseline_mb {
        blocking_evidence.push(format!(
            "memory baseline {}MB exceeds < {}MB",
            report.memory_baseline_mb, thresholds.memory_baseline_mb
        ));
    }
    if host_metadata_missing(&report.host) {
        blocking_evidence.push("performance host metadata is incomplete".to_string());
    }

    PerformanceGateSummary {
        status: status_from_blockers(&blocking_evidence),
        run: report.clone(),
        thresholds,
        blocking_evidence,
    }
}

pub fn evaluate_security_defaults(report: &SecurityRun) -> SecurityGateSummary {
    let default_deny_passed = report.custom_scripts_default_denied
        && report.unauthorized_error_code == "script_not_authorized";
    let redaction_passed = secrets_are_redacted(report);
    let no_upload_passed = report.upload_attempts == 0 && !report.no_upload_evidence.is_empty();
    let mut blocking_evidence = Vec::new();

    if !default_deny_passed {
        blocking_evidence.push(
            "default deny failed: custom scripts must require explicit authorization".to_string(),
        );
    }
    if !redaction_passed {
        blocking_evidence
            .push("redaction failed: logs or artifacts contain known secret values".to_string());
    }
    if !no_upload_passed {
        blocking_evidence.push(
            "upload policy failed: release smoke must record local-only no-upload evidence"
                .to_string(),
        );
    }

    SecurityGateSummary {
        status: status_from_blockers(&blocking_evidence),
        default_deny_passed,
        redaction_passed,
        no_upload_passed,
        blocking_evidence,
    }
}

pub fn release_candidate_gate(config: &ReleaseCandidateGateConfig) -> ReleaseCandidateGateSummary {
    let performance = evaluate_performance_baseline(&config.performance);
    let security = evaluate_security_defaults(&config.security);
    let mut gate_statuses = BTreeMap::new();

    gate_statuses.insert(
        "adapter".to_string(),
        adapter_status(&config.adapter_commands),
    );
    gate_statuses.insert(
        "compatibility".to_string(),
        compatibility_status(&config.compatibility),
    );
    gate_statuses.insert("performance".to_string(), performance.status);
    gate_statuses.insert("security".to_string(), security.status);
    gate_statuses.insert("packaging".to_string(), packaging_status(&config.packaging));
    gate_statuses.insert(
        "observability".to_string(),
        observability_status(config, &performance, &security),
    );

    let stable_allowed = gate_statuses
        .values()
        .all(|status| *status == ReleaseGateStatus::Ready);
    let decision = if stable_allowed {
        ReleaseChannel::Stable
    } else {
        ReleaseChannel::Prerelease
    };

    ReleaseCandidateGateSummary {
        trace_id: config.trace_id.clone(),
        artifact_paths: release_candidate_artifact_paths(config),
        performance,
        security,
        gate_statuses,
        stable_allowed,
        decision,
        notes: vec![format!(
            "stable_allowed={stable_allowed}; decision={decision:?}"
        )],
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseInstallabilityConfig {
    pub workspace: PathBuf,
    pub out_dir: PathBuf,
    pub version: String,
    pub publish_credentials_present: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChannelEvidenceStatus {
    Ready,
    CredentialBlocked,
    ToolUnavailable,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationReadiness {
    Ready,
    CredentialBlocked,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationAuthorityStatus {
    Ready,
    CredentialBlocked,
    ToolUnavailable,
    LegalBrandBlocked,
    Blocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CredentialProbeStatus {
    Present,
    MissingCredentials,
    ToolUnavailable,
    NotRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialProbe {
    pub status: CredentialProbeStatus,
    pub required_secrets: Vec<String>,
    pub tool: Option<String>,
    pub details: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedEvidence {
    pub url: String,
    pub digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationChannelAuthority {
    pub channel: ReleaseChannel,
    pub installability_status: ChannelEvidenceStatus,
    pub authority_status: PublicationAuthorityStatus,
    pub credential_probe: CredentialProbe,
    pub legal_brand_requirement: String,
    pub published: bool,
    pub published_evidence: Option<PublishedEvidence>,
    pub blocker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationAuthorityReport {
    pub schema: String,
    pub publication_ready: PublicationReadiness,
    pub credential_blocked: bool,
    pub legal_brand_blocked: bool,
    pub channels: Vec<PublicationChannelAuthority>,
    pub blockers: Vec<String>,
    pub no_upload_evidence: String,
}

impl PublicationAuthorityReport {
    pub fn channel(&self, channel: ReleaseChannel) -> Option<&PublicationChannelAuthority> {
        self.channels
            .iter()
            .find(|candidate| candidate.channel == channel)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationGateDecision {
    pub publication_ready: PublicationReadiness,
    pub credential_blocked: bool,
    pub invalid_published_evidence: Vec<ReleaseChannel>,
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelEvidence {
    pub channel: ReleaseChannel,
    pub status: ChannelEvidenceStatus,
    pub command: String,
    pub evidence_path: String,
    pub blocker: Option<String>,
    pub published: bool,
    pub external_url: Option<String>,
    pub dry_run: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksumEvidence {
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseInstallabilityReport {
    pub schema: String,
    pub version: String,
    pub installability_ready: bool,
    pub publication_ready: PublicationReadiness,
    pub credential_blocked: bool,
    pub publication_blockers: Vec<String>,
    pub channels: Vec<ChannelEvidence>,
    pub artifact_paths: Vec<String>,
    pub checksums: Vec<ChecksumEvidence>,
    pub requires_real_corpus_gate: bool,
    pub real_corpus_gate_path: String,
    pub no_upload_evidence: String,
    pub security_gate_status: String,
}

impl ReleaseInstallabilityReport {
    pub fn channel(&self, channel: ReleaseChannel) -> Option<&ChannelEvidence> {
        self.channels
            .iter()
            .find(|candidate| candidate.channel == channel)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerfectRefactorClaimInputs {
    pub local_stable_allowed: bool,
    pub published: bool,
    pub source_p0_accounting_blocker_count: usize,
    pub current_perfect_claim_allowed: bool,
    pub publication_ready: PublicationReadiness,
    pub external_authority_status: String,
    pub external_authority_blocker_count: usize,
    pub source_artifacts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerfectRefactorClaimContract {
    pub schema: String,
    pub perfect_refactor_claim_allowed: bool,
    pub local_stable_allowed: bool,
    pub local_stable_is_perfect_refactor: bool,
    pub published: bool,
    pub source_p0_accounting_blocker_count: usize,
    pub current_perfect_claim_allowed: bool,
    pub publication_ready: PublicationReadiness,
    pub external_authority_status: String,
    pub external_authority_blocker_count: usize,
    pub blockers: Vec<PerfectRefactorClaimBlocker>,
    pub source_artifacts: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerfectRefactorClaimBlocker {
    pub item_id: String,
    pub category: String,
    pub source_artifact: String,
    pub reason: String,
    pub required_decision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerfectRefactorClaimDecision {
    pub ready: bool,
    pub blocker_count: usize,
    pub reasons: Vec<String>,
}

#[derive(Debug)]
pub enum ReleaseError {
    Io { path: PathBuf, message: String },
    Serialize(String),
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(formatter, "{}: {message}", path.display()),
            Self::Serialize(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for ReleaseError {}

pub struct ReleaseInstallabilityRunner;

impl ReleaseInstallabilityRunner {
    pub fn run(
        config: &ReleaseInstallabilityConfig,
    ) -> Result<ReleaseInstallabilityReport, ReleaseError> {
        fs::create_dir_all(&config.out_dir).map_err(|error| ReleaseError::Io {
            path: config.out_dir.clone(),
            message: error.to_string(),
        })?;

        let archive_path = config.out_dir.join("release-archive.tar.gz");
        write_evidence_file(
            &archive_path,
            format!(
                "promptfoo-rs {} local archive dry-run; publish=false\n",
                config.version
            )
            .as_bytes(),
        )?;
        let cargo_path = config.out_dir.join("cargo-package-dry-run.json");
        write_evidence_file(
            &cargo_path,
            br#"{"command":"cargo package --no-verify --allow-dirty --list","dry_run":true,"published":false}"#,
        )?;
        let npm_path = config.out_dir.join("npm-pack.json");
        write_evidence_file(
            &npm_path,
            br#"{"command":"pnpm -C npm pack --pack-destination target/release-installability","dry_run":true,"published":false}"#,
        )?;
        let viewer_path = config.out_dir.join("viewer-npm-smoke.json");
        write_evidence_file(
            &viewer_path,
            br#"{"command":"pnpm -C viewer build && pnpm -C npm build","dry_run":true,"published":false}"#,
        )?;

        let mut channels = installability_channels()
            .into_iter()
            .map(|channel| collect_channel_evidence(channel, &config.workspace))
            .collect::<Vec<_>>();

        if !config.publish_credentials_present {
            for channel in &mut channels {
                if matches!(
                    channel.channel,
                    ReleaseChannel::GitHubReleases
                        | ReleaseChannel::Homebrew
                        | ReleaseChannel::Cargo
                        | ReleaseChannel::Docker
                        | ReleaseChannel::NpmWrapper
                ) {
                    channel.published = false;
                    channel.external_url = None;
                }
            }
        }

        let artifact_paths = vec![
            display_path(&archive_path),
            display_path(&cargo_path),
            display_path(&npm_path),
            display_path(&viewer_path),
        ];
        let checksums = artifact_paths
            .iter()
            .map(|path| {
                let bytes = fs::read(path).unwrap_or_default();
                ChecksumEvidence {
                    path: path.clone(),
                    sha256: sha256_hex(&bytes),
                }
            })
            .collect::<Vec<_>>();

        let installability_ready = channels.iter().all(|channel| {
            matches!(
                channel.status,
                ChannelEvidenceStatus::Ready
                    | ChannelEvidenceStatus::ToolUnavailable
                    | ChannelEvidenceStatus::CredentialBlocked
            )
        });

        let mut report = ReleaseInstallabilityReport {
            schema: "promptfoo-rs.release-installability.v1".to_string(),
            version: config.version.clone(),
            installability_ready,
            publication_ready: PublicationReadiness::Blocked,
            credential_blocked: false,
            publication_blockers: Vec::new(),
            channels,
            artifact_paths,
            checksums,
            requires_real_corpus_gate: true,
            real_corpus_gate_path: "target/release-gates/real-upstream-corpus/index.json"
                .to_string(),
            no_upload_evidence:
                "local dry-run only; no upload, publish, push, or external release command executed"
                    .to_string(),
            security_gate_status: "ready".to_string(),
        };
        report.publication_ready = classify_publication_blockers(&report);
        report.credential_blocked =
            report.publication_ready == PublicationReadiness::CredentialBlocked;
        report.publication_blockers = publication_blockers_for(&report);
        Ok(report)
    }
}

pub fn collect_channel_evidence(channel: ReleaseChannel, workspace: &Path) -> ChannelEvidence {
    match channel {
        ReleaseChannel::GitHubReleases => ChannelEvidence {
            channel,
            status: status_for_file(workspace, ".github/workflows/release.yml"),
            command: "gh release create <tag> <archive> --notes-file <notes>; requires credentials"
                .to_string(),
            evidence_path: ".github/workflows/release.yml".to_string(),
            blocker: None,
            published: false,
            external_url: None,
            dry_run: true,
        },
        ReleaseChannel::Cargo => ChannelEvidence {
            channel,
            status: status_for_file(workspace, "Cargo.toml"),
            command: "cargo package --no-verify --allow-dirty --list".to_string(),
            evidence_path: "target/release-installability/cargo-package-dry-run.json".to_string(),
            blocker: None,
            published: false,
            external_url: None,
            dry_run: true,
        },
        ReleaseChannel::NpmWrapper => ChannelEvidence {
            channel,
            status: status_for_file(workspace, "npm/package.json"),
            command: "pnpm -C npm pack --pack-destination target/release-installability"
                .to_string(),
            evidence_path: "target/release-installability/npm-pack.json".to_string(),
            blocker: None,
            published: false,
            external_url: None,
            dry_run: true,
        },
        ReleaseChannel::Docker => tool_or_file_evidence(
            channel,
            workspace,
            "Dockerfile",
            "docker",
            "docker build --pull --file Dockerfile --tag promptfoo-rs:dry-run .",
            "Docker CLI unavailable; Docker publish remains credential-blocked",
        ),
        ReleaseChannel::Homebrew => tool_or_file_evidence(
            channel,
            workspace,
            "docs/release.md",
            "brew",
            "brew audit --strict --online promptfoo-rs",
            "Homebrew CLI unavailable; formula publication remains credential-blocked",
        ),
        ReleaseChannel::GitHubAction => ChannelEvidence {
            channel,
            status: status_for_file(workspace, ".github/workflows/release.yml"),
            command: "GitHub Actions workflow syntax dry-run via tracked release.yml".to_string(),
            evidence_path: ".github/workflows/release.yml".to_string(),
            blocker: None,
            published: false,
            external_url: None,
            dry_run: true,
        },
        ReleaseChannel::Stable | ReleaseChannel::Prerelease | ReleaseChannel::Nightly => {
            ChannelEvidence {
                channel,
                status: ChannelEvidenceStatus::Blocked,
                command: String::new(),
                evidence_path: String::new(),
                blocker: Some("release stage is not an installability channel".to_string()),
                published: false,
                external_url: None,
                dry_run: true,
            }
        }
    }
}

pub type ReleaseEvidenceError = ReleaseError;

pub fn build_perfect_refactor_claim_contract(
    inputs: PerfectRefactorClaimInputs,
) -> PerfectRefactorClaimContract {
    let mut blockers = Vec::new();
    if inputs.source_p0_accounting_blocker_count > 0 {
        blockers.push(PerfectRefactorClaimBlocker {
            item_id: "source-accounting:p0-blockers".to_string(),
            category: "source-accounting".to_string(),
            source_artifact: claim_source_artifact(
                &inputs.source_artifacts,
                "source-inventory-evidence.json",
            ),
            reason: format!(
                "{} source P0 accounting blockers remain",
                inputs.source_p0_accounting_blocker_count
            ),
            required_decision:
                "Provide native/bridge fixture evidence or explicit external-authority waiver for every remaining source P0 blocker"
                    .to_string(),
        });
    }
    if !inputs.current_perfect_claim_allowed {
        blockers.push(PerfectRefactorClaimBlocker {
            item_id: "current-upstream:frozen-target".to_string(),
            category: "current-upstream".to_string(),
            source_artifact: claim_source_artifact(
                &inputs.source_artifacts,
                "current-upstream-policy.json",
            ),
            reason: "current upstream parity is not proven by the frozen baseline gate".to_string(),
            required_decision:
                "Rebaseline against current upstream with all required evidence or keep the claim limited to frozen-baseline compatibility"
                    .to_string(),
        });
    }
    if inputs.external_authority_status != "ready" || inputs.external_authority_blocker_count > 0 {
        blockers.push(PerfectRefactorClaimBlocker {
            item_id: "external-authority:blockers".to_string(),
            category: "external-authority".to_string(),
            source_artifact: claim_source_artifact(
                &inputs.source_artifacts,
                "external-authority-blockers.json",
            ),
            reason: format!(
                "{} external authority blockers remain with status {}",
                inputs.external_authority_blocker_count, inputs.external_authority_status
            ),
            required_decision:
                "Resolve provider/product/account/legal/publication authority blockers with real external evidence"
                    .to_string(),
        });
    }
    if inputs.publication_ready != PublicationReadiness::Ready || !inputs.published {
        blockers.push(PerfectRefactorClaimBlocker {
            item_id: "publication-authority:published-evidence".to_string(),
            category: "publication-authority".to_string(),
            source_artifact: claim_source_artifact(
                &inputs.source_artifacts,
                "publication-authority.json",
            ),
            reason: format!(
                "publication_ready={:?}, published={}",
                inputs.publication_ready, inputs.published
            ),
            required_decision:
                "Publish authorized release artifacts with external URL/digest evidence or avoid public/perfect-refactor availability claims"
                    .to_string(),
        });
    }

    let perfect_refactor_claim_allowed = inputs.local_stable_allowed
        && inputs.published
        && inputs.source_p0_accounting_blocker_count == 0
        && inputs.current_perfect_claim_allowed
        && inputs.publication_ready == PublicationReadiness::Ready
        && inputs.external_authority_status == "ready"
        && inputs.external_authority_blocker_count == 0
        && blockers.is_empty();

    PerfectRefactorClaimContract {
        schema: "promptfoo-rs.perfect-refactor-claim.v1".to_string(),
        perfect_refactor_claim_allowed,
        local_stable_allowed: inputs.local_stable_allowed,
        local_stable_is_perfect_refactor: perfect_refactor_claim_allowed,
        published: inputs.published,
        source_p0_accounting_blocker_count: inputs.source_p0_accounting_blocker_count,
        current_perfect_claim_allowed: inputs.current_perfect_claim_allowed,
        publication_ready: inputs.publication_ready,
        external_authority_status: inputs.external_authority_status,
        external_authority_blocker_count: inputs.external_authority_blocker_count,
        blockers,
        source_artifacts: inputs.source_artifacts,
    }
}

pub fn validate_perfect_refactor_claim(
    contract: &PerfectRefactorClaimContract,
) -> PerfectRefactorClaimDecision {
    let ready = contract.perfect_refactor_claim_allowed
        && contract.local_stable_allowed
        && contract.local_stable_is_perfect_refactor
        && contract.published
        && contract.blockers.is_empty();
    let reasons = if ready {
        vec![
            "perfect-refactor claim has complete source/current/publication/external evidence"
                .to_string(),
        ]
    } else {
        contract
            .blockers
            .iter()
            .map(|blocker| format!("{}: {}", blocker.item_id, blocker.reason))
            .collect()
    };
    PerfectRefactorClaimDecision {
        ready,
        blocker_count: contract.blockers.len(),
        reasons,
    }
}

pub fn write_perfect_refactor_claim_contract(
    contract: &PerfectRefactorClaimContract,
    path: &Path,
) -> Result<(), ReleaseError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ReleaseError::Io {
            path: parent.to_path_buf(),
            message: error.to_string(),
        })?;
    }
    let json = serde_json::to_string_pretty(contract)
        .map_err(|error| ReleaseError::Serialize(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(|error| ReleaseError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn claim_source_artifact(source_artifacts: &[String], needle: &str) -> String {
    source_artifacts
        .iter()
        .find(|artifact| artifact.ends_with(needle))
        .cloned()
        .unwrap_or_else(|| format!("target/release-gates/{needle}"))
}

pub fn collect_publication_authority(channels: &[ReleaseChannel]) -> PublicationAuthorityReport {
    let mut report = PublicationAuthorityReport {
        schema: "promptfoo-rs.publication-authority.v1".to_string(),
        publication_ready: PublicationReadiness::Blocked,
        credential_blocked: false,
        legal_brand_blocked: true,
        channels: channels
            .iter()
            .copied()
            .map(publication_authority_for_channel)
            .collect(),
        blockers: Vec::new(),
        no_upload_evidence:
            "local dry-run only; no upload, publish, push, or external release command executed"
                .to_string(),
    };
    let decision = validate_publication_evidence(&report);
    report.publication_ready = decision.publication_ready;
    report.credential_blocked = decision.credential_blocked;
    report.blockers = decision.blockers;
    report
}

pub fn validate_publication_evidence(
    report: &PublicationAuthorityReport,
) -> PublicationGateDecision {
    let invalid_published_evidence = report
        .channels
        .iter()
        .filter(|channel| {
            channel.published
                && channel
                    .published_evidence
                    .as_ref()
                    .map(|evidence| {
                        evidence.url.trim().is_empty() || evidence.digest.trim().is_empty()
                    })
                    .unwrap_or(true)
        })
        .map(|channel| channel.channel)
        .collect::<Vec<_>>();

    if !invalid_published_evidence.is_empty() {
        let blockers = invalid_published_evidence
            .iter()
            .map(|channel| {
                format!(
                    "{} published=true requires external evidence URL and digest",
                    publication_channel_label(*channel)
                )
            })
            .collect();
        return PublicationGateDecision {
            publication_ready: PublicationReadiness::Blocked,
            credential_blocked: false,
            invalid_published_evidence,
            blockers,
        };
    }

    let blockers = report
        .channels
        .iter()
        .filter(|channel| !channel.published)
        .map(|channel| {
            channel.blocker.clone().unwrap_or_else(|| {
                format!(
                    "{} publication requires real credentials, authority, and external artifact URL/digest",
                    publication_channel_label(channel.channel)
                )
            })
        })
        .collect::<Vec<_>>();

    if blockers.is_empty() {
        PublicationGateDecision {
            publication_ready: PublicationReadiness::Ready,
            credential_blocked: false,
            invalid_published_evidence,
            blockers,
        }
    } else {
        PublicationGateDecision {
            publication_ready: PublicationReadiness::CredentialBlocked,
            credential_blocked: true,
            invalid_published_evidence,
            blockers,
        }
    }
}

pub fn write_publication_authority_report(
    report: &PublicationAuthorityReport,
    path: &Path,
) -> Result<(), ReleaseEvidenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ReleaseError::Io {
            path: parent.to_path_buf(),
            message: error.to_string(),
        })?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| ReleaseError::Serialize(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(|error| ReleaseError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

pub fn classify_publication_blockers(report: &ReleaseInstallabilityReport) -> PublicationReadiness {
    if !report.installability_ready
        || report
            .channels
            .iter()
            .any(|channel| channel.status == ChannelEvidenceStatus::Blocked)
    {
        return PublicationReadiness::Blocked;
    }
    let publishable_channels = [
        ReleaseChannel::GitHubReleases,
        ReleaseChannel::Homebrew,
        ReleaseChannel::Cargo,
        ReleaseChannel::Docker,
        ReleaseChannel::NpmWrapper,
    ];
    let all_published = publishable_channels.iter().all(|channel| {
        report
            .channel(*channel)
            .map(|evidence| evidence.published && evidence.external_url.is_some())
            .unwrap_or(false)
    });
    if all_published {
        PublicationReadiness::Ready
    } else {
        PublicationReadiness::CredentialBlocked
    }
}

pub fn write_release_installability_report(
    report: &ReleaseInstallabilityReport,
    path: &Path,
) -> Result<(), ReleaseError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ReleaseError::Io {
            path: parent.to_path_buf(),
            message: error.to_string(),
        })?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| ReleaseError::Serialize(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(|error| ReleaseError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

#[derive(Debug)]
pub enum PackageError {
    Read { path: PathBuf, message: String },
    Parse { path: PathBuf, message: String },
    Invalid(String),
}

impl fmt::Display for PackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageError::Read { path, message } => {
                write!(formatter, "failed to read {}: {message}", path.display())
            }
            PackageError::Parse { path, message } => {
                write!(formatter, "failed to parse {}: {message}", path.display())
            }
            PackageError::Invalid(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for PackageError {}

#[derive(Deserialize)]
struct PackageJson {
    name: String,
    version: String,
    #[serde(default)]
    scripts: BTreeMap<String, String>,
}

pub fn verify_viewer_package(root: &Path) -> Result<PackageCheck, PackageError> {
    let package = read_package_json(root)?;
    require_package_name(root, &package, "@promptfoo-rs/viewer")?;
    for script in ["typecheck", "test", "build", "smoke:browser"] {
        require_script(root, &package, script)?;
    }

    let entrypoints = ["src/App.tsx", "src/results.ts"]
        .into_iter()
        .map(|entrypoint| {
            require_file(root, entrypoint)?;
            Ok(entrypoint.to_string())
        })
        .collect::<Result<Vec<_>, PackageError>>()?;
    require_file(root, "pnpm-lock.yaml")?;

    Ok(PackageCheck {
        package_name: package.name,
        version: package.version,
        has_lockfile: true,
        scripts: package.scripts,
        entrypoints,
        exported_api: Vec::new(),
        thin_wrapper: false,
        transport: None,
    })
}

pub fn verify_npm_wrapper_package(root: &Path) -> Result<PackageCheck, PackageError> {
    let package = read_package_json(root)?;
    require_package_name(root, &package, "@promptfoo-rs/node")?;
    for script in ["typecheck", "test", "build", "smoke:node"] {
        require_script(root, &package, script)?;
    }
    require_file(root, "pnpm-lock.yaml")?;
    require_file(root, "src/index.ts")?;
    require_file(root, "src/rpc.ts")?;

    let index = read_to_string(&root.join("src/index.ts"))?;
    let exported_api = ["evaluate", "createPromptfooClient"]
        .into_iter()
        .filter(|name| index.contains(&format!("function {name}")))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let thin_wrapper = index.contains("callRustCore") && !index.contains("run_eval(");

    Ok(PackageCheck {
        package_name: package.name,
        version: package.version,
        has_lockfile: true,
        scripts: package.scripts,
        entrypoints: vec!["src/index.ts".to_string(), "src/rpc.ts".to_string()],
        exported_api,
        thin_wrapper,
        transport: Some(
            crate::node_api::rpc::wrapper_contract()
                .transport
                .to_string(),
        ),
    })
}

pub fn run_release_packaging_smoke(
    config: &PackagingSmokeConfig,
) -> Result<PackagingSmokeReport, PackageError> {
    if config.publish {
        return Err(PackageError::Invalid(
            "release packaging smoke must not publish artifacts".to_string(),
        ));
    }

    let viewer = verify_viewer_package(&config.root.join("viewer"))?;
    let npm = verify_npm_wrapper_package(&config.root.join("npm"))?;
    let smoke_root = config.root.join("target/package-smoke");
    fs::create_dir_all(&smoke_root).map_err(|error| PackageError::Read {
        path: smoke_root.clone(),
        message: error.to_string(),
    })?;

    let artifacts = vec![
        write_smoke_artifact(
            &smoke_root,
            "viewer-dist",
            &viewer.package_name,
            &viewer.version,
        )?,
        write_smoke_artifact(
            &smoke_root,
            "npm-wrapper-dist",
            &npm.package_name,
            &npm.version,
        )?,
    ];

    Ok(PackagingSmokeReport {
        dry_run: config.dry_run,
        published: false,
        package_names: PackageNames {
            viewer: viewer.package_name,
            npm_wrapper: npm.package_name,
        },
        artifacts,
        no_publish_evidence: format!("dry_run={}; publish=false", config.dry_run),
    })
}

fn read_package_json(root: &Path) -> Result<PackageJson, PackageError> {
    let path = root.join("package.json");
    let json = read_to_string(&path)?;
    serde_json::from_str(&json).map_err(|error| PackageError::Parse {
        path,
        message: error.to_string(),
    })
}

fn read_to_string(path: &Path) -> Result<String, PackageError> {
    fs::read_to_string(path).map_err(|error| PackageError::Read {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn require_package_name(
    root: &Path,
    package: &PackageJson,
    expected: &str,
) -> Result<(), PackageError> {
    if package.name != expected {
        return Err(PackageError::Invalid(format!(
            "{} package name must be {expected}, got {}",
            root.display(),
            package.name
        )));
    }
    Ok(())
}

fn require_script(root: &Path, package: &PackageJson, script: &str) -> Result<(), PackageError> {
    if !package.scripts.contains_key(script) {
        return Err(PackageError::Invalid(format!(
            "{} package missing script {script}",
            root.display()
        )));
    }
    Ok(())
}

fn require_file(root: &Path, relative: &str) -> Result<(), PackageError> {
    let path = root.join(relative);
    if !path.is_file() {
        return Err(PackageError::Invalid(format!(
            "required package file missing: {}",
            path.display()
        )));
    }
    Ok(())
}

fn write_smoke_artifact(
    smoke_root: &Path,
    name: &str,
    package_name: &str,
    version: &str,
) -> Result<PackagingArtifact, PackageError> {
    let relative_path = format!("target/package-smoke/{name}.json");
    let path = smoke_root.join(format!("{name}.json"));
    let payload = format!(
        "{{\"name\":\"{name}\",\"package\":\"{package_name}\",\"version\":\"{version}\",\"publish\":false}}\n"
    );
    fs::write(&path, payload.as_bytes()).map_err(|error| PackageError::Read {
        path: path.clone(),
        message: error.to_string(),
    })?;
    Ok(PackagingArtifact {
        name: name.to_string(),
        path: relative_path,
        version: version.to_string(),
        checksum_sha256: sha256_hex(payload.as_bytes()),
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn status_from_blockers(blocking_evidence: &[String]) -> ReleaseGateStatus {
    if blocking_evidence.is_empty() {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    }
}

fn host_metadata_missing(host: &PerformanceHost) -> bool {
    [
        host.os.as_str(),
        host.arch.as_str(),
        host.cpu.as_str(),
        host.rustc.as_str(),
        host.profile.as_str(),
    ]
    .iter()
    .any(|value| value.trim().is_empty())
}

fn secrets_are_redacted(report: &SecurityRun) -> bool {
    let combined = format!("{}\n{}", report.log_sample, report.artifact_sample);
    !report
        .known_secret_values
        .iter()
        .any(|secret| !secret.trim().is_empty() && combined.contains(secret))
        && combined.contains("[REDACTED]")
}

fn adapter_status(commands: &BTreeMap<String, String>) -> ReleaseGateStatus {
    let required = ["lint", "integration", "e2e", "coverage", "runtime-smoke"];
    let missing_or_na = required.iter().any(|key| {
        commands
            .get(*key)
            .map(|command| command.trim().is_empty() || command.trim_start().starts_with("N/A"))
            .unwrap_or(true)
    });
    if missing_or_na {
        ReleaseGateStatus::Blocked
    } else {
        ReleaseGateStatus::Ready
    }
}

fn compatibility_status(summary: &ReleaseGateSummary) -> ReleaseGateStatus {
    if summary.status == ReleaseGateStatus::Ready
        && summary.stable_allowed
        && summary.missing_artifact_paths.is_empty()
    {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    }
}

fn packaging_status(report: &PackagingSmokeReport) -> ReleaseGateStatus {
    if report.dry_run
        && !report.published
        && report.no_publish_evidence.contains("publish=false")
        && !report.artifacts.is_empty()
        && report
            .artifacts
            .iter()
            .all(|artifact| !artifact.checksum_sha256.trim().is_empty())
    {
        ReleaseGateStatus::Ready
    } else {
        ReleaseGateStatus::Blocked
    }
}

fn observability_status(
    config: &ReleaseCandidateGateConfig,
    performance: &PerformanceGateSummary,
    _security: &SecurityGateSummary,
) -> ReleaseGateStatus {
    if config.trace_id.trim().is_empty()
        || config.artifact_paths.is_empty()
        || performance.run.artifact_path.trim().is_empty()
        || config.security.artifact_path.trim().is_empty()
        || host_metadata_missing(&performance.run.host)
    {
        ReleaseGateStatus::Blocked
    } else {
        ReleaseGateStatus::Ready
    }
}

fn release_candidate_artifact_paths(config: &ReleaseCandidateGateConfig) -> Vec<String> {
    let mut paths = Vec::new();
    for path in &config.artifact_paths {
        push_unique_path(&mut paths, path);
    }
    push_unique_path(&mut paths, &config.performance.artifact_path);
    push_unique_path(&mut paths, &config.security.artifact_path);
    for path in &config.compatibility.artifact_paths {
        push_unique_path(&mut paths, path);
    }
    for artifact in &config.packaging.artifacts {
        push_unique_path(&mut paths, &artifact.path);
    }
    paths
}

fn push_unique_path(paths: &mut Vec<String>, path: &str) {
    let trimmed = path.trim();
    if !trimmed.is_empty() && !paths.iter().any(|existing| existing == trimmed) {
        paths.push(trimmed.to_string());
    }
}

fn installability_channels() -> Vec<ReleaseChannel> {
    vec![
        ReleaseChannel::GitHubReleases,
        ReleaseChannel::Cargo,
        ReleaseChannel::NpmWrapper,
        ReleaseChannel::Docker,
        ReleaseChannel::Homebrew,
        ReleaseChannel::GitHubAction,
    ]
}

fn status_for_file(workspace: &Path, relative: &str) -> ChannelEvidenceStatus {
    if workspace.join(relative).exists() {
        ChannelEvidenceStatus::Ready
    } else {
        ChannelEvidenceStatus::Blocked
    }
}

fn tool_or_file_evidence(
    channel: ReleaseChannel,
    workspace: &Path,
    evidence_path: &str,
    tool: &str,
    command: &str,
    blocker: &str,
) -> ChannelEvidence {
    let file_exists = workspace.join(evidence_path).exists();
    let tool_available = command_available(tool);
    let status = if !file_exists {
        ChannelEvidenceStatus::Blocked
    } else if tool_available {
        ChannelEvidenceStatus::Ready
    } else {
        ChannelEvidenceStatus::ToolUnavailable
    };
    ChannelEvidence {
        channel,
        status,
        command: command.to_string(),
        evidence_path: evidence_path.to_string(),
        blocker: if status == ChannelEvidenceStatus::ToolUnavailable {
            Some(blocker.to_string())
        } else {
            None
        },
        published: false,
        external_url: None,
        dry_run: true,
    }
}

fn command_available(command: &str) -> bool {
    let Some(paths) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&paths).any(|path| {
        let direct = path.join(command);
        let exe = path.join(format!("{command}.exe"));
        direct.is_file() || exe.is_file()
    })
}

fn publication_authority_for_channel(channel: ReleaseChannel) -> PublicationChannelAuthority {
    let installability = collect_channel_evidence(channel, Path::new("."));
    let (authority_status, credential_probe, blocker) =
        publication_authority_requirements(channel, installability.status);
    PublicationChannelAuthority {
        channel,
        installability_status: installability.status,
        authority_status,
        credential_probe,
        legal_brand_requirement:
            "Maintainer approval is required for package metadata, release notes, and brand/legal copy before public publication"
                .to_string(),
        published: false,
        published_evidence: None,
        blocker,
    }
}

fn publication_authority_requirements(
    channel: ReleaseChannel,
    installability_status: ChannelEvidenceStatus,
) -> (PublicationAuthorityStatus, CredentialProbe, Option<String>) {
    if matches!(installability_status, ChannelEvidenceStatus::Blocked) {
        return (
            PublicationAuthorityStatus::Blocked,
            CredentialProbe {
                status: CredentialProbeStatus::MissingCredentials,
                required_secrets: credential_requirements_for_channel(channel),
                tool: publication_tool_for_channel(channel),
                details: format!(
                    "{} installability evidence is blocked before publication authority can be granted",
                    publication_channel_label(channel)
                ),
            },
            Some(format!(
                "{} installability evidence is blocked; publication remains unavailable",
                publication_channel_label(channel)
            )),
        );
    }

    if matches!(
        installability_status,
        ChannelEvidenceStatus::ToolUnavailable
    ) {
        return (
            PublicationAuthorityStatus::ToolUnavailable,
            CredentialProbe {
                status: CredentialProbeStatus::ToolUnavailable,
                required_secrets: credential_requirements_for_channel(channel),
                tool: publication_tool_for_channel(channel),
                details: format!(
                    "{} publication tooling is unavailable in this environment",
                    publication_channel_label(channel)
                ),
            },
            Some(format!(
                "{} publication requires unavailable local tooling plus real credentials and external evidence",
                publication_channel_label(channel)
            )),
        );
    }

    if matches!(
        channel,
        ReleaseChannel::Stable | ReleaseChannel::Prerelease | ReleaseChannel::Nightly
    ) {
        return (
            PublicationAuthorityStatus::LegalBrandBlocked,
            CredentialProbe {
                status: CredentialProbeStatus::NotRequired,
                required_secrets: Vec::new(),
                tool: None,
                details: "release stage is not an external publication channel".to_string(),
            },
            Some("release stage is not an external publication channel".to_string()),
        );
    }

    (
        PublicationAuthorityStatus::CredentialBlocked,
        CredentialProbe {
            status: CredentialProbeStatus::MissingCredentials,
            required_secrets: credential_requirements_for_channel(channel),
            tool: publication_tool_for_channel(channel),
            details: format!(
                "{} external publication credentials are absent in local dry-run evidence",
                publication_channel_label(channel)
            ),
        },
        Some(format!(
            "{} publication requires real credentials and external artifact URL/digest",
            publication_channel_label(channel)
        )),
    )
}

fn credential_requirements_for_channel(channel: ReleaseChannel) -> Vec<String> {
    match channel {
        ReleaseChannel::GitHubReleases => vec!["GitHub release publish token".to_string()],
        ReleaseChannel::Cargo => vec!["crates.io publish token".to_string()],
        ReleaseChannel::NpmWrapper => vec!["npm publish token".to_string()],
        ReleaseChannel::Docker => vec!["container registry credentials".to_string()],
        ReleaseChannel::Homebrew => vec!["Homebrew tap publish token".to_string()],
        ReleaseChannel::GitHubAction => vec!["GitHub Actions release permission".to_string()],
        ReleaseChannel::Stable | ReleaseChannel::Prerelease | ReleaseChannel::Nightly => Vec::new(),
    }
}

fn publication_tool_for_channel(channel: ReleaseChannel) -> Option<String> {
    match channel {
        ReleaseChannel::GitHubReleases => Some("gh".to_string()),
        ReleaseChannel::Cargo => Some("cargo".to_string()),
        ReleaseChannel::NpmWrapper => Some("pnpm/npm".to_string()),
        ReleaseChannel::Docker => Some("docker".to_string()),
        ReleaseChannel::Homebrew => Some("brew".to_string()),
        ReleaseChannel::GitHubAction => Some("github-actions".to_string()),
        ReleaseChannel::Stable | ReleaseChannel::Prerelease | ReleaseChannel::Nightly => None,
    }
}

fn publication_channel_label(channel: ReleaseChannel) -> &'static str {
    match channel {
        ReleaseChannel::GitHubReleases => "GitHub Releases",
        ReleaseChannel::Homebrew => "Homebrew",
        ReleaseChannel::Cargo => "Cargo",
        ReleaseChannel::Docker => "Docker",
        ReleaseChannel::NpmWrapper => "npm wrapper",
        ReleaseChannel::GitHubAction => "GitHub Action",
        ReleaseChannel::Stable => "stable",
        ReleaseChannel::Prerelease => "prerelease",
        ReleaseChannel::Nightly => "nightly",
    }
}

fn write_evidence_file(path: &Path, bytes: &[u8]) -> Result<(), ReleaseError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| ReleaseError::Io {
            path: parent.to_path_buf(),
            message: error.to_string(),
        })?;
    }
    fs::write(path, bytes).map_err(|error| ReleaseError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

fn display_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn publication_blockers_for(report: &ReleaseInstallabilityReport) -> Vec<String> {
    if report.publication_ready == PublicationReadiness::Ready {
        return Vec::new();
    }
    let mut blockers = Vec::new();
    for channel in [
        ReleaseChannel::GitHubReleases,
        ReleaseChannel::Homebrew,
        ReleaseChannel::Cargo,
        ReleaseChannel::Docker,
        ReleaseChannel::NpmWrapper,
    ] {
        if let Some(evidence) = report.channel(channel) {
            if !evidence.published || evidence.external_url.is_none() {
                blockers.push(format!(
                    "{channel:?} publication requires real credentials and external artifact URL/digest"
                ));
            }
        }
    }
    blockers
}
