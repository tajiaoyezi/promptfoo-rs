use std::process::Command;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_2_1_1_cargo_workspace_builds_cli_binary() {
    let output = promptfoo_rs()
        .arg("--version")
        .output()
        .expect("TEST-2.1.1 binary should execute");

    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("promptfoo-rs"), "{stdout}");
}

#[test]
fn test_2_1_2_cli_exposes_promptfoo_command_skeletons() {
    let output = promptfoo_rs()
        .arg("--help")
        .output()
        .expect("TEST-2.1.2 help should execute");

    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in [
        "eval",
        "view",
        "cache",
        "redteam",
        "mcp",
        "code-scans",
        "scan-model",
        "import",
        "export",
    ] {
        assert!(stdout.contains(command), "missing {command} in help:\n{stdout}");
    }
}

#[test]
fn test_2_1_3_unknown_command_and_invalid_flag_have_stable_errors() {
    let unknown = promptfoo_rs()
        .arg("definitely-unknown")
        .output()
        .expect("TEST-2.1.3 unknown command should execute");
    assert_eq!(unknown.status.code(), Some(2), "{unknown:?}");
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stderr.contains("unrecognized subcommand"),
        "unexpected stderr:\n{stderr}"
    );

    let invalid_flag = promptfoo_rs()
        .args(["eval", "--definitely-invalid"])
        .output()
        .expect("TEST-2.1.3 invalid flag should execute");
    assert_eq!(invalid_flag.status.code(), Some(2), "{invalid_flag:?}");
    let stderr = String::from_utf8_lossy(&invalid_flag.stderr);
    assert!(
        stderr.contains("unexpected argument"),
        "unexpected stderr:\n{stderr}"
    );
}
