use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    build_upstream_distribution_target, parse_npm_package_observation,
    write_upstream_distribution_target, CurrentUpstreamObservation, FrozenSourceReference,
};
use serde_json::Value;

const FROZEN_SHA: &str = "4860e990c7e9a2f8f677173fb92cf9867b34d03f";
const CURRENT_HEAD_SHA: &str = "ff8eafd743cf6d63dd85b790ad8a4c73ede5828d";
const CODE_SCAN_RELEASE_SHA: &str = "1c743afe0e4807882e858c4f322fc064fa5f0770";
const DYNAMIC_RELEASE_SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const NPM_INTEGRITY: &str =
    "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==";

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

const LS_REMOTE: &str = "\
ff8eafd743cf6d63dd85b790ad8a4c73ede5828d\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

const DYNAMIC_LATEST_RELEASE_VIEW: &str = r#"{
  "tagName": "code-scan-action-0.2.0",
  "targetCommitish": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "name": "code-scan-action 0.2.0"
}"#;

const DYNAMIC_LS_REMOTE: &str = "\
ff8eafd743cf6d63dd85b790ad8a4c73ede5828d\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\trefs/tags/code-scan-action-0.2.0
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

#[test]
fn test_21_1_1_npm_core_package_metadata_matches_frozen_baseline() {
    /* TEST-21.1.1 */
    let npm = parse_npm_package_observation(NPM_VIEW).expect("npm package metadata parses");
    let target = build_upstream_distribution_target(npm, current_observation(), frozen_reference());

    assert_eq!(
        target.schema,
        "promptfoo-rs.upstream-distribution-target.v1"
    );
    assert_eq!(target.npm_core.package_name, "promptfoo");
    assert_eq!(target.npm_core.package_version, "0.121.13");
    assert_eq!(target.npm_core.git_head, FROZEN_SHA);
    assert_eq!(target.npm_core.integrity, NPM_INTEGRITY);
    assert_eq!(
        target.npm_core.tarball,
        "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz"
    );
    assert_eq!(target.frozen.git_commit, FROZEN_SHA);
    assert!(target.npm_core_matches_frozen_baseline, "{target:#?}");
}

#[test]
fn test_21_1_2_github_head_and_release_are_separate_observations() {
    /* TEST-21.1.2 */
    let target = distribution_target();

    assert_eq!(target.github.current_head, CURRENT_HEAD_SHA);
    assert_eq!(target.github.frozen_tag_commit, FROZEN_SHA);
    assert_eq!(
        target.github.observed_release_ref.as_deref(),
        Some("refs/tags/code-scan-action-0.1.7")
    );
    assert_eq!(
        target.github.observed_release_commit.as_deref(),
        Some(CODE_SCAN_RELEASE_SHA)
    );
    assert!(!target.repository_head_matches_npm_core, "{target:#?}");
}

#[test]
fn test_21_1_3_non_core_github_release_cannot_allow_current_perfect_claim() {
    /* TEST-21.1.3 */
    let target = distribution_target();

    assert_eq!(target.status, "ready-with-drift");
    assert_eq!(target.github_latest_release_channel, "github-action");
    assert!(!target.github_latest_release_is_core_package, "{target:#?}");
    assert!(
        !target.current_repository_perfect_claim_allowed,
        "{target:#?}"
    );
    assert!(target.reason.contains("npm core package"), "{target:#?}");
    assert!(target.reason.contains("repository HEAD"), "{target:#?}");
}

#[test]
fn test_21_1_4_runtime_smoke_wires_distribution_target_artifact() {
    /* TEST-21.1.4 */
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-upstream-distribution-target-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_upstream_distribution_target(&distribution_target(), Path::new(&path))
        .expect("distribution target should write");
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("target should be readable"))
            .expect("target should be valid json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(json["npm_core_matches_frozen_baseline"], true);
    assert_eq!(json["current_repository_perfect_claim_allowed"], false);

    let fixture_dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-upstream-distribution-fixtures-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture_dir);
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir should create");
    let npm_path = fixture_dir.join("npm-view.json");
    let ls_remote_path = fixture_dir.join("ls-remote.txt");
    std::fs::write(&npm_path, NPM_VIEW).expect("npm fixture should write");
    std::fs::write(&ls_remote_path, LS_REMOTE).expect("ls-remote fixture should write");

    let command = format!(
        "UPSTREAM_NPM_VIEW_FILE='{}' UPSTREAM_LS_REMOTE_FILE='{}' bash scripts/release/upstream-distribution-target.sh",
        shell_escape(&npm_path),
        shell_escape(&ls_remote_path)
    );
    let script_output = Command::new(git_bash())
        .args(["-lc", &command])
        .output()
        .expect("distribution target script should execute");
    assert!(
        script_output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&script_output.stdout),
        String::from_utf8_lossy(&script_output.stderr)
    );
    let release_target: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/upstream-distribution-target.json")
            .expect("release distribution target should exist"),
    )
    .expect("release distribution target should be valid JSON");
    assert_eq!(
        release_target["github_latest_release_is_core_package"],
        false
    );

    let runtime_smoke =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_smoke.contains("upstream-distribution-target.sh"));
    assert!(runtime_smoke.contains("\"distribution_target\""));

    for docs_path in [
        "docs/compatibility/target-policy.md",
        "docs/compatibility/matrix.md",
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    ] {
        let docs = std::fs::read_to_string(docs_path).expect("docs should be readable");
        assert!(docs.contains("Task 21.1"), "{docs_path}");
        assert!(
            docs.contains("upstream-distribution-target.json"),
            "{docs_path}"
        );
    }

    let _ = std::fs::remove_dir_all(fixture_dir);
}

