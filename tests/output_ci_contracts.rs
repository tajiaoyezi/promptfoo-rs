use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::output::{
    write_junit, write_output, write_sarif, Finding, FindingLevel, OutputFormat, RunSummary,
};
use promptfoo_rs::results::{
    AssertionResultRecord, ResultRecord, ResultStatus,
};
use serde_json::{json, Value};

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

fn sample_summary() -> RunSummary {
    RunSummary {
        eval_id: "eval-ci".to_string(),
        records: vec![
            ResultRecord {
                eval_id: "eval-ci".to_string(),
                case_id: "case-pass".to_string(),
                provider_id: "openai:gpt-4.1-mini".to_string(),
                status: ResultStatus::Passed,
                result: Some(json!({ "output": "approved", "score": 1.0 })),
                assertion_results: vec![AssertionResultRecord {
                    assertion_type: "equals".to_string(),
                    status: ResultStatus::Passed,
                    message: Some("matched".to_string()),
                }],
                latency_ms: 31,
                metadata: json!({ "source": "TEST-5.2.1" }),
                error: None,
            },
            ResultRecord {
                eval_id: "eval-ci".to_string(),
                case_id: "case-fail".to_string(),
                provider_id: "openai:gpt-4.1-mini".to_string(),
                status: ResultStatus::Failed,
                result: Some(json!({ "output": "denied", "score": 0.0 })),
                assertion_results: vec![AssertionResultRecord {
                    assertion_type: "contains".to_string(),
                    status: ResultStatus::Failed,
                    message: Some("missing expected phrase".to_string()),
                }],
                latency_ms: 29,
                metadata: json!({ "source": "SCEN-5.2.1" }),
                error: Some("assertion failed".to_string()),
            },
        ],
    }
}

#[test]
fn test_5_2_1_json_junit_and_csv_are_ci_consumable() {
    let summary = sample_summary();

    let mut json_output = Vec::new();
    write_output(OutputFormat::Json, &summary, &mut json_output).expect("JSON output writes");
    let json_value: Value =
        serde_json::from_slice(&json_output).expect("JSON output is parseable");
    assert_eq!(json_value["schema_version"], "promptfoo-rs.output.v1");
    assert_eq!(json_value["eval_id"], "eval-ci");
    assert_eq!(json_value["summary"]["total"], 2);
    assert_eq!(json_value["summary"]["failed"], 1);

    let mut junit_output = Vec::new();
    write_junit(&summary, &mut junit_output).expect("JUnit XML writes");
    let junit = String::from_utf8(junit_output).expect("JUnit is utf8");
    assert!(junit.contains(r#"<testsuite name="eval-ci" tests="2" failures="1" errors="0">"#));
    assert!(junit.contains(r#"<testcase classname="openai:gpt-4.1-mini" name="case-pass""#));
    assert!(junit.contains(r#"<failure message="assertion failed">missing expected phrase</failure>"#));

    let mut csv_output = Vec::new();
    write_output(OutputFormat::Csv, &summary, &mut csv_output).expect("CSV output writes");
    let csv = String::from_utf8(csv_output).expect("CSV is utf8");
    assert!(csv.starts_with("eval_id,case_id,provider_id,status,latency_ms,error"));
    assert!(csv.contains("eval-ci,case-pass,openai:gpt-4.1-mini,passed,31,"));
    assert!(csv.contains("eval-ci,case-fail,openai:gpt-4.1-mini,failed,29,assertion failed"));
}

#[test]
fn test_5_2_2_sarif_and_html_have_stable_data_contract_snapshots() {
    let findings = vec![Finding {
        rule_id: "promptfoo.assertion.failed".to_string(),
        level: FindingLevel::Error,
        message: "case-fail failed contains assertion".to_string(),
        file_path: "promptfooconfig.yaml".to_string(),
        line: 17,
    }];

    let mut sarif_output = Vec::new();
    write_sarif(&findings, &mut sarif_output).expect("SARIF writes");
    let sarif: Value = serde_json::from_slice(&sarif_output).expect("SARIF is json");
    assert_eq!(sarif["version"], "2.1.0");
    assert_eq!(sarif["runs"][0]["tool"]["driver"]["name"], "promptfoo-rs");
    assert_eq!(
        sarif["runs"][0]["results"][0]["ruleId"],
        "promptfoo.assertion.failed"
    );
    assert_eq!(
        sarif["runs"][0]["results"][0]["locations"][0]["physicalLocation"]["artifactLocation"]
            ["uri"],
        "promptfooconfig.yaml"
    );

    let mut html_output = Vec::new();
    write_output(OutputFormat::Html, &sample_summary(), &mut html_output)
        .expect("HTML output writes");
    let html = String::from_utf8(html_output).expect("HTML is utf8");
    assert!(html.contains(r#"data-contract-version="promptfoo-rs.html.v1""#));
    assert!(html.contains(r#"<script id="promptfoo-rs-data" type="application/json">"#));
    assert!(html.contains(r#""eval_id":"eval-ci""#));
    assert!(html.contains(r#""failed":1"#));
}

#[test]
fn test_5_2_3_stdout_stderr_and_exit_code_match_p0_cli_fixtures() {
    let fixture = FixtureDir::new("test_5_2_3");
    fixture.write(
        "promptfooconfig.yaml",
        r#"
providers:
  - id: echo
prompts:
  - "Hello {{name}}"
tests:
  - vars:
      name: Ada
"#,
    );

    let output = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("eval command executes");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("stdout is JSON");
    assert_eq!(stdout["status"], "ok");
    assert_eq!(stdout["summary"]["total_cases"], 1);

    fixture.write("invalid.yaml", "providers: [");
    let error = promptfoo_rs()
        .args(["eval", "--config"])
        .arg(fixture.path("invalid.yaml"))
        .output()
        .expect("invalid eval command executes");

    assert_eq!(error.status.code(), Some(1), "{error:?}");
    assert!(error.stdout.is_empty(), "{error:?}");
    let stderr = String::from_utf8_lossy(&error.stderr);
    assert!(stderr.contains("config"), "{stderr}");
    assert!(stderr.contains("invalid.yaml"), "{stderr}");
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
