use std::process::Command;

use promptfoo_rs::cli::{validate_cli_surface, CliSurface, CommandInventory};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;

fn promptfoo_rs() -> Command {
    Command::new(env!("CARGO_BIN_EXE_promptfoo-rs"))
}

#[test]
fn test_13_1_1_every_command_and_flag_inventory_item_has_status_mapping() {
    /* TEST-13.1.1 */
    let matrix = CapabilityMatrix::from_json_file(std::path::Path::new(
        "compatibility/matrix/items.json",
    ))
    .expect("item matrix should load");
    let inventory = CommandInventory::from_matrix(&matrix);
    let report = validate_cli_surface(&CliSurface::current(), &inventory);

    assert!(inventory.items.len() >= 12, "{inventory:#?}");
    assert!(report.unmapped_items.is_empty(), "{report:#?}");
    assert!(
        report
            .status_by_item
            .iter()
            .any(|(item, status)| item == "command:view-directory" && status == "later")
    );
    assert!(
        report
            .status_by_item
            .iter()
            .any(|(item, status)| item == "command:eval" && status == "implemented")
    );
}

#[test]
fn test_13_1_2_user_visible_commands_do_not_return_empty_success_placeholders() {
    /* TEST-13.1.2 */
    let matrix = CapabilityMatrix::from_json_file(std::path::Path::new(
        "compatibility/matrix/items.json",
    ))
    .expect("item matrix should load");
    let inventory = CommandInventory::from_matrix(&matrix);
    let report = validate_cli_surface(&CliSurface::current(), &inventory);

    assert!(report.empty_success_commands.is_empty(), "{report:#?}");
    for command in ["view", "cache", "import", "export"] {
        let output = promptfoo_rs()
            .arg(command)
            .output()
            .expect("placeholder command should execute");
        assert_eq!(output.status.code(), Some(1), "{command}: {output:?}");
        assert!(output.stdout.is_empty(), "{command}: {output:?}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(command), "{stderr}");
        assert!(
            stderr.contains("not yet implemented") || stderr.contains("unsupported"),
            "{stderr}"
        );
    }
}

#[test]
fn test_13_1_3_cli_help_invalid_flag_and_exit_code_snapshots_are_stable() {
    /* TEST-13.1.3 */
    let help = promptfoo_rs()
        .args(["eval", "--help"])
        .output()
        .expect("eval help should execute");
    assert_eq!(help.status.code(), Some(0), "{help:?}");
    let stdout = String::from_utf8_lossy(&help.stdout);
    for flag in ["--config", "--output", "--max-concurrency"] {
        assert!(stdout.contains(flag), "missing {flag} in help:\n{stdout}");
    }

    let invalid = promptfoo_rs()
        .args(["eval", "--max-concurrency", "not-a-number"])
        .output()
        .expect("invalid flag should execute");
    assert_eq!(invalid.status.code(), Some(2), "{invalid:?}");
    assert!(invalid.stdout.is_empty(), "{invalid:?}");
    let stderr = String::from_utf8_lossy(&invalid.stderr);
    assert!(stderr.contains("invalid value"), "{stderr}");
    assert!(stderr.contains("--max-concurrency"), "{stderr}");
}
