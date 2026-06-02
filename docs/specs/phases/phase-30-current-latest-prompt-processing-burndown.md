# Phase 30: current-latest-prompt-processing-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down current-latest `prompt-processing` P0 golden blockers by applying already-proven config loader, file prompt, variable rendering, and prompt utility evidence to the locked current-latest inventory. Rows covered by deterministic local config/eval fixtures become P0 native fixture evidence; static/external prompt helper rows move to P1 snapshot evidence; unproven JSON, Markdown, Jinja, JavaScript, Python, and executable processors remain explicit P0 blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 2.2、task 2.3、task 4.2、task 9.1、Phase 29 §9。

## 2. Business Value

After Phase 29, phase-smoke artifacts report 60 current-latest P0 golden blockers, including 13 `prompt-processing` blockers. Reusing existing prompt config/eval fixtures prevents locally proven string/text prompt behavior from being double-counted as missing, while preserving rich prompt processor gaps that still need dedicated implementation and cross-runtime evidence.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_prompt_processing_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 30.1 | current-latest-prompt-processing-burndown | ../tasks/task-30.1-current-latest-prompt-processing-burndown.md | Done | 将 current-latest 13 个 prompt-processing blockers 拆为 4 个 fixture-covered rows、3 个 P1 snapshot rows 和 6 个保留 P0 blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target artifacts、Phase 25 taxonomy burndown、Phase 29 eval-runner burndown、task 2.2 config loader、task 2.3 eval command smoke、task 4.2 model-graded prompt evidence、task 9.1 script-bridge sandbox、ADR-009 和 ADR-011。该 phase 不需要真实 provider credentials 或 external service authority；未证明的 JSON/Markdown/Jinja/JS/Python/executable processors 必须继续阻塞。

## 6. Phase Acceptance Criteria

- [x] current-latest prompt index/string/text/utils rows already covered by local deterministic config/eval fixtures no longer appear as generic P0 prompt-processing blockers and carry `fixture:` evidence references.
- [x] current-latest prompt constants, grading helpers, and external Ragas prompt helper are recorded as P1 snapshot evidence rather than P0 native parity.
- [x] current-latest JSON, Markdown, Jinja, JavaScript, Python, and executable prompt processors remain explicit P0 prompt-processing blockers.
- [x] `current-latest-golden-corpus.json` prompt-processing blocker count drops from 13 to 6, total blocker count is 53 under the live Phase 29 smoke target and 52 under the deterministic tracked-lock phase smoke target, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- Broadly marking all `src/prompts/**` native would hide real processor behavior gaps. Classification must use explicit fixture, snapshot, and blocker allowlists.
- Jinja/JSON/Markdown processors are current-latest functionality, but existing evidence is not enough for P0 native parity; they must stay blockers until a dedicated task proves behavior.
- JavaScript/Python/executable prompt processors touch script execution boundaries and must not be converted to native evidence without explicit authorization, subprocess, timeout, env, and redaction tests.

## 8. Definition of Done

Task 30.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, fixture-covered prompt-processing rows stop generating P0 golden findings, unproven prompt processor rows remain visible, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：2026-06-02
- **Phase smoke**：PASS - `s2v_preflight_phase docs/specs/phases/phase-30-current-latest-prompt-processing-burndown.md` 通过；`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 全套通过。Phase smoke 过程中发现 live GitHub latest release API 403 会让 release target scripts 不稳定，已用 RED commit `c86d46f` 和 GREEN commit `af9f124` 将 runtime smoke 改为在未显式传 env 时从锁定 target 生成 fixture env。
- **Artifact evidence**：最终 phase smoke 使用 tracked current-latest target `1d09dfeb5f0766905409117f923dd5c4b0838d9f`；`current-latest-golden-corpus.json` 为 `ready-with-blockers`、`blocker_count=52`、分组 `cache-store=9, config=7, eval-runner=7, prompt-processing=6, provider=16, script-bridge=7`；`current-latest-matrix.json` 记录 prompt-processing `4 native fixture / 3 later snapshot / 6 blocked blocker`；`current-latest-quality.json` 为 `ready-with-blockers`、`local_current_latest_ready=false`、`perfect_refactor_claim_allowed=false`。task 30.1 §9 live target 观测为 `96e556507e4bbee5110d94286d500c4605ccc38b` / `blocker_count=53`，差异来自 moving current-latest provider external-authority 行数 16 vs 17。
- **保留边界**：Phase 30 只消减 prompt-processing generic blockers，不处理 JSON/Markdown/Jinja/JS/Python/executable processor native parity、cache-store、config external、eval-runner adaptive/rate-limit、provider external-authority、script-bridge runtime discovery、current-target drift、publication authority 或“无任何潜在 bug”承诺。
