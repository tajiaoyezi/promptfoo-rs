use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::config::{
    load_promptfoo_config, record_config_diff, ConfigDiff, DiffClass, EnvOverlay,
};

#[test]
fn test_2_2_1_config_loader_outputs_normalized_config_model() {
    let fixture = FixtureDir::new("test_2_2_1");
    fixture.write(
        "promptfooconfig.yaml",
        r#"
description: smoke eval
providers:
  - id: http
    config:
      url: https://example.invalid/chat
prompts:
  - "Hello {{name}}"
tests:
  - vars:
      name: Ada
    assert:
      - type: contains
        value: Ada
"#,
    );

    let config = load_promptfoo_config(
        &fixture.path("promptfooconfig.yaml"),
        &EnvOverlay::default(),
    )
    .expect("TEST-2.2.1 config should load");

    assert_eq!(config.description.as_deref(), Some("smoke eval"));
    assert_eq!(config.providers[0].id, "http");
    assert_eq!(
        config.providers[0].config["url"],
        "https://example.invalid/chat"
    );
    assert_eq!(config.prompts[0].body, "Hello {{name}}");
    assert_eq!(config.tests[0].vars["name"], "Ada");
    assert_eq!(config.tests[0].assertions[0].assertion_type, "contains");
}

#[test]
fn test_2_2_2_paths_env_vars_prompts_and_tests_are_normalized() {
    let fixture = FixtureDir::new("test_2_2_2");
    fixture.write(".env", "MODEL=mock-model\n");
    fixture.write("prompts/hello.txt", "Hello {{name}} from ${MODEL}");
    fixture.write(
        "promptfooconfig.yaml",
        r#"
providers:
  - id: http
prompts:
  - file://prompts/hello.txt
tests:
  - vars:
      name: Grace
"#,
    );

    let env = EnvOverlay::from_dotenv(&fixture.path(".env")).expect("TEST-2.2.2 env should load");
    let config =
        load_promptfoo_config(&fixture.path("promptfooconfig.yaml"), &env).expect("config loads");

    assert_eq!(env.get("MODEL"), Some("mock-model"));
    assert_eq!(config.prompts[0].source.as_deref(), Some("file://prompts/hello.txt"));
    assert_eq!(config.prompts[0].body, "Hello {{name}} from mock-model");
    assert_eq!(config.tests[0].vars["name"], "Grace");
}

#[test]
fn test_2_2_3_parse_differences_record_to_compatibility_report() {
    let finding = record_config_diff(ConfigDiff::unsupported(
        "providers[0].transform",
        "JavaScript transform requires script bridge",
    ));

    assert_eq!(finding.path, "providers[0].transform");
    assert_eq!(finding.class, DiffClass::Unsupported);
    assert!(finding.message.contains("script bridge"));
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
