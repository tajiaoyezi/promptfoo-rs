# Phase 29: current-latest-eval-runner-burndown

**Status**: In Progress
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down current-latest `eval-runner` P0 golden blockers by applying existing eval command, output/cache, retry, and scheduler fixture evidence to the locked current-latest inventory. Rows already covered by deterministic eval/scheduler fixtures become P0 native fixture evidence; optimizer, event, and test synthesis rows move to P1 snapshot evidence; unproven adaptive/rate-limit/provider-wrapper rows remain explicit P0 blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 3.1、task 3.2、task 13.2、Phase 28 §9。

## 2. Business Value

After Phase 28, release artifacts still report 18 current-latest eval-runner blockers. Reusing existing eval, scheduler, retry, and output fixtures prevents locally proven runner behavior from being double-counted as missing, while preserving rate-limit/adaptive/provider-wrapper gaps that still need dedicated current-latest evidence.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_eval_runner_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 29.1 | current-latest-eval-runner-burndown | ../tasks/task-29.1-current-latest-eval-runner-burndown.md | Done | 将 current-latest 18 个 eval-runner blockers 拆为 8 个 fixture-covered rows、3 个 P1 snapshot rows 和 7 个保留 P0 blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target artifacts、Phase 25 taxonomy burndown、Phase 28 provider burndown、task 3.1 scheduler runtime、task 3.2 cache/retry precedent、task 13.2 eval-output-cache parity、ADR-009 和 ADR-011。该 phase 不需要真实 provider credentials 或 external service authority；未证明的 adaptive/rate-limit rows 必须继续阻塞。

## 6. Phase Acceptance Criteria

- [ ] current-latest eval/evaluator/scheduler rows already covered by local deterministic fixtures no longer appear as generic P0 eval-runner blockers and carry `fixture:` evidence references.
- [ ] current-latest optimizer, scheduler event, and test synthesis rows are recorded as P1 snapshot evidence rather than P0 native parity.
- [ ] current-latest adaptive concurrency, rate-limit/header parsing, and provider wrapper rows remain explicit P0 eval-runner blockers.
- [ ] `current-latest-golden-corpus.json` eval-runner blocker count drops from 18 to 7, total blocker count drops from 70 to 59, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- Broadly marking all `src/scheduler/**` native would hide rate-limit/header parsing/provider-wrapper gaps. Classification must use explicit fixture and blocker allowlists.
- Optimizer and synthesis rows are current-latest functionality, but existing evidence is not enough for P0 native parity; they must stay P1 snapshot/later until a dedicated task proves behavior.
- This phase reduces eval-runner generic blockers only; it does not prove prompt-processing, cache-store, script-bridge, provider external authority, publication, or current-target readiness.

## 8. Definition of Done

Task 29.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, eval-runner fixture-covered rows stop generating P0 golden findings, unproven eval-runner rows remain visible, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：待实施
- **Phase smoke**：待实施
- **Artifact evidence**：待实施
- **保留边界**：待实施
