use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    evaluate_current_claim_policy, write_current_upstream_policy, CurrentUpstreamObservation,
    FrozenSourceReference, TargetMode,
};
use serde_json::Value;

const FROZEN_SHA: &str = "4860e990c7e9a2f8f677173fb92cf9867b34d03f";
const CURRENT_HEAD_SHA: &str = "ff8eafd743cf6d63dd85b790ad8a4c73ede5828d";
const CODE_SCAN_RELEASE_SHA: &str = "1c743afe0e4807882e858c4f322fc064fa5f0770";
const LS_REMOTE: &str = "\
ff8eafd743cf6d63dd85b790ad8a4c73ede5828d\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

#[test]
fn test_18_3_1_current_and_frozen_refs_are_recorded_without_floating_stable_target() {
    /* TEST-18.3.1 */
    let observation = CurrentUpstreamObservation::from_ls_remote(LS_REMOTE)
        .expect("ls-remote output should parse");
    let frozen = frozen_reference();

    assert_eq!(observation.current_head, CURRENT_HEAD_SHA);
    assert_eq!(observation.frozen_tag_ref, "refs/tags/0.121.13");
    assert_eq!(observation.frozen_tag_commit, FROZEN_SHA);
    assert_eq!(
        observation.observed_release_ref.as_deref(),
        Some("refs/tags/code-scan-action-0.1.7")
    );
    assert_eq!(
        observation.observed_release_commit.as_deref(),
        Some(CODE_SCAN_RELEASE_SHA)
    );
    assert!(observation
        .current_head
        .chars()
        .all(|c| c.is_ascii_hexdigit()));
    assert_eq!(observation.current_head.len(), 40);
    assert!(frozen.validate_non_floating().is_ok());

    let floating = FrozenSourceReference::new(
        "latest",
        "HEAD",
        "main",
        "sha512-floating",
        "git ls-remote promptfoo HEAD",
    );
    assert!(floating.validate_non_floating().is_err());
}

#[test]
fn test_18_3_2_frozen_mode_rejects_current_perfect_claim_when_head_differs() {
    /* TEST-18.3.2 */
    let observation = CurrentUpstreamObservation::from_ls_remote(LS_REMOTE)
        .expect("ls-remote output should parse");
    let policy =
        evaluate_current_claim_policy(&frozen_reference(), &observation, TargetMode::Frozen);

    assert_eq!(policy.target_mode, TargetMode::Frozen);
    assert!(!policy.current_perfect_claim_allowed, "{policy:#?}");
    assert_eq!(policy.stable_claim, "frozen-baseline compatibility");
    assert!(policy.reason.contains(CURRENT_HEAD_SHA), "{policy:#?}");
    assert!(policy.reason.contains(FROZEN_SHA), "{policy:#?}");
}

#[test]
fn test_18_3_3_current_mode_requires_all_evidence_to_share_observed_ref() {
    /* TEST-18.3.3 */
    let observation = CurrentUpstreamObservation::from_ls_remote(LS_REMOTE)
        .expect("ls-remote output should parse");
    let frozen = frozen_reference();

    let missing = evaluate_current_claim_policy(&frozen, &observation, TargetMode::Current);
    assert!(!missing.current_perfect_claim_allowed, "{missing:#?}");
    assert_eq!(
        missing.required_current_evidence,
        vec![
            "source_inventory",
            "matrix",
            "fixtures",
            "golden_corpus",
            "release_candidate"
        ]
    );
    assert_eq!(missing.missing_current_evidence.len(), 5);

    let current_ready = observation.clone().with_current_evidence_refs([
        ("source_inventory", CURRENT_HEAD_SHA),
        ("matrix", CURRENT_HEAD_SHA),
        ("fixtures", CURRENT_HEAD_SHA),
        ("golden_corpus", CURRENT_HEAD_SHA),
        ("release_candidate", CURRENT_HEAD_SHA),
    ]);
    let ready = evaluate_current_claim_policy(&frozen, &current_ready, TargetMode::Current);
    assert!(ready.current_perfect_claim_allowed, "{ready:#?}");
    assert!(ready.missing_current_evidence.is_empty(), "{ready:#?}");
    assert!(ready.mismatched_current_evidence.is_empty(), "{ready:#?}");

    let mismatched = observation.with_current_evidence_refs([
        ("source_inventory", CURRENT_HEAD_SHA),
        ("matrix", FROZEN_SHA),
        ("fixtures", CURRENT_HEAD_SHA),
        ("golden_corpus", CURRENT_HEAD_SHA),
        ("release_candidate", CURRENT_HEAD_SHA),
    ]);
    let blocked = evaluate_current_claim_policy(&frozen, &mismatched, TargetMode::Current);
    assert!(!blocked.current_perfect_claim_allowed, "{blocked:#?}");
    assert!(blocked
        .mismatched_current_evidence
        .iter()
        .any(|item| item == "matrix"));
}

#[test]
fn test_18_3_4_policy_artifact_and_release_candidate_display_target_mode() {
    /* TEST-18.3.4 */
    let observation = CurrentUpstreamObservation::from_ls_remote(LS_REMOTE)
        .expect("ls-remote output should parse");
    let policy =
        evaluate_current_claim_policy(&frozen_reference(), &observation, TargetMode::Frozen);
    let output_path = Path::new("target/test-current-upstream-policy.json");
    write_current_upstream_policy(&policy, output_path).expect("policy artifact should write");
    let artifact: Value = serde_json::from_str(
        &std::fs::read_to_string(output_path).expect("policy artifact should be readable"),
    )
    .expect("policy artifact should be valid JSON");
    assert_eq!(artifact["target_mode"], "frozen");
    assert_eq!(artifact["current_perfect_claim_allowed"], false);
    assert_eq!(artifact["current"]["current_head"], CURRENT_HEAD_SHA);

    std::fs::write("target/test-current-upstream-ls-remote.txt", LS_REMOTE)
        .expect("fixture should write");
    let script_output = Command::new(git_bash())
        .args([
            "-lc",
            "CURRENT_UPSTREAM_LS_REMOTE_FILE=target/test-current-upstream-ls-remote.txt bash scripts/release/current-upstream-policy.sh",
        ])
        .output()
        .expect("current upstream policy script should execute");
    assert!(
        script_output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&script_output.stdout),
        String::from_utf8_lossy(&script_output.stderr)
    );
    let release_policy: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/current-upstream-policy.json")
            .expect("release policy artifact should exist"),
    )
    .expect("release policy should be valid JSON");
    assert_eq!(release_policy["target_mode"], "frozen");
    assert_eq!(release_policy["current_perfect_claim_allowed"], false);

    let runtime_smoke =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_smoke.contains("current-upstream-policy.sh"));
    assert!(runtime_smoke.contains("\"target_policy\""));
}

fn frozen_reference() -> FrozenSourceReference {
    FrozenSourceReference::new(
        "0.121.13",
        "refs/tags/0.121.13",
        FROZEN_SHA,
        "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==",
        "git ls-remote https://github.com/promptfoo/promptfoo.git refs/tags/0.121.13",
    )
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
