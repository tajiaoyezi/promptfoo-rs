use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::harness::{
    build_current_latest_golden_corpus, evaluate_current_latest_release_blockers,
};
use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use serde_json::Value;

const EXPECTED_GITHUB_HEAD: &str = "c54a30668ad8319d76c20ae96e6680ad6c51a2c6";
const NPM_GIT_HEAD: &str = "4805856060d026521794d4e69decb938155580ad";

const NPM_VIEW_012115: &str = r#"{
  "version": "0.121.15",
  "gitHead": "4805856060d026521794d4e69decb938155580ad",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.15.tgz",
    "integrity": "sha512-UP+7vkLGYHq+06oB4TODWPb6pucNzuOiQTvfgz4XgTrTsBTPFgWTHGYXcqsMKuCtLVsq55HvIIl0PYCjns+CPQ=="
  },
  "time": {
    "modified": "2026-06-05T14:31:09.645Z"
  }
}"#;

const GITHUB_LATEST_RELEASE_012115: &str = r#"{
  "tag_name": "0.121.15",
  "name": "0.121.15",
  "target_commitish": "4805856060d026521794d4e69decb938155580ad",
  "published_at": "2026-06-05T14:21:17Z",
  "html_url": "https://github.com/promptfoo/promptfoo/releases/tag/0.121.15"
}"#;

const LS_REMOTE_TARGET_REFRESH: &str = "\
c54a30668ad8319d76c20ae96e6680ad6c51a2c6\tHEAD
4805856060d026521794d4e69decb938155580ad\trefs/tags/0.121.15
7a48c5fce614bee617efbb3b7fc93d404c75b628\trefs/tags/0.121.14
";

#[test]
fn test_48_1_1_parser_records_refreshed_0_121_15_target_with_github_head_drift() {
    /* TEST-48.1.1 */
    let lock = current_latest_lock();

    assert_eq!(lock.npm_latest.package_version, "0.121.15");
    assert_eq!(lock.npm_latest.git_head, NPM_GIT_HEAD);
    assert_eq!(lock.github.default_branch_head, EXPECTED_GITHUB_HEAD);
    assert_eq!(lock.github.npm_tag_ref, "refs/tags/0.121.15");
    assert_eq!(lock.github.npm_tag_commit, NPM_GIT_HEAD);
    assert_eq!(lock.github.latest_release_ref, "refs/tags/0.121.15");
    assert_eq!(lock.github.latest_release_commit, NPM_GIT_HEAD);
    assert_eq!(lock.github.latest_release_channel, "core-package");
    assert!(lock.github.latest_release_is_core_package, "{lock:#?}");
    assert_eq!(lock.status, "locked-with-drift");
    assert!(lock.target_selection_blocker_resolved, "{lock:#?}");
    assert!(!lock.current_latest_claim_allowed, "{lock:#?}");
}

#[test]
fn test_48_1_2_tracked_lock_artifacts_record_refreshed_0_121_15_target() {
    /* TEST-48.1.2 */
    let lock = read_json(Path::new(
        "compatibility/inventory/current-latest-target.json",
    ));
    assert_tracked_current_latest_target(
        Path::new("compatibility/inventory/current-latest-target.json"),
        "0.121.15",
        EXPECTED_GITHUB_HEAD,
    );
    assert_eq!(lock["npm_latest"]["package_version"], "0.121.15");
    assert_eq!(lock["npm_latest"]["git_head"], NPM_GIT_HEAD);
    assert_eq!(
        lock["npm_latest"]["tarball"],
        "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.15.tgz"
    );
    assert_eq!(
        lock["npm_latest"]["integrity"],
        "sha512-UP+7vkLGYHq+06oB4TODWPb6pucNzuOiQTvfgz4XgTrTsBTPFgWTHGYXcqsMKuCtLVsq55HvIIl0PYCjns+CPQ=="
    );
    assert_eq!(lock["github"]["latest_release_ref"], "refs/tags/0.121.15");
    assert_eq!(lock["github"]["latest_release_commit"], NPM_GIT_HEAD);
    assert_eq!(lock["current_latest_claim_allowed"], false, "{lock:#}");

    let markdown = std::fs::read_to_string("docs/compatibility/current-latest.lock.md")
        .expect("tracked current latest markdown lock should be readable");
    assert!(markdown.contains("promptfoo@0.121.15"), "{markdown}");
    assert!(markdown.contains(EXPECTED_GITHUB_HEAD), "{markdown}");
    assert!(markdown.contains(NPM_GIT_HEAD), "{markdown}");
    assert!(!markdown.contains("promptfoo@0.121.14"), "{markdown}");
}

