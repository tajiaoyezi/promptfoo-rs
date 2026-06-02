# Phase 32: current-latest-local-prompt-processor-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down the local current-latest prompt processor blockers that can be proven without script execution authority. JSON, Markdown, and Jinja prompt processor rows become P0 native fixture evidence; JavaScript, Python, and executable prompt processors remain explicit P0 script-bridge blockers until subprocess/script execution parity is proven. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 2.2、task 2.3、task 9.1、task 30.1、Phase 31 §9。

## 2. Business Value

Phase 31 leaves 44 current-latest P0 golden blockers, including 6 prompt-processing blockers. Three of those rows are local parser/template surfaces that can be covered by deterministic config/prompt fixtures; the other three require authorized script/subprocess runtime behavior. This phase reduces locally provable prompt-processing gaps without hiding script bridge risks.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_local_prompt_processor_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/s2v-adapter.md`, `docs/prds/promptfoo-rs.prd.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 32.1 | current-latest-local-prompt-processor-burndown | ../tasks/task-32.1-current-latest-local-prompt-processor-burndown.md | Done | 将 6 个 prompt-processing blockers 拆为 3 个 local fixture rows 和 3 个保留 script-bridge blockers |

## 5. Dependencies

Depends on Phase 24 current-latest artifacts, Phase 30 prompt-processing split, Phase 31 cache-store phase smoke, task 2.2 config loader, task 2.3 eval command prompt rendering, task 9.1 script bridge boundary, ADR-009, and ADR-011. This phase does not require real provider credentials, private services, legal/brand confirmation, or publication authority. Script-backed JS/Python/executable processor rows must remain blocked unless dedicated script bridge parity evidence exists.

## 6. Phase Acceptance Criteria

- [x] JSON, Markdown, and Jinja current-latest prompt processor rows have P0 native fixture evidence and stop producing prompt-processing golden blockers.
- [x] JavaScript, Python, and executable current-latest prompt processor rows remain explicit P0 script-bridge blockers.
- [x] Rust and shell current-latest artifact generation emit equivalent prompt processor classification.
- [x] `current-latest-golden-corpus.json` prompt-processing blocker count drops from 6 to 3, total blocker count drops from 44 to 41 under the tracked-lock phase smoke target, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- Marking JS/Python/executable processors native would hide script/subprocess authorization, timeout, env allowlist, and redaction gaps.
- Local JSON/Markdown/Jinja fixture evidence is prompt parsing evidence only; it does not prove rich script bridge or provider behavior.
- Reducing prompt-processing blockers does not prove eval deletion, provider external authority, script bridge runtime discovery, config external authority, eval-runner adaptive/rate-limit behavior, publication, or impossible zero-bug claims.

## 8. Definition of Done

Task 32.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, local prompt processor rows stop generating P0 golden findings, script-backed prompt processor rows remain visible blockers, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：2026-06-02
- **Phase smoke**：PASS - `s2v_preflight_phase "docs/specs/phases/phase-32-current-latest-local-prompt-processor-burndown.md"` 通过，随后执行 `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 全套通过。
- **Artifact evidence**：`current-latest-golden-corpus.status=ready-with-blockers`、`blocker_count=41`、`prompt-processing=3`，总分组为 `cache-store=1, config=7, eval-runner=7, prompt-processing=3, provider=16, script-bridge=7`；`current-latest-quality.status=ready-with-blockers`、`local_current_latest_ready=false`、`perfect_refactor_claim_allowed=false`。
- **保留边界**：JS/Python/executable prompt processors 仍是 P0 script-bridge blockers；本 phase 不解除 cache deletion、external provider/config authority、eval-runner adaptive/rate-limit、current-target、publication authority 或“无任何潜在 bug”不可证明承诺。
