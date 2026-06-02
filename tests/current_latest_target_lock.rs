use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    write_current_latest_target_lock, CurrentLatestTargetLock,
};
use serde_json::Value;

const NPM_VIEW: &str = r#"{
  "version": "0.121.13",
  "gitHead": "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz",
    "integrity": "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g=="
  },
  "time": {
    "modified": "2026-05-28T23:59:40.582Z"
  }
}"#;

const GITHUB_LATEST_RELEASE: &str = r#"{
  "tag_name": "code-scan-action-0.1.7",
  "name": "code-scan-action: 0.1.7",
  "target_commitish": "1c743afe0e4807882e858c4f322fc064fa5f0770",
  "published_at": "2026-05-29T03:02:57Z",
  "html_url": "https://github.com/promptfoo/promptfoo/releases/tag/code-scan-action-0.1.7"
}"#;

const LS_REMOTE: &str = "\
1d09dfeb5f0766905409117f923dd5c4b0838d9f\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

const FLOATING_NPM_VIEW: &str = r#"{
  "version": "latest",
  "gitHead": "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-latest.tgz",
    "integrity": "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g=="
  },
  "time": {
    "modified": "2026-05-28T23:59:40.582Z"
  }
}"#;

#[test]
fn test_24_1_1_lock_records_npm_github_head_and_release_channel() {
    /* TEST-24.1.1 */
    let lock =
        CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
            .expect("current latest target lock should parse");

    assert_eq!(lock.schema, "promptfoo-rs.current-latest-target.v1");
    assert_eq!(lock.npm_latest.package_version, "0.121.13");
    assert_eq!(
        lock.npm_latest.git_head,
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
    );
    assert_eq!(
        lock.github.default_branch_head,
        "1d09dfeb5f0766905409117f923dd5c4b0838d9f"
    );
    assert_eq!(
        lock.github.latest_release_ref,
        "refs/tags/code-scan-action-0.1.7"
    );
    assert_eq!(
        lock.github.latest_release_commit,
        "1c743afe0e4807882e858c4f322fc064fa5f0770"
    );
    assert_eq!(lock.github.latest_release_channel, "github-action");
    assert!(lock.target_selection_blocker_resolved, "{lock:#?}");
}

#[test]
fn test_24_1_2_lock_rejects_floating_latest_as_completion_proof() {
    /* TEST-24.1.2 */
    let err = CurrentLatestTargetLock::from_observations(
        FLOATING_NPM_VIEW,
        GITHUB_LATEST_RELEASE,
        LS_REMOTE,
    )
    .expect_err("floating latest package version must be rejected");

    assert!(
        err.to_string().contains("floating"),
        "unexpected error: {err}"
    );
}

#[test]
fn test_24_1_3_non_core_release_channel_does_not_allow_perfect_claim() {
    /* TEST-24.1.3 */
    let lock =
        CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
            .expect("current latest target lock should parse");

    assert_eq!(lock.status, "locked-with-drift");
    assert_eq!(lock.github.latest_release_channel, "github-action");
    assert!(!lock.github.latest_release_is_core_package, "{lock:#?}");
    assert!(!lock.current_latest_claim_allowed, "{lock:#?}");
    assert!(
        lock.downstream_required_evidence
            .iter()
            .any(|item| item == "current_latest_source_inventory"),
        "{lock:#?}"
    );
}

#[test]
fn test_24_1_4_runtime_smoke_wires_current_latest_lock_artifacts() {
    /* TEST-24.1.4 */
    let lock =
        CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
            .expect("current latest target lock should parse");
    let fixture_dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-target-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture_dir);
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir should create");
    let json_path = fixture_dir.join("current-latest-target.json");
    let md_path = fixture_dir.join("current-latest.lock.md");
    write_current_latest_target_lock(&lock, Path::new(&json_path), Path::new(&md_path))
        .expect("lock should write both JSON and markdown");

    let json: Value = serde_json::from_str(
        &std::fs::read_to_string(&json_path).expect("json lock should be readable"),
    )
    .expect("json lock should parse");
    let markdown = std::fs::read_to_string(&md_path).expect("markdown lock should be readable");
    assert_eq!(json["current_latest_claim_allowed"], false);
    assert!(markdown.contains("1d09dfeb5f0766905409117f923dd5c4b0838d9f"));
    assert!(markdown.contains("code-scan-action-0.1.7"));

    let npm_path = fixture_dir.join("npm-view.json");
    let release_path = fixture_dir.join("github-latest-release.json");
    let ls_remote_path = fixture_dir.join("ls-remote.txt");
    std::fs::write(&npm_path, NPM_VIEW).expect("npm fixture should write");
    std::fs::write(&release_path, GITHUB_LATEST_RELEASE).expect("release fixture should write");
    std::fs::write(&ls_remote_path, LS_REMOTE).expect("ls remote fixture should write");

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

    let runtime_smoke =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_smoke.contains("current-latest-target-lock.sh"));
    assert!(runtime_smoke.contains("CURRENT_LATEST_GITHUB_RELEASE_FILE"));
    let release_target: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/current-latest-target.json")
            .expect("release current latest target should exist"),
    )
    .expect("release current latest target should be valid JSON");
    assert_eq!(release_target["target_selection_blocker_resolved"], true);
    assert_eq!(release_target["current_latest_claim_allowed"], false);

    let _ = std::fs::remove_dir_all(fixture_dir);
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
