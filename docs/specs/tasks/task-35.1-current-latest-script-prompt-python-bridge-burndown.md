# Task 35.1: current-latest-script-prompt-python-bridge-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 35 - current-latest-script-prompt-python-bridge-burndown
**Dependencies**: task-4.3-custom-assertion-contracts, task-9.1-script-bridge-sandbox, task-24.4-current-latest-exhaustive-quality-gate, task-32.1-current-latest-local-prompt-processor-burndown, task-34.1-current-latest-eval-scheduler-rate-limit-burndown

## 1. Background

Phase 34 leaves tracked-lock phase-smoke artifacts at 33 P0 golden blockers, including three script-backed prompt processor blockers (`executable.ts`, `javascript.ts`, `python.ts`) and seven script bridge blockers (`src/python/*`, `src/ruby/*`). Task 32.1 intentionally kept script-backed processors blocked until subprocess evidence existed; task 9.1 already established default-deny, explicit authorization, timeout, env allowlist, stdout/stderr, and redaction boundaries. This task adds deterministic local Node/Python subprocess evidence for JS/Python/executable processors and Python bridge rows, while preserving Ruby rows because this environment has no Ruby runtime and no PRD/ADR basis to fake Ruby parity. 依据 PRD §Core Capabilities / §Compatibility Matrix、adapter §Constraints、ADR-005、ADR-009、ADR-011、task 4.3、task 9.1、task 32.1、Phase 34 §9。

## 2. Goal

Implement deterministic script-backed prompt processor and Python bridge behavior, promote only the three prompt processor rows and five Python bridge rows to P0 native fixture evidence, reduce total current-latest blockers from 33 to 25, and keep Ruby/config/provider/publication/perfect-refactor blockers explicit.

## 3. Scope

### In Scope

- `src/script_bridge/mod.rs`
- `src/script_bridge/prompt_processor.rs`
- `src/script_bridge/python.rs`
- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_script_prompt_python_bridge_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不安装或声称 Ruby runtime parity；`src/ruby/*` rows 必须保持 explicit blockers。
- 不调用真实 provider、外部服务、private SDK、账号或 API key。
- 不放宽 script bridge default-deny、timeout、env allowlist、stderr capture、secret redaction 或 subprocess authorization 边界。
- 不解决 config external authority、provider external authority、current-target drift、publication authority 或“无任何潜在 bug”承诺。
- 不把本地 Node/Python mock script evidence 伪装成 live provider/account/SDK authority。

## 4. Users / Actors

- Script bridge maintainer: needs deterministic processor and Python bridge behavior tested under explicit authorization.
- Release reviewer: needs script-backed prompt and Python rows promoted only when backed by item-level tests and artifacts.
- Security reviewer: needs Ruby and external authority gaps to remain visible instead of hidden by broad script bridge classification.

## 5. Behavior Contract

