use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn promptfoo_bin(name: &str) -> Command {
    let path = match name {
        "promptfoo" => env!("CARGO_BIN_EXE_promptfoo"),
        "promptfoo-rs" => env!("CARGO_BIN_EXE_promptfoo-rs"),
        other => panic!("unknown test binary {other}"),
    };
    let mut command = Command::new(path);
    command.env("NO_COLOR", "1");
    command
}

#[test]
fn test_45_1_1_promptfoo_alias_help_uses_drop_in_command_name() {
    // TEST-45.1.1
    let output = promptfoo_bin("promptfoo")
        .arg("--help")
        .output()
        .expect("promptfoo --help should execute");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        contains_usage(&stdout, "promptfoo"),
        "help should expose promptfoo spelling:\n{stdout}"
    );
    assert!(
        !contains_usage(&stdout, "promptfoo-rs"),
        "promptfoo alias should not show promptfoo-rs as its root command:\n{stdout}"
    );
}

#[test]
fn test_45_1_2_promptfoo_alias_runs_minimal_eval_like_promptfoo_rs() {
    // TEST-45.1.2
    let fixture = FixtureDir::new("test_45_1_2");
    fixture.write_minimal_promptfoo_config();

    let promptfoo = promptfoo_bin("promptfoo")
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("promptfoo eval should execute");
    let promptfoo_rs = promptfoo_bin("promptfoo-rs")
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("promptfoo-rs eval should execute");

    assert_eq!(promptfoo.status.code(), promptfoo_rs.status.code());
    assert_success(&promptfoo);
    assert_json_eq(&promptfoo.stdout, &promptfoo_rs.stdout);
    assert_eq!(promptfoo.stderr, promptfoo_rs.stderr);
}

#[test]
fn test_45_1_3_promptfoo_alias_preserves_error_behavior() {
    // TEST-45.1.3
    let fixture = FixtureDir::new("test_45_1_3");
    fixture.write("promptfooconfig.yaml", "providers: [");

    let invalid_config_promptfoo = promptfoo_bin("promptfoo")
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("promptfoo invalid config should execute");
    let invalid_config_promptfoo_rs = promptfoo_bin("promptfoo-rs")
        .args(["eval", "-c"])
        .arg(fixture.path("promptfooconfig.yaml"))
        .output()
        .expect("promptfoo-rs invalid config should execute");

    assert_eq!(
        invalid_config_promptfoo.status.code(),
        invalid_config_promptfoo_rs.status.code()
    );
    assert_eq!(
        normalized_command_name(&invalid_config_promptfoo.stderr),
        normalized_command_name(&invalid_config_promptfoo_rs.stderr)
    );

    let unknown_promptfoo = promptfoo_bin("promptfoo")
        .arg("definitely-unknown")
        .output()
        .expect("promptfoo unknown command should execute");
    let unknown_promptfoo_rs = promptfoo_bin("promptfoo-rs")
        .arg("definitely-unknown")
        .output()
        .expect("promptfoo-rs unknown command should execute");

    assert_eq!(
        unknown_promptfoo.status.code(),
        unknown_promptfoo_rs.status.code()
    );
    assert_eq!(
        normalized_command_name(&unknown_promptfoo.stderr),
        normalized_command_name(&unknown_promptfoo_rs.stderr)
    );
}

#[test]
fn test_45_1_4_promptfoo_rs_binary_remains_available() {
    // TEST-45.1.4
    let output = promptfoo_bin("promptfoo-rs")
        .arg("--help")
        .output()
        .expect("promptfoo-rs --help should execute");

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        contains_usage(&stdout, "promptfoo-rs"),
        "promptfoo-rs help should keep the explicit binary spelling:\n{stdout}"
    );
}

fn contains_usage(stdout: &str, command: &str) -> bool {
    stdout.contains(&format!("Usage: {command} [COMMAND]"))
        || stdout.contains(&format!("Usage: {command}.exe [COMMAND]"))
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_json_eq(left: &[u8], right: &[u8]) {
    let left: serde_json::Value = serde_json::from_slice(left).expect("left stdout should be JSON");
    let right: serde_json::Value =
        serde_json::from_slice(right).expect("right stdout should be JSON");
    assert_eq!(left, right);
}

fn normalized_command_name(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .replace("promptfoo-rs", "__PROMPTFOO_BIN__")
        .replace("promptfoo", "__PROMPTFOO_BIN__")
        .replace("__PROMPTFOO_BIN__", "<promptfoo-bin>")
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

    fn write_minimal_promptfoo_config(&self) {
        self.write(
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
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if Path::new(&self.root).exists() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
