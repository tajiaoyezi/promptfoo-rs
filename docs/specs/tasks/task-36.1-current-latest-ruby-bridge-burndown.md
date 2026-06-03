# Task 36.1: current-latest-ruby-bridge-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 36 - current-latest-ruby-bridge-burndown
**Dependencies**: task-9.1-script-bridge-sandbox, task-24.4-current-latest-exhaustive-quality-gate, task-35.1-current-latest-script-prompt-python-bridge-burndown

## 1. Background

Phase 35 leaves tracked-lock phase-smoke artifacts at 25 P0 golden blockers, including exactly two `script-bridge` blockers: `src/ruby/rubyUtils.ts` and `src/ruby/wrapper.ts`. Task 35.1 intentionally kept these rows blocked because Ruby runtime evidence was unavailable at that time. RubyInstaller 3.4 is now installed locally at `C:\Ruby34-x64\bin\ruby.exe`, so the same explicit authorization, timeout, env allowlist, stdout/stderr, and JSON payload contract used by ScriptBridge can be proven for Ruby. 依据 PRD §Core Capabilities / §Compatibility Matrix、adapter §Constraints、ADR-005、ADR-009、ADR-011、task 9.1、task 35.1、Phase 35 §9。

## 2. Goal

Implement deterministic Ruby bridge behavior, promote only the two current-latest Ruby bridge rows to P0 native fixture evidence, reduce total current-latest blockers from 25 to 23, and keep config/provider external-authority and perfect-refactor blockers explicit.

## 3. Scope

### In Scope

