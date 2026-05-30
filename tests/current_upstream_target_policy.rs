use std::path::Path;

use promptfoo_rs::compatibility::baseline_lock::{
    record_moving_upstream_observation, validate_single_stable_target, BaselineLock,
    CompatibilityTargetPolicy, StableTarget, StableTargetKind,
};

#[test]
fn test_11_1_1_stable_policy_separates_frozen_target_from_moving_upstream() {
    /* TEST-11.1.1 */
    let policy =
        CompatibilityTargetPolicy::load(Path::new("docs/compatibility/target-policy.md"))
            .expect("target policy should parse");

    let report = validate_single_stable_target(&policy);

    assert!(report.is_release_ready(), "{report:#?}");
    assert_eq!(report.stable_target_count, 1);
    assert!(report.moving_upstream_is_tracking_only);
    assert_eq!(
        policy.stable_targets[0].kind,
        StableTargetKind::FrozenBaseline
    );
    assert_eq!(policy.stable_targets[0].package_version, "0.121.13");
    assert_eq!(
        policy.stable_targets[0].git_commit,
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
    );
    assert_eq!(
        policy.moving_upstream_observations[0].head,
        "945fda5d965ed27abb302fe0f0910b7dddea5dde"
    );
}

#[test]
fn test_11_1_2_validator_rejects_floating_or_multiple_stable_targets() {
    /* TEST-11.1.2 */
    let policy = CompatibilityTargetPolicy {
        stable_targets: vec![
            StableTarget {
                id: "floating-latest".to_string(),
                kind: StableTargetKind::Floating,
                package_version: "latest".to_string(),
                git_ref: "HEAD".to_string(),
                git_commit: "main".to_string(),
                npm_integrity: "sha512-floating".to_string(),
                container_digest: "latest".to_string(),
            },
            StableTarget {
                id: "second-target".to_string(),
                kind: StableTargetKind::Rebaselined,
                package_version: "0.121.13".to_string(),
                git_ref: "refs/tags/0.121.13".to_string(),
                git_commit: "4860e990c7e9a2f8f677173fb92cf9867b34d03f".to_string(),
                npm_integrity: "sha512-stable".to_string(),
                container_digest:
                    "sha256:3993e7c105bcbc1c8f763309552728dd2bf30ff5c9c2e14ec69297b42d096f80"
                        .to_string(),
            },
        ],
        moving_upstream_observations: Vec::new(),
    };

    let report = validate_single_stable_target(&policy);

    assert!(!report.is_release_ready());
    assert!(report
        .rejected_reasons
        .iter()
        .any(|reason| reason.contains("multiple stable targets")));
    assert!(report
        .rejected_reasons
        .iter()
        .any(|reason| reason.contains("floating stable target")));
}

#[test]
fn test_11_1_3_moving_upstream_observation_is_append_only_tracking() {
    /* TEST-11.1.3 */
    let observation = record_moving_upstream_observation(
        "945fda5d965ed27abb302fe0f0910b7dddea5dde",
        "0.121.13",
    );

    assert_eq!(
        observation.head,
        "945fda5d965ed27abb302fe0f0910b7dddea5dde"
    );
    assert_eq!(observation.package_version, "0.121.13");
    assert_eq!(observation.source, "upstream origin/main tracking");
    assert!(!observation.collected_at.trim().is_empty());
    assert!(!observation.modifies_frozen_baseline);

    let lock = BaselineLock::from_markdown(Path::new("docs/compatibility/baseline.lock.md"))
        .expect("baseline lock should remain parseable");
    assert_eq!(
        lock.git_commit.sha,
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
    );
}
