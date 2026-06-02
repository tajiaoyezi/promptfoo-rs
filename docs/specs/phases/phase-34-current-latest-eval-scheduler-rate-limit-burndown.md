# Phase 34: current-latest-eval-scheduler-rate-limit-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down the remaining current-latest `eval-runner` P0 blockers by implementing deterministic local scheduler rate-limit, adaptive concurrency, provider call context, and provider wrapper evidence. The seven `src/scheduler/*` current-latest rows become P0 native fixture evidence only after Rust behavior tests and Rust/shell artifact classification agree. 依据 PRD §Technical Approach / §Compatibility Matrix、ADR-009、ADR-011、task 3.1、task 3.2、task 13.2、task 29.1、Phase 33 §9。

## 2. Business Value

Phase 33 leaves 40 current-latest P0 golden blockers, including exactly seven `eval-runner` blockers for adaptive concurrency, provider rate-limit header parsing, provider rate-limit state/key/registry, provider call execution context, and provider wrapper behavior. Implementing a deterministic local contract removes these local eval-runner blockers without touching external provider credentials, config/cloud authority, script bridge subprocess parity, publication, or current-target decisions.

## 3. Scope / Modules

`src/eval/rate_limit.rs`, `src/eval/mod.rs`, `src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_eval_scheduler_rate_limit_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/s2v-adapter.md`, `docs/prds/promptfoo-rs.prd.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 34.1 | current-latest-eval-scheduler-rate-limit-burndown | ../tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md | Done | 实现 deterministic scheduler rate-limit/adaptive/provider-wrapper 证据并将 7 个 eval-runner blockers 转为 native fixture evidence |

## 5. Dependencies

Depends on Phase 24 current-latest artifacts, Phase 29 eval-runner blocker split, Phase 33 phase smoke, task 3.1 scheduler runtime, task 3.2 retry/backoff, task 13.2 eval output/cache parity, ADR-009, and ADR-011. This phase does not require real provider credentials, private services, legal/brand confirmation, or publication authority. Non-eval-runner blockers must remain visible.

## 6. Phase Acceptance Criteria

- [x] Provider rate-limit headers parse into deterministic remaining/reset state, and rate-limit keys are stable across provider/model/scope inputs.
- [x] Provider rate-limit registry returns deterministic delay decisions and records response headers without requiring real provider calls.
- [x] Adaptive concurrency adjusts concurrency deterministically for success, failure, and rate-limit observations while respecting configured min/max bounds.
- [x] Provider call execution context and wrapper behavior expose stable metadata, delay decisions, output, error, and observed header records.
- [x] The seven current-latest eval-runner scheduler rows have P0 native fixture evidence in both Rust and shell artifacts, eval-runner blockers drop to zero, total blockers drop from 40 to 33, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- Overfitting to synthetic headers could hide upstream rate-limit behavior. Tests must cover common remaining/reset header aliases and deterministic fallback behavior.
- A provider wrapper that performs real network calls would need credentials and external authority; this phase must keep evidence local and deterministic.
- Reducing eval-runner blockers does not prove config/provider external authority, script bridge runtime discovery, JS/Python/executable prompt processor parity, publication, current-target readiness, or impossible zero-bug claims.

## 8. Definition of Done

Task 34.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, `eval-runner=0`, total blockers are 33, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：2026-06-02
- **Phase smoke**：PASS - `s2v_preflight_phase "docs/specs/phases/phase-34-current-latest-eval-scheduler-rate-limit-burndown.md"` 通过，随后执行 `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 全套通过。
- **Artifact evidence**：`current-latest-golden-corpus.status=ready-with-blockers`、`blocker_count=33`、`eval-runner=0`，剩余分组为 `config=7, prompt-processing=3, provider=16, script-bridge=7`；`current-latest-quality.status=ready-with-blockers`、`local_current_latest_ready=false`、`perfect_refactor_claim_allowed=false`。
- **保留边界**：本 phase 只移除本地 deterministic eval scheduler rate-limit/adaptive/provider-wrapper blockers；不解除 config/provider external authority、script bridge runtime discovery、JS/Python/executable prompt processor parity、current-target、publication authority 或“无任何潜在 bug”不可证明承诺。
