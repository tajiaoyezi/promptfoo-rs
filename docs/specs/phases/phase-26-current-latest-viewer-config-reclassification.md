# Phase 26: current-latest-viewer-config-reclassification

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Correct the current-latest source inventory taxonomy so `src/app/**` viewer configuration files no longer create duplicate P0 `config` blockers while their viewer evidence rows remain tracked. This mirrors the frozen-baseline correction from task 19.1 for the current-latest target. 依据 PRD §Compatibility Matrix / §Current Latest Rebaseline Addendum、Phase 25 §9 artifact evidence、ADR-009、ADR-011、task 19.1。

## 2. Business Value

Phase 25 proved all current-latest rows are classified, but `current-latest-golden-corpus.json` still includes config blockers that are actually upstream React viewer source files. Reclassifying those rows reduces false P0 blocker noise and lets follow-up work focus on real core config fixtures, provider fixtures, script bridge fixtures, external authority, and publication blockers.

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`scripts/release/current-latest-source-inventory.sh`、`tests/current_latest_viewer_config_reclassification.rs`、`target/release-gates/current-latest-source-inventory.json`、`target/release-gates/current-latest-matrix.json`、`target/release-gates/current-latest-golden-corpus.json`、`target/release-gates/current-latest-quality.json`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 26.1 | current-latest-viewer-config-reclassification | ../tasks/task-26.1-current-latest-viewer-config-reclassification.md | Done | 将 current-latest `src/app/**` config duplicate P0 blockers 纠正为 viewer evidence，同时保留 non-app core config P0 blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target lock/source inventory/golden corpus/quality gate 和 Phase 25 taxonomy burndown。该 phase 不需要真实 provider credentials、账号、私有服务、法律/品牌确认或 publication credentials；这些仍由 external/publication gates 阻塞。

## 6. Phase Acceptance Criteria

- [x] `target/release-gates/current-latest-source-inventory.json` 中 `source_file` 匹配 `src/app/**` 且文件名/路径包含 config 的行不再生成 `category=config` / `level=P0` / `evidence_kind=blocker`。
- [x] 同一批 `src/app/**` viewer config source rows 仍以 `category=viewer`、P1/snapshot evidence 或 equivalent viewer evidence 被登记，row 不被删除。
- [x] `src/util/config/**`、`src/commands/config.ts`、`src/globalConfig/**`、`src/configTypes.ts`、`src/server/config/**` 等 non-app config rows 继续保持 P0 config fixture/blocker 语义。
- [x] `current-latest-golden-corpus.json` 和 `current-latest-quality.json` 的 perfect-refactor claim 仍为 false，剩余 blockers 以真实 P0 fixture、external authority、current-target 或 publication authority 形式暴露。

## 7. Phase Risks

- 过宽排除规则会错误降低 core config、server config 或 command config 的 P0 兼容要求。规则必须限定 `src/app/**`。
- 该 phase 只纠正 viewer source accounting，不声明 upstream React UI 像素 parity。
- Blocker 数下降不代表“完美重构完成”；task 必须保留 quality gate 的 claim boundary。

## 8. Definition of Done

Task 26.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, app viewer config duplicate P0 blockers are removed, non-app config blockers remain, and perfect-refactor claim remains blocked until all current-latest local/external/publication evidence is complete or waived.

## 9. Phase Completion Notes

- **完成日期**：2026-06-02
- **Phase smoke**：PASS — `s2v_verify_full "install lint typecheck unit-test integration e2e build coverage runtime-smoke"` 全 9 项通过；真实 artifact 复核完成。
- **Artifact evidence**：
  - `target/release-gates/current-latest-source-inventory.json`：status=`ready`，rows=3858，`src/app/**` config rows=0，`src/app/**` viewer config rows=58，non-app config rows=18。
  - `target/release-gates/current-latest-matrix.json`：status=`ready`，rows=3858，`src/app/**` config rows=0。
  - `target/release-gates/current-latest-golden-corpus.json`：status=`ready-with-blockers`，blocker_count=103，`config:src-app*` blockers=0，remaining P0 groups provider=38、config=18、eval-runner=18、prompt-processing=13、cache-store=9、script-bridge=7。
  - `target/release-gates/current-latest-quality.json`：status=`ready-with-blockers`，local_current_latest_ready=false，perfect_refactor_claim_allowed=false；剩余 blockers 为 golden-corpus、current-target、external-authority、publication-authority。
- **保留边界**：Phase 26 完成的是 current-latest viewer config scope correction，不等于 current-latest perfect-refactor 完成。后续仍需消除 103 个 P0 golden blockers，或取得/正式 waiver external authority、publication authority 和 current-target claim evidence。
