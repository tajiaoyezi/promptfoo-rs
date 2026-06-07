use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

#[test]
fn test_37_1_1_packet_records_current_latest_scope_and_counts() {
    /* TEST-37.1.1 */
    let gate_dir = gate_fixture("scope-counts", compact_current_latest_blockers());
    let packet = run_packet(&gate_dir);

    assert_eq!(packet["target_scope"], "current-latest", "{packet:#}");
    assert_eq!(
        packet["current_latest_golden_blocker_count"], 3,
        "{packet:#}"
    );
    assert_eq!(
        packet["current_latest_external_authority_blocker_count"], 3,
        "{packet:#}"
    );
    assert_eq!(
        packet["current_latest_required_decision_count"], packet["required_user_decision_count"],
        "{packet:#}"
    );
    assert_eq!(
        packet["current_latest_required_decision_count"], 6,
        "{packet:#}"
    );

    let _ = fs::remove_dir_all(gate_dir);
}

#[test]
fn test_37_1_2_every_current_latest_golden_blocker_has_one_decision() {
    /* TEST-37.1.2 */
    let blockers = compact_current_latest_blockers();
    let gate_dir = gate_fixture("golden-decisions", blockers.clone());
    let packet = run_packet(&gate_dir);
    let decisions = decision_items(&packet);

    for blocker in &blockers {
        let items = decisions
            .iter()
            .filter(|item| item["item_id"] == blocker.item_id)
            .collect::<Vec<_>>();
        assert_eq!(items.len(), 1, "{}: {packet:#}", blocker.item_id);
        let item = items[0];
        assert_eq!(item["auto_resolvable"], false, "{item:#}");
        assert!(
            item["source_artifact"]
                .as_str()
                .unwrap_or_default()
                .contains("current-latest"),
            "{item:#}"
        );
        assert_eq!(
            item["source_reference"].as_str(),
            Some(blocker.source_reference.as_str()),
            "{item:#}"
        );
        for field in [
            "required_actor",
            "required_evidence",
            "release_impact",
            "safe_local_fallback",
        ] {
            assert!(
                !item[field].as_str().unwrap_or_default().trim().is_empty(),
                "{field}: {item:#}"
            );
        }
    }

    for stale_id in ["provider:legacy-only", "config:legacy-source"] {
        assert!(
            !decisions.iter().any(|item| item["item_id"] == stale_id),
            "{stale_id}: {packet:#}"
        );
    }

    let _ = fs::remove_dir_all(gate_dir);
}

#[test]
fn test_37_1_3_current_target_and_publication_decisions_require_external_evidence() {
    /* TEST-37.1.3 */
    let gate_dir = gate_fixture("target-publication", compact_current_latest_blockers());
    let packet = run_packet(&gate_dir);
    let decisions = decision_items(&packet);

    let current_target = find_decision(decisions, "current-latest:target");
    assert_eq!(
        current_target["category"], "current-target",
        "{current_target:#}"
    );
    assert_eq!(
        current_target["auto_resolvable"], false,
        "{current_target:#}"
    );
    assert!(
        current_target["source_artifact"]
            .as_str()
            .unwrap_or_default()
            .ends_with("current-latest-target.json"),
        "{current_target:#}"
    );
    assert!(
        current_target["required_evidence"]
            .as_str()
            .unwrap_or_default()
            .contains("same locked current-latest target"),
        "{current_target:#}"
    );

    for item_id in ["publication:cargo", "publication:npm-wrapper"] {
        let publication = find_decision(decisions, item_id);
        assert_eq!(
            publication["category"], "publication-authority",
            "{publication:#}"
        );
        let required_evidence = publication["required_evidence"]
            .as_str()
            .unwrap_or_default();
        assert!(
            required_evidence.contains("credentials")
                && required_evidence.contains("legal/brand approval")
                && required_evidence.contains("external URL/digest"),
            "{publication:#}"
        );
        assert!(
            !required_evidence.contains("dry-run is enough"),
            "{publication:#}"
        );
        assert_eq!(publication["auto_resolvable"], false, "{publication:#}");
    }

    let _ = fs::remove_dir_all(gate_dir);
}

