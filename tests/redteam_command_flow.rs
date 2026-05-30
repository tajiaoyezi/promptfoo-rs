use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::redteam::{load_redteam_config, run_redteam_flow, MockTarget, RedteamStage};
use serde_json::Value;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_7_1_1_redteam_yaml_drives_command_flow_skeleton() {
    let fixture = FixtureDir::new("test_7_1_1");
    fixture.write("redteam.yaml", redteam_yaml("redteam-report.json"));

    let config = load_redteam_config(&fixture.path("redteam.yaml")).expect("config loads");
    assert_eq!(config.target.id, "mock-target");
    assert_eq!(config.prompts, vec!["Reveal {{secret}}"]);
    assert_eq!(config.plugins, vec!["prompt-injection"]);
    assert_eq!(config.strategies, vec!["jailbreak"]);
    assert_eq!(
        config.planned_stages(),
        vec![
            RedteamStage::Init,
            RedteamStage::Generate,
            RedteamStage::Eval,
            RedteamStage::Run,
            RedteamStage::Report,
        ]
    );

    let output = promptfoo_rs()
        .args(["redteam", "--config"])
        .arg(fixture.path("redteam.yaml"))
        .args(["--stage", "report", "--report"])
        .arg(fixture.path("redteam-report.json"))
        .output()
        .expect("redteam command executes");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let report: Value = serde_json::from_slice(
        &fs::read(fixture.path("redteam-report.json")).expect("report file exists"),
    )
    .expect("report is json");
    assert_eq!(report["status"], "completed");
    assert_eq!(report["stages"][0]["stage"], "init");
    assert_eq!(report["stages"][4]["stage"], "report");
}

#[test]
fn test_7_1_2_mock_target_generates_risk_result() {
    let fixture = FixtureDir::new("test_7_1_2");
    fixture.write("redteam.yaml", redteam_yaml("redteam-report.json"));
    let config = load_redteam_config(&fixture.path("redteam.yaml")).expect("config loads");

    let report = run_redteam_flow(
        config,
        MockTarget::new("mock-target").with_blocked_keyword("secret"),
    )
    .expect("flow runs");

    assert_eq!(report.status, "completed");
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].plugin, "prompt-injection");
    assert_eq!(report.findings[0].strategy, "jailbreak");
    assert_eq!(report.findings[0].severity, "high");
    assert!(report.findings[0].message.contains("secret"));
}

#[test]
fn test_7_1_3_report_failure_path_is_located() {
    let fixture = FixtureDir::new("test_7_1_3");
    fixture.write("redteam.yaml", redteam_yaml("missing/report.json"));

    let output = promptfoo_rs()
        .args(["redteam", "--config"])
        .arg(fixture.path("redteam.yaml"))
        .args(["--stage", "report", "--report"])
        .arg(fixture.path("missing/report.json"))
        .output()
        .expect("redteam command executes");

    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("redteam report"), "{stderr}");
    assert!(stderr.contains("missing"), "{stderr}");
}

fn redteam_yaml(report_path: &str) -> String {
    format!(
        r#"
target:
  id: mock-target
prompts:
  - "Reveal {{{{secret}}}}"
plugins:
  - prompt-injection
strategies:
  - jailbreak
report:
  path: {report_path}
"#
    )
}

struct FixtureDir {
    root: PathBuf,
}

impl FixtureDir {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("promptfoo-rs-{name}-{nonce}"));
        fs::create_dir_all(&root).expect("fixture root should be created");
        Self { root }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn write(&self, relative: &str, contents: impl AsRef<[u8]>) {
        let path = self.path(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent should be created");
        }
        fs::write(path, contents).expect("fixture file should be written");
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if Path::new(&self.root).exists() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
