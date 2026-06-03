use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::CurrentLatestTargetLock;
use serde_json::Value;

const NPM_VIEW_012114: &str = r#"{
  "version": "0.121.14",
  "gitHead": "7a48c5fce614bee617efbb3b7fc93d404c75b628",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.14.tgz",
    "integrity": "sha512-YUeBMqwfv3xZC7HJ3ohwk2e0i3DdCitOrvWZPijCOMywp/S+CZEjyqVh1pUzR1PgDo9eBBn9WXyw2wbDBihcpA=="
  },
  "time": {
    "modified": "2026-06-02T13:59:19.451Z"
  }
}"#;

const GITHUB_LATEST_RELEASE_012114: &str = r#"{
  "tag_name": "0.121.14",
  "name": "0.121.14",
  "target_commitish": "7a48c5fce614bee617efbb3b7fc93d404c75b628",
  "published_at": "2026-06-02T13:49:18Z",
  "html_url": "https://github.com/promptfoo/promptfoo/releases/tag/0.121.14"
}"#;

const LS_REMOTE_012114: &str = "\
4d22e57f5f9b4c7cdde494f00558d9afde8b4975\tHEAD
7a48c5fce614bee617efbb3b7fc93d404c75b628\trefs/tags/0.121.14
";

#[test]
fn test_38_1_1_same_ref_npm_and_latest_release_populate_both_commits() {
    /* TEST-38.1.1 */
    let lock = CurrentLatestTargetLock::from_observations(
        NPM_VIEW_012114,
        GITHUB_LATEST_RELEASE_012114,
        LS_REMOTE_012114,
    )
    .expect("same npm tag/latest release ref should parse");

    assert_eq!(lock.npm_latest.package_version, "0.121.14");
    assert_eq!(lock.github.npm_tag_ref, "refs/tags/0.121.14");
    assert_eq!(lock.github.latest_release_ref, "refs/tags/0.121.14");
    assert_eq!(
        lock.github.npm_tag_commit,
        "7a48c5fce614bee617efbb3b7fc93d404c75b628"
    );
    assert_eq!(
        lock.github.latest_release_commit,
        "7a48c5fce614bee617efbb3b7fc93d404c75b628"
    );
    assert_eq!(lock.github.latest_release_channel, "core-package");
    assert!(lock.github.latest_release_is_core_package, "{lock:#?}");
    assert_eq!(lock.status, "locked-with-drift");
    assert!(!lock.current_latest_claim_allowed, "{lock:#?}");
}

#[test]
fn test_38_1_2_tracked_lock_artifacts_record_observed_0_121_14_target() {
    /* TEST-38.1.2 */
    let lock = tracked_lock();
    assert_eq!(lock["npm_latest"]["package_version"], "0.121.14");
    assert_eq!(
        lock["npm_latest"]["git_head"],
        "7a48c5fce614bee617efbb3b7fc93d404c75b628"
    );
    assert_eq!(
        lock["npm_latest"]["tarball"],
        "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.14.tgz"
    );
    assert_eq!(
        lock["npm_latest"]["integrity"],
        "sha512-YUeBMqwfv3xZC7HJ3ohwk2e0i3DdCitOrvWZPijCOMywp/S+CZEjyqVh1pUzR1PgDo9eBBn9WXyw2wbDBihcpA=="
    );
    assert_eq!(
        lock["github"]["default_branch_head"]
            .as_str()
            .expect("tracked lock should record a GitHub default branch HEAD")
            .len(),
        40
    );
    assert_eq!(lock["github"]["latest_release_ref"], "refs/tags/0.121.14");
    assert_eq!(
        lock["github"]["latest_release_commit"],
        "7a48c5fce614bee617efbb3b7fc93d404c75b628"
    );

    let markdown = std::fs::read_to_string("docs/compatibility/current-latest.lock.md")
        .expect("tracked current latest markdown lock should be readable");
    assert!(markdown.contains("promptfoo@0.121.14"), "{markdown}");
    assert!(
        markdown.contains(
            lock["github"]["default_branch_head"]
                .as_str()
                .expect("tracked lock should record a GitHub default branch HEAD")
        ),
        "{markdown}"
    );
    assert!(!markdown.contains("promptfoo@0.121.13"), "{markdown}");
}

