use std::path::Path;

const REQUIRED_GATE_COMMANDS: &[(&str, &str)] = &[
    ("Lint", "scripts/release/lint.sh"),
    ("Integration tests", "scripts/release/integration.sh"),
    ("E2E tests", "scripts/release/e2e.sh"),
    ("Coverage", "scripts/release/coverage.sh"),
    ("Runtime smoke", "scripts/release/runtime-smoke.sh"),
];

#[test]
fn test_15_2_1_adapter_commands_are_executable_release_gates() {
    /* TEST-15.2.1 */
    let adapter = std::fs::read_to_string("docs/s2v-adapter.md").expect("adapter should exist");

    assert!(
        adapter.contains("Git for Windows Bash"),
        "adapter must document Windows Git Bash execution: {adapter}"
    );

    for (command_name, script_path) in REQUIRED_GATE_COMMANDS {
        let command = adapter_command(&adapter, command_name);
        assert!(
            !command.starts_with("N/A"),
            "{command_name} must be non-N/A: {command}"
        );
        assert!(
            command.contains(script_path),
            "{command_name} must call {script_path}: {command}"
        );
        assert!(
            Path::new(script_path).is_file(),
            "{script_path} must be a tracked executable release gate script"
        );
    }
}

fn adapter_command(adapter: &str, command_name: &str) -> String {
    let prefix = format!("- **{command_name}**:");
    adapter
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("missing adapter command {command_name}"))
        .trim()
        .to_string()
}
