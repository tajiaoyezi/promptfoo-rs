# Phase 27: current-latest-core-config-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down the 18 remaining current-latest non-app `config` P0 golden blockers by applying the task 19.2 decision model to the current-latest inventory: local runtime config rows get fixture evidence, auxiliary command/scan config rows move to P1 snapshot evidence, and cloud/server/telemetry/global authority rows remain explicit external blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、Phase 26 §9 artifact evidence、task 19.2、ADR-009、ADR-011。

## 2. Business Value

After Phase 26, false viewer config blockers are gone, but release artifacts still report 18 generic current-latest config blockers. Splitting those rows prevents the project from treating already-covered promptfooconfig/env/file behavior as missing, while preserving external authority blockers that cannot be solved by local code.

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`scripts/release/current-latest-source-inventory.sh`、`tests/current_latest_core_config_burndown.rs`、`target/release-gates/current-latest-source-inventory.json`、`target/release-gates/current-latest-matrix.json`、`target/release-gates/current-latest-golden-corpus.json`、`target/release-gates/current-latest-quality.json`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 27.1 | current-latest-core-config-burndown | ../tasks/task-27.1-current-latest-core-config-burndown.md | Done | 将 current-latest 18 个 non-app config blockers 分解为 8 个 local fixture-covered rows、3 个 P1 auxiliary rows、7 个 explicit external blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target artifacts、Phase 25 taxonomy burndown、Phase 26 viewer config reclassification，以及 task 19.2 的 frozen-baseline config decision precedent。该 phase 不需要真实 provider credentials、私有服务账号、法律/品牌确认或 publication credentials；external rows 必须继续阻塞。

## 6. Phase Acceptance Criteria

- [x] current-latest runtime config rows（`src/commands/config.ts`、`src/configTypes.ts`、`src/util/config/**`、redteam `promptfooconfig.yaml`）不再是 generic P0 blockers，并具备 fixture evidence reference。
- [x] current-latest auxiliary config rows（code scan config、MCP config validation）登记为 P1 snapshot evidence，不计入 P0 golden blockers。
- [x] current-latest cloud/server/telemetry/global config rows 保持 explicit external P0 blockers，不伪造成 local parity。
- [x] `current-latest-golden-corpus.json` 的 config blocker count 从 18 降到 external-only 7，`current-latest-quality.json` 仍保持 perfect_refactor_claim_allowed=false。

## 7. Phase Risks

- 过宽 native fixture 规则会把 cloud/server/telemetry authority gaps 伪装成本地完成。分类必须逐路径白名单。
- Evidence reference 不能继续按 `category=config` 硬编码 `blocker:`；否则 metadata 变更不会真正燃尽 golden blockers。
- 该 phase 只处理 config 类 blocker，不解决 provider/eval/cache/prompt/script bridge blockers。

## 8. Definition of Done

Task 27.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, config blockers drop to explicit external-only rows, and perfect-refactor claim remains blocked by remaining local/external/publication/current-target evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：2026-06-02
- **Phase smoke**：PASS — `s2v_verify_full "install lint typecheck unit-test integration e2e build coverage runtime-smoke"` 全 9 项通过；真实 artifact 复核完成。
- **Artifact evidence**：
  - `target/release-gates/current-latest-source-inventory.json`：status=`ready`，rows=3858，config fixture=8，config auxiliary=3，config external=7。
  - `target/release-gates/current-latest-matrix.json`：status=`ready`，rows=3858，config fixture=8，config auxiliary=3，config external=7。
  - `target/release-gates/current-latest-golden-corpus.json`：status=`ready-with-blockers`，blocker_count=92，config_blockers=7，remaining P0 groups provider=38、eval-runner=18、prompt-processing=13、cache-store=9、config=7、script-bridge=7。
  - `target/release-gates/current-latest-quality.json`：status=`ready-with-blockers`，local_current_latest_ready=false，perfect_refactor_claim_allowed=false；剩余 blockers 为 golden-corpus、current-target、external-authority、publication-authority。
- **保留边界**：Phase 27 完成的是 current-latest core config generic blocker burndown，不等于 current-latest perfect-refactor 完成。后续仍需消除 92 个 P0 golden blockers，或取得/正式 waiver external authority、publication authority 和 current-target claim evidence。
