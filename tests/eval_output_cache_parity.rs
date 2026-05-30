use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::cache::resume::ResumeRecord;
use promptfoo_rs::cache::CacheStore;
use promptfoo_rs::config::{load_promptfoo_config, EnvOverlay};
use promptfoo_rs::eval::resume_eval_from_cache;
use serde_json::Value;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_13_2_1_eval_writes_requested_output_targets() {
    /* TEST-13.2.1 */
    let fixture = FixtureDir::new("test_13_2_1");
    fixture.write("promptfooconfig.yaml", passing_config());

    let output = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .args(["--output"])
        .arg(fixture.path("results.jsonl"))
        .args(["--output"])
        .arg(fixture.path("junit.xml"))
        .args(["--output"])
        .arg(fixture.path("results.csv"))
        .args(["--output"])
        .arg(fixture.path("findings.sarif"))
        .args(["--output"])
        .arg(fixture.path("report.html"))
        .output()
        .expect("eval with outputs should execute");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("stdout is JSON");
    assert_eq!(stdout["status"], "ok");

    let jsonl = fs::read_to_string(fixture.path("results.jsonl")).expect("jsonl exists");
    assert!(jsonl.lines().count() >= 2, "{jsonl}");
    let junit = fs::read_to_string(fixture.path("junit.xml")).expect("junit exists");
    assert!(junit.contains(r#"<testsuite name="eval-cli""#), "{junit}");
    let csv = fs::read_to_string(fixture.path("results.csv")).expect("csv exists");
    assert!(
        csv.starts_with("eval_id,case_id,provider_id,status"),
        "{csv}"
    );
    let sarif: Value =
        serde_json::from_slice(&fs::read(fixture.path("findings.sarif")).expect("sarif exists"))
            .expect("sarif is json");
    assert_eq!(sarif["version"], "2.1.0");
    let html = fs::read_to_string(fixture.path("report.html")).expect("html exists");
    assert!(html.contains("promptfoo-rs.html.v1"), "{html}");
}

#[test]
fn test_13_2_2_resume_eval_from_cache_runs_only_remaining_cases() {
    /* TEST-13.2.2 */
    let fixture = FixtureDir::new("test_13_2_2");
    fixture.write("promptfooconfig.yaml", three_case_config());
    let config = load_promptfoo_config(
        &fixture.path("promptfooconfig.yaml"),
        &EnvOverlay::default(),
    )
    .expect("config loads");
    let cache = CacheStore::from_records(vec![ResumeRecord::completed(
        "case-0",
        "sha256:cached",
        "Hello Ada",
    )]);

    let envelope = resume_eval_from_cache(&config, &cache).expect("resume eval succeeds");

    assert_eq!(envelope.summary.total_cases, 2);
    assert_eq!(envelope.results[0].case_id, "case-1");
    assert_eq!(envelope.results[1].case_id, "case-2");
    assert_eq!(envelope.metadata.resume.completed_cases, vec!["case-0"]);
}

#[test]
fn test_13_2_3_eval_stdout_stderr_exit_codes_cover_success_failure_and_invalid_config() {
    /* TEST-13.2.3 */
    let fixture = FixtureDir::new("test_13_2_3");
    fixture.write("success.yaml", passing_config());
    fixture.write("assertion-fail.yaml", assertion_failure_config());
    fixture.write("provider-fail.yaml", provider_failure_config());
    fixture.write("invalid.yaml", "providers: [");

    let success = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("success.yaml"))
        .output()
        .expect("success eval executes");
    assert_eq!(success.status.code(), Some(0), "{success:?}");
    assert!(success.stderr.is_empty(), "{success:?}");
    assert_eq!(json_stdout(&success)["status"], "ok");

    let assertion_fail = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("assertion-fail.yaml"))
        .output()
        .expect("assertion failure eval executes");
    assert_eq!(assertion_fail.status.code(), Some(1), "{assertion_fail:?}");
    assert!(assertion_fail.stderr.is_empty(), "{assertion_fail:?}");
    assert_eq!(json_stdout(&assertion_fail)["status"], "failed");

    let provider_fail = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("provider-fail.yaml"))
        .output()
        .expect("provider failure eval executes");
    assert_eq!(provider_fail.status.code(), Some(1), "{provider_fail:?}");
    assert!(provider_fail.stderr.is_empty(), "{provider_fail:?}");
    assert_eq!(json_stdout(&provider_fail)["status"], "error");

    let invalid = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("invalid.yaml"))
        .output()
        .expect("invalid eval executes");
    assert_eq!(invalid.status.code(), Some(1), "{invalid:?}");
    assert!(invalid.stdout.is_empty(), "{invalid:?}");
    let stderr = String::from_utf8_lossy(&invalid.stderr);
    assert!(stderr.contains("config"), "{stderr}");
    assert!(stderr.contains("invalid.yaml"), "{stderr}");
}

fn json_stdout(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout is JSON")
}

fn passing_config() -> &'static str {
    r#"
providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars: { name: Ada }
    assert:
      - type: contains
        value: Ada
  - vars: { name: Grace }
    assert:
      - type: contains
        value: Grace
"#
}

fn three_case_config() -> &'static str {
    r#"
providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars: { name: Ada }
  - vars: { name: Grace }
  - vars: { name: Linus }
"#
}

fn assertion_failure_config() -> &'static str {
    r#"
providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars: { name: Ada }
    assert:
      - type: contains
        value: Grace
"#
}

fn provider_failure_config() -> &'static str {
    r#"
providers:
  - id: fail
prompts:
  - "Hello {{name}}"
tests:
  - vars: { name: Ada }
"#
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

    fn write(&self, relative: &str, contents: &str) {
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