#[test]
fn test_37_1_4_packet_remains_blocked_and_not_auto_resolvable() {
    /* TEST-37.1.4 */
    let gate_dir = gate_fixture("blocked", compact_current_latest_blockers());
    let packet = run_packet(&gate_dir);

    assert_eq!(packet["status"], "blocked", "{packet:#}");
    assert_eq!(
        packet["perfect_refactor_claim_allowed"], false,
        "{packet:#}"
    );
    assert_eq!(packet["auto_resolvable"], false, "{packet:#}");
    assert!(
        decision_items(&packet)
            .iter()
            .all(|item| item["auto_resolvable"] == false),
        "{packet:#}"
    );

    let _ = fs::remove_dir_all(gate_dir);
}

#[test]
fn test_37_1_5_packet_reconciles_phase36_evidence_without_script_bridge_decisions() {
    /* TEST-37.1.5 */
    let gate_dir = gate_fixture("phase36", phase36_current_latest_blockers());
    let packet = run_packet(&gate_dir);
    let decisions = decision_items(&packet);
    let golden_decisions = decisions
        .iter()
        .filter(|item| {
            item["source_artifact"]
                .as_str()
                .unwrap_or_default()
                .ends_with("current-latest-golden-corpus.json")
        })
        .collect::<Vec<_>>();

    assert_eq!(
        packet["current_latest_golden_blocker_count"], 23,
        "{packet:#}"
    );
    assert_eq!(golden_decisions.len(), 23, "{packet:#}");
    assert!(
        !decisions.iter().any(|item| {
            item["item_id"]
                .as_str()
                .unwrap_or_default()
                .starts_with("script-bridge:")
        }),
        "{packet:#}"
    );

    let _ = fs::remove_dir_all(gate_dir);
}

#[derive(Clone, Debug)]
struct CurrentLatestBlocker {
    item_id: String,
    source_reference: String,
    category: String,
    reason: String,
}

fn compact_current_latest_blockers() -> Vec<CurrentLatestBlocker> {
    vec![
        blocker(
            "config:src-globalconfig-cloud",
            "config",
            "cloud config requires product service authority",
        ),
        blocker(
            "provider:src-providers-openai-billing",
            "provider",
            "billing provider requires account authority",
        ),
        blocker(
            "provider:src-providers-openai-codexskillmetadata",
            "provider",
            "Codex skill metadata requires current product authority",
        ),
    ]
}

fn phase36_current_latest_blockers() -> Vec<CurrentLatestBlocker> {
    let config = [
        "config:src-globalconfig-accounts",
        "config:src-globalconfig-cloud",
        "config:src-globalconfig-globalconfig",
        "config:src-server-config-serverconfig",
        "config:src-server-routes-configs",
        "config:src-tracing-otelconfig",
        "config:src-types-api-configs",
    ];
    let provider = [
        "provider:src-providers-anthropic-claudecodeauth",
        "provider:src-providers-openai-agents",
        "provider:src-providers-openai-agents-loader",
        "provider:src-providers-openai-agents-model-settings",
        "provider:src-providers-openai-agents-tracing",
        "provider:src-providers-openai-agents-types",
        "provider:src-providers-openai-assistant",
        "provider:src-providers-openai-billing",
        "provider:src-providers-openai-chatkit",
        "provider:src-providers-openai-chatkit-pool",
        "provider:src-providers-openai-chatkit-types",
        "provider:src-providers-openai-codex-app-server",
        "provider:src-providers-openai-codex-sdk",
        "provider:src-providers-openai-codexdefaults",
        "provider:src-providers-openai-codexskillmetadata",
        "provider:src-providers-openai-realtime",
    ];
    config
        .into_iter()
        .map(|item_id| blocker(item_id, "config", "config external authority remains"))
        .chain(
            provider
                .into_iter()
                .map(|item_id| blocker(item_id, "provider", "provider authority remains")),
        )
        .collect()
}

