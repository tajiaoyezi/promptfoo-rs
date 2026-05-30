# Phase 12: compatibility-fixtures-golden-diff

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

建立至少 50 个 P0 compatibility fixtures，并让 upstream promptfoo 与 promptfoo-rs 在同一 fixture 上真实执行、归一化、diff、持久化 release gate artifact。依据 PRD §Compatibility Harness Design / §Success Metrics、ADR-006、ADR-007。

## 2. Business Value

把“本地 contract 测试绿”升级为“可复现 upstream-vs-rs 行为证据”，使 stable release gate 可以真实阻断不兼容变更。

## 3. Scope / Modules

compatibility/fixtures/、compatibility/artifacts/、src/compatibility/harness.rs、src/compatibility/diff.rs、src/compatibility/release_gate.rs、tests/compatibility_fixtures.rs、tests/golden_diff_e2e.rs

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 12.1 | p0-fixture-corpus | ../tasks/task-12.1-p0-fixture-corpus.md | Ready | 建立 50+ P0 fixtures 与 metadata schema |
| 12.2 | executable-upstream-rs-runner | ../tasks/task-12.2-executable-upstream-rs-runner.md | Ready | 真实执行 upstream promptfoo 与 promptfoo-rs 并产生 artifacts |
| 12.3 | golden-diff-ci-release-gate | ../tasks/task-12.3-golden-diff-ci-release-gate.md | Ready | 将 golden diff artifact 接入 CI/release gate |

## 5. Dependencies

依赖 Phase 11 inventory/matrix artifact。

## 6. Phase Acceptance Criteria

- [ ] 至少 50 个 P0 fixtures 进入版本控制，覆盖 CLI/config/eval/cache/output/P0 providers/core assertions/redteam core/script default-deny。
- [ ] 每个 fixture 生成 upstream、rs、normalized、diff、gate summary artifact。
- [ ] P0 bug 或 unclassified diff 会使 stable release gate 失败。

## 7. Phase Risks

- upstream Node runtime 可能在无网络环境下初始化缓慢；必须固定 npm artifact 和禁用 update check。
- 非确定性字段必须统一归一化，否则 diff 噪声会掩盖真实兼容问题。

## 8. Definition of Done

- Phase 12 smoke gate 运行 fixture corpus count、artifact persistence、P0 gate block/pass scenarios。
- docs/compatibility/matrix.md 每个 P0 row 都能追溯到至少一个 fixture 或明确 blocked reason。
