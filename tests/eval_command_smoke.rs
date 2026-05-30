use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_2_3_1_eval_config_completes_minimal_smoke() {
    let fixture = FixtureDir::new("test_2_3_1");
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
        .expect("TEST-2.3.1 eval command should execute");

    assert!(output.status.success(), "{output:?}");
    let envelope: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be JSON");
    assert_eq!(envelope["status"], "ok");
    assert_eq!(envelope["summary"]["total_cases"], 1);
}

#[test]
fn test_2_3_2_runner_outputs_structured_result_envelope() {
    let fixture = FixtureDir::new("test_2_3_2");
    fixture.write(
        "promptfooconfig.yaml",
        r#"
providers:
  - id: echo
prompts:
  - "Hi {{name}}"
tests:
  - vars:
      name: Grace
"#,
    );

    let output = promptfoo_rs()
        .args(["eval", "--config"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("TEST-2.3.2 eval command should execute");

    assert!(output.status.success(), "{output:?}");
    let envelope: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be JSON");

    assert_eq!(envelope["results"][0]["provider_id"], "echo");
    assert_eq!(envelope["results"][0]["prompt"], "Hi Grace");
    assert_eq!(envelope["results"][0]["output"], "Hi Grace");
    assert_eq!(envelope["errors"].as_array().unwrap().len(), 0);
}

#[test]
fn test_2_3_3_invalid_config_returns_located_error_and_nonzero_exit() {
    let fixture = FixtureDir::new("test_2_3_3");
    fixture.write("promptfooconfig.yaml", "providers: [");

    let output = promptfoo_rs()
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("TEST-2.3.3 eval command should execute");

    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("config"), "{stderr}");
    assert!(stderr.contains("promptfooconfig.yaml"), "{stderr}");
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