Script-backed prompt processors must execute only with `ScriptAuthorization::Allow`, serialize prompt/vars payloads to JSON stdin, parse JSON stdout into a stable processed prompt, capture stderr, pass only allowlisted env values, enforce timeout/stdin limits through `ScriptBridge`, and return stable errors for unauthorized execution. Python bridge calls must execute authorized Python subprocesses with JSON payloads, parse JSON stdout, preserve stderr, propagate timeout/I/O errors, and provide deterministic worker-pool result ordering with bounded concurrency. Current-latest rows for `src/prompts/processors/{executable,javascript,python}.ts` must classify as `level=P0`, `implementation_status=native`, `verification_owner=script-bridge`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. Current-latest `src/python/{pythonUtils,stderr,worker,workerPool,wrapper}.ts` rows must classify the same way. Current-latest `src/ruby/{rubyUtils,wrapper}.ts` rows must remain `level=P0`, `implementation_status=blocked`, `verification_owner=script-bridge`, `evidence_kind=blocker`, with blocker reasons naming missing Ruby runtime evidence. Rust and shell artifact generation must use equivalent rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-32-current-latest-local-prompt-processor-burndown.md
- docs/specs/phases/phase-34-current-latest-eval-scheduler-rate-limit-burndown.md
- docs/specs/tasks/task-4.3-custom-assertion-contracts.md
- docs/specs/tasks/task-9.1-script-bridge-sandbox.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md
- docs/specs/tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/script-bridge.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::script_bridge::{PromptProcessorRequest, PromptProcessorResponse, PythonBridge, PythonBridgeRequest, PythonWorkerPool, ScriptAuthorization, ScriptBridgeErrorKind, ScriptKind, ScriptPromptProcessor, ScriptSandboxOptions}`, `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::{json, Value}`, `std::collections::BTreeMap`, `std::path::{Path, PathBuf}`, `std::process::Command`, `std::time::Duration`.
- Tooling commands: `node --version`, `python --version`, `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `ScriptPromptProcessor::process(request: PromptProcessorRequest, auth: ScriptAuthorization) -> Result<PromptProcessorResponse, promptfoo_rs::script_bridge::ScriptBridgeError>`
- `PromptProcessorRequest::new(script_kind: ScriptKind, script_path: impl Into<PathBuf>, program: impl Into<PathBuf>, args: Vec<String>, prompt: impl Into<String>, vars: serde_json::Value, options: ScriptSandboxOptions) -> PromptProcessorRequest`
- `PythonBridge::call(request: PythonBridgeRequest, auth: ScriptAuthorization) -> Result<promptfoo_rs::script_bridge::PythonBridgeResponse, promptfoo_rs::script_bridge::ScriptBridgeError>`
- `PythonBridgeRequest::new(script_path: impl Into<PathBuf>, program: impl Into<PathBuf>, args: Vec<String>, payload: serde_json::Value, options: ScriptSandboxOptions) -> PythonBridgeRequest`
- `PythonWorkerPool::new(max_workers: usize) -> PythonWorkerPool`
- `PythonWorkerPool::execute(&self, requests: Vec<PythonBridgeRequest>, auth: ScriptAuthorization) -> Vec<Result<promptfoo_rs::script_bridge::PythonBridgeResponse, promptfoo_rs::script_bridge::ScriptBridgeError>>`
- `current_latest_script_prompt_processor_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `current_latest_python_script_bridge_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- Shell contract: `currentLatestScriptPromptProcessorFixtureIds(id)`, `isCurrentLatestScriptPromptProcessorFixture(id, file)`, `currentLatestPythonScriptBridgeFixtureIds(id)`, `isCurrentLatestPythonScriptBridgeFixture(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (ADR-005 / task 9.1): JavaScript, Python, and executable prompt processors require explicit authorization, exchange JSON stdin/stdout, capture stderr, and enforce env/timeout boundaries through ScriptBridge.
- [x] **AC2** (ADR-005 / task 9.1): Python bridge wrapper and worker-pool behavior execute authorized Python subprocess calls with stable JSON parsing, stderr capture, error propagation, and deterministic result ordering.
- [x] **AC3** (ADR-009 / ADR-011): three current-latest script-backed prompt processor rows have P0 native fixture evidence with `script-bridge` owner in both Rust and shell artifacts.
- [x] **AC4** (ADR-009 / ADR-011): five current-latest Python bridge rows have P0 native fixture evidence, while two Ruby bridge rows remain explicit P0 blockers naming missing Ruby runtime evidence.
- [x] **AC5** (PRD §Compatibility Matrix): total current-latest golden blockers drop from 33 to 25, prompt-processing blockers drop to zero, script-bridge blockers are Ruby-only, and perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-35.1.1 | TEST-35.1.1 | tests/current_latest_script_prompt_python_bridge_burndown.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-35.1.1 | TEST-35.1.2 | tests/current_latest_script_prompt_python_bridge_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-35.1.1 | TEST-35.1.3 | tests/current_latest_script_prompt_python_bridge_burndown.rs | install, lint, typecheck, unit-test, e2e, build | Done |
| AC4 | SCEN-35.1.1 | TEST-35.1.4 | tests/current_latest_script_prompt_python_bridge_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC5 | SCEN-35.1.1 | TEST-35.1.5 | tests/current_latest_script_prompt_python_bridge_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, runtime-smoke, build | Done |

## 8. Risks

- Missing Node/Python on another environment would make this task fail correctly under adapter constraints; this task should not silently skip runtime-backed tests.
- Ruby rows are intentionally not promoted; doing so without Ruby would violate ADR-009 evidence semantics.
- Script fixtures must remain local and deterministic; no fixture may require network, provider credentials, private SDKs, or account-level state.
- Reducing script blockers does not prove config/provider external authority, publication, current-target readiness, or impossible zero-bug claims.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-02
- **改动文件**：
  - `docs/specs/phases/phase-35-current-latest-script-prompt-python-bridge-burndown.md`
  - `docs/specs/tasks/task-35.1-current-latest-script-prompt-python-bridge-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `src/script_bridge/mod.rs`
  - `src/script_bridge/prompt_processor.rs`
  - `src/script_bridge/python.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
  - `tests/current_latest_script_prompt_python_bridge_burndown.rs`
  - `tests/current_latest_local_prompt_processor_burndown.rs`
  - `tests/current_latest_prompt_processing_burndown.rs`
- **commit 列表**：
  - `7970f0a` `docs(spec): add phase 35 current latest script bridge burndown`
  - `fa6dc5c` `docs(spec): task-35.1 enters implementation`
  - `140e07a` `test(script-bridge): add current latest script prompt Python bridge RED tests`
  - `cfb9072` `feat(script-bridge): implement current latest script prompt Python evidence`
  - 本次 docs 回填提交：`docs(spec): complete task 35.1 current latest script bridge burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-35.1.1 ~ TEST-35.1.5 与累计 current-latest prompt/script 分类测试均通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - build: PASS - helper 执行 adapter Build 通过。
  - coverage: PASS - helper 执行 adapter Coverage，通过覆盖率阈值守卫。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.json` 为 `ready-with-blockers`、`blocker_count=25`、分组 `config=7, provider=16, script-bridge=2`，`prompt-processing=0`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：仍保留 config=7、provider external-authority=16、Ruby script-bridge=2、current-target drift、publication-authority 等 blockers；不承诺“无任何潜在 bug”。本 task 只证明本地 deterministic Node/Python subprocess prompt processor 与 Python bridge contract，不证明 Ruby runtime、真实 provider 服务、账号级权限、private SDK 或公开发布行为。
- **下游 task 影响**：后续 current-latest burndown 可从 25 个总 blockers 继续推进；Ruby bridge 需要 Ruby runtime fixture 或明确 Waive，config/provider external authority rows 仍需真实权限、产品/服务合同证据或正式 waiver。