fn blocker(item_id: &str, category: &str, reason: &str) -> CurrentLatestBlocker {
    CurrentLatestBlocker {
        item_id: item_id.to_string(),
        source_reference: format!("promptfoo@current-latest:1d09df:{item_id}"),
        category: category.to_string(),
        reason: reason.to_string(),
    }
}

fn gate_fixture(name: &str, blockers: Vec<CurrentLatestBlocker>) -> PathBuf {
    let gate_dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-unblock-{name}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&gate_dir).expect("gate fixture dir should be created");
    write_gate_fixture(&gate_dir, blockers);
    gate_dir
}

fn write_gate_fixture(gate_dir: &Path, blockers: Vec<CurrentLatestBlocker>) {
    let release_blockers = blockers
        .iter()
        .map(|blocker| {
            json!({
                "capability": blocker.item_id,
                "category": blocker.category,
                "path": "p0_fixture_evidence",
                "class": "Bug",
                "message": blocker.reason,
                "source_reference": blocker.source_reference,
            })
        })
        .collect::<Vec<_>>();
    let matrix_rows = blockers
        .iter()
        .map(|blocker| {
            json!({
                "item_id": blocker.item_id,
                "capability": blocker.item_id,
                "level": "P0",
                "implementation_status": "blocked",
                "verification_owner": blocker.category,
                "evidence_kind": "blocker",
                "source_reference": blocker.source_reference,
                "blocker_reason": blocker.reason,
            })
        })
        .collect::<Vec<_>>();

    write_json(
        &gate_dir.join("current-latest-golden-corpus.json"),
        json!({
            "schema": "promptfoo-rs.current-latest-golden-corpus.v1",
            "status": "ready-with-blockers",
            "blocker_count": release_blockers.len(),
            "perfect_refactor_claim_allowed": false,
            "release_blockers": release_blockers,
            "blocker_groups": {
                "config": blockers.iter().filter(|blocker| blocker.category == "config").count(),
                "provider": blockers.iter().filter(|blocker| blocker.category == "provider").count()
            }
        }),
    );
    write_json(
        &gate_dir.join("current-latest-matrix.json"),
        json!({
            "schema": "promptfoo-rs.current-latest-matrix.v1",
            "status": "ready-with-blockers",
            "perfect_refactor_claim_allowed": false,
            "rows": matrix_rows,
        }),
    );
    write_json(
        &gate_dir.join("current-latest-quality.json"),
        json!({
            "schema": "promptfoo-rs.current-latest-quality.v1",
            "status": "ready-with-blockers",
            "local_current_latest_ready": false,
            "perfect_refactor_claim_allowed": false,
            "gate_statuses": {
                "golden_corpus": "ready-with-blockers",
                "current_target": "ready",
                "publication_authority": "credential-blocked"
            },
            "blockers": [
                {"item_id": "current-latest:golden-corpus", "category": "golden-corpus"},
                {"item_id": "current-latest:target", "category": "current-target"},
                {"item_id": "current-latest:publication-authority", "category": "publication-authority"}
            ]
        }),
    );
    write_json(
        &gate_dir.join("current-latest-target.json"),
        json!({
            "schema": "promptfoo-rs.current-latest-target.v1",
            "status": "ready",
            "target_selection_blocker_resolved": true,
            "current_latest_claim_allowed": false,
            "github": {"default_branch_head": "1d09dfeb5f0766905409117f923dd5c4b0838d9f"}
        }),
    );

    write_json(
        &gate_dir.join("perfect-refactor-claim.json"),
        json!({
            "schema": "promptfoo-rs.perfect-refactor-claim.v1",
            "perfect_refactor_claim_allowed": false,
            "current_perfect_claim_allowed": false,
            "source_p0_accounting_blocker_count": 1,
            "external_authority_blocker_count": 1,
            "blockers": [
                {"item_id": "source-accounting:p0-blockers"},
                {"item_id": "external-authority:blockers"},
                {"item_id": "publication-authority:published-evidence"},
                {"item_id": "current-upstream:frozen-target"}
            ]
        }),
    );
    write_json(
        &gate_dir.join("source-inventory-evidence.json"),
        json!({
            "schema": "promptfoo-rs.source-inventory-evidence.v1",
            "status": "blocked",
            "p0_accounting_blocker_count": 1,
            "remaining_p0_blockers": ["config:legacy-source"]
        }),
    );
    write_json(
        &gate_dir.join("external-authority-blockers.json"),
        json!({
            "schema": "promptfoo-rs.external-authority-blockers.v1",
            "status": "blocked",
            "blocker_count": 1,
            "blockers": [
                {
                    "item_id": "provider:legacy-only",
                    "authority_type": "product-authority",
                    "required_decision": "legacy source authority evidence",
                    "source_reference": "promptfoo@0.121.13:legacy",
                    "safe_local_fallback": "Keep legacy fixture blocked",
                    "release_impact": "Legacy-only blocker must not drive current-latest packet"
                }
            ]
        }),
    );
    write_json(
        &gate_dir.join("publication-authority.json"),
        json!({
            "schema": "promptfoo-rs.publication-authority.v1",
            "publication_ready": "credential-blocked",
            "credential_blocked": true,
            "legal_brand_blocked": true,
            "channels": [
                {
                    "channel": "cargo",
                    "published": false,
                    "authority_status": "credential-blocked",
                    "published_evidence": null
                },
                {
                    "channel": "npm-wrapper",
                    "published": false,
                    "authority_status": "credential-blocked",
                    "published_evidence": null
                }
            ]
        }),
    );
    write_json(
        &gate_dir.join("upstream-distribution-target.json"),
        json!({
            "schema": "promptfoo-rs.upstream-distribution-target.v1",
            "status": "ready-with-blockers",
            "current_repository_perfect_claim_allowed": false
        }),
    );
    write_json(
        &gate_dir.join("current-upstream-policy.json"),
        json!({
            "schema": "promptfoo-rs.current-upstream-policy.v1",
            "target_mode": "current-latest",
            "product_baseline_frozen": false,
            "current_upstream_rebaseline_required": true
        }),
    );
    write_json(
        &gate_dir.join("authority-decisions.fixture.json"),
        json!({
            "schema": "promptfoo-rs.authority-decisions.v1",
            "status": "ready",
            "rows": []
        }),
    );
    write_json(
        &gate_dir.join("publication-evidence.fixture.json"),
        json!({
            "schema": "promptfoo-rs.publication-evidence.v1",
            "status": "ready",
            "rows": []
        }),
    );
}

