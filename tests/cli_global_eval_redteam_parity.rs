use std::process::Command;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn write_eval_config() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("promptfoo-rs-cli-17-2-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir should be created");
    let config = dir.join("promptfooconfig.yaml");
    std::fs::write(
        &config,
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
"#,
    )
    .expect("config should be written");
    config
}

#[test]
fn test_17_2_1_top_level_upstream_commands_are_mapped_or_explicit_gap() {
    /* TEST-17.2.1 */
    let help = promptfoo_rs()
        .arg("--help")
        .output()
        .expect("top-level help should execute");
    assert_eq!(help.status.code(), Some(0), "{help:?}");
    let help = stdout(&help);
    for command in [
        "eval",
        "view",
        "cache",
        "redteam",
        "mcp",
        "code-scans",
        "scan-model",
        "model-audit",
        "import",
        "export",
        "init",
        "share",
        "auth",
        "config",
        "debug",
        "delete",
        "generate",
        "feedback",
        "list",
        "logs",
        "optimize",
        "retry",
        "validate",
        "show",
    ] {
        assert!(
            help.contains(command),
            "missing command {command} in:\n{help}"
        );
    }

    for command in ["share", "auth"] {
        let output = promptfoo_rs()
            .arg(command)
            .output()
            .expect("gap command should execute");
        assert_eq!(output.status.code(), Some(1), "{command}: {output:?}");
        assert!(output.stdout.is_empty(), "{command}: {output:?}");
        let stderr = stderr(&output);
        assert!(stderr.contains("unsupported"), "{command}: {stderr}");
        assert!(stderr.contains("no-upload"), "{command}: {stderr}");
        assert!(
            stderr.contains(&format!("command:{command}")),
            "{command}: {stderr}"
        );
    }
}

#[test]
fn test_17_2_2_eval_p0_flags_parse_or_return_classified_errors() {
    /* TEST-17.2.2 */
    let help = promptfoo_rs()
        .args(["eval", "--help"])
        .output()
        .expect("eval help should execute");
    assert_eq!(help.status.code(), Some(0), "{help:?}");
    let help = stdout(&help);
    for flag in [
        "--config",
        "--prompts",
        "--providers",
        "--tests",
        "--vars",
        "--output",
        "--max-concurrency",
        "--repeat",
        "--delay",
        "--no-cache",
        "--resume",
        "--retry-errors",
        "--filter-sample",
        "--env-file",
        "--no-write",
        "--table",
        "--no-table",
        "--share",
        "--no-share",
    ] {
        assert!(help.contains(flag), "missing flag {flag} in:\n{help}");
    }

    let config = write_eval_config();
    let env_file = config.with_file_name(".env");
    std::fs::write(&env_file, "CITY=Paris\n").expect("env file should be written");
    let output = promptfoo_rs()
        .args([
            "eval",
            "-c",
            config.to_str().expect("config path should be utf8"),
            "--prompts",
            "Hello {{name}}",
            "--providers",
            "echo",
            "--vars",
            "name=Ada",
            "--repeat",
            "1",
            "--delay",
            "0",
            "--no-cache",
            "--resume",
            "--retry-errors",
            "--filter-sample",
            "case-0",
            "--env-file",
            env_file.to_str().expect("env path should be utf8"),
            "--no-write",
            "--no-table",
            "--no-share",
        ])
        .output()
        .expect("eval flag surface should parse");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    let body: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("eval output should be json");
    assert_eq!(body["status"], "ok");

    let share = promptfoo_rs()
        .args(["eval", "-c", config.to_str().unwrap(), "--share"])
        .output()
        .expect("eval --share should execute as explicit gap");
    assert_eq!(share.status.code(), Some(1), "{share:?}");
    let stderr = stderr(&share);
    assert!(stderr.contains("flag:share"), "{stderr}");
    assert!(stderr.contains("no-upload"), "{stderr}");
}

#[test]
fn test_17_2_3_redteam_subcommands_have_stable_surface_and_gap_errors() {
    /* TEST-17.2.3 */
    let help = promptfoo_rs()
        .args(["redteam", "--help"])
        .output()
        .expect("redteam help should execute");
    assert_eq!(help.status.code(), Some(0), "{help:?}");
    let help = stdout(&help);
    for command in [
        "init", "eval", "generate", "run", "report", "plugins", "discover", "poison", "setup",
    ] {
        assert!(
            help.contains(command),
            "missing redteam {command} in:\n{help}"
        );
    }

    let plugins = promptfoo_rs()
        .args(["redteam", "plugins"])
        .output()
        .expect("redteam plugins should execute");
    assert_eq!(plugins.status.code(), Some(0), "{plugins:?}");
    let body: serde_json::Value =
        serde_json::from_slice(&plugins.stdout).expect("plugins output should be json");
    assert_eq!(body["schema_version"], "promptfoo-rs.redteam.plugins.v1");
    assert!(body["plugins"].as_array().map_or(0, Vec::len) >= 1);

    for subcommand in ["discover", "poison", "setup"] {
        let output = promptfoo_rs()
            .args(["redteam", subcommand])
            .output()
            .expect("redteam gap subcommand should execute");
        assert_eq!(output.status.code(), Some(1), "{subcommand}: {output:?}");
        let stderr = stderr(&output);
        assert!(stderr.contains("later"), "{subcommand}: {stderr}");
        assert!(
            stderr.contains(&format!("redteam:{subcommand}")),
            "{subcommand}: {stderr}"
        );
    }
}

#[test]
fn test_17_2_4_cloud_share_auth_paths_are_no_upload_unsupported() {
    /* TEST-17.2.4 */
    for args in [
        vec!["share"],
        vec!["auth"],
        vec!["config", "set", "PROMPTFOO_REMOTE_API_KEY", "secret"],
    ] {
        let output = promptfoo_rs()
            .args(args.clone())
            .output()
            .expect("cloud gap command should execute");
        assert_eq!(output.status.code(), Some(1), "{args:?}: {output:?}");
        assert!(output.stdout.is_empty(), "{args:?}: {output:?}");
        let stderr = stderr(&output);
        assert!(stderr.contains("unsupported"), "{args:?}: {stderr}");
        assert!(stderr.contains("no-upload"), "{args:?}: {stderr}");
        assert!(!stderr.contains("secret"), "{args:?}: {stderr}");
    }
}
