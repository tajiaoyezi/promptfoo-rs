# Task 34.1: current-latest-eval-scheduler-rate-limit-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 34 - current-latest-eval-scheduler-rate-limit-burndown
**Dependencies**: task-3.1-scheduler-runtime, task-3.2-cache-resume-retry, task-13.2-eval-output-cache-parity, task-24.4-current-latest-exhaustive-quality-gate, task-29.1-current-latest-eval-runner-burndown, task-33.1-current-latest-eval-deletion-burndown

## 1. Background

Phase 33 leaves tracked-lock phase-smoke artifacts at 40 P0 golden blockers, including exactly seven `eval-runner` blockers: `adaptiveConcurrency`, `headerParser`, `providerCallExecutionContext`, `providerRateLimitState`, `providerWrapper`, `rateLimitKey`, and `rateLimitRegistry`. Task 29.1 intentionally kept these rows blocked because task 3.1/3.2/13.2 did not prove dedicated current-latest scheduler rate-limit semantics. This task adds that local deterministic evidence without making real provider calls or weakening external authority boundaries. 依据 PRD §Eval runner / §Compatibility Matrix、ADR-009、ADR-011、task 3.1、task 3.2、task 13.2、task 29.1、Phase 33 §9。

## 2. Goal

Implement deterministic local scheduler rate-limit/adaptive/provider-wrapper behavior and promote only the seven current-latest eval-runner scheduler rows to P0 native fixture evidence, reducing eval-runner blockers from 7 to 0 and total blockers from 40 to 33.

## 3. Scope

### In Scope

- `src/eval/rate_limit.rs`
- `src/eval/mod.rs`
- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_eval_scheduler_rate_limit_burndown.rs`
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
- 不实现 external provider product authority、stateful Assistant/Realtime/Agents/ChatKit/Codex/Billing behavior。
- 不解决 config external authority、script bridge runtime discovery、JS/Python/executable prompt processor parity、current-target drift、publication authority 或“无任何潜在 bug”承诺。
- 不把 provider wrapper local mock evidence 伪装成 live provider parity；本 task 只证明 scheduler/wrapper contract。

## 4. Users / Actors

- Eval maintainer: needs deterministic scheduler rate-limit semantics that can be tested without provider credentials.
- Release reviewer: needs all seven eval-runner blocker rows promoted only when backed by item-level tests and artifacts.
- CI maintainer: needs current-latest golden and quality artifacts to drop eval-runner blockers without hiding remaining external/script blockers.

## 5. Behavior Contract

Provider rate-limit headers must parse common remaining/reset aliases into deterministic state. Rate-limit keys must be stable and scope provider/model/channel inputs. A registry must return deterministic delay decisions before a call and update state after response headers. Adaptive concurrency must increase after successes, decrease after failures, and sharply reduce on rate-limit observations while respecting configured min/max. Provider call execution context and wrapper behavior must expose stable metadata, pre-call delay, output/error, and recorded headers using local closures only. Current-latest rows for adaptive concurrency, header parser, provider call execution context, provider rate-limit state, provider wrapper, rate-limit key, and rate-limit registry must classify as `level=P0`, `implementation_status=native`, `verification_owner=eval-runner`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. Rust and shell artifact generation must use equivalent rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-29-current-latest-eval-runner-burndown.md
- docs/specs/phases/phase-33-current-latest-eval-deletion-burndown.md
- docs/specs/tasks/task-3.1-scheduler-runtime.md
- docs/specs/tasks/task-3.2-cache-resume-retry.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
- docs/specs/tasks/task-33.1-current-latest-eval-deletion-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/eval-runner.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::eval::rate_limit::{AdaptiveConcurrencyController, AdaptiveObservation, ProviderCallExecutionContext, ProviderCallOutcome, ProviderRateLimitRegistry, RateLimitDecision, RateLimitHeaderState, RateLimitKey, WrappedProviderCallResult}`, `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::collections::BTreeMap`, `std::path::Path`, `std::process::Command`, `std::time::Duration`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `RateLimitHeaderState::parse(headers: &BTreeMap<String, String>) -> RateLimitHeaderState`
- `RateLimitKey::new(provider_id: impl Into<String>, model: impl Into<String>, scope: impl Into<String>) -> RateLimitKey`
- `ProviderRateLimitRegistry::delay_for(&self, key: &RateLimitKey) -> RateLimitDecision`
- `ProviderRateLimitRegistry::record_headers(&mut self, key: RateLimitKey, headers: &BTreeMap<String, String>)`
- `AdaptiveConcurrencyController::observe(&mut self, observation: AdaptiveObservation) -> usize`
- `ProviderCallExecutionContext::new(provider_id: impl Into<String>, model: impl Into<String>, scope: impl Into<String>, attempt: usize) -> ProviderCallExecutionContext`
- `ProviderRateLimitRegistry::call_with_policy<F>(&mut self, context: ProviderCallExecutionContext, call: F) -> WrappedProviderCallResult where F: FnOnce(&ProviderCallExecutionContext) -> ProviderCallOutcome`
- `current_latest_eval_runner_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- Shell contract: `currentLatestEvalRunnerFixtureIds(id)`, `isCurrentLatestEvalRunnerFixture(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Eval runner / task 3.1): provider rate-limit header parsing and rate-limit key derivation are deterministic and fixture-tested.
- [x] **AC2** (PRD §Eval runner / task 3.2): provider rate-limit registry records headers and returns deterministic delay decisions without real provider calls.
- [x] **AC3** (PRD §Eval runner): adaptive concurrency responds deterministically to success, failure, and rate-limit observations within configured bounds.
- [x] **AC4** (PRD §Eval runner / ADR-009): provider call execution context and wrapper expose stable metadata, delay, output/error, and header recording behavior through local fixtures.
- [x] **AC5** (ADR-009 / ADR-011): seven current-latest eval-runner scheduler rows have P0 native fixture evidence, Rust and shell extractors emit equivalent classification, total blockers drop to 33, and perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-34.1.1 | TEST-34.1.1 | tests/current_latest_eval_scheduler_rate_limit_burndown.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-34.1.1 | TEST-34.1.2 | tests/current_latest_eval_scheduler_rate_limit_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-34.1.1 | TEST-34.1.3 | tests/current_latest_eval_scheduler_rate_limit_burndown.rs | install, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-34.1.1 | TEST-34.1.4 | tests/current_latest_eval_scheduler_rate_limit_burndown.rs | install, lint, typecheck, unit-test, e2e, build | Done |
| AC5 | SCEN-34.1.1 | TEST-34.1.5 | tests/current_latest_eval_scheduler_rate_limit_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, coverage, runtime-smoke, build | Done |