fn run_packet(gate_dir: &Path) -> Value {
    let output = Command::new(git_bash())
        .arg("scripts/release/perfect-refactor-unblock-packet.sh")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("GATE_DIR", slash_path(gate_dir))
        .env(
            "AUTHORITY_DECISIONS_MANIFEST",
            slash_path(&gate_dir.join("authority-decisions.fixture.json")),
        )
        .env(
            "PUBLICATION_EVIDENCE_MANIFEST",
            slash_path(&gate_dir.join("publication-evidence.fixture.json")),
        )
        .output()
        .expect("unblock packet script should start");
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let packet_path = gate_dir.join("perfect-refactor-unblock-packet.json");
    serde_json::from_str(&fs::read_to_string(&packet_path).expect("packet should be readable"))
        .expect("packet should be valid json")
}

fn decision_items(packet: &Value) -> &Vec<Value> {
    packet["decision_items"]
        .as_array()
        .expect("decision_items should be an array")
}

fn find_decision<'a>(items: &'a [Value], item_id: &str) -> &'a Value {
    items
        .iter()
        .find(|item| item["item_id"] == item_id)
        .unwrap_or_else(|| panic!("missing decision item {item_id}: {items:#?}"))
}

fn write_json(path: &Path, value: Value) {
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&value).expect("fixture should serialize")
        ),
    )
    .expect("fixture should be written");
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        "C:/Program Files/Git/bin/bash.exe"
    } else {
        "bash"
    }
}