#[test]
fn test_48_1_3_shell_lock_script_accepts_refreshed_0_121_15_target_fixture() {
    /* TEST-48.1.3 */
    let fixture_dir = fixture_dir("target-lock-script");
    let npm_path = fixture_dir.join("npm-view.json");
    let release_path = fixture_dir.join("github-latest-release.json");
    let ls_remote_path = fixture_dir.join("ls-remote.txt");
    std::fs::write(&npm_path, NPM_VIEW_012115).expect("npm fixture should write");
    std::fs::write(&release_path, GITHUB_LATEST_RELEASE_012115)
        .expect("release fixture should write");
    std::fs::write(&ls_remote_path, LS_REMOTE_TARGET_REFRESH)
        .expect("ls remote fixture should write");

    let command = format!(
        "CURRENT_LATEST_NPM_VIEW_FILE='{}' CURRENT_LATEST_GITHUB_RELEASE_FILE='{}' CURRENT_LATEST_LS_REMOTE_FILE='{}' bash scripts/release/current-latest-target-lock.sh",
        shell_escape(&npm_path),
        shell_escape(&release_path),
        shell_escape(&ls_remote_path)
    );
    run_bash(&command);

    let generated = read_json(Path::new("target/release-gates/current-latest-target.json"));
    assert_eq!(
        generated["github"]["default_branch_head"],
        EXPECTED_GITHUB_HEAD
    );
    assert_eq!(generated["npm_latest"]["package_version"], "0.121.15");
    assert_eq!(
        generated["github"]["latest_release_ref"],
        "refs/tags/0.121.15"
    );
    assert_eq!(generated["github"]["latest_release_commit"], NPM_GIT_HEAD);
    assert_eq!(generated["current_latest_claim_allowed"], false);
    assert_eq!(generated["status"], "locked-with-drift");

    let _ = std::fs::remove_dir_all(fixture_dir);
}

#[test]
fn test_48_1_4_evaluator_inmemorystore_fixture_survives_refreshed_target_inventory() {
    /* TEST-48.1.4 */
    let root = fixture_dir("evaluator-inmemorystore-source");
    write_file(
        &root,
        "src/evaluator/inMemoryStore.ts",
        "export const inMemoryStore = true;",
    );
    let gate_dir = fixture_dir("evaluator-inmemorystore-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let row = inventory
        .rows
        .iter()
        .find(|row| row.stable_id == "eval-runner:src-evaluator-inmemorystore")
        .unwrap_or_else(|| panic!("missing evaluator in-memory store row: {inventory:#?}"));
    assert_eq!(row.implementation_status, "native", "{row:#?}");
    assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
    assert_eq!(
        row.evidence_reference,
        "fixture:eval-runner:src-evaluator-inmemorystore"
    );
    assert!(
        row.source_reference.contains(EXPECTED_GITHUB_HEAD),
        "{row:#?}"
    );

    let matrix = read_json(&gate_dir.join("current-latest-matrix.json"));
    let matrix_row = matrix["rows"]
        .as_array()
        .expect("matrix rows should be array")
        .iter()
        .find(|row| row["item_id"] == "eval-runner:src-evaluator-inmemorystore")
        .unwrap_or_else(|| panic!("missing evaluator in-memory store matrix row: {matrix:#?}"));
    assert_eq!(matrix_row["implementation_status"], "native");
    assert_eq!(matrix_row["evidence_kind"], "fixture");

    let report = build_current_latest_golden_corpus(
        &gate_dir.join("current-latest-matrix.json"),
        &gate_dir.join("fixtures"),
        &gate_dir.join("artifacts"),
    )
    .expect("current latest golden corpus should build");
    let blockers = evaluate_current_latest_release_blockers(&report);
    assert!(
        blockers
            .iter()
            .all(|finding| finding.capability != "eval-runner:src-evaluator-inmemorystore"),
        "evaluator in-memory store should not return as a release blocker: {blockers:#?}"
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(
        NPM_VIEW_012115,
        GITHUB_LATEST_RELEASE_012115,
        LS_REMOTE_TARGET_REFRESH,
    )
    .expect("current latest lock should parse")
}

fn assert_tracked_current_latest_target(
    path: &Path,
    expected_version: &str,
    expected_github_head: &str,
) {
    let lock = read_json(path);
    assert_eq!(
        lock["npm_latest"]["package_version"], expected_version,
        "tracked current-latest target lock should record refreshed npm version"
    );
    assert_eq!(
        lock["github"]["default_branch_head"], expected_github_head,
        "tracked current-latest target lock should record refreshed GitHub HEAD"
    );
}

fn run_current_latest_source_inventory_script(root: &Path, gate_dir: &Path) {
    let lock_path = gate_dir.join("current-latest-target.json");
    std::fs::write(
        &lock_path,
        serde_json::to_string_pretty(&current_latest_lock()).expect("lock should serialize"),
    )
    .expect("lock fixture should write");

    let command = format!(
        "CURRENT_LATEST_TARGET_LOCK_FILE='{}' CURRENT_LATEST_SOURCE_ROOT='{}' CURRENT_LATEST_GATE_DIR='{}' bash scripts/release/current-latest-source-inventory.sh",
        shell_escape(&lock_path),
        shell_escape(root),
        shell_escape(gate_dir)
    );
    run_bash(&command);
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-0-121-15-target-refresh-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir should create");
    dir
}

fn write_file(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dir should create");
    }
    std::fs::write(path, contents).expect("fixture file should write");
}

fn run_bash(command: &str) {
    let output = Command::new(git_bash())
        .args(["-lc", command])
        .output()
        .expect("bash script should execute");
    assert!(
        output.status.success(),
        "command:\n{command}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).expect("json should be readable"))
        .expect("json should parse")
}

fn shell_escape(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .replace('\'', "'\\''")
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
