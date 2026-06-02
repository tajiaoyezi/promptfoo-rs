use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use promptfoo_rs::script_bridge::{
    PromptProcessorRequest, PythonBridge, PythonBridgeRequest, PythonWorkerPool,
    ScriptAuthorization, ScriptBridgeErrorKind, ScriptKind, ScriptPromptProcessor,
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
fn test_35_1_1_script_prompt_processors_use_authorized_json_bridge() {
    /* TEST-35.1.1 */
    assert_runtime_available(node_program(), "--version");
    assert_runtime_available(python_program(), "--version");

    let root = fixture_dir("prompt-processors");
    let js_script = root.join("processor.js");
    let py_script = root.join("processor.py");
    write_file(
        &js_script,
        r#"
const chunks = [];
process.stdin.on('data', (chunk) => chunks.push(chunk));
process.stdin.on('end', () => {
  const payload = JSON.parse(Buffer.concat(chunks).toString('utf8'));
  console.error('stderr:javascript');
  process.stdout.write(JSON.stringify({
    prompt: `${payload.prompt}|${payload.vars.name}|${process.env.PROMPTFOO_ALLOWED}|${process.env.PROMPTFOO_SECRET || ''}`
  }));
});
"#,
    );
    write_file(
        &py_script,
        r#"
import json
import os
import sys
payload = json.loads(sys.stdin.read())
sys.stderr.write('stderr:python\n')
print(json.dumps({
    'prompt': f"{payload['prompt']}|{payload['vars']['name']}|{os.environ.get('PROMPTFOO_ALLOWED', '')}|{os.environ.get('PROMPTFOO_SECRET', '')}"
}))
"#,
    );

    let unauthorized = ScriptPromptProcessor::process(
        prompt_request(
            ScriptKind::JavaScript,
            &js_script,
            node_program(),
            vec![js_script.to_string_lossy().to_string()],
        ),
        ScriptAuthorization::Deny,
    )
    .expect_err("default-deny must reject script prompt processors");
    assert_eq!(unauthorized.kind, ScriptBridgeErrorKind::Unauthorized);

    let js = ScriptPromptProcessor::process(
        prompt_request(
            ScriptKind::JavaScript,
            &js_script,
            node_program(),
            vec![js_script.to_string_lossy().to_string()],
        ),
        ScriptAuthorization::Allow,
    )
    .expect("authorized JavaScript prompt processor should run");
    assert_eq!(js.prompt, "hello|Ada|visible|");
    assert!(js.stderr.contains("stderr:javascript"), "{js:?}");

    let py = ScriptPromptProcessor::process(
        prompt_request(
            ScriptKind::Python,
            &py_script,
            python_program(),
            vec![py_script.to_string_lossy().to_string()],
        ),
        ScriptAuthorization::Allow,
    )
    .expect("authorized Python prompt processor should run");
    assert_eq!(py.prompt, "hello|Ada|visible|");
    assert!(py.stderr.contains("stderr:python"), "{py:?}");

    let executable = ScriptPromptProcessor::process(
        prompt_request(
            ScriptKind::Shell,
            Path::new("test/fixtures/script-bridge/executable-inline"),
            shell_program(),
            shell_prompt_args(),
        ),
        ScriptAuthorization::Allow,
    )
    .expect("authorized executable prompt processor should run");
    assert_eq!(executable.prompt, "hello|Ada|visible|");

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_35_1_2_python_bridge_wrapper_worker_and_pool_are_deterministic() {
    /* TEST-35.1.2 */
    assert_runtime_available(python_program(), "--version");

    let root = fixture_dir("python-bridge");
    let worker = root.join("worker.py");
    write_file(
        &worker,
        r#"
import json
import sys
payload = json.loads(sys.stdin.read())
sys.stderr.write(f"stderr:{payload['id']}\n")
print(json.dumps({"id": payload["id"], "value": payload["value"] * 2}))
"#,
    );

    let response = PythonBridge::call(
        python_request(&worker, json!({"id": 1, "value": 21}), script_options()),
        ScriptAuthorization::Allow,
    )
    .expect("authorized Python bridge call should run");
    assert_eq!(response.exit_code, Some(0));
    assert_eq!(response.json, json!({"id": 1, "value": 42}));
    assert!(response.stderr.contains("stderr:1"), "{response:?}");

    let pool = PythonWorkerPool::new(2);
    let results = pool.execute(
        vec![
            python_request(&worker, json!({"id": 1, "value": 1}), script_options()),
            python_request(&worker, json!({"id": 2, "value": 2}), script_options()),
            python_request(&worker, json!({"id": 3, "value": 3}), script_options()),
        ],
        ScriptAuthorization::Allow,
    );
    let ids = results
        .into_iter()
        .map(|result| {
            result
                .expect("worker result should succeed")
                .json
                .get("id")
                .and_then(Value::as_i64)
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(ids, vec![1, 2, 3]);

    let sleeper = root.join("sleep.py");
    write_file(
        &sleeper,
        "import time\nimport sys\ntime.sleep(0.5)\nsys.stdout.write('{}')\n",
    );
    let mut timeout_options = script_options();
    timeout_options.timeout = Duration::from_millis(25);
    let timeout = PythonBridge::call(
        python_request(&sleeper, json!({"id": 9, "value": 1}), timeout_options),
        ScriptAuthorization::Allow,
    )
    .expect_err("timeout should propagate from ScriptBridge");
    assert_eq!(timeout.kind, ScriptBridgeErrorKind::Timeout);

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_35_1_3_script_prompt_processor_rows_have_native_fixture_evidence() {
    /* TEST-35.1.3 */
    let root = fixture_dir("prompt-inventory");
    write_current_latest_script_sources(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in script_prompt_processor_sources() {
        let row = row_for_source(&inventory.rows, source, "prompt-processing");
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "script-bridge", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference
                .starts_with("fixture:prompt-processing:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_35_1_4_python_bridge_rows_are_native_and_ruby_rows_remain_blockers() {
    /* TEST-35.1.4 */
    let root = fixture_dir("python-ruby-inventory");
    write_current_latest_script_sources(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in python_bridge_sources() {
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

    for source in ruby_bridge_sources() {
        let row = row_for_source(&inventory.rows, source, "script-bridge");
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "blocked", "{row:#?}");
        assert_eq!(row.verification_owner, "script-bridge", "{row:#?}");
        assert_eq!(row.evidence_kind, "blocker", "{row:#?}");
        assert!(
            row.blocker_reason
                .as_deref()
                .unwrap_or_default()
                .contains("Ruby"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_35_1_5_golden_keeps_only_ruby_script_blockers_visible() {
    /* TEST-35.1.5 */
    let root = fixture_dir("script-golden-source");
    write_current_latest_script_sources(&root);
    let gate_dir = fixture_dir("script-golden-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let source_inventory = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = source_inventory["rows"]
        .as_array()
        .expect("script rows should be an array");
    assert_eq!(
        rows_with_json(script_rows, "prompt-processing", "P0", "native", "fixture").len(),
        3
    );
    assert_eq!(
        rows_with_json(script_rows, "script-bridge", "P0", "native", "fixture").len(),
        5
    );
    assert_eq!(
        rows_with_json(script_rows, "script-bridge", "P0", "blocked", "blocker").len(),
        2
    );

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

    assert_eq!(golden["blocker_count"], Value::from(2));
    assert_eq!(
        capabilities,
        vec![
            "script-bridge:src-ruby-rubyutils",
            "script-bridge:src-ruby-wrapper",
        ]
    );
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
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

fn prompt_request(
    script_kind: ScriptKind,
    script_path: &Path,
    program: impl Into<PathBuf>,
    args: Vec<String>,
) -> PromptProcessorRequest {
    let mut request = PromptProcessorRequest::new(
        script_kind,
        script_path,
        program,
        args,
        "hello",
        json!({"name": "Ada"}),
        script_options(),
    );
    request.env = BTreeMap::from([
        ("PROMPTFOO_ALLOWED".to_string(), "visible".to_string()),
        ("PROMPTFOO_SECRET".to_string(), "hidden".to_string()),
    ]);
    request
}

fn python_request(
    script_path: &Path,
    payload: Value,
    options: ScriptSandboxOptions,
) -> PythonBridgeRequest {
    PythonBridgeRequest::new(
        script_path,
        python_program(),
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
        "promptfoo-rs-current-latest-script-bridge-{name}-{}",
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

fn write_current_latest_script_sources(root: &Path) {
    for source in script_prompt_processor_sources()
        .iter()
        .chain(python_bridge_sources().iter())
        .chain(ruby_bridge_sources().iter())
    {
        write_relative(root, source, "export const scriptBridgeEvidence = true;");
    }
}

fn script_prompt_processor_sources() -> &'static [&'static str] {
    &[
        "src/prompts/processors/executable.ts",
        "src/prompts/processors/javascript.ts",
        "src/prompts/processors/python.ts",
    ]
}

fn python_bridge_sources() -> &'static [&'static str] {
    &[
        "src/python/pythonUtils.ts",
        "src/python/stderr.ts",
        "src/python/worker.ts",
        "src/python/workerPool.ts",
        "src/python/wrapper.ts",
    ]
}

fn ruby_bridge_sources() -> &'static [&'static str] {
    &["src/ruby/rubyUtils.ts", "src/ruby/wrapper.ts"]
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

fn node_program() -> &'static str {
    "node"
}

fn python_program() -> &'static str {
    "python"
}

#[cfg(windows)]
fn shell_program() -> &'static str {
    "powershell.exe"
}

#[cfg(not(windows))]
fn shell_program() -> &'static str {
    "sh"
}

#[cfg(windows)]
fn shell_prompt_args() -> Vec<String> {
    vec![
        "-NoProfile".to_string(),
        "-NonInteractive".to_string(),
        "-Command".to_string(),
        "$payload = [Console]::In.ReadToEnd() | ConvertFrom-Json; [Console]::Out.Write((@{prompt = \"$($payload.prompt)|$($payload.vars.name)|$env:PROMPTFOO_ALLOWED|$env:PROMPTFOO_SECRET\"} | ConvertTo-Json -Compress))".to_string(),
    ]
}

#[cfg(not(windows))]
fn shell_prompt_args() -> Vec<String> {
    vec![
        "-c".to_string(),
        "python3 -c 'import json, os, sys; p=json.loads(sys.stdin.read()); print(json.dumps({\"prompt\": f\"{p[\"prompt\"]}|{p[\"vars\"][\"name\"]}|{os.environ.get(\"PROMPTFOO_ALLOWED\", \"\")}|{os.environ.get(\"PROMPTFOO_SECRET\", \"\")}\"}))'".to_string(),
    ]
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