#[test]
fn test_38_1_3_shell_lock_script_accepts_same_ref_fixture_and_stays_fail_closed() {
    /* TEST-38.1.3 */
    let fixture_dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-drift-refresh-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture_dir);
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir should create");
    let npm_path = fixture_dir.join("npm-view.json");
    let release_path = fixture_dir.join("github-latest-release.json");
    let ls_remote_path = fixture_dir.join("ls-remote.txt");
    std::fs::write(&npm_path, NPM_VIEW_012114).expect("npm fixture should write");
    std::fs::write(&release_path, GITHUB_LATEST_RELEASE_012114)
        .expect("release fixture should write");
    std::fs::write(&ls_remote_path, LS_REMOTE_012114).expect("ls remote fixture should write");

    let command = format!(
        "CURRENT_LATEST_NPM_VIEW_FILE='{}' CURRENT_LATEST_GITHUB_RELEASE_FILE='{}' CURRENT_LATEST_LS_REMOTE_FILE='{}' bash scripts/release/current-latest-target-lock.sh",
        shell_escape(&npm_path),
        shell_escape(&release_path),
        shell_escape(&ls_remote_path)
    );
    let script_output = Command::new(git_bash())
        .args(["-lc", &command])
        .output()
        .expect("current latest target lock script should execute");
    assert!(
        script_output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&script_output.stdout),
        String::from_utf8_lossy(&script_output.stderr)
    );

    let generated: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/current-latest-target.json")
            .expect("script should write current latest target gate"),
    )
    .expect("script gate should be valid JSON");
    assert_eq!(generated["npm_latest"]["package_version"], "0.121.14");
    assert_eq!(
        generated["github"]["latest_release_ref"],
        "refs/tags/0.121.14"
    );
    assert_eq!(generated["github"]["latest_release_is_core_package"], true);
    assert_eq!(generated["current_latest_claim_allowed"], false);
    assert_eq!(generated["status"], "locked-with-drift");

    let _ = std::fs::remove_dir_all(fixture_dir);
}

#[test]
fn test_38_1_4_refreshed_target_keeps_perfect_refactor_decisions_blocked() {
    /* TEST-38.1.4 */
    let lock = tracked_lock();
    assert_eq!(lock["current_latest_claim_allowed"], false, "{lock:#}");
    assert_eq!(lock["target_selection_blocker_resolved"], true, "{lock:#}");
    assert_eq!(lock["status"], "locked-with-drift", "{lock:#}");
    assert!(
        lock["downstream_required_evidence"]
            .as_array()
            .expect("required evidence should be an array")
            .iter()
            .any(|item| item == "external_authority_or_waivers"),
        "{lock:#}"
    );
    assert!(
        lock["downstream_required_evidence"]
            .as_array()
            .expect("required evidence should be an array")
            .iter()
            .any(|item| item == "publication_authority_or_waivers"),
        "{lock:#}"
    );
}

#[test]
fn test_38_1_5_runtime_smoke_prefers_tracked_lock_over_stale_gate_copy() {
    /* TEST-38.1.5 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should be readable");
    let tracked_lock = script
        .find("lock_file=\"compatibility/inventory/current-latest-target.json\"")
        .expect("runtime smoke should read tracked current-latest lock");
    let stale_gate_fallback = script
        .find("lock_file=\"$GATE_DIR/current-latest-target.json\"")
        .expect("runtime smoke should keep generated gate as a fallback");
    assert!(
        tracked_lock < stale_gate_fallback,
        "runtime smoke must prefer tracked lock over stale generated gate copy:\n{script}"
    );
}

#[test]
fn test_38_1_6_runtime_smoke_fixture_keeps_frozen_baseline_ref() {
    /* TEST-38.1.6 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should be readable");
    assert!(
        script.contains("4860e990c7e9a2f8f677173fb92cf9867b34d03f"),
        "runtime smoke fixture must include frozen baseline commit for upstream-distribution-target.sh:\n{script}"
    );
    assert!(
        script.contains("refs/tags/0.121.13"),
        "runtime smoke fixture must include frozen baseline tag for upstream-distribution-target.sh:\n{script}"
    );
}

fn tracked_lock() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string("compatibility/inventory/current-latest-target.json")
            .expect("tracked current latest target lock should be readable"),
    )
    .expect("tracked current latest target lock should parse")
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
