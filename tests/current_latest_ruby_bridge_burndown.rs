use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use promptfoo_rs::script_bridge::{
    RubyBridge, RubyBridgeRequest, RubyWorkerPool, ScriptAuthorization, ScriptBridgeErrorKind,
    ScriptSandboxOptions,
};
use serde_json::{json, Value};

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
fn test_36_1_1_ruby_bridge_uses_authorized_json_bridge() {
    /* TEST-36.1.1 */
    assert_runtime_available(ruby_program(), "--version");

    let root = fixture_dir("ruby-bridge");
    let script = root.join("worker.rb");
    write_file(
        &script,
        r#"
require 'json'
payload = JSON.parse(STDIN.read)
STDERR.puts "stderr:#{payload['id']}"
print JSON.generate({
  id: payload['id'],
  value: payload['value'] * 2,
  allowed: ENV.fetch('PROMPTFOO_ALLOWED', ''),
  secret: ENV.fetch('PROMPTFOO_SECRET', '')
})
"#,
    );

    let unauthorized = RubyBridge::call(
        ruby_request(&script, json!({"id": 1, "value": 21}), script_options()),
        ScriptAuthorization::Deny,
    )
    .expect_err("default-deny must reject Ruby bridge execution");
    assert_eq!(unauthorized.kind, ScriptBridgeErrorKind::Unauthorized);

    let mut request = ruby_request(&script, json!({"id": 1, "value": 21}), script_options());
    request.env = BTreeMap::from([
        ("PROMPTFOO_ALLOWED".to_string(), "visible".to_string()),
        ("PROMPTFOO_SECRET".to_string(), "hidden".to_string()),
    ]);
    let response =
        RubyBridge::call(request, ScriptAuthorization::Allow).expect("Ruby bridge should run");

    assert_eq!(response.exit_code, Some(0));
    assert_eq!(
        response.json,
        json!({"id": 1, "value": 42, "allowed": "visible", "secret": ""})
    );
    assert!(response.stderr.contains("stderr:1"), "{response:?}");

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_36_1_2_ruby_worker_pool_is_deterministic_and_errors_are_stable() {
    /* TEST-36.1.2 */
    assert_runtime_available(ruby_program(), "--version");

    let root = fixture_dir("ruby-worker-pool");
    let worker = root.join("worker.rb");
    write_file(
        &worker,
        r#"
require 'json'
payload = JSON.parse(STDIN.read)
STDERR.puts "stderr:#{payload['id']}"
print JSON.generate({ id: payload['id'], value: payload['value'] * 3 })
"#,
    );

    let pool = RubyWorkerPool::new(2);
    let results = pool.execute(
        vec![
            ruby_request(&worker, json!({"id": 1, "value": 1}), script_options()),
            ruby_request(&worker, json!({"id": 2, "value": 2}), script_options()),
            ruby_request(&worker, json!({"id": 3, "value": 3}), script_options()),
        ],
        ScriptAuthorization::Allow,
    );
    let ids = results
        .into_iter()
        .map(|result| {
            result
                .expect("Ruby worker result should succeed")
                .json
                .get("id")
                .and_then(Value::as_i64)
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(ids, vec![1, 2, 3]);

    let invalid = root.join("invalid.rb");
    write_file(&invalid, "print 'not-json'\n");
    let invalid_error = RubyBridge::call(
        ruby_request(&invalid, json!({"id": 9, "value": 1}), script_options()),
        ScriptAuthorization::Allow,
    )
    .expect_err("invalid JSON should be a stable bridge error");
    assert_eq!(invalid_error.kind, ScriptBridgeErrorKind::Io);
    assert_eq!(invalid_error.code, "ruby_bridge_invalid_json");

    let sleeper = root.join("sleep.rb");
    write_file(&sleeper, "sleep 0.5\nprint '{}'\n");
    let mut timeout_options = script_options();
    timeout_options.timeout = Duration::from_millis(25);
    let timeout = RubyBridge::call(
        ruby_request(&sleeper, json!({"id": 10, "value": 1}), timeout_options),
        ScriptAuthorization::Allow,
    )
    .expect_err("timeout should propagate from ScriptBridge");
    assert_eq!(timeout.kind, ScriptBridgeErrorKind::Timeout);

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_36_1_3_ruby_bridge_rows_have_native_fixture_evidence() {
    /* TEST-36.1.3 */
    let root = fixture_dir("ruby-inventory");
    write_current_latest_sources(&root, false);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in ruby_bridge_sources() {
        let row = row_for_source(&inventory.rows, source, "script-bridge");
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "script-bridge", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:script-bridge:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_36_1_4_ruby_bridge_artifacts_clear_script_bridge_blockers() {
    /* TEST-36.1.4 */
    let root = fixture_dir("ruby-artifacts");
    write_current_latest_sources(&root, true);
    let gate_dir = fixture_dir("ruby-artifacts-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let source_inventory = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = source_inventory["rows"]
        .as_array()
        .expect("script rows should be an array");
    assert_eq!(
        rows_with_json(script_rows, "script-bridge", "P0", "native", "fixture").len(),
        2
    );
    assert_eq!(
        rows_with_json(script_rows, "script-bridge", "P0", "blocked", "blocker").len(),
        0
    );

    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");
    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    let script_blockers = blockers
        .iter()
        .filter(|blocker| {
            blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("script-bridge:")
        })
        .collect::<Vec<_>>();
    assert_eq!(script_blockers.len(), 0, "{script_blockers:#?}");
    assert_eq!(golden["blocker_count"], Value::from(23));

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

#[test]
fn test_36_1_5_perfect_claim_remains_false_with_external_authority_blockers() {
    /* TEST-36.1.5 */
    let root = fixture_dir("ruby-quality");
    write_current_latest_sources(&root, true);
    let gate_dir = fixture_dir("ruby-quality-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    let capabilities = blockers
        .iter()
        .map(|blocker| blocker["capability"].as_str().unwrap_or_default())
        .collect::<Vec<_>>();

    assert_eq!(golden["blocker_count"], Value::from(23));
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );
    assert!(
        capabilities
            .iter()
            .all(|capability| capability.starts_with("config:")
                || capability.starts_with("provider:"))
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn ruby_request(
    script_path: &Path,
    payload: Value,
    options: ScriptSandboxOptions,
) -> RubyBridgeRequest {
    RubyBridgeRequest::new(
        script_path,
        ruby_program(),
        vec![script_path.to_string_lossy().to_string()],
        payload,
        options,
    )
}

fn script_options() -> ScriptSandboxOptions {
    ScriptSandboxOptions {
        timeout: Duration::from_secs(3),
        env_allowlist: vec!["PROMPTFOO_ALLOWED".to_string()],
        cwd: None,
        stdin_limit: 4096,
    }
}

fn fixture_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-ruby-bridge-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir should create");
    dir
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dir should create");
    }
    std::fs::write(path, contents).expect("fixture file should write");
}

fn write_relative(root: &Path, relative: &str, contents: &str) {
    write_file(&root.join(relative), contents);
}

fn write_current_latest_sources(root: &Path, include_external_blockers: bool) {
    for source in ruby_bridge_sources() {
        write_relative(root, source, "export const rubyBridgeEvidence = true;");
    }
    if include_external_blockers {
        for source in external_authority_sources() {
            write_relative(
                root,
                source,
                "export const externalAuthorityEvidence = true;",
            );
        }
    }
}

fn ruby_bridge_sources() -> &'static [&'static str] {
    &["src/ruby/rubyUtils.ts", "src/ruby/wrapper.ts"]
}

fn external_authority_sources() -> &'static [&'static str] {
    &[
        "src/globalConfig/accounts.ts",
        "src/globalConfig/cloud.ts",
        "src/globalConfig/globalConfig.ts",
        "src/server/config/serverConfig.ts",
        "src/server/routes/configs.ts",
        "src/tracing/otelConfig.ts",
        "src/types/api/configs.ts",
        "src/providers/anthropic/claudeCodeAuth.ts",
        "src/providers/openai/agents.ts",
        "src/providers/openai/agents-loader.ts",
        "src/providers/openai/agents-model-settings.ts",
        "src/providers/openai/agents-tracing.ts",
        "src/providers/openai/agents-types.ts",
        "src/providers/openai/assistant.ts",
        "src/providers/openai/billing.ts",
        "src/providers/openai/chatkit.ts",
        "src/providers/openai/chatkit-pool.ts",
        "src/providers/openai/chatkit-types.ts",
        "src/providers/openai/codex-app-server.ts",
        "src/providers/openai/codex-sdk.ts",
        "src/providers/openai/codexDefaults.ts",
        "src/providers/openai/codexSkillMetadata.ts",
        "src/providers/openai/realtime.ts",
    ]
}

fn row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
    category: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == category)
        .unwrap_or_else(|| panic!("missing {category} row for {source}: {rows:#?}"))
}

fn rows_with_json<'a>(
    rows: &'a [Value],
    category: &str,
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String(category.to_string())
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

fn assert_runtime_available(program: impl AsRef<Path>, arg: &str) {
    let output = Command::new(program.as_ref())
        .arg(arg)
        .output()
        .unwrap_or_else(|err| panic!("runtime {:?} should be available: {err}", program.as_ref()));
    assert!(
        output.status.success(),
        "runtime {:?} failed: stdout={} stderr={}",
        program.as_ref(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn ruby_program() -> PathBuf {
    if let Some(path) = std::env::var_os("PROMPTFOO_RS_RUBY") {
        return PathBuf::from(path);
    }
    let windows_ruby = PathBuf::from(r"C:\Ruby34-x64\bin\ruby.exe");
    if cfg!(windows) && windows_ruby.exists() {
        return windows_ruby;
    }
    PathBuf::from("ruby")
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