#[test]
fn test_23_1_1_script_uses_dynamic_latest_release_metadata() {
    /* TEST-23.1.1 / TEST-23.1.2 / TEST-23.1.3 / TEST-23.1.4 */
    let fixture_dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-dynamic-release-fixtures-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&fixture_dir);
    std::fs::create_dir_all(&fixture_dir).expect("fixture dir should create");
    let npm_path = fixture_dir.join("npm-view.json");
    let latest_release_path = fixture_dir.join("github-latest-release.json");
    let ls_remote_path = fixture_dir.join("ls-remote.txt");
    std::fs::write(&npm_path, NPM_VIEW).expect("npm fixture should write");
    std::fs::write(&latest_release_path, DYNAMIC_LATEST_RELEASE_VIEW)
        .expect("latest release fixture should write");
    std::fs::write(&ls_remote_path, DYNAMIC_LS_REMOTE).expect("ls remote fixture should write");

    let command = format!(
        "UPSTREAM_NPM_VIEW_FILE='{}' UPSTREAM_GITHUB_RELEASE_FILE='{}' UPSTREAM_LS_REMOTE_FILE='{}' bash scripts/release/upstream-distribution-target.sh",
        shell_escape(&npm_path),
        shell_escape(&latest_release_path),
        shell_escape(&ls_remote_path)
    );
    let script_output = Command::new(git_bash())
        .args(["-lc", &command])
        .output()
        .expect("distribution target script should execute");
    assert!(
        script_output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&script_output.stdout),
        String::from_utf8_lossy(&script_output.stderr)
    );
    let release_target: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/upstream-distribution-target.json")
            .expect("release distribution target should exist"),
    )
    .expect("release distribution target should be valid JSON");

    assert_eq!(
        release_target["github"]["observed_release_ref"],
        "refs/tags/code-scan-action-0.2.0"
    );
    assert_eq!(
        release_target["github"]["observed_release_commit"],
        DYNAMIC_RELEASE_SHA
    );
    let source = release_target["github"]["source"]
        .as_str()
        .expect("github source should be a string");
    assert!(
        source.contains("refs/tags/code-scan-action-0.2.0"),
        "{source}"
    );
    assert!(
        !source.contains("refs/tags/code-scan-action-0.1.7"),
        "{source}"
    );
    assert_eq!(
        release_target["github_latest_release_channel"],
        "github-action"
    );
    assert_eq!(
        release_target["github_latest_release_is_core_package"],
        false
    );
    assert_eq!(
        release_target["current_repository_perfect_claim_allowed"],
        false
    );

    for docs_path in [
        "docs/compatibility/target-policy.md",
        "docs/compatibility/matrix.md",
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
        "BLOCKED-task-22.1-perfect-refactor-external-authority.md",
    ] {
        let docs = std::fs::read_to_string(docs_path).expect("docs should be readable");
        assert!(docs.contains("dynamic latest release"), "{docs_path}");
        assert!(
            docs.contains("perfect-refactor"),
            "{docs_path} should keep perfect-refactor blocker language"
        );
    }

    let _ = std::fs::remove_dir_all(fixture_dir);
}

fn distribution_target() -> promptfoo_rs::compatibility::inventory::UpstreamDistributionTarget {
    build_upstream_distribution_target(
        parse_npm_package_observation(NPM_VIEW).expect("npm package metadata parses"),
        current_observation(),
        frozen_reference(),
    )
}

fn current_observation() -> CurrentUpstreamObservation {
    CurrentUpstreamObservation::from_ls_remote(LS_REMOTE).expect("ls-remote output parses")
}

fn frozen_reference() -> FrozenSourceReference {
    FrozenSourceReference::new(
        "0.121.13",
        "refs/tags/0.121.13",
        FROZEN_SHA,
        NPM_INTEGRITY,
        "git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13",
    )
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