- `src/script_bridge/mod.rs`
- `src/script_bridge/ruby.rs`
- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_ruby_bridge_burndown.rs`
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

- 不调用真实 provider、外部服务、private SDK、账号或 API key。
- 不解决 config external authority、provider external authority、current-target drift、publication authority 或“无任何潜在 bug”承诺。
- 不放宽 script bridge default-deny、timeout、env allowlist、stderr capture、secret redaction 或 subprocess authorization 边界。
- 不把本地 Ruby script evidence 伪装成 provider/account/SDK authority。

## 4. Users / Actors

- Script bridge maintainer: needs Ruby bridge behavior tested under explicit authorization.
- Release reviewer: needs Ruby rows promoted only when backed by real runtime tests and artifacts.
- Security reviewer: needs config/provider external authority blockers to remain visible after local script bridge blockers are removed.

## 5. Behavior Contract

Ruby bridge calls must execute only with `ScriptAuthorization::Allow`, serialize JSON payloads to stdin, parse JSON stdout into a stable value, preserve stderr, pass only allowlisted env values, enforce timeout/stdin limits through `ScriptBridge`, and return stable errors for unauthorized, timeout, subprocess failure, or invalid JSON output. Ruby worker-pool execution must preserve input ordering while bounding concurrent workers. Current-latest rows for `src/ruby/rubyUtils.ts` and `src/ruby/wrapper.ts` must classify as `level=P0`, `implementation_status=native`, `verification_owner=script-bridge`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. Rust and shell artifact generation must use equivalent rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-35-current-latest-script-prompt-python-bridge-burndown.md
- docs/specs/tasks/task-9.1-script-bridge-sandbox.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-35.1-current-latest-script-prompt-python-bridge-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/script-bridge.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::script_bridge::{RubyBridge, RubyBridgeRequest, RubyWorkerPool, ScriptAuthorization, ScriptBridgeErrorKind, ScriptSandboxOptions}`, `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::{json, Value}`, `std::path::{Path, PathBuf}`, `std::process::Command`, `std::time::Duration`.
- Tooling commands: `C:\Ruby34-x64\bin\ruby.exe --version` or `ruby --version`, `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `RubyBridge::call(request: RubyBridgeRequest, auth: ScriptAuthorization) -> Result<promptfoo_rs::script_bridge::RubyBridgeResponse, promptfoo_rs::script_bridge::ScriptBridgeError>`
- `RubyBridgeRequest::new(script_path: impl Into<PathBuf>, program: impl Into<PathBuf>, args: Vec<String>, payload: serde_json::Value, options: ScriptSandboxOptions) -> RubyBridgeRequest`
- `RubyWorkerPool::new(max_workers: usize) -> RubyWorkerPool`
- `RubyWorkerPool::execute(&self, requests: Vec<RubyBridgeRequest>, auth: ScriptAuthorization) -> Vec<Result<promptfoo_rs::script_bridge::RubyBridgeResponse, promptfoo_rs::script_bridge::ScriptBridgeError>>`
- `current_latest_ruby_script_bridge_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- Shell contract: `currentLatestRubyScriptBridgeFixtureIds(id)`, `isCurrentLatestRubyScriptBridgeFixture(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (ADR-005 / task 9.1): Ruby bridge calls require explicit authorization, exchange JSON stdin/stdout, capture stderr, and enforce env/timeout/stdin boundaries through ScriptBridge.
- [x] **AC2** (ADR-005 / task 9.1): Ruby worker-pool behavior executes authorized Ruby subprocess calls with stable JSON parsing, stderr capture, error propagation, and deterministic result ordering.
- [x] **AC3** (ADR-009 / ADR-011): two current-latest Ruby bridge rows have P0 native fixture evidence with `script-bridge` owner in both Rust and shell artifacts.
- [x] **AC4** (ADR-009 / ADR-011): current-latest script-bridge blockers drop to zero and total current-latest golden blockers drop from 25 to 23.
- [x] **AC5** (PRD §Compatibility Matrix): perfect-refactor completion remains false because remaining blockers are config/provider external-authority, publication, current-target, or impossible zero-bug claim boundaries.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-36.1.1 | TEST-36.1.1 | tests/current_latest_ruby_bridge_burndown.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-36.1.1 | TEST-36.1.2 | tests/current_latest_ruby_bridge_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-36.1.1 | TEST-36.1.3 | tests/current_latest_ruby_bridge_burndown.rs | install, lint, typecheck, unit-test, e2e, build | Done |
| AC4 | SCEN-36.1.1 | TEST-36.1.4 | tests/current_latest_ruby_bridge_burndown.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Done |
| AC5 | SCEN-36.1.1 | TEST-36.1.5 | tests/current_latest_ruby_bridge_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, runtime-smoke, build | Done |

## 8. Risks

- Ruby runtime availability can differ across machines; tests must fail closed without `ruby` or `C:\Ruby34-x64\bin\ruby.exe`.
- Ruby fixtures must remain local and deterministic; no fixture may require network, provider credentials, private SDKs, or account-level state.
- Removing script-bridge blockers does not prove config/provider external authority, publication, current-target readiness, or impossible zero-bug claims.

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

- **完成日期**：2026-06-03
- **改动文件**：
  - `docs/specs/phases/phase-36-current-latest-ruby-bridge-burndown.md`
  - `docs/specs/tasks/task-36.1-current-latest-ruby-bridge-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `src/script_bridge/mod.rs`
  - `src/script_bridge/ruby.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
  - `tests/current_latest_ruby_bridge_burndown.rs`
  - `tests/current_latest_script_prompt_python_bridge_burndown.rs`
  - `tests/current_latest_source_taxonomy_burndown.rs`
- **commit 列表**：
  - `8cbd370` `docs(spec): add phase 36 current latest ruby bridge burndown`
  - `df262b9` `docs(spec): task-36.1 enters implementation`
  - `c81c1e3` `test(script-bridge): add current latest Ruby bridge RED tests`
  - `c93c5e3` `feat(script-bridge): implement current latest Ruby bridge evidence`
  - 本次 docs 回填提交：`docs(spec): complete task 36.1 current latest ruby bridge burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-36.1.1 ~ TEST-36.1.5 与相关 current-latest 回归均通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - build: PASS - helper 执行 adapter Build 通过。
  - coverage: PASS - helper 执行 adapter Coverage，并通过 `s2v_coverage_threshold_guard`。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.status=ready-with-blockers`、`blocker_count=23`、分组 `config=7, provider=16`、`script-bridge=0`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：仍保留 config=7、provider external-authority=16、current-target、publication-authority 和“无任何潜在 bug”不可证明承诺边界。本 task 只证明本地 deterministic Ruby subprocess contract，不证明真实 provider 服务、账号级权限、private SDK、外部云配置或公开发布行为。
- **下游 task 影响**：current-latest 本地可证明的 script-bridge blockers 已清零；后续只能继续处理 config/provider external authority（需要真实权限、产品/服务合同证据或正式 waiver）以及 publication/current-target/claim wording gates。
