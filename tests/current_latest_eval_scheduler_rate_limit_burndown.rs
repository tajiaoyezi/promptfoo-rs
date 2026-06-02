use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use promptfoo_rs::eval::rate_limit::{
    AdaptiveConcurrencyController, AdaptiveObservation, ProviderCallExecutionContext,
    ProviderCallOutcome, ProviderRateLimitRegistry, RateLimitDecision, RateLimitHeaderState,
    RateLimitKey,
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

#[test]
fn test_34_1_1_rate_limit_headers_and_keys_are_deterministic() {
    /* TEST-34.1.1 */
    let headers = BTreeMap::from([
        (
            "x-ratelimit-remaining-requests".to_string(),
            "0".to_string(),
        ),
        (
            "x-ratelimit-reset-requests".to_string(),
            "1250ms".to_string(),
        ),
    ]);

    let parsed = RateLimitHeaderState::parse(&headers);

    assert_eq!(parsed.remaining, Some(0));
    assert_eq!(parsed.reset_after, Some(Duration::from_millis(1250)));

    let retry_after = RateLimitHeaderState::parse(&BTreeMap::from([(
        "retry-after".to_string(),
        "2".to_string(),
    )]));
    assert_eq!(retry_after.remaining, None);
    assert_eq!(retry_after.reset_after, Some(Duration::from_secs(2)));

    let key = RateLimitKey::new("OpenAI Compatible", "gpt-4o-mini", "Requests");
    assert_eq!(key.as_str(), "openai-compatible:gpt-4o-mini:requests");
}

#[test]
fn test_34_1_2_rate_limit_registry_returns_deterministic_delay_decisions() {
    /* TEST-34.1.2 */
    let mut registry = ProviderRateLimitRegistry::default();
    let key = RateLimitKey::new("openai", "gpt-4o-mini", "requests");

    assert_eq!(registry.delay_for(&key), RateLimitDecision::Ready);

    registry.record_headers(
        key.clone(),
        &BTreeMap::from([
            (
                "x-ratelimit-remaining-requests".to_string(),
                "0".to_string(),
            ),
            (
                "x-ratelimit-reset-requests".to_string(),
                "500ms".to_string(),
            ),
        ]),
    );
    assert_eq!(
        registry.delay_for(&key),
        RateLimitDecision::Delay(Duration::from_millis(500))
    );

    registry.record_headers(
        key.clone(),
        &BTreeMap::from([
            (
                "x-ratelimit-remaining-requests".to_string(),
                "3".to_string(),
            ),
            ("x-ratelimit-reset-requests".to_string(), "0ms".to_string()),
        ]),
    );
    assert_eq!(registry.delay_for(&key), RateLimitDecision::Ready);
}

#[test]
fn test_34_1_3_adaptive_concurrency_respects_bounds() {
    /* TEST-34.1.3 */
    let mut controller = AdaptiveConcurrencyController::new(2, 8, 4);

    assert_eq!(controller.current_limit(), 4);
    assert_eq!(controller.observe(AdaptiveObservation::Success), 5);
    assert_eq!(controller.observe(AdaptiveObservation::Failure), 4);
    assert_eq!(controller.observe(AdaptiveObservation::RateLimited), 2);
    assert_eq!(controller.observe(AdaptiveObservation::RateLimited), 2);

    for _ in 0..10 {
        controller.observe(AdaptiveObservation::Success);
    }
    assert_eq!(controller.current_limit(), 8);
}

#[test]
fn test_34_1_4_provider_call_context_and_wrapper_record_local_policy() {
    /* TEST-34.1.4 */
    let mut registry = ProviderRateLimitRegistry::default();
    let context = ProviderCallExecutionContext::new("openai", "gpt-4o-mini", "requests", 2);

    assert_eq!(
        context.rate_limit_key.as_str(),
        "openai:gpt-4o-mini:requests"
    );
    assert_eq!(context.attempt, 2);

    let observed_headers = BTreeMap::from([
        (
            "x-ratelimit-remaining-requests".to_string(),
            "0".to_string(),
        ),
        (
            "x-ratelimit-reset-requests".to_string(),
            "250ms".to_string(),
        ),
    ]);
    let result = registry.call_with_policy(context.clone(), |call_context| {
        assert_eq!(call_context.provider_id, "openai");
        ProviderCallOutcome::success("fixture-output", observed_headers.clone())
    });

    assert_eq!(result.context, context);
    assert_eq!(result.pre_call_decision, RateLimitDecision::Ready);
    assert_eq!(result.output.as_deref(), Some("fixture-output"));
    assert_eq!(result.error, None);
    assert_eq!(result.observed_headers, observed_headers);
    assert_eq!(
        registry.delay_for(&context.rate_limit_key),
        RateLimitDecision::Delay(Duration::from_millis(250))
    );

    let delayed = registry.call_with_policy(context.clone(), |_call_context| {
        ProviderCallOutcome::error("rate limit: 429", BTreeMap::new())
    });
    assert_eq!(
        delayed.pre_call_decision,
        RateLimitDecision::Delay(Duration::from_millis(250))
    );
    assert_eq!(delayed.output, None);
    assert_eq!(delayed.error.as_deref(), Some("rate limit: 429"));
}

#[test]
fn test_34_1_5_current_latest_eval_scheduler_rows_have_native_fixture_evidence() {
    /* TEST-34.1.5 */
    let root = fixture_dir("rust-fixture");
    write_eval_scheduler_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in eval_scheduler_rate_limit_sources() {
        let row = eval_runner_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "eval-runner", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:eval-runner:"),
            "{row:#?}"
        );
    }

    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    let script = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = script["rows"]
        .as_array()
        .expect("script rows should be an array");

    assert_eq!(
        eval_runner_rows_with_json(script_rows, "P0", "native", "fixture").len(),
        7
    );
    assert_eq!(
        eval_runner_rows_with_json(script_rows, "P0", "blocked", "blocker").len(),
        0
    );

    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");
    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    assert_eq!(golden["blocker_count"], Value::from(0));
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(true));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-eval-scheduler-{name}-{}",
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

fn write_eval_scheduler_source(root: &Path) {
    for relative in eval_scheduler_rate_limit_sources() {
        write_file(root, relative, "export const schedulerEvidence = true;");
    }
}

fn eval_scheduler_rate_limit_sources() -> &'static [&'static str] {
    &[
        "src/scheduler/adaptiveConcurrency.ts",
        "src/scheduler/headerParser.ts",
        "src/scheduler/providerCallExecutionContext.ts",
        "src/scheduler/providerRateLimitState.ts",
        "src/scheduler/providerWrapper.ts",
        "src/scheduler/rateLimitKey.ts",
        "src/scheduler/rateLimitRegistry.ts",
    ]
}

fn eval_runner_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "eval-runner")
        .unwrap_or_else(|| panic!("missing eval-runner row for {source}: {rows:#?}"))
}

fn eval_runner_rows_with_json<'a>(
    rows: &'a [Value],
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String("eval-runner".to_string())
                && row["level"] == Value::String(level.to_string())
                && row["implementation_status"] == Value::String(implementation_status.to_string())
                && row["evidence_kind"] == Value::String(evidence_kind.to_string())
        })
        .collect()
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

fn run_current_latest_script(gate_dir: &Path, script: &str) {
    let command = format!(
        "CURRENT_LATEST_GATE_DIR='{}' bash {}",
        shell_escape(gate_dir),
        script
    );
    run_bash(&command);
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
        "C:/Program Files/Git/bin/bash.exe"
    } else {
        "bash"
    }
}
