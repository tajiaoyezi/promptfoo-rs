use crate::compatibility::release_gate::{ReleaseGateStatus, ReleaseGateSummary};
use serde::Deserialize;
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
