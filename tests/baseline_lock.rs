use std::path::Path;

use promptfoo_rs::compatibility::baseline_lock::{
    baseline_lock_release_status, validate_baseline_lock, BaselineLock, ReleaseGateStatus,
};

#[test]
fn test_1_1_1_records_all_expected_artifacts() {
    let lock = BaselineLock::from_markdown(Path::new("docs/compatibility/baseline.lock.md"))
        .expect("TEST-1.1.1 baseline lock should parse");

    assert_eq!(lock.git_tag.reference, "refs/tags/0.121.13");
    assert_eq!(
        lock.git_commit.sha,
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
    );
    assert_eq!(lock.npm_artifact.package, "promptfoo@0.121.13");
    assert_eq!(
        lock.container_artifact.reference,
        "ghcr.io/promptfoo/promptfoo:0.121.13"
    );

    let report = validate_baseline_lock(&lock);
    assert!(report.is_complete(), "{report:#?}");
}

#[test]
fn test_1_1_2_blocks_release_when_any_artifact_is_missing() {
    let mut lock = BaselineLock::from_markdown(Path::new("docs/compatibility/baseline.lock.md"))
        .expect("TEST-1.1.2 baseline lock should parse");
    lock.container_artifact.digest.clear();

    let report = validate_baseline_lock(&lock);

    assert!(!report.is_complete(), "{report:#?}");
    assert_eq!(
        baseline_lock_release_status(&report),
        ReleaseGateStatus::Blocked
    );
}

#[test]
fn test_1_1_3_rejects_latest_or_other_floating_references() {
    let lock = BaselineLock::from_markdown(Path::new("docs/compatibility/baseline.lock.md"))
        .expect("TEST-1.1.3 baseline lock should parse");

    let report = validate_baseline_lock(&lock);

    assert!(
        report.floating_references.is_empty(),
        "floating references must not be present: {report:#?}"
    );
}
