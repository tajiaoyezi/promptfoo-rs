use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::output::write_sarif;
use promptfoo_rs::scan::{known_limitations, run_scan, ScanInput};
use serde_json::Value;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_8_2_1_code_scans_command_outputs_finding_schema_snapshot() {
    let fixture = FixtureDir::new("test_8_2_1");
    fixture.write(
        "unsafe.js",
        r#"
const userInput = getUserInput();
eval(userInput);
"#,
    );

    let output = promptfoo_rs()
        .args(["code-scans", "--input"])
        .arg(fixture.path("unsafe.js"))
        .args(["--format", "json"])
        .output()
        .expect("code-scans command executes");

    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("stdout is JSON");
    assert_eq!(stdout["schema_version"], "promptfoo-rs.scan.v1");
    assert_eq!(stdout["command"], "code-scans");
    assert_eq!(stdout["findings"][0]["rule_id"], "promptfoo.scan.eval");
    assert_eq!(stdout["findings"][0]["level"], "warning");
    assert_eq!(stdout["findings"][0]["locations"][0]["line"], 3);
    assert_eq!(stdout["findings"][0]["metadata"]["scanner"], "promptfoo-rs");
}

#[test]
fn test_8_2_2_sarif_writer_accepts_scan_findings_schema_fixture() {
    let findings = run_scan(ScanInput::source(
        "src/unsafe.js",
        "const userInput = getUserInput();\neval(userInput);\n",
    ))
    .expect("scan succeeds");

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "promptfoo.scan.eval");
    assert_eq!(findings[0].locations[0].path, "src/unsafe.js");
    assert_eq!(findings[0].locations[0].line, 2);

    let mut sarif_output = Vec::new();
    write_sarif(&findings, &mut sarif_output).expect("SARIF writes");
    let sarif: Value = serde_json::from_slice(&sarif_output).expect("SARIF is json");
    assert_eq!(sarif["version"], "2.1.0");
    assert_eq!(
        sarif["runs"][0]["tool"]["driver"]["name"],
        "promptfoo-rs"
    );
    assert_eq!(
        sarif["runs"][0]["results"][0]["ruleId"],
        "promptfoo.scan.eval"
    );
    assert_eq!(
        sarif["runs"][0]["results"][0]["locations"][0]["physicalLocation"]
            ["artifactLocation"]["uri"],
        "src/unsafe.js"
    );
    assert_eq!(
        sarif["runs"][0]["results"][0]["properties"]["metadata"]["scanner"],
        "promptfoo-rs"
    );
}

#[test]
fn test_8_2_3_false_positive_rate_is_known_limitation_not_1_0_gate() {
    let limitations = known_limitations();
    let false_positive_limit = limitations
        .iter()
        .find(|limitation| limitation.id == "scan.false-positive-rate")
        .expect("false-positive limitation is registered");

    assert_eq!(false_positive_limit.gate_level, "not-1.0-gate");
    assert!(false_positive_limit.applies_to.contains(&"code-scans"));
    assert!(false_positive_limit.applies_to.contains(&"scan-model"));
    assert!(false_positive_limit.applies_to.contains(&"model-audit"));
    assert!(false_positive_limit.reason.contains("PRD"));
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
