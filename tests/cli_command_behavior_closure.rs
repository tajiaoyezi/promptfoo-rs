use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::cache::resume::ResumeRecord;
use promptfoo_rs::results::{AssertionResultRecord, ResultRecord, ResultStatus};
use serde_json::Value;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_16_1_1_view_reads_result_directory_and_prints_viewer_json() {
    /* TEST-16.1.1 */
    let fixture = FixtureDir::new("test_16_1_1");
    write_result_jsonl(&fixture.path("results.jsonl"));

    let output = promptfoo_rs()
        .arg("view")
        .arg(fixture.root())
        .output()
        .expect("view command should execute");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let json = json_stdout(&output);
    assert_eq!(json["schema_version"], "promptfoo-rs.viewer.cli.v1");
    assert_eq!(json["source"]["kind"], "jsonl");
    assert_eq!(json["record_count"], 1);
    assert_eq!(json["rows"][0]["case_id"], "case-1");
    assert_eq!(json["rows"][0]["status"], "failed");
}

#[test]
fn test_16_1_2_cache_reports_resume_state_and_clear_is_local_only() {
    /* TEST-16.1.2 */
    let fixture = FixtureDir::new("test_16_1_2");
    let cache = fixture.path("cache.jsonl");
    fs::write(
        &cache,
        format!(
            "{}\nnot-json\n",
            serde_json::to_string(&ResumeRecord::completed(
                "case-0",
                "sha256:cached",
                "cached-output"
            ))
            .expect("resume record should serialize")
        ),
    )
    .expect("cache fixture should be written");

    let report = promptfoo_rs()
        .args(["cache", "--path"])
        .arg(&cache)
        .args(["--expected-case", "case-0", "--expected-case", "case-1"])
        .output()
        .expect("cache command should execute");

    assert_eq!(report.status.code(), Some(0), "{report:?}");
    assert!(report.stderr.is_empty(), "{report:?}");
    let json = json_stdout(&report);
    assert_eq!(json["schema_version"], "promptfoo-rs.cache.cli.v1");
    assert_eq!(json["completed_count"], 1);
    assert_eq!(json["corrupt_count"], 1);
    assert_eq!(json["remaining_cases"], serde_json::json!(["case-1"]));

    let clear = promptfoo_rs()
        .args(["cache", "--path"])
        .arg(&cache)
        .arg("--clear")
        .output()
        .expect("cache clear should execute");

    assert_eq!(clear.status.code(), Some(0), "{clear:?}");
    assert!(clear.stderr.is_empty(), "{clear:?}");
    assert!(!cache.exists(), "cache clear must remove only the local cache file");
    let json = json_stdout(&clear);
    assert_eq!(json["status"], "cleared");
    assert_eq!(json["upload_attempts"], 0);
}

#[test]
fn test_16_1_3_import_and_export_convert_local_result_artifacts() {
    /* TEST-16.1.3 */
    let fixture = FixtureDir::new("test_16_1_3");
    let input = fixture.path("results.jsonl");
    let output_csv = fixture.path("export/results.csv");
    write_result_jsonl(&input);

    let imported = promptfoo_rs()
        .arg("import")
        .arg(&input)
        .output()
        .expect("import command should execute");

    assert_eq!(imported.status.code(), Some(0), "{imported:?}");
    assert!(imported.stderr.is_empty(), "{imported:?}");
    let json = json_stdout(&imported);
    assert_eq!(json["schema_version"], "promptfoo-rs.import.cli.v1");
    assert_eq!(json["record_count"], 1);
    assert_eq!(json["status_counts"]["failed"], 1);

    let exported = promptfoo_rs()
        .args(["export", "--input"])
        .arg(&input)
        .args(["--output"])
        .arg(&output_csv)
        .output()
        .expect("export command should execute");

    assert_eq!(exported.status.code(), Some(0), "{exported:?}");
    assert!(exported.stderr.is_empty(), "{exported:?}");
    let json = json_stdout(&exported);
    assert_eq!(json["schema_version"], "promptfoo-rs.export.cli.v1");
    assert_eq!(json["record_count"], 1);
    assert_eq!(json["output"], output_csv.to_string_lossy().as_ref());
    let csv = fs::read_to_string(output_csv).expect("exported CSV should exist");
    assert!(
        csv.starts_with("case_id,provider_id,status,assertion_types"),
        "{csv}"
    );
}

fn write_result_jsonl(path: impl AsRef<Path>) {
    let record = ResultRecord {
        eval_id: "eval-1".to_string(),
        case_id: "case-1".to_string(),
        provider_id: "echo".to_string(),
        status: ResultStatus::Failed,
        result: Some(serde_json::json!({ "output": "bad answer" })),
        assertion_results: vec![AssertionResultRecord {
            assertion_type: "contains".to_string(),
            status: ResultStatus::Failed,
            message: Some("missing expected text".to_string()),
        }],
        latency_ms: 12,
        metadata: serde_json::json!({ "source": "TEST-16.1" }),
        error: Some("assertion failed".to_string()),
    };
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("result parent should be created");
    }
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string(&record).expect("record should serialize")
        ),
    )
    .expect("result fixture should be written");
}

fn json_stdout(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout is JSON")
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

    fn root(&self) -> &Path {
        &self.root
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if self.root.exists() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