## 8. Risks

- Header aliases differ across providers; this task must parse common promptfoo scheduler aliases and leave unrecognized headers as non-delaying state instead of panicking.
- Provider wrapper evidence must stay local and deterministic; no real API calls, private SDKs, or credentials are allowed.
- Marking all `src/scheduler/**` native would hide optimizer/event/synthesis or future scheduler gaps; this task may promote only the seven named blocked rows.
- This task removes only eval-runner scheduler blockers and does not affect external authority, provider, config, script bridge, prompt processor, publication, current-target, or impossible zero-bug claim blockers.

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
  - `docs/specs/phases/phase-34-current-latest-eval-scheduler-rate-limit-burndown.md`
  - `docs/specs/tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `src/eval/rate_limit.rs`
  - `src/eval/mod.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
  - `tests/current_latest_eval_scheduler_rate_limit_burndown.rs`
  - `tests/current_latest_eval_runner_burndown.rs`
  - `tests/current_latest_source_taxonomy_burndown.rs`
- **commit 列表**：
  - `a7ce91a` `docs(spec): add phase 34 current latest eval scheduler rate limit burndown`
  - `fb775ab` `docs(spec): task-34.1 enters implementation`
  - `cfa1aee` `test(eval-runner): add current latest scheduler rate limit RED tests`
  - `7e87452` `feat(eval-runner): implement current latest scheduler rate limit evidence`
  - 本次 docs 回填提交：`docs(spec): complete task 34.1 current latest eval scheduler rate limit burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-34.1.1 ~ TEST-34.1.5、累计 TEST-29.1.1 ~ TEST-29.1.5 和 TEST-25.1.2 更新后均通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - build: PASS - helper 执行 adapter Build 通过。
  - coverage: PASS - helper 执行 adapter Coverage，通过覆盖率阈值守卫。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.json` 为 `ready-with-blockers`、`p0_total=92`、`fixture_case_count=92`、`blocker_count=33`、分组 `config=7, prompt-processing=3, provider=16, script-bridge=7`，`eval-runner=0`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：仍保留 config=7、prompt-processing=3、provider external-authority=16、script-bridge=7、current-target drift、external-authority、publication-authority 等 blockers；不承诺“无任何潜在 bug”。本 task 只证明本地 deterministic scheduler rate-limit/adaptive/provider-wrapper contract，不证明真实 provider 服务限流、账号级 quota 或 private SDK 行为。
- **下游 task 影响**：后续 current-latest burndown 可从 33 个总 blockers 继续推进；script-backed prompt processors 与 Python/Ruby bridge rows 仍需专用 subprocess/runtime discovery fixture，config/provider external authority rows 仍需真实权限或正式 waiver。
